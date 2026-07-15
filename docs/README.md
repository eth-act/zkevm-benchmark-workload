# Documentation Map

## Fast Path

- To understand the full benchmark flow, read [Benchmark Execution](benchmark-execution.md).
- To understand the canonical EEST JSON accepted by `ere-hosts`, read [Benchmark Execution Inputs](benchmark-execution-inputs.md).
- To understand the files and JSON written by `ere-hosts`, read [Benchmark Execution Output](benchmark-execution-output.md).
- To publish canonical stateless input batches as a public R2 dataset or validate that dataset locally with EEST, read [Stateless Input Publication](stateless-input-publication.md).

## Guides

- [Benchmark Execution](benchmark-execution.md): common `ere-hosts` commands, action-aware input requirements, proof verification, guest artifacts, and operational notes.
- [Benchmark Execution Inputs](benchmark-execution-inputs.md): canonical EEST schema, input discovery, fixture filtering, execution-client routing, and legacy-format rejection.
- [Benchmark Execution Output](benchmark-execution-output.md): metrics directory layout, `BenchmarkRun` JSON, hardware metadata, proof handling, and input dumps.
- [Stateless Input Publication](stateless-input-publication.md): the separate `witness-generator-spec-cli` R2 publication flow, download examples, systemd assets, and local EEST validation.

The `zkevm-metrics` crate API documentation lives in [`crates/metrics/README.md`](../crates/metrics/README.md). The CLI metrics files written by `ere-hosts` are documented in [Benchmark Execution Output](benchmark-execution-output.md).
