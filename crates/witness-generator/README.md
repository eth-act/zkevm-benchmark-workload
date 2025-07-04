# Witness Generator

This crate provides a standalone binary for generating execution witnesses for Ethereum blockchain test cases.

## Overview

The primary purpose of this crate is to process standard Ethereum test suites (specifically blockchain tests found in `zkevm-fixtures`) or RPC endpoints and produce execution witnesses as individual fixture files for use by the benchmark runner.

It defines the `BlockAndWitness` struct which encapsulates:

- The name of a specific test case.
- A single `StatelessInput` object containing an Ethereum block with its corresponding execution witness.
- The `ForkSpec` indicating the network rules under which the test was executed. It is needed for guest execution since we want to execute blocks on a particular network (Mainnet, Hoodi, etc).

The binary provides different data sources:
- **EEST (Execution Spec Tests)**: Processes blockchain test fixtures using `ef_tests::cases::blockchain_test::run_case`. Can use fixtures from a specific release tag or from a local directory path.
- **RPC**: Pulls blocks directly from RPC endpoints and generates witnesses. Supports one-time generation of specific blocks, last N blocks, or continuous streaming of new blocks.

Each test case generates an individual JSON fixture file that can be consumed by the `ere-hosts` benchmark runner.

## Usage

### Docker Usage

The witness generator supports containerized deployment via Docker:

```bash
# Build the Docker image
docker build -f Dockerfile -t witness-generator .

# Run with Docker (mounting output directory)
docker run -v $(pwd)/output:/app/output witness-generator tests --include Prague
```

### Binary Usage

The crate provides a standalone binary for generating fixture files:

```bash
# Generate fixtures from execution spec tests
cargo run -- tests

# Generate from specific tag
cargo run -- tests --tag v0.1.0

# Include/exclude specific tests
cargo run -- tests --include "Prague" --exclude "SSTORE"

# Generate from local EEST fixtures path
cargo run -- tests --eest-fixtures-path /path/to/local/eest/fixtures

# Generate from RPC (last 5 blocks)
cargo run -- rpc --last-n-blocks 5 --rpc-url "https://mainnet.infura.io/v3/YOUR_KEY"

# Generate specific block from RPC
cargo run -- rpc --block 20000000 --rpc-url "https://mainnet.infura.io/v3/YOUR_KEY"

# Listen for new blocks continuously
cargo run -- rpc --follow --rpc-url "https://mainnet.infura.io/v3/YOUR_KEY"

# Custom output folder
cargo run -- --output-folder my-fixtures tests
```

### EEST Fixture Sources

When using the `tests` subcommand, you have two options for specifying the source of EEST fixtures:

1. **Release Tag** (default): Use `--tag` to specify a particular EEST release tag (e.g., "v0.1.0"). If no tag is specified, the latest release will be used.
2. **Local Path**: Use `--eest-fixtures-path` to point to a local directory containing EEST fixture files.

**Note:** The `--tag` and `--eest-fixtures-path` options are mutually exclusive - you can only use one at a time.

**Example with local path:**
```bash
cargo run -- tests --eest-fixtures-path ./my-local-fixtures --include "Prague"
```

### RPC Streaming Support

The RPC data source now supports continuous streaming of new blocks using the `--follow` flag:

```bash
cargo run -- rpc --follow --rpc-url "https://your-rpc.com" --rpc-header "Authorization=Bearer YOUR_TOKEN"
```

When using `--follow`, the generator will:
- Listen for new blocks as they are finalized
- Generate witness data for each new block
- Write fixture files as they are processed
- Continue until interrupted with Ctrl+C
- Handle network disconnections gracefully

### Library Usage

This crate is primarily intended to be used as a binary, but can also be integrated into other tooling:

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

The binary will exit with an error code if fixture generation fails. Individual fixture file creation is handled gracefully with detailed error messages.


## License

This crate inherits its license from the workspace. See the root `Cargo.toml` or `LICENSE` file.
