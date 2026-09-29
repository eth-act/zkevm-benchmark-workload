use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashSet},
    path::Path,
};
use tracing::{info, warn};

const EEST_SAFE_FILE_STEM_MAX_LEN: usize = 220;

#[derive(Debug, Clone)]
pub(crate) struct EestStatelessFixture {
    pub(crate) name: String,
    pub(crate) original_test_name: String,
    pub(crate) source_path: String,
    pub(crate) block_index: usize,
    pub(crate) network: String,
    pub(crate) chain_id: u64,
    pub(crate) block_number: Option<u64>,
    pub(crate) block_used_gas: Option<u64>,
    pub(crate) opcode_count: Option<BTreeMap<String, u64>>,
    pub(crate) target_opcode: Option<String>,
    pub(crate) stateless_input_bytes: Vec<u8>,
    pub(crate) stateless_output_bytes: Vec<u8>,
}

/// EEST `blockchain_test_engine` test case.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EestBlockchainTest {
    network: String,
    config: EestConfig,
    engine_new_payloads: Vec<EestEngineNewPayload>,
    #[serde(default, rename = "_info")]
    info: EestInfo,
}

#[derive(Debug, Deserialize)]
struct EestConfig {
    chainid: String,
}

#[derive(Debug, Default, Deserialize)]
struct EestInfo {
    #[serde(default)]
    metadata: EestMetadata,
}

/// EEST writes `_info.metadata` keys in snake case.
#[derive(Debug, Default, Deserialize)]
struct EestMetadata {
    #[serde(default)]
    opcode_count_per_block: Option<Vec<BTreeMap<String, u64>>>,
    #[serde(default)]
    target_opcode: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EestEngineNewPayload {
    #[serde(default)]
    stateless_input_bytes: Option<String>,
    #[serde(default)]
    stateless_output_bytes: Option<String>,
    /// `engine_newPayload` params. Only `params[0]`, the execution payload, is read.
    #[serde(default)]
    params: Vec<serde_json::Value>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EestExecutionPayload {
    #[serde(default)]
    block_number: Option<String>,
    #[serde(default)]
    gas_used: Option<String>,
}

pub(crate) fn load_eest_benchmark_fixtures(
    value: serde_json::Value,
    path: &Path,
    input_root: &Path,
) -> Result<Vec<EestStatelessFixture>> {
    let cases: BTreeMap<String, EestBlockchainTest> =
        serde_json::from_value(value).with_context(|| {
            format!(
                "Failed to parse fixture {} as an EEST blockchain_test_engine fixture",
                path.display()
            )
        })?;

    let source_path = relative_source_path(path, input_root);
    let mut fixtures = Vec::new();
    let mut fixture_names = HashSet::new();

    for (test_name, mut case) in cases {
        let chain_id = parse_json_u64(&case.config.chainid)
            .with_context(|| format!("Failed to parse chainid for EEST test {test_name}"))?;

        let payload_count = case.engine_new_payloads.len();
        let opcode_count_per_block = match case.info.metadata.opcode_count_per_block.as_ref() {
            Some(counts) if counts.len() != payload_count => {
                // Mismatched counts cannot be assigned to blocks reliably, but guest I/O is still usable.
                warn!(
                    "Ignoring opcode_count_per_block for EEST test {test_name} from {source_path}: {} entries but {payload_count} payloads",
                    counts.len(),
                );
                None
            }
            counts => counts,
        };

        // For EEST benchmark fixtures, the worst case block is the last block and the others are setup blocks,
        // so here we only load the last block as benchmark fixture.
        let block_index = payload_count.saturating_sub(1);
        if let Some(payload) = case.engine_new_payloads.pop() {
            let Some(input_hex) = payload.stateless_input_bytes else {
                info!(
                    "Skipping EEST test {test_name} block {block_index} from {source_path}: missing statelessInputBytes"
                );
                continue;
            };
            let stateless_input_bytes = decode_hex_bytes("statelessInputBytes", &input_hex)
                .with_context(|| {
                    format!(
                        "Failed to decode statelessInputBytes for EEST test {test_name} block {block_index}"
                    )
                })?;

            let output_hex = payload.stateless_output_bytes.with_context(|| {
                format!(
                    "EEST test {test_name} block {block_index} has statelessInputBytes but no statelessOutputBytes"
                )
            })?;
            let stateless_output_bytes = decode_hex_bytes("statelessOutputBytes", &output_hex)
                .with_context(|| {
                    format!(
                        "Failed to decode statelessOutputBytes for EEST test {test_name} block {block_index}"
                    )
                })?;
            let (block_number, block_used_gas) = parse_block_number_and_gas_used(&payload.params)
                .with_context(|| {
                format!("Invalid params[0] for EEST test {test_name} block {block_index}")
            })?;

            fixtures.push(EestStatelessFixture {
                name: unique_eest_fixture_name(&test_name, block_index, &mut fixture_names),
                original_test_name: test_name.clone(),
                source_path: source_path.clone(),
                block_index,
                network: case.network.clone(),
                chain_id,
                block_number,
                block_used_gas,
                opcode_count: opcode_count_per_block
                    .map(|opcode_count_per_block| opcode_count_per_block[block_index].clone()),
                target_opcode: case.info.metadata.target_opcode,
                stateless_input_bytes,
                stateless_output_bytes,
            });
        }
    }

    Ok(fixtures)
}

/// Parses the block number and gas used from `params[0]`, the execution payload.
fn parse_block_number_and_gas_used(
    params: &[serde_json::Value],
) -> Result<(Option<u64>, Option<u64>)> {
    let execution_payload = params
        .first()
        .map(EestExecutionPayload::deserialize)
        .transpose()?
        .unwrap_or_default();
    let block_number = parse_optional_json_u64(execution_payload.block_number.as_deref())
        .context("Failed to parse blockNumber")?;
    let gas_used = parse_optional_json_u64(execution_payload.gas_used.as_deref())
        .context("Failed to parse gasUsed")?;

    Ok((block_number, gas_used))
}

fn decode_hex_bytes(field_name: &str, value: &str) -> Result<Vec<u8>> {
    let hex = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);

    if !hex.len().is_multiple_of(2) {
        bail!("{field_name} must contain an even number of hex digits");
    }

    (0..hex.len())
        .step_by(2)
        .map(|index| {
            u8::from_str_radix(&hex[index..index + 2], 16)
                .with_context(|| format!("{field_name} contains invalid hex at byte {index}"))
        })
        .collect()
}

fn parse_optional_json_u64(value: Option<&str>) -> Result<Option<u64>> {
    value.map(parse_json_u64).transpose()
}

fn parse_json_u64(value: &str) -> Result<u64> {
    let value = value.trim();
    if let Some(hex) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        return u64::from_str_radix(hex, 16)
            .with_context(|| format!("failed to parse hex u64 value {value}"));
    }

