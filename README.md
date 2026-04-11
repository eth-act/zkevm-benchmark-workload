<p align="center">
  <img src="assets/logo-white-transparent-bg.png" alt="ZK-EVM Bench" width="300"/>
</p>

<h1 align="center">zkEVM Benchmarking Workload</h1>


This repository benchmarks Ethereum-related guest programs across multiple zkVMs. The normal workflow has two phases:

1. Generate JSON fixtures from EEST, RPC, or raw-input sources.
2. Run those fixtures through dockerized zkVM hosts and write metrics, proofs, or verification results.

## Workspace At a Glance

- **`crates/witness-generator-cli`**: fixture-generation CLI for EEST, RPC, and raw-input sources.
- **`crates/ere-hosts`**: benchmark CLI for execution, proving, and verification jobs.
- **`crates/benchmark-runner`**: shared orchestration for guest resolution, execution, proof flow, and verification.
- **`crates/metrics`**: serializable result types such as `BenchmarkRun`.

Guest programs are maintained in the [eth-act/ere-guests](https://github.com/eth-act/ere-guests) repository and are downloaded automatically unless `--bin-path` is provided.

## Prerequisites

- Rust via `rustup`
- Docker
- Git

## Quickstart

Verify that both CLIs are reachable from the repo root:

```bash
cargo run -p witness-generator-cli -- --help
cargo run -p ere-hosts -- --help
```

Generate sample fixtures into `zkevm-fixtures-input/`:

```bash
cargo run -p witness-generator-cli --release -- tests --include 10M --include Prague
```

Run a benchmark against those fixtures:

```bash
cargo run -p ere-hosts --release -- --zkvms sp1 stateless-validator --execution-client reth
```

## Guides

- [Fixture generation guide](docs/fixture-generation.md)
- [Benchmark execution, proofs, and verification guide](docs/benchmark-execution.md)
- [Witness Generator CLI crate notes](crates/witness-generator-cli/README.md)
- [Metrics crate reference](crates/metrics/README.md)

The root README is intentionally short. Detailed workflow documentation lives under `docs/`.

## License

Licensed under either of

* MIT license (LICENSE-MIT or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 (LICENSE-APACHE or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
