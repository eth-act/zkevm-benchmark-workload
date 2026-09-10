# zkevm-metrics

This crate provides serializable data structures and file helpers for benchmark metrics.

For the exact metrics files written by `ere-hosts`, see [`docs/benchmark-execution-output.md`](../../docs/benchmark-execution-output.md).

## Overview

The core data structure is `BenchmarkRun<Metadata>`, which stores:

- `name`: The benchmark or fixture name.
- `timestamp_completed`: Timestamp when the benchmark run ended.
- `metadata`: Benchmark-specific metadata, generic over the caller's type.
- `execution`: Optional execution metrics.
- `proving`: Optional proving metrics.
- `verification`: Optional standalone verification metrics.
- `cost_estimation`: Optional component costs, heap estimate, and comparison context.

`ExecutionMetrics`, `ProvingMetrics`, `VerificationMetrics`, and `CostEstimationMetrics` can contain either a success payload or crash information, depending on the run outcome.

`HardwareInfo` detects and stores:

- `cpu_model`: CPU model name.
- `total_ram_gib`: Total system RAM in GiB.
- `gpus`: Available GPUs, detected via `nvidia-smi` when available.

The crate can:

- Serialize a `BenchmarkRun<Metadata>` to JSON.
- Deserialize a `BenchmarkRun<Metadata>` from JSON.
- Write a `BenchmarkRun<Metadata>` to a file, creating parent directories if needed.
- Read a `BenchmarkRun<Metadata>` from a file.

The metadata type must implement `Serialize` and `DeserializeOwned`.

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
zkevm-metrics = { path = "../metrics" }
```

Example:

```rust
use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
use zkevm_metrics::{BenchmarkRun, ExecutionMetrics};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Metadata {
    block_used_gas: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let metrics = BenchmarkRun::<Metadata> {
        name: "workload name".into(),
        timestamp_completed: zkevm_metrics::chrono::Utc::now(),
        metadata: Metadata {
            block_used_gas: 12345,
        },
        execution: Some(ExecutionMetrics::Success {
            output_matched: true,
            execution_duration: Duration::from_millis(300),
        }),
        proving: None,
        verification: None,
        cost_estimation: None,
    };

    let json = BenchmarkRun::to_json(&[metrics])?;
    println!("{json}");
    Ok(())
}
```

## Action Updates

`merge_to_path` replaces populated action fields and preserves other action payloads.
It retains existing fixture metadata, or fills metadata that was `null`.
The completion timestamp identifies the latest action update.
Invalid existing JSON and fixture-name conflicts stop the update.
Both `to_path` and `merge_to_path` replace files atomically.
Updates to the same fixture file must run sequentially across processes.

Execution success contains `output_matched` and `execution_duration`. The metrics types no longer contain cycle fields.
Cost success contains `output_matched`, `cost`, `peak_heap_bytes`, and `context`.
`cost` maps upstream component names to unsigned 64-bit integers. An unavailable heap estimate is JSON `null`.
`CostEstimationContext` records the client, zkVM, SDK, Ere revision, ELF/input hashes, and effective estimator settings.

## Error Handling

File and JSON helpers return `Result<_, MetricsError>`.

## License

This crate inherits its license from the workspace. See the root `Cargo.toml` or license files.