    value
        .parse()
        .with_context(|| format!("failed to parse decimal u64 value {value}"))
}

fn relative_source_path(path: &Path, input_root: &Path) -> String {
    let relative = path
        .strip_prefix(input_root)
        .ok()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(path);

    normalize_path_string(relative)
}

fn normalize_path_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn unique_eest_fixture_name(
    test_name: &str,
    block_index: usize,
    fixture_names: &mut HashSet<String>,
) -> String {
    let base = eest_fixture_name(test_name, block_index);
    let mut index = 1;

    loop {
        let suffix = if index == 1 {
            String::new()
        } else {
            format!("__{index}")
        };
        let candidate = truncate_fixture_name(&base, &suffix);

        if fixture_names.insert(candidate.clone()) {
            return candidate;
        }

        index += 1;
    }
}

fn eest_fixture_name(test_name: &str, block_index: usize) -> String {
    let sanitized = sanitize_fixture_name(test_name);

    format!("eest__{sanitized}__block{block_index}")
}

fn sanitize_fixture_name(value: &str) -> String {
    let mut sanitized = String::new();
    let mut last_was_separator = false;

    for ch in value.chars() {
        let next = if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
            last_was_separator = false;
            ch
        } else if last_was_separator {
            continue;
        } else {
            last_was_separator = true;
            '_'
        };

        sanitized.push(next);
    }

    let sanitized = sanitized.trim_matches('_');
    if sanitized.is_empty() {
        return "fixture".to_string();
    }

    sanitized.to_string()
}

