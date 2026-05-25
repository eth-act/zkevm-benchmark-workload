# Documentation Map

## Fast Path

- To understand the full benchmark flow, read [Benchmark Execution](benchmark-execution.md).
- To understand exactly what JSON `ere-hosts` accepts, read [Benchmark Execution Inputs](benchmark-execution-inputs.md).
- To understand exactly what files and JSON `ere-hosts` writes, read [Benchmark Execution Output](benchmark-execution-output.md).
- To understand how legacy generated fixtures are produced, read [Fixture Generation](fixture-generation.md).

## Guides

- [Fixture Generation](fixture-generation.md): when to use `witness-generator-cli`, direct EEST fixtures, RPC fixtures, and raw-input fixtures.
- [Benchmark Execution](benchmark-execution.md): common `ere-hosts` commands, action model, proof verification, and operational notes.
- [Benchmark Execution Inputs](benchmark-execution-inputs.md): input discovery, fixture filtering, legacy generated fixture schema, and direct EEST `blockchain_tests` schema.
- [Benchmark Execution Output](benchmark-execution-output.md): metrics directory layout, `BenchmarkRun` JSON, hardware metadata, proof handling, and input dumps.
- [Witness Generator CLI](witness-generator-cli.md): Docker usage and binary-local notes for `witness-generator-cli`.

The `zkevm-metrics` crate API documentation lives in [`crates/metrics/README.md`](../crates/metrics/README.md). The CLI metrics files written by `ere-hosts` are documented in [Benchmark Execution Output](benchmark-execution-output.md).
