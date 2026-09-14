use std::{future::Future, path::PathBuf, time::Instant};

use anyhow::Context;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::time;
use tracing::{info, warn};
use witness_generator_spec_cli::{
    BlockSelector, GeneratedInput, NetworkWitnessClient, NetworkWitnessConfig,
};

use crate::{
    artifact::{
        self, ArtifactWriteResult, StatelessInputArtifact, append_index_entry, write_json_atomic,
    },
    config::CollectorConfig,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PersistedArtifact {
    pub(crate) artifact: StatelessInputArtifact,
    pub(crate) write: ArtifactWriteResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CollectorState {
    last_head_hash: String,
    last_block_number: u64,
    last_slot_number: u64,
    updated_at: String,
}

/// Collects every block from the chain tip at start-up onward, in block order, so no height is
/// skipped while the collector runs.
pub(crate) async fn collect(config: CollectorConfig, once: bool) -> anyhow::Result<()> {
    let mut network_config =
        NetworkWitnessConfig::new(config.cl_url.clone(), config.el_url.clone());
    network_config.timeout = config.request_timeout;
    network_config.cl_headers = config.cl_headers.clone();
    network_config.el_headers = config.el_headers.clone();
    let client = NetworkWitnessClient::new(network_config)?;

    let mut next_block_number = client
        .latest_block_number()
        .await
        .context("failed to resolve the chain tip to start collecting from")?;
    info!(block_number = next_block_number, "collecting from block");

    loop {
        let start = Instant::now();
        match collect_to_tip(&client, &config, next_block_number).await {
            Ok(next) => next_block_number = next,
            Err(error) => warn!(?error, "failed to resolve the chain tip"),
        }
        if once {
            return Ok(());
        }
        time::sleep(config.poll_interval.saturating_sub(start.elapsed())).await;
    }
}

/// Collects every block from `from` up to the chain tip and returns the next block number to
/// collect.
async fn collect_to_tip(
    client: &NetworkWitnessClient,
    config: &CollectorConfig,
    from: u64,
) -> anyhow::Result<u64> {
    let latest = client.latest_block_number().await?;

    Ok(collect_in_order(
        from,
        latest,
        config.max_concurrency,
        |block_number| {
            client.stateless_input_bytes(BlockSelector::ExecutionBlockNumber(block_number))
        },
        |generated| {
            let persisted = collect_generated(config, generated)?;
            info!(
                block_number = persisted.artifact.block_number,
                block_hash = persisted.artifact.block_hash,
                path = %persisted.write.path.display(),
                "collected stateless EEST fixture",
            );
            Ok(())
        },
    )
    .await)
}

/// Fetches `from..=latest` with at most `max_concurrency` fetches in flight, persists each result
/// in block order, and returns the next uncollected block number.
///
/// Fetches run concurrently so collection keeps up when a single block takes longer than the block
/// time. They complete out of order, so `persist` is driven strictly in block order and stops at
/// the first failure. The recorded position never runs ahead of a block that was not persisted,
/// and a failed block is retried on the next round instead of skipped.
async fn collect_in_order<T, Fut>(
    from: u64,
    latest: u64,
    max_concurrency: usize,
    fetch: impl Fn(u64) -> Fut,
    mut persist: impl FnMut(T) -> anyhow::Result<()>,
) -> u64
where
    Fut: Future<Output = anyhow::Result<T>>,
{
    let mut blocks = futures::stream::iter(from..=latest)
        .map(|block_number| {
            let fetched = fetch(block_number);
            async move { (block_number, fetched.await) }
        })
        .buffered(max_concurrency);

    let mut next = from;
    while let Some((block_number, fetched)) = blocks.next().await {
        if let Err(error) = fetched.and_then(&mut persist) {
            warn!(
                block_number,
                ?error,
                "failed to collect block, retrying it next round",
            );
            break;
        }
        next = block_number + 1;
    }

    next
}

pub(crate) fn collect_generated(
    config: &CollectorConfig,
    generated: GeneratedInput,
) -> anyhow::Result<PersistedArtifact> {
    let artifact = StatelessInputArtifact::from_generated(&config.network, "head", &generated)?;
    let write = artifact::write_artifact_atomic(&config.blocks_root(), &artifact)?;
    if write.created {
        let index_entry = artifact.index_entry(&PathBuf::from("blocks").join(&write.relative_path));
        append_index_entry(&config.index_path(), &index_entry)?;
    }
    write_state(config, &artifact)?;

    Ok(PersistedArtifact { artifact, write })
}

fn write_state(config: &CollectorConfig, artifact: &StatelessInputArtifact) -> anyhow::Result<()> {
    let state = CollectorState {
        last_head_hash: artifact.block_hash.clone(),
        last_block_number: artifact.block_number,
        last_slot_number: artifact.slot_number,
        updated_at: artifact::utc_now_rfc3339()?,
    };
    write_json_atomic(&config.state_path(), &state)
}

#[cfg(test)]
mod tests {
    use std::{fs, sync::Mutex, time::Duration};

    use alloy_primitives::B256;

    use crate::artifact::test_generated_input;

    use super::*;

    #[test]
    fn collect_generated_writes_the_same_block_once() {
        let config = test_config("idempotent");
        let generated = generated_input(42, B256::repeat_byte(0xaa));

        let first = collect_generated(&config, generated.clone()).unwrap();
        let second = collect_generated(&config, generated).unwrap();

        assert!(first.write.created);
        assert!(!second.write.created);
        assert_eq!(
            fs::read_to_string(config.index_path())
                .unwrap()
                .lines()
                .count(),
            1
        );
    }

    #[test]
    fn collect_generated_preserves_reorg_variants() {
        let config = test_config("reorg");
        let first =
            collect_generated(&config, generated_input(42, B256::repeat_byte(0xaa))).unwrap();
        let second =
            collect_generated(&config, generated_input(42, B256::repeat_byte(0xbb))).unwrap();

        assert_ne!(first.write.path, second.write.path);
        assert!(first.write.path.exists());
        assert!(second.write.path.exists());
    }

    #[tokio::test]
    async fn collect_in_order_persists_in_block_order_despite_concurrent_fetches() {
        // Later blocks are fetched first, so persisting on fetch completion would invert the order.
        let fetched = Mutex::new(Vec::new());
        let persisted = Mutex::new(Vec::new());

        let next = collect_in_order(
            10,
            17,
            4,
            |block_number| {
                let fetched = &fetched;
                async move {
                    time::sleep(Duration::from_millis(10 * (17 - block_number))).await;
                    fetched.lock().unwrap().push(block_number);
                    Ok(block_number)
                }
            },
            |block_number| {
                persisted.lock().unwrap().push(block_number);
                Ok(())
            },
        )
        .await;

        assert_eq!(next, 18);
        assert_eq!(*persisted.lock().unwrap(), (10..=17).collect::<Vec<_>>());
        assert_ne!(*fetched.lock().unwrap(), (10..=17).collect::<Vec<_>>());
    }

    #[tokio::test]
    async fn collect_in_order_stops_at_the_first_failure_without_leaving_a_gap() {
        let persisted = Mutex::new(Vec::new());

        let next = collect_in_order(
            10,
            17,
            4,
            |block_number| async move {
                if block_number == 13 {
                    anyhow::bail!("witness unavailable");
                }
                Ok(block_number)
            },
            |block_number| {
                persisted.lock().unwrap().push(block_number);
                Ok(())
            },
        )
        .await;

        assert_eq!(next, 13);
        assert_eq!(*persisted.lock().unwrap(), vec![10, 11, 12]);
    }

    fn test_config(name: &str) -> CollectorConfig {
        let out_root = std::env::temp_dir().join(format!(
            "witness-generator-spec-cli-collector-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&out_root);
        CollectorConfig {
            network: "glamsterdam-devnet-8".to_owned(),
            cl_url: "http://cl".to_owned(),
            el_url: "http://el".to_owned(),
            cl_headers: Vec::new(),
            el_headers: Vec::new(),
            out_root,
            poll_interval: std::time::Duration::from_secs(4),
            request_timeout: std::time::Duration::from_secs(30),
            batch_size: 500,
            zstd_window_log: None,
            max_concurrency: 4,
            r2: None,
        }
    }

    fn generated_input(block_number: u64, block_hash: B256) -> GeneratedInput {
        test_generated_input(block_number, block_hash)
    }
}