fn truncate_fixture_name(base: &str, suffix: &str) -> String {
    let base_max_len = EEST_SAFE_FILE_STEM_MAX_LEN.saturating_sub(suffix.len());
    if base.len() <= base_max_len {
        return format!("{base}{suffix}");
    }

    let truncated = base[..base_max_len].trim_end_matches('_');
    format!("{truncated}{suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn load_eest_fixture_flattens_blocks_and_preserves_raw_guest_io() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let fixture_path = dir
            .path()
            .join("blockchain_tests_engine/for_amsterdam/compute/mcopy.json");
        fs::create_dir_all(fixture_path.parent().unwrap())?;
        fs::write(&fixture_path, sample_eest_fixture())?;

        let fixtures = load_eest_benchmark_fixtures(
            serde_json::from_str(sample_eest_fixture())?,
            &fixture_path,
            dir.path(),
        )?;
        assert_eq!(fixtures.len(), 3);

        let names: Vec<_> = fixtures
            .iter()
            .map(|fixture| fixture.name.clone())
            .collect();
        assert_eq!(
            names,
            vec![
                "eest__tests_foo_py_test_same_empty_input__block0".to_string(),
                "eest__tests_foo_py_test_same_name_a__block2".to_string(),
                "eest__tests_foo_py_test_same_name_a__block2__2".to_string(),
            ]
        );
        assert!(names.iter().all(|name| name.starts_with("eest__")));
        assert!(names.iter().all(|name| !name.contains('/')));
        assert!(names.iter().all(|name| !name.contains(':')));
        assert!(names.iter().all(|name| !name.contains('[')));

        let fixture = fixtures
            .iter()
            .find(|fixture| fixture.original_test_name == "tests/foo.py::test_same[name/a]")
            .unwrap();
        assert_eq!(fixture.stateless_input_bytes, [0x15, 0x01, 0x02]);
        assert_eq!(fixture.stateless_output_bytes, [0xaa, 0xbb]);
        assert_eq!(
            fixture.source_path,
            "blockchain_tests_engine/for_amsterdam/compute/mcopy.json"
        );
        assert_eq!(fixture.block_index, 2);
        assert_eq!(fixture.chain_id, 1);
        assert_eq!(fixture.block_number, Some(1));
        assert_eq!(fixture.block_used_gas, Some(16));
        assert_eq!(
            fixture.opcode_count,
            Some(BTreeMap::from([
                ("PUSH1".to_string(), 5),
                ("SSTORE".to_string(), 2)
            ]))
        );
        assert_eq!(fixture.target_opcode, Some("MCOPY".to_string()));

        let info_without_per_block = fixtures
            .iter()
            .find(|fixture| fixture.original_test_name == "tests/foo.py::test_same[name?a]")
            .unwrap();
        assert!(info_without_per_block.opcode_count.is_none());
        assert_eq!(
            info_without_per_block.target_opcode,
            Some("ADD".to_string())
        );
        assert_eq!(info_without_per_block.block_number, Some(3));
        assert_eq!(info_without_per_block.block_used_gas, Some(48));

        let empty_input = fixtures
            .iter()
            .find(|fixture| fixture.original_test_name == "tests/foo.py::test_same[empty_input]")
            .unwrap();
        assert!(empty_input.stateless_input_bytes.is_empty());
        assert_eq!(empty_input.stateless_output_bytes, [0xcc]);
        assert_eq!(empty_input.block_number, None);
        assert_eq!(empty_input.block_used_gas, None);

        Ok(())
    }

    #[test]
    fn eest_fixture_name_preserves_full_sanitized_name_when_it_fits() {
        let name = eest_fixture_name(
            "tests/amsterdam/eip8025_optional_proofs/test_witness_state_writes.py::test_witness_state_sstore_into_empty_storage_omits_post_state_nodes[fork_Amsterdam-blockchain_test_engine]",
            0,
        );

        assert_eq!(
            name,
            "eest__tests_amsterdam_eip8025_optional_proofs_test_witness_state_writes_py_test_witness_state_sstore_into_empty_storage_omits_post_state_nodes_fork_Amsterdam-blockchain_test_engine__block0"
        );
    }

    #[test]
    fn eest_fixture_name_truncates_only_when_it_exceeds_safe_file_stem_limit() {
        let test_name = format!("tests/foo.py::test_{}", "a".repeat(300));
        let name = eest_fixture_name(&test_name, 0);
        let truncated = truncate_fixture_name(&name, "");

        assert!(name.len() > EEST_SAFE_FILE_STEM_MAX_LEN);
        assert_eq!(truncated.len(), EEST_SAFE_FILE_STEM_MAX_LEN);
        assert!(truncated.starts_with("eest__tests_foo_py_test_"));
        assert!(!truncated.ends_with('_'));
    }

    #[test]
    fn eest_blockchain_test_fixture_is_rejected() {
        let fixture_path = Path::new("fixtures/blockchain_tests/mcopy.json");
        let value = serde_json::json!({
            "tests/foo.py::test_same[fork_Amsterdam-blockchain_test]": {
                "network": "Amsterdam",
                "config": {"chainid": "0x01"},
                "blocks": [{"statelessInputBytes": "0x0102", "statelessOutputBytes": "0xaa"}]
            }
        });

        let err =
            load_eest_benchmark_fixtures(value, fixture_path, Path::new("fixtures")).unwrap_err();
        let message = format!("{err:#}");
        assert!(
            message.contains("as an EEST blockchain_test_engine fixture"),
            "{message}"
        );
        assert!(message.contains("engineNewPayloads"), "{message}");
    }

    #[test]
    fn eest_block_with_input_requires_output() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let fixture_path = dir.path().join("missing-output.json");
        fs::write(
            &fixture_path,
            r#"{
                "tests/foo.py::test_missing_output": {
                    "network": "Amsterdam",
                    "config": {"chainid": "0x01"},
                    "engineNewPayloads": [{"statelessInputBytes": "0x0102"}]
                }
            }"#,
        )?;

        let err = load_eest_benchmark_fixtures(
            serde_json::from_str(&fs::read_to_string(&fixture_path)?)?,
            &fixture_path,
            dir.path(),
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("has statelessInputBytes but no statelessOutputBytes"));

        Ok(())
    }

    #[test]
    fn eest_opcode_counts_are_used_only_when_aligned_with_blocks() -> Result<()> {
        let input_root = Path::new("fixtures");
        let fixture_path = input_root.join("opcode-count.json");
        for (count_len, expected_count) in [(0, None), (4, None), (5, Some(5)), (6, None)] {
            let counts: Vec<_> = (1..=count_len)
                .map(|count| serde_json::json!({"PUSH1": count}))
                .collect();
            let value = serde_json::json!({
                "tests/foo.py::test_opcode_counts": {
                    "network": "Amsterdam",
                    "config": {"chainid": "0x01"},
                    "engineNewPayloads": [{}, {}, {}, {}, {
                        "statelessInputBytes": "0x0102",
                        "statelessOutputBytes": "0xaa"
                    }],
                    "_info": {
                        "metadata": {
                            "opcode_count_per_block": counts,
                            "target_opcode": "PUSH1"
                        }
                    }
                }
            });

            let fixtures = load_eest_benchmark_fixtures(value, &fixture_path, input_root)?;
            assert_eq!(fixtures.len(), 1);
            let fixture = &fixtures[0];
            assert_eq!(fixture.block_index, 4);
            assert_eq!(fixture.stateless_input_bytes, [0x01, 0x02]);
            assert_eq!(fixture.stateless_output_bytes, [0xaa]);
            assert_eq!(fixture.target_opcode.as_deref(), Some("PUSH1"));
            assert_eq!(
                fixture.opcode_count,
                expected_count.map(|count| BTreeMap::from([("PUSH1".to_string(), count)])),
                "unexpected opcode counts with {count_len} entries for 5 payloads"
            );
        }

        Ok(())
    }

    pub(crate) fn sample_eest_fixture() -> &'static str {
        r#"{
            "tests/foo.py::test_same[empty_input]": {
                "network": "Amsterdam",
                "config": {"chainid": "0x01"},
                "engineNewPayloads": [
                    {
                        "statelessInputBytes": "0x",
                        "statelessOutputBytes": "0xcc"
                    }
                ]
            },
            "tests/foo.py::test_same[name/a]": {
                "network": "Amsterdam",
                "config": {"chainid": "0x01"},
                "engineNewPayloads": [
                    {
                        "statelessInputBytes": "0x150102",
                        "statelessOutputBytes": "0xcc"
                    },
                    {
                        "params": [{"blockNumber": "0x02", "gasUsed": "0x20"}]
                    },
                    {
                        "newPayloadVersion": "5",
                        "params": [{"blockNumber": "0x01", "gasUsed": "0x10"}, [], "0x00", []],
                        "statelessInputBytes": "0x150102",
                        "statelessOutputBytes": "0xaabb"
                    }
                ],
                "_info": {
                    "metadata": {
                        "opcode_count": {"PUSH1": 8, "SSTORE": 2, "MCOPY": 7},
                        "target_opcode": "MCOPY",
                        "opcode_count_per_block": [
                            {"MCOPY": 7},
                            {"PUSH1": 3},
                            {"PUSH1": 5, "SSTORE": 2}
                        ]
                    }
                }
            },
            "tests/foo.py::test_same[name?a]": {
                "network": "Amsterdam",
                "config": {"chainid": "0x01"},
                "engineNewPayloads": [
                    {},
                    {},
                    {
                        "statelessInputBytes": "0x0f",
                        "statelessOutputBytes": "0xdead",
                        "validationError": "BlockException.INVALID_BLOCK_ACCESS_LIST",
                        "params": [{"blockNumber": "0x03", "gasUsed": "0x30"}]
                    }
                ],
                "_info": {
                    "metadata": {
                        "opcode_count": {"ADD": 3},
                        "target_opcode": "ADD"
                    }
                }
            }
        }"#
    }
}
