# metrics

This crate provides data structures and utilities for handling workload performance metrics, specifically cycle counts.

## Overview

The core data structure is `BenchmarkRun<Metadata>`, which stores:

- `name`: The name of the benchmark (e.g., `fft_bench`, `aes_bench`).
- `timestamp_completed`: Timestamp when the benchmark run ended.
- `metadata`: Generic metadata of type `M` containing benchmark-specific information (e.g., block gas usage, loop counts).
- `execution`: Optional execution metrics (`Option<ExecutionMetrics>`).
- `proving`: Optional proving metrics (`Option<ProvingMetrics>`).

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

`HardwareInfo` is a separate utility struct that automatically detects and stores:
- `cpu_model`: The CPU model name.
- `total_ram_gib`: Total system RAM in GiB.
- `gpus`: Information about available GPUs (detected via nvidia-smi if available).

This struct can be used independently to capture system information and can be serialized to JSON files.

The crate offers functionality to:

- Serialize a `BenchmarkRun<Metadata>` to a JSON string.
- Deserialize a `BenchmarkRun<Metadata>` from a JSON string.
- Serialize and write a `BenchmarkRun<Metadata>` to a file (creating parent directories if needed).
- Read and deserialize a `BenchmarkRun<Metadata>` from a file.

The metadata type `M` must implement `Serialize` and `DeserializeOwned` to enable JSON serialization.

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
zkevm-metrics = { path = "../metrics" } # Adjust path as needed
```

Example:

```rust
use zkevm_metrics::{BenchmarkRun, ExecutionMetrics, ProvingMetrics};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::env::temp_dir;
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Metadata {
    block_used_gas: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let metrics_data = vec![
        BenchmarkRun::<Metadata> {
            name: "workload name".into(),
            timestamp_completed: zkevm_metrics::chrono::Utc::now(),
            metadata: Metadata {
                block_used_gas: 12345,
            },
            execution: Some(ExecutionMetrics::Success {
                total_num_cycles: 1_000,
                region_cycles: HashMap::from_iter([
                    ("setup".to_string(), 100),
                    ("compute".to_string(), 800),
                    ("teardown".to_string(), 100),
                ]),
                execution_duration: Duration::from_millis(300),
            }),
            proving: None,
        },
        BenchmarkRun {
            name: "proving workload".into(),
            timestamp_completed: zkevm_metrics::chrono::Utc::now(),
            metadata: Metadata {
                block_used_gas: 67890,
            },
            execution: None,
            proving: Some(ProvingMetrics::Success {
                proof_size: 256,
                proving_time_ms: 2_000,
            }),
        },
        // ... other workloads
    ];

    // Serialize to JSON string
    let json_string = BenchmarkRun::to_json(&metrics_data)?;
    println!("Serialized JSON: {}", json_string);

    
    for metrics in metrics_data.into_iter() {
        // Create a path in the system's temp directory
        let output_path = temp_dir().join("metrics_output.json");

        // Write to file
        metrics.to_path(&output_path)?;
        println!("Metrics written to {:?}", &output_path);

        // Read from file
        let read_metrics = BenchmarkRun::from_path(output_path)?;
        assert_eq!(metrics, read_metrics);
    }
    println!("Successfully read metrics back from file.");

    Ok(())
}
```

## Error Handling

Functions return `Result<_, MetricsError>`.

## License

This crate inherits its license from the workspace. See the root `Cargo.toml` or `LICENSE` file.
