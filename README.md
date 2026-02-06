<p align="center">
  <img src="assets/logo-white-transparent-bg.png" alt="ZK-EVM Bench" width="300"/>
</p>

<h1 align="center">zkEVM Benchmarking Workload</h1>

This workspace contains code for benchmarking guest programs within different zkVMs. Although different guest programs are supported, the main use case is benchmarking the Ethereum STF by running benchmarks from the [spec tests](https://github.com/ethereum/execution-specs).

## Workspace Structure

- **`crates/metrics`**: Defines common data structures (`BenchmarkRun<Metadata>`) for storing and serializing benchmark results with generic metadata support.
- **`crates/witness-generator`**: A library that provides functionality for generating benchmark fixture files (`BlockAndWitness`: individual block + witness pairs) required for stateless block validation by processing standard Ethereum test fixtures, RPC endpoints, or pre-collected raw input files.
- **`crates/witness-generator-cli`**: A standalone binary that uses the `witness-generator` library to generate fixture files. These are saved in the `zkevm-fixtures-input` folder.
- **`crates/ere-hosts`**: A standalone binary that runs benchmarks across different zkVM platforms using pre-generated fixture files from `zkevm-fixtures-input`.
- **`crates/benchmark-runner`**: Provides a unified framework for running benchmarks across different zkVM implementations, including guest program input generation and execution orchestration.
- **`scripts/`**: Contains helper scripts (e.g., fetching fixtures).

Guest programs are maintained in the [eth-act/ere-guests](https://github.com/eth-act/ere-guests) repository and downloaded automatically during benchmark runs.

## Workflow Overview

The benchmarking process is decoupled into two distinct phases:

1. **Fixture Generation** (`witness-generator-cli`): Processes Ethereum benchmark fixtures (EEST), RPC data, or raw input files to generate individual `BlockAndWitness` fixtures as JSON files saved in `zkevm-fixtures-input/`.
2. **Benchmark Execution** (`ere-hosts`): Reads from `zkevm-fixtures-input/` and runs performance benchmarks across different zkVM platforms.

This decoupling provides several benefits:
- Independent fixture generation and benchmark execution
- Reuse of generated fixtures across multiple benchmark runs

## Prerequisites

1. **Rust Toolchain:** A standard Rust installation managed by `rustup`.
2. **Docker:** All zkVMs use EreDockerized, which means you don't need to install zkVM-specific toolchains locally. Docker handles all the compilation and execution environments.
3. **Git:** Required for cloning the repository.
4. **Common Shell Utilities:** The scripts require a `bash`-compatible shell and standard utilities like `curl`, `jq`, and `tar`.

## Setup

1. **Clone the Repository:**

    ```bash
    git clone https://github.com/eth-applied-research-group/zkevm-benchmark-workload.git
    cd zkevm-benchmark-workload
    ```

2. **Fetch/Update Benchmark Fixtures:**

    ```bash
    ./scripts/download-and-extract-fixtures.sh
    ```

3. **Generate Benchmark Input Files (required for `stateless-validator` guest program):**

    ```bash
    cargo run --release -- tests --include 10M- --include Prague

    # Or generate from local EEST fixtures
    cargo run --release -- tests --eest-fixtures-path /path/to/local/eest/fixtures

    # Or generate from RPC
    cargo run --release -- rpc --last-n-blocks 2 --rpc-url <your-rpc-url>

    # Or listen for new blocks continuously
    cargo run --release -- rpc --follow --rpc-url <your-rpc-url>

    # Or generate from pre-collected raw input files
    cargo run --release -- raw-input --input-folder /path/to/raw/inputs
    ```

    This creates individual `.json` files in the `zkevm-fixtures-input/` directory that will be consumed by the benchmark runner.

4. **Run Benchmarks:**

    Run benchmarks using the generated fixture files. All zkVMs are dockerized, so no additional setup is required:

    ```bash
    cd crates/ere-hosts

    # Run Ethereum stateless validator benchmarks with Reth execution client
    cargo run --release -- --zkvms risc0 stateless-validator --execution-client reth

    # Run Ethereum stateless validator benchmarks with Ethrex execution client
    cargo run --release -- --zkvms sp1 stateless-validator --execution-client ethrex

    # Run empty program benchmarks (for measuring zkVM overhead)
    cargo run --release -- empty-program

    # Run block encoding length benchmarks
    cargo run --release -- block-encoding-length --loop-count 100 --format rlp
    
    # Run block encoding length benchmarks (with SSZ encoding format)
    cargo run --release -- block-encoding-length --loop-count 100 --format ssz
    
    # Use custom input folder for stateless validator benchmarks
    cargo run --release -- stateless-validator --execution-client reth --input-folder my-fixtures

    # Dump raw input files used in benchmarks (opt-in)
    cargo run --release -- --zkvms sp1 --dump-inputs my-inputs stateless-validator --execution-client reth
    ```

    See the respective README files in each crate for detailed usage instructions.

### Dumping Input Files

The `--dump-inputs` flag allows you to save the raw serialized input bytes used for each benchmark run. This is useful for:
- Debugging guest programs independently
- Analyzing input data characteristics
- Replaying specific test cases outside the benchmark framework

When specified, input files are saved to the designated folder with the following structure:
```
{dump-folder}/
  └── {sub-folder}/       # e.g., "reth" for stateless-validator, empty for others
      └── {name}.bin      # Input files (one per benchmark)
```

Example usage:
```bash
cd crates/ere-hosts

# Dump inputs for stateless validator with Reth
cargo run --release -- --zkvms sp1 --dump-inputs debug-inputs stateless-validator --execution-client reth

# This creates files like:
# debug-inputs/reth/block-12345.bin
# debug-inputs/reth/block-12346.bin
```

Note: Input files are zkVM-independent (the same input is used across all zkVMs), so they're only written once even when benchmarking multiple zkVMs.

## License

Licensed under either of

* MIT license (LICENSE‑MIT or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 (LICENSE‑APACHE or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
