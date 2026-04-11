# Fixture Generation

This is the source-of-truth workflow guide for `witness-generator-cli`. For crate-local notes such as Docker usage, see [`crates/witness-generator-cli/README.md`](../crates/witness-generator-cli/README.md).

## Overview

`witness-generator-cli` generates JSON fixtures consumed by `ere-hosts`. By default it writes them to `zkevm-fixtures-input/`.

Inspect the current CLI surface from the repo root:

```bash
cargo run -p witness-generator-cli -- --help
```

Override the default output folder with `-o, --output-folder <PATH>` when needed.

## EEST Fixtures

Generate fixtures from execution-spec tests:

```bash
cargo run -p witness-generator-cli --release -- tests --include 10M --include Prague
```

Use a specific tag:

```bash
cargo run -p witness-generator-cli --release -- tests --tag v0.1.0
```

Use a local EEST fixture directory instead of downloading a release:

```bash
cargo run -p witness-generator-cli --release -- tests \
    --eest-fixtures-path /path/to/local/eest/fixtures \
    --include Prague
```

Notes:

- `--tag` and `--eest-fixtures-path` are mutually exclusive.
- `--include` and `--exclude` may be repeated to narrow the generated set.

## RPC Fixtures

Generate fixtures from the latest N blocks:

```bash
cargo run -p witness-generator-cli --release -- rpc \
    --last-n-blocks 2 \
    --rpc-url <RPC_URL>
```

Generate a specific block:

```bash
cargo run -p witness-generator-cli --release -- rpc \
    --block 20000000 \
    --rpc-url <RPC_URL>
```

Follow finalized blocks continuously:

```bash
cargo run -p witness-generator-cli --release -- rpc \
    --follow \
    --rpc-url <RPC_URL>
```

Use custom headers:

```bash
cargo run -p witness-generator-cli --release -- rpc \
    --last-n-blocks 2 \
    --rpc-url <RPC_URL> \
    --rpc-header "Authorization:Bearer <TOKEN>" \
    --rpc-header "X-Trace-Id:bench-run-01"
```

Use a custom or devnet genesis file when the RPC chain ID is not baked into the Reth chain config set:

```bash
cargo run -p witness-generator-cli --release -- rpc \
    --last-n-blocks 2 \
    --rpc-url <RPC_URL> \
    --genesis /path/to/genesis.json
```

Notes:

- `--last-n-blocks`, `--block`, and `--follow` are mutually exclusive.
- `--rpc-header` values must use `key:value` or `key: value`.
- `--genesis` expects a geth-style `genesis.json` and the CLI will fail fast if `eth_chainId` does not match `genesis.config.chainId`.

## Raw-Input Fixtures

Generate fixtures from a folder containing `chain_config.json` and `raw_input_parts.txt`:

```bash
cargo run -p witness-generator-cli --release -- raw-input \
    --input-folder /path/to/raw-inputs
```

Each raw-input pair should resolve to `eth_block.json` and `debug_executionWitness.json`.

## Output Behavior

- Generated fixtures are JSON files written to `zkevm-fixtures-input/` unless `--output-folder` is set.
- The CLI creates the output directory if it does not already exist.
- The generated fixture set can be reused across multiple benchmark runs.

## Common Failure Modes

- `cargo run` from the repo root must include `-p witness-generator-cli`; the workspace has multiple binaries.
- `--last-n-blocks 0` is rejected.
- Unsupported RPC chain IDs require `--genesis`.
- Header formatting errors must be fixed to `key:value`.
- RPC, GitHub, or EEST download failures may be environmental rather than code regressions.
