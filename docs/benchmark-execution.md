# Benchmark Execution

This is the source-of-truth workflow guide for `ere-hosts`.

## Overview

`ere-hosts` consumes fixtures, resolves guest binaries, runs benchmarks across selected zkVMs, and optionally persists or verifies proofs.

Inspect the current CLI surface from the repo root:

```bash
cargo run -p ere-hosts -- --help
```

Prerequisites:

- Docker is required because zkVM hosts are managed through `ere-dockerized`.
- Fixture JSON files are expected in `zkevm-fixtures-input/` unless `--input-folder` is provided.

## Common Benchmark Commands

Run the stateless validator with Reth:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    stateless-validator --execution-client reth
```

Run the stateless validator with Ethrex:

```bash
cargo run -p ere-hosts --release -- --zkvms risc0 \
    stateless-validator --execution-client ethrex
```

Run the empty program:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 empty-program
```

Run the block-encoding-length guest:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    block-encoding-length --loop-count 100 --format rlp
```

Use a custom fixture folder:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    stateless-validator --execution-client reth \
    --input-folder my-fixtures
```

Filter the selected fixtures by prefix:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    stateless-validator --execution-client reth \
    --fixture test_sha256.py::test_sha256 \
    --fixture test_memory.py::test_mcopy
```

The same prefix-based `--fixture` filter is also available on `block-encoding-length`.

## Action Model

`ere-hosts` supports three actions:

- `--action execute`: execute the guest only. This is the default.
- `--action prove`: execute and generate a proof.
- `--action verify`: verify proofs loaded from disk or downloaded from a `.tar.gz` archive.

Timeouts are action-scoped:

- Default execute timeout: `5m`
- Default prove timeout: `15m`
- Default verify timeout: `2s`

Override the timeout for the selected action only:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    --timeout 90s \
    empty-program
```

## Inputs And Outputs

- Fixture input folder default: `zkevm-fixtures-input/`
- Metrics output folder default: `zkevm-metrics/`
- Verification proof folder default: `zkevm-fixtures-proofs/`
- Zisk profile output folder default: `zisk-profiles/`

Proofs are only saved when `--save-proofs <PATH>` is provided.

Dump the raw serialized guest inputs used for a run:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    --dump-inputs debug-inputs \
    stateless-validator --execution-client reth
```

These input dumps are zkVM-independent, so each fixture input is written once even if multiple zkVMs are selected.

## Proof Persistence And Verification

Generate and save proofs:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    --action prove \
    --save-proofs my-proofs \
    stateless-validator --execution-client reth
```

Verify proofs from a local folder:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    --action verify \
    --proofs-folder my-proofs \
    stateless-validator --execution-client reth
```

Verify proofs from a remote archive:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    --action verify \
    --proofs-url https://example.com/proofs.tar.gz \
    stateless-validator --execution-client reth
```

When `--proofs-url` is used, the archive is downloaded, extracted to a temporary directory, and cleaned up after verification.

## Operational Notes

- Guest binaries are downloaded automatically unless `--bin-path` is set.
- Use `--force-rerun` to ignore existing output and rerun a workload.
- `--resource gpu` selects GPU proving resources where supported.
- `--zisk-profile` only works with `--zkvms zisk` and `--action execute`.

## Common Failure Modes

- `cargo run` from the repo root must include `-p ere-hosts`; the workspace has multiple binaries.
- Docker availability issues will block benchmark execution.
- `--save-proofs` is only valid with `--action prove`.
- `--proofs-url` is only valid with `--action verify`.
- Local offline failures may come from guest downloads, RPC access, or proof archive downloads rather than from a compile regression.
