# Witness Generator

This crate provides a library for generating execution witnesses for Ethereum blockchain test cases. For the standalone binary, see the `witness-generator-cli` crate.

## Overview

The primary purpose of this crate is to provide library functionality for processing standard Ethereum test suites (specifically blockchain tests found in `zkevm-fixtures`) or RPC endpoints and produce execution witnesses as individual fixture files for use by the benchmark runner.

It defines the `BlockAndWitness` struct which encapsulates:

- The name of a specific test case.
- A single `StatelessInput` object containing an Ethereum block with its corresponding execution witness.
- The `ForkSpec` indicating the network rules under which the test was executed. It is needed for guest execution since we want to execute blocks on a particular network (Mainnet, Hoodi, etc).

The library provides different data sources:
- **EEST (Execution Spec Tests)**: Processes blockchain test fixtures using `ef_tests::cases::blockchain_test::run_case`. Can use fixtures from a specific release tag or from a local directory path.
- **RPC**: Pulls blocks directly from RPC endpoints and generates witnesses. Supports one-time generation of specific blocks, last N blocks, or continuous streaming of new blocks.

Each test case generates an individual JSON fixture file that can be consumed by the `ere-hosts` benchmark runner.

## Usage

### Library Usage

This crate is primarily intended to be used as a library by the `witness-generator-cli` binary, but can also be integrated into other tooling:

```toml
[dependencies]
witness-generator = { path = "../witness-generator" } # Adjust path as needed
```

Example (conceptual):

```rust,no_run
use witness_generator::{WitnessGenerator, eest_generator::ExecSpecTestBlocksAndWitnessBuilder};
use std::env::temp_dir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating witnesses...");
    
    let generator = ExecSpecTestBlocksAndWitnessBuilder::default().build()?;
    
    // Create a path in the system's temp directory
    let output_path = temp_dir().join("generated_witnesses");
    std::fs::create_dir_all(&output_path)?;
    
    // Generate and write fixture files directly to the output path
    let count = generator.generate_to_path(&output_path).await?;
    
    println!("Generated {} witness files in {:?}", count, &output_path);

    Ok(())
}
```

## Error Handling

The library provides detailed error handling for fixture generation failures. Individual fixture file creation is handled gracefully with detailed error messages.

## Binary Usage

For binary usage, see the `witness-generator-cli` crate which provides a standalone binary interface to this library with support for:

- EEST fixture processing with tag and local path options
- RPC block processing with streaming support  
- Docker containerization

## License

This crate inherits its license from the workspace. See the root `Cargo.toml` or `LICENSE` file.
