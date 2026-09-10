# Benchmark Execution

This is the source-of-truth workflow guide for `ere-hosts`.

## Overview

`ere-hosts` consumes canonical EEST fixtures, resolves guest binaries, runs benchmarks across selected zkVMs, and optionally persists or verifies proofs.

Inspect the current CLI surface from the repository root:

```bash
cargo run -p ere-hosts -- --help
```

Prerequisites:

- Docker is required because zkVM hosts are managed through `ere-dockerized`.
- Execute, estimate-cost, and prove actions require an explicit `--input-folder` pointing to a canonical EEST JSON file, a directory of EEST JSON files, or an EEST checkout containing `blockchain_tests/`.
- Verification reads proofs and does not require `--input-folder`. A supplied verification input path is accepted and ignored for backward compatibility.

## Common Benchmark Commands

Run the stateless validator with Reth:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    stateless-validator --execution-client reth \
    --input-folder /path/to/eest-fixtures
```

Run the stateless validator with Ethrex:

```bash
cargo run -p ere-hosts --release -- --zkvms openvm \
    stateless-validator --execution-client ethrex \
    --input-folder /path/to/eest-fixtures
```

Run Zesu on ZisK:

```bash
cargo run -p ere-hosts --release -- --zkvms zisk \
    stateless-validator --execution-client zesu \
    --input-folder /path/to/eest-fixtures
```

Zesu supports ZisK only. Unsupported guest/zkVM pairs fail before artifact downloads or container startup.

Run directly from an EEST fixture checkout:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    stateless-validator --execution-client reth \
    --input-folder /path/to/execution-specs/fixtures
```

When the path contains a `blockchain_tests/` subdirectory, only that subtree is used. A direct EEST JSON file is also accepted.

Filter selected fixtures by prefix:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    stateless-validator --execution-client reth \
    --input-folder /path/to/eest-fixtures \
    --fixture test_sha256.py::test_sha256 \
    --fixture test_memory.py::test_mcopy
```

## Action Model

`ere-hosts` supports four actions:

- `--action execute`: execute the guest only. This is the default and requires `--input-folder`.
- `--action estimate-cost`: execute the cost estimator without generating a proof. This action requires `--input-folder`.
- `--action prove`: execute, generate a proof, and require `--input-folder`.
- `--action verify`: verify proofs loaded from disk or a downloaded `.tar.gz` archive. Input fixtures are not read.

Execution, estimation, and proving require an existing input path before artifact resolution.
Verification does not require input fixtures. A supplied verification input path is ignored, even if it no longer exists.

Timeouts are action-scoped:

- Default execute and estimate-cost timeout: `5m`
- Default prove timeout: `15m`
- Default verify timeout: `2s`

Ere initializes the guest before the action timeout starts.
OpenVM also compiles its metered executor on the first estimate. This compilation uses the estimation timeout.

Override the selected action's timeout:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    --timeout 90s \
    stateless-validator --execution-client reth \
    --input-folder /path/to/eest-fixtures
```

## Cost Estimation And Comparison

Run the estimator against the same fixtures as an execution run:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 --action estimate-cost \
    stateless-validator --execution-client reth \
    --input-folder /path/to/eest-fixtures
```

This action adds `cost_estimation` to each fixture JSON and preserves execution or proof results.
It calls the estimator once per fixture. It does not measure ordinary execution duration.

Compare two result directories:

```bash
python3 scripts/compare_costs.py baseline-results candidate-results > cost-comparison.md
python3 scripts/compare_executions.py baseline-results candidate-results
```

Both commands discover fixture JSON recursively. The cost command also accepts individual client or zkVM directories.
Cost comparisons require the same client kind, fixture identity, input hash, zkVM, SDK, Ere revision, and estimator settings.
Guest versions and ELF hashes can differ. Duplicate matches produce an error.
The report excludes crashes and output mismatches, and lists unmatched results.
Component costs sum over matched fixtures. Heap summaries use the maximum available measurement and show measurement coverage.
Missing values and percentages with a zero baseline appear as `N/A`.
Positive cost changes indicate an increase. The command exits with code 1 if no compatible pairs exist, or 2 for invalid input.

The execution comparison uses duration, including fractional seconds.
The [output reference](benchmark-execution-output.md#cost-estimation) explains cost units and heap limitations.

## Inputs And Outputs

- Metrics output folder default: `zkevm-metrics/`
- Verification proof folder default: `zkevm-fixtures-proofs/`
- ZisK profile output folder default: `zisk-profiles/`

Use the focused references for exact schemas and file layouts:

- [Benchmark Execution Inputs](benchmark-execution-inputs.md) describes canonical input discovery, filtering, client compatibility, and explicit legacy-format rejection.
- [Benchmark Execution Output](benchmark-execution-output.md) describes metrics JSON, `hardware.json`, proof files, input dumps, and workload metadata.

Dump the raw serialized guest inputs used for a run:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    --dump-inputs debug-inputs \
    stateless-validator --execution-client reth \
    --input-folder /path/to/eest-fixtures
```

These input dumps are zkVM-independent, so each fixture input is written once even if multiple zkVMs are selected.

## Proof Persistence And Verification

Generate and save proofs:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    --action prove \
    --save-proofs my-proofs \
    stateless-validator --execution-client reth \
    --input-folder /path/to/eest-fixtures
```

Verify proofs from a local folder without fixture input:

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

## Guest Artifact Resolution

Default guests use the `ere-guests v0.17.0` release assets.
`GH_TOKEN` or `GITHUB_TOKEN` is optional for release downloads.
The upstream downloader uses the first nonempty token in that order.
Commit or branch dependencies still require a token for GitHub Actions artifacts.

Prebuilt Ere images are available with `ERE_IMAGE_REGISTRY=ghcr.io/eth-act/ere`.
The dependency selects image revision `5023513`.

Artifacts are named `stateless-validator-<execution-client>-<zkvm>-<zkvm-sdk-version>`, so a zkVM SDK bump changes the resolved file names.

Use compatible local artifacts with `--bin-path <DIRECTORY>`, or provide a compatible remote directory with `--guest-artifact-base-url <URL>`. Those options remain mutually exclusive. Zesu remains restricted to ZisK for all artifact sources.

## Operational Notes

- Each action skips only its own existing result, including a recorded crash.
- `--force-rerun` replaces the selected action and preserves the other action results.
- Actions that target the same fixture file must run sequentially across processes.
- `--resource gpu` selects GPU proving resources where supported.
- `--zisk-profile` only works with `--zkvms zisk` and `--action execute`.
- `--save-proofs` is only valid with `--action prove`.
- `--proofs-url` and `--proofs-folder` are only valid with `--action verify`.
- Cluster resources support proving and verification only.
- Default release assets do not include profiling ELFs. `--zisk-profile` requires a compatible custom `-profiling.elf` artifact.
