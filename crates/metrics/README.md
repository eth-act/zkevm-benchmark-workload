# metrics

This crate provides data structures and utilities for handling workload performance metrics, specifically cycle counts.

## Overview

The core data structure is `BenchmarkRun`, which stores:

- `name`: The name of the benchmark (e.g., "fft_bench", "aes_bench").
- `block_used_gas`: The amount of gas used by the block in the benchmark.
- `hardware`: Hardware information about the system where the benchmark was run, including CPU model, RAM, and GPU details.
- `actions_metrics`: A list of `ActionMetrics`, which can be either `Execution` or `Proving` metrics.

`ActionMetrics` can be one of:
- `Execution(ExecutionMetrics)`: Metrics from execution workloads
- `Proving(ProvingMetrics)`: Metrics from proving workloads

Both `ExecutionMetrics` and `ProvingMetrics` can be either:
- `Success { ... }`: Contains metrics from successful runs
- `Crashed(CrashInfo)`: Contains information about crashes that occurred

`ExecutionMetrics::Success` stores:
- `total_num_cycles`: The total cycle count for the whole execution.
- `region_cycles`: A map associating names (e.g., "setup", "compute") with the cycle counts for specific regions within the workload.
- `execution_duration`: The duration of the execution.

`ProvingMetrics::Success` stores:
- `proof_size`: The size of the generated proof in bytes.
- `proving_time_ms`: The time taken to generate the proof in milliseconds.

`HardwareInfo` automatically detects and stores:
- `cpu_model`: The CPU model name.
- `total_ram_gib`: Total system RAM in GiB.
- `gpus`: Information about available GPUs (detected via nvidia-smi if available).

The crate offers functionality to:

- Serialize a list of `BenchmarkRun` to a JSON string.
- Deserialize a list of `BenchmarkRun` from a JSON string.
- Serialize and write a list of `BenchmarkRun` to a file (creating parent directories if needed).
- Read and deserialize a list of `BenchmarkRun` from a file.

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
zkevm-metrics = { path = "../metrics" } # Adjust path as needed
```

Example:

```rust
use zkevm_metrics::{ActionMetrics, BenchmarkRun, ExecutionMetrics, HardwareInfo};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::env::temp_dir;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let metrics_data = vec![
        BenchmarkRun {
            name: "workload name".into(),
            block_used_gas: 12345,
            hardware: HardwareInfo::detect(), // Automatically detect hardware
            actions_metrics: vec![ActionMetrics::Execution(ExecutionMetrics::Success {
                total_num_cycles: 1_000,
                region_cycles: HashMap::from_iter([
                    ("setup".to_string(), 100),
                    ("compute".to_string(), 800),
                    ("teardown".to_string(), 100),
                ]),
                execution_duration: Duration::from_millis(300),
            })],
        },
        // ... other workloads
    ];

    // Serialize to JSON string
    let json_string = BenchmarkRun::to_json(&metrics_data)?;
    println!("Serialized JSON: {}", json_string);

    // Create a path in the system's temp directory
    let output_path = temp_dir().join("metrics_output.json");

    // Write to file
    BenchmarkRun::to_path(&output_path, &metrics_data)?;
    println!("Metrics written to {:?}", &output_path);

    // Read from file
    let read_metrics = BenchmarkRun::from_path(output_path)?;
    assert_eq!(metrics_data, read_metrics);
    println!("Successfully read metrics back from file.");

    Ok(())
}
```

## Error Handling

Functions return `Result<_, MetricsError>`.

## License

This crate inherits its license from the workspace. See the root `Cargo.toml` or `LICENSE` file.
