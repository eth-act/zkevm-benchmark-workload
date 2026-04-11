# Witness Generator CLI

This README is crate-local. Use [`../../docs/fixture-generation.md`](../../docs/fixture-generation.md) for the primary operator workflow from the workspace root, and use this file for crate-specific notes such as Docker usage and binary-local behavior.

## Overview

`witness-generator-cli` is the standalone binary for generating JSON fixtures consumed by `ere-hosts`.

It supports three data sources:

- **EEST**: generate fixtures from execution-spec test releases or a local EEST directory.
- **RPC**: generate fixtures from live RPC blocks and execution witnesses.
- **Raw Input**: generate fixtures from pre-collected JSON-RPC response files listed in `raw_input_parts.txt`.

The current CLI surface is defined by `clap`; inspect it from the workspace root with:

```bash
cargo run -p witness-generator-cli -- --help
```

## Running From The Workspace Root

Common examples:

```bash
# Generate fixtures from execution spec tests
cargo run -p witness-generator-cli --release -- tests --include Prague

# Generate fixtures from a specific EEST tag
cargo run -p witness-generator-cli --release -- tests --tag v0.1.0

# Generate fixtures from a local EEST folder
cargo run -p witness-generator-cli --release -- tests \
    --eest-fixtures-path /path/to/local/eest/fixtures

# Generate fixtures from the last 5 RPC blocks
cargo run -p witness-generator-cli --release -- rpc \
    --last-n-blocks 5 \
    --rpc-url <RPC_URL>

# Listen for finalized blocks continuously
cargo run -p witness-generator-cli --release -- rpc \
    --follow \
    --rpc-url <RPC_URL>

# Generate fixtures from pre-collected raw inputs
cargo run -p witness-generator-cli --release -- raw-input \
    --input-folder /path/to/raw/inputs

# Write fixtures to a custom folder
cargo run -p witness-generator-cli --release -- \
    --output-folder my-fixtures \
    tests --include Prague
```

## Docker Usage

Build the image from the workspace root:

```bash
docker build -f crates/witness-generator-cli/Dockerfile -t witness-generator-cli .
```

Run it with an explicit output folder mounted from the host:

```bash
docker run --rm -v "$(pwd)/output:/app/output" witness-generator-cli \
    --output-folder /app/output \
    tests --include Prague
```

The Docker build context must be the repository root because the Dockerfile copies the full workspace.

## Source-Specific Notes

### EEST

- `--tag` and `--eest-fixtures-path` are mutually exclusive.
- `--include` and `--exclude` may be repeated to narrow the selected cases.

### RPC

- `--last-n-blocks`, `--block`, and `--follow` are mutually exclusive.
- `--rpc-header` values must use `key:value` or `key: value`.
- `--genesis <PATH>` loads a geth-style `genesis.json` for custom or devnet chain configs.
- The CLI validates `eth_chainId` against `genesis.config.chainId` when `--genesis` is used.

### Raw Input

- `--input-folder` must contain `chain_config.json` and `raw_input_parts.txt`.
- Each raw-input pair should resolve to `eth_block.json` and `debug_executionWitness.json`.

## Library Integration

For programmatic use, prefer the `witness-generator` library directly instead of this CLI wrapper.

## License

This crate inherits its license from the workspace. See the root `Cargo.toml` or `LICENSE` files.
