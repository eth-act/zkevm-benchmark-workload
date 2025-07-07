# Witness Generator CLI

This crate provides a standalone binary for generating execution witnesses for Ethereum blockchain test cases using the `witness-generator` library.

## Overview

The witness generator CLI is a command-line interface that processes standard Ethereum test suites (specifically blockchain tests found in `zkevm-fixtures`) or RPC endpoints and produces execution witnesses as individual fixture files for use by the benchmark runner.

The binary provides different data sources:
- **EEST (Execution Spec Tests)**: Processes blockchain test fixtures using `ef_tests::cases::blockchain_test::run_case`. Can use fixtures from a specific release tag or from a local directory path.
- **RPC**: Pulls blocks directly from RPC endpoints and generates witnesses. Supports one-time generation of specific blocks, last N blocks, or continuous streaming of new blocks.

Each test case generates an individual JSON fixture file that can be consumed by the `ere-hosts` benchmark runner.

## Usage

### Docker Usage

The witness generator supports containerized deployment via Docker:

```bash
# Build the Docker image
docker build -f Dockerfile -t witness-generator-cli .

# Run with Docker (mounting output directory)
docker run -v $(pwd)/output:/app/output witness-generator-cli tests --include Prague
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

## Command Line Options

| Option | Description | Example |
|--------|-------------|---------|
| `tests` | Generate from EEST fixtures | `cargo run -- tests` |
| `rpc` | Generate from RPC endpoint | `cargo run -- rpc --rpc-url <url>` |
| `--tag` | Specify EEST release tag | `--tag v0.1.0` |
| `--eest-fixtures-path` | Use local EEST fixtures | `--eest-fixtures-path ./fixtures` |
| `--include` | Include tests matching pattern | `--include "Prague"` |
| `--exclude` | Exclude tests matching pattern | `--exclude "SSTORE"` |
| `--last-n-blocks` | Process last N blocks from RPC | `--last-n-blocks 5` |
| `--block` | Process specific block number | `--block 20000000` |
| `--follow` | Continuously stream new blocks | `--follow` |
| `--rpc-url` | RPC endpoint URL | `--rpc-url "https://..."` |
| `--rpc-header` | Custom RPC header | `--rpc-header "Auth=token"` |
| `--output-folder` | Custom output directory | `--output-folder my-fixtures` |

## Error Handling

The binary will exit with an error code if fixture generation fails. Individual fixture file creation is handled gracefully with detailed error messages.

## Architecture

The CLI acts as a wrapper around the `witness-generator` library, providing:

- Command-line argument parsing using `clap`
- Docker support for containerized deployment

## Library Integration

For programmatic use, consider using the `witness-generator` library directly instead of this CLI. See the `witness-generator` crate documentation for library usage examples.

## License

This crate inherits its license from the workspace. See the root `Cargo.toml` or `LICENSE` file.
