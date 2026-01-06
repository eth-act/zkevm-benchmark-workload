<p align="center">
  <img src="assets/logo-white-transparent-bg.png" alt="ZK-EVM Bench" width="300"/>
</p>

<h1 align="center">zkEVM Benchmarking Workload</h1>

This workspace contains code for benchmarking guest programs within different zkVMs. Although different guest programs are supported, the main use case is benchmarking the Ethereum STF by running benchmarks from the [spec tests](https://github.com/ethereum/execution-specs).

## Workspace Structure

The workspace is organized into several key components:

- **`crates/metrics`**: Defines common data structures (`BenchmarkRun<Metadata>`) for storing and serializing benchmark results with generic metadata support.
- **`crates/witness-generator`**: A library that provides functionality for generating benchmark fixture files (`BlockAndWitness`: individual block + witness pairs) required for stateless block validation by processing standard Ethereum test fixtures or RPC endpoints.
- **`crates/witness-generator-cli`**: A standalone binary that uses the `witness-generator` library to generate fixture files. These are saved in the `zkevm-fixtures-input` folder. The crate includes Docker support for containerized deployment.
- **`crates/ere-hosts`**: A standalone binary that runs benchmarks across different zkVM platforms using pre-generated fixture files from `zkevm-fixtures-input`.
- **`crates/benchmark-runner`**: Provides a unified framework for running benchmarks across different zkVM implementations, including guest program input generation and execution orchestration.
- **`ere-guests/`**: Directory containing guest program implementations organized by program type, with each type containing implementations for different zkVM platforms. See the [Guest Program Types](#guest-program-types) section for detailed information about each type.
- **`scripts`**: Contains helper scripts (e.g., fetching fixtures).
- **`xtask`**: Cargo xtask runner for automating tasks, used for automatic zkVM precompile patching.

## Workflow Overview

The benchmarking process is decoupled into two distinct phases:

1. **Fixture Generation** (`witness-generator-cli`): Processes Ethereum benchmark fixtures (EEST) or RPC data to generate individual `BlockAndWitness` fixtures as JSON files saved in `zkevm-fixtures-input/`.
2. **Benchmark Execution** (`ere-hosts`): Reads from `zkevm-fixtures-input/` and runs performance benchmarks across different zkVM platforms.

This decoupling provides several benefits:
- Independent fixture generation and benchmark execution
- Reuse of generated fixtures across multiple benchmark runs

## Core Concepts

Each zkVM benchmark implementation follows a common pattern using the EreDockerized system:

1. **Guest Program:**
    - Located within the `ere-guests/` directory, organized by guest program type and execution client (for stateless-validator).
    - For stateless validator programs: Contains implementations for different execution clients (`reth/` and `ethrex/`) with each supporting multiple zkVMs.
    - For other program types: Contains implementations organized by zkVM platform.
    - All guest programs are automatically compiled in Docker containers specific to each zkVM platform.

2. **Host Program:**
    - Located within `crates/ere-hosts/` with unified logic across all zkVM platforms.
    - A standalone Rust binary that orchestrates benchmarking for different guest program types and execution clients.
    - Uses the `benchmark-runner` crate with EreDockerized to automatically compile and execute guest programs.
    - Supports multiple guest program types via command-line arguments (e.g., `stateless-validator`, `empty-program`).
    - For stateless-validator, supports multiple execution clients (`--execution-client reth` or `--execution-client ethrex`).
    - Saves results using the `metrics` crate into the appropriate subdirectory within `zkevm-metrics/`. For `stateless-validator` guest programs, results are organized by execution client then zkVM type. For other guest program types, results are organized directly by zkVM type.

3. **Automatic zkVM Management:**
    - All zkVMs are now managed through EreDockerized, eliminating the need for manual toolchain setup.
    - Docker containers handle compilation, patching, and execution for each zkVM platform.
    - The benchmark runner automatically applies precompile patches as needed.

## Prerequisites

1. **Rust Toolchain:** A standard Rust installation managed by `rustup`.
2. **Docker:** All zkVMs now use EreDockerized, which means you don't need to install zkVM-specific toolchains locally. Docker handles all the compilation and execution environments.
3. **Git:** Required for cloning the repository.
4. **Common Shell Utilities:** The scripts in the `./scripts` directory require a `bash`-compatible shell and standard utilities like `curl`, `jq`, and `tar`.

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

3. **Generate Benchmark Input Files (only required for `stateless-validator` guest program):**

    Generate fixture files:

    ```bash
    cargo run --release -- tests --include 10M- --include Prague
    
    # Or generate from local EEST fixtures
    cargo run --release -- tests --eest-fixtures-path /path/to/local/eest/fixtures
    
    # Or generate from RPC
    cargo run --release -- rpc --last-n-blocks 2 --rpc-url <your-rpc-url>
    
    # Or listen for new blocks continuously
    cargo run --release -- rpc --follow --rpc-url <your-rpc-url>
    ```

    This creates individual `.json` files in the `zkevm-fixtures-input/` directory that will be consumed by the benchmark runner.

4. **Run Benchmarks:**

    Run benchmarks using the generated fixture files (if required). You must specify which guest program type to benchmark. All zkVMs are now dockerized, so no additional setup is required:

    ```bash
    cd crates/ere-hosts
    
    # Run Ethereum stateless validator benchmarks with Reth execution client
    cargo run --release -- --zkvms risc0 stateless-validator --execution-client reth
    
    # Run Ethereum stateless validator benchmarks with Ethrex execution client
    cargo run --release -- --zkvms sp1 stateless-validator --execution-client ethrex
    
    # Run empty program benchmarks (for measuring zkVM overhead)
    cargo run --release -- empty-program
    
    # Run block encoding length benchmarks (with RLP encoding format)
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

## Guest Program Types

This repository supports multiple guest program types for comprehensive zkVM benchmarking across different computational workloads. Each guest program type is designed to measure specific aspects of zkVM performance:

### Available Guest Program Types

1. **`stateless-validator`** - The primary benchmarking workload that executes Ethereum stateless block validation logic
   - **Purpose**: Measures zkVM performance on realistic Ethereum state transition computations
   - **Input**: Requires `BlockAndWitness` fixture files generated by `witness-generator-cli`
   - **Computation**: Validates Ethereum blocks using either Reth or Ethrex execution clients
   - **Execution Clients**: 
     - `reth`: Uses `reth_stateless::validation::stateless_validation`
     - `ethrex`: Uses Ethrex's stateless validation implementation

2. **`empty-program`** - Minimal program for measuring zkVM overhead
   - **Purpose**: Measures the baseline overhead of zkVM execution without computational workload
   - **Input**: No input files required
   - **Computation**: Minimal operations to establish zkVM baseline performance
   - **Usage**: `cargo run -- empty-program`

3. **`block-encoding-length`** - Measures block encoding performance with support for both RLP and SSZ formats
   - **Purpose**: Benchmarks the performance of calculating encoded length of Ethereum blocks using different encoding formats
   - **Input**: Uses the same `BlockAndWitness` fixture files as `stateless-validator`
   - **Computation**: Repeatedly calls encoding length calculation functions on block headers and bodies
   - **Encoding Formats**: Supports both RLP (`--format rlp`) and SSZ (`--format ssz`) encoding
   - **Usage**: `cargo run -- block-encoding-length --loop-count 100 --format rlp`

4. **`stateless-executor`** - Pure EVM execution without validation overhead
   - **Purpose**: Measures raw EVM execution cycles in zkVMs without the overhead of pre/post validation checks. Ideal for benchmarking pure opcode execution costs.
   - **Input**: Uses the same `BlockAndWitness` fixture files as `stateless-validator`
   - **Computation**: Executes EVM transactions but **skips**:
     - Pre-execution consensus validation (header checks, ancestor verification)
     - Post-execution consensus checks (receipts root, gas used validation)
     - State root verification (both pre-state and post-state)
   - **Execution Clients**: 
     - `reth`: Reth execution client only (Ethrex not supported)
   - **Key Difference from `stateless-validator`**: While `stateless-validator` performs full block validation including all consensus checks, `stateless-executor` only performs the EVM transaction execution, providing more accurate measurements of pure execution cycles.
   - **Usage**: `cargo run -- stateless-executor --execution-client reth`

### Guest Program Architecture

Each guest program type follows a consistent structure across different zkVM platforms. For `stateless-validator`, there are separate implementations for different execution clients:

```
ere-guests/<program-type>/
├── reth/         # Reth execution client implementations
│   ├── sp1/          # SP1 zkVM implementation
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   ├── risc0/        # RISC0 implementation
│   ├── openvm/       # OpenVM implementation
│   ├── pico/         # Pico implementation
│   ├── zisk/         # Zisk implementation
│   └── airbender/    # Airbender implementation
├── ethrex/       # Ethrex execution client implementations
│   ├── sp1/          # SP1 zkVM implementation
│   ├── risc0/        # RISC0 implementation (if available)
│   └── ... (other zkVMs)
└── ... (other execution clients)
```

For other guest program types (e.g., `empty-program`, `block-encoding-length`):
```
ere-guests/<program-type>/
├── sp1/          # SP1 implementation
├── risc0/        # RISC0 implementation  
├── zisk/         # Zisk implementation
└── ... (other zkVMs)
```

This might be removed in the future after EL compilation and inputs are standarized, favouring running
provided ELFs directly. Until this is done, the guest program definition and compilation is necessary to
do in this codebase.

### Adding New Guest Program Types

The architecture is designed to be extensible. To add a new guest program type:

1. **Create the guest program directory structure:**
   ```
   ere-guests/your-new-program-type/
   ├── sp1/
   │   ├── Cargo.toml
   │   └── src/main.rs
   ├── risc0/
   │   ├── Cargo.toml
   │   └── src/main.rs
   └── ... (other zkVMs)
   ```

2. **Add the new command to `ere-hosts`:**
   - Add a new variant to the `GuestProgramCommand` enum in `crates/ere-hosts/src/main.rs`
   - Add corresponding logic to handle the new guest program type
   - Update the `get_zkvm_instances` function to point to the correct subdirectory

3. **Update documentation** to include the new guest program type in the supported benchmarks table and usage examples

## CI/CD and Docker Support

The Docker workflow (`.github/workflows/docker-build.yml`) provides:
- **Automated Builds**: Docker images for `witness-generator-cli` are automatically built and pushed to GitHub Container Registry.
- **Comprehensive testing**: On each PR we test all supported EL/zkVM combinations against mainnet blocks inputs to ensure correctness.

## License

Licensed under either of

* MIT license (LICENSE‑MIT or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 (LICENSE‑APACHE or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.