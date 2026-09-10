<p align="center">
  <img src="assets/logo-white-transparent-bg.png" alt="ZK-EVM Bench" width="300"/>
</p>

<h1 align="center">zkEVM Benchmarking Workload</h1>

This repository benchmarks Ethereum stateless-validator guests across multiple zkVMs. The normal workflow has two phases:

1. Obtain canonical EEST `blockchain_tests` fixtures containing `statelessInputBytes` and `statelessOutputBytes`.
2. Pass a fixture file, fixture directory, or EEST fixture checkout to `ere-hosts` and write execution metrics, cost estimates, proofs, or verification results.

## Workspace At a Glance

- **`crates/ere-hosts`**: benchmark CLI for execution, estimation, proving, and verification jobs.
- **`crates/benchmark-runner`**: shared orchestration for canonical fixture loading, guest resolution, execution, proof flow, and verification.
- **`crates/metrics`**: serializable result types such as `BenchmarkRun`.
- **`crates/witness-generator-spec-cli`**: separate CLI and library for producing and publishing benchmark-ready EEST stateless fixtures from CL/EL RPC endpoints.

Reth `v0.1.0-rc.3` and Ethrex `v26.0.0` support OpenVM, SP1, and ZisK.
Zesu `tests-glamsterdam-devnet@v8.1.4` supports ZisK only.
The workspace pins [ere-guests v0.17.0](https://github.com/eth-act/ere-guests/releases/tag/v0.17.0)
and Ere v0.17.0. Default guest downloads use release assets. GitHub authentication is optional.
Compatible custom artifacts can use `--bin-path` or `--guest-artifact-base-url`.

## Prerequisites

- Rust via `rustup`
- Docker
- Canonical EEST `blockchain_tests` fixtures
- Python 3.10 or later for the comparison reports

## Quickstart

Inspect both maintained CLIs:

```bash
cargo run -p ere-hosts -- --help
cargo run -p witness-generator-spec-cli -- --help
```

The witness generator produces benchmark-ready EEST fixtures from live CL/EL
networks. Use `generate` for one block or `collect` for continuous per-block
collection. Exported live batches contain a `blockchain_tests/` tree and can be
passed to `ere-hosts` immediately after extraction.

Obtain the
[`tests-zkevm@v0.8.4`](https://github.com/ethereum/execution-specs/releases/tag/tests-zkevm%40v0.8.4)
`fixtures_zkevm.tar.gz` bundle, whose `blockchain_tests` cases contain canonical stateless
bytes. Then benchmark either the extracted fixture root, a directory of EEST
JSON files, or one EEST JSON file:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 \
    stateless-validator --execution-client reth \
    --input-folder /path/to/execution-specs/fixtures
```

Execution, cost estimation, and proving require `--input-folder`. Verification reads saved proofs and does not require fixtures.

Estimate proving costs without generating a proof:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 --action estimate-cost \
    stateless-validator --execution-client reth \
    --input-folder /path/to/execution-specs/fixtures
```

Actions merge their results into each fixture JSON. Execution records duration. Estimation records component costs and optional heap usage.
Cost units differ by zkVM. Compare compatible baseline and candidate runs with `python3 scripts/compare_costs.py BASELINE CANDIDATE`.

## Guides

- [Documentation map](docs/README.md)
- [Benchmark execution, proofs, and verification guide](docs/benchmark-execution.md)
- [Benchmark input reference](docs/benchmark-execution-inputs.md)
- [Benchmark output reference](docs/benchmark-execution-output.md)
- [Stateless input publication guide](docs/stateless-input-publication.md)

The root README is intentionally short. Detailed workflow documentation lives under `docs/`.

## License

Licensed under either of

* MIT license (LICENSE-MIT or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 (LICENSE-APACHE or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
