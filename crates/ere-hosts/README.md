# zkVM Benchmarker

A command-line tool for benchmarking different Ere compatible zero-knowledge virtual machines (zkVMs) using pre-generated fixture files. The tool supports multiple guest program types and multiple execution clients, allowing for comprehensive performance evaluation across different use cases. All zkVMs are now dockerized, eliminating the need for zkVM-specific toolchain installation.

## Overview

This benchmarker consumes pre-generated fixture files (created by the `witness-generator-cli` binary) and runs performance benchmarks across various zkVM implementations including SP1, Risc Zero, OpenVM, Pico, and Zisk. The tool automatically handles zkVM compilation and execution through Docker containers, making it easy to benchmark without complex setup requirements.

## Guest Program Types

The benchmarker supports multiple guest program types:

- **`stateless-validator`**: Runs Ethereum stateless block validation logic using different execution clients. Supports both Reth and Ethrex execution clients. Requires input fixture files containing `BlockAndWitness` data.
- **`empty-program`**: Runs minimal programs to measure zkVM overhead without the computational complexity of Ethereum validation.
- **`block-encoding-length`**: Measures the performance of calculating encoded length of Ethereum blocks. Supports both RLP and SSZ encoding formats. Requires input fixture files containing `BlockAndWitness` data and accepts a `--loop-count` parameter to control the number of iterations and a `--format` parameter to specify the encoding format (`rlp` or `ssz`).

## Prerequisites

Before running benchmarks:

1. **Docker:** All zkVMs are now dockerized, so you only need Docker installed locally. No zkVM-specific toolchains are required.

2. **For `stateless-validator` and `block-encoding-length` benchmarks:** You must first generate fixture files using the `witness-generator-cli` binary:

   ```bash
   cd ../witness-generator-cli
   cargo run -- tests  # or use rpc source
   cd ../ere-hosts
   ```

   The benchmarker expects fixture files to be available in the input directory (default: `zkevm-fixtures-input/`). Each file should contain a `BlockAndWitness` object.

3. **For `empty-program` benchmarks:** No fixture files are required as these programs don't process external input data.

## Usage

All zkVMs are now dockerized and managed automatically. You no longer need to specify feature flags or manage zkVM toolchains manually.

### Basic Usage

**Note:** You must:
1. Specify which guest program type to benchmark
2. For `stateless-validator`: Choose which execution client to use (`reth` or `ethrex`)
3. For `stateless-validator` and `block-encoding-length`: Generate fixture files using the `witness-generator-cli` binary

Run stateless validator benchmarks with Reth execution client:

```bash
cargo run --release -- stateless-validator --execution-client reth
```

Run stateless validator benchmarks with Ethrex execution client:

```bash
cargo run --release -- stateless-validator --execution-client ethrex
```

Run empty program benchmarks:

```bash
cargo run --release -- empty-program
```

Run block encoding length benchmarks (using RLP format):

```bash
cargo run --release -- block-encoding-length --loop-count 100 --format rlp
```

Run block encoding length benchmarks (using SSZ format):

```bash
cargo run --release -- block-encoding-length --loop-count 100 --format ssz
```

### Input Source Configuration

For `stateless-validator` and `block-encoding-length` benchmarks, the tool reads pre-generated fixture files from an input directory:

```bash
# Use default input directory (zkevm-fixtures-input/)
cargo run --release -- stateless-validator --execution-client reth

# Specify custom input directory
cargo run --release -- stateless-validator --execution-client reth --input-folder my-fixtures

# Block encoding length benchmarks also support custom input directories
cargo run --release -- block-encoding-length --input-folder my-fixtures --loop-count 50 --format rlp
```

For `empty-program` benchmarks, no input files are required:

```bash
# Empty programs don't require input files
cargo run --features sp1 -- empty-program
```

### Resource Configuration

### Resource and Action Configuration

Choose compute resource type and benchmark operation:

```bash
# Use CPU resources (default)
cargo run --release -- stateless-validator --execution-client reth --resource cpu

# Use GPU resources  
cargo run --release -- stateless-validator --execution-client reth --resource gpu

# Execute programs (default)
cargo run --release -- stateless-validator --execution-client reth --action execute

# Generate proofs
cargo run --release -- stateless-validator --execution-client reth --action prove
```

### Force Rerun

By default, the benchmarker will skip tests that already have output files in the `zkevm-metrics/` directory to avoid redundant computation. Use `--force-rerun` to override this behavior:

```bash
# Skip tests that already have results (default behavior)
cargo run --release -- stateless-validator --execution-client reth

# Rerun all tests, overwriting existing results
cargo run --release -- stateless-validator --execution-client reth --force-rerun
```

### Output Folder Configuration

By default, benchmark results are saved to the `zkevm-metrics/` directory. You can specify a custom output directory using the `--output-folder` flag:

```bash
# Use default output folder (zkevm-metrics/)
cargo run --release -- stateless-validator --execution-client reth

# Use custom output folder
cargo run --release -- stateless-validator --execution-client reth --output-folder my-custom-results

# Use absolute path
cargo run --release -- stateless-validator --execution-client reth --output-folder /tmp/benchmark-results
```

The benchmark results will be organized by zkVM type within the specified folder (e.g., `my-custom-results/sp1/`, `my-custom-results/risc0/`, etc.).

### Combined Examples

Run stateless validator with Reth execution client using GPU proving:

```bash
cargo run --release -- stateless-validator \
  --execution-client reth \
  --resource gpu \
  --action prove
```

Run stateless validator with Ethrex execution client using CPU execution:

```bash
cargo run --release -- stateless-validator \
  --execution-client ethrex \
  --resource cpu \
  --action execute
```

Force rerun stateless validator benchmarks with Reth, overwriting existing results:

```bash
cargo run --release -- stateless-validator \
  --execution-client reth \
  --force-rerun \
  --action execute
```

Run stateless validator with custom input and output directories:

```bash
cargo run --release -- stateless-validator \
  --execution-client reth \
  --input-folder custom-fixtures \
  --output-folder custom-benchmarks \
  --action execute
```

Run block encoding length benchmarks with different encoding formats:

```bash
# Test RLP encoding performance
cargo run --release -- block-encoding-length \
  --loop-count 100 \
  --format rlp

# Test SSZ encoding performance  
cargo run --release -- block-encoding-length \
  --loop-count 100 \
  --format ssz
```

Compare zkVM overhead by running empty programs:

```bash
cargo run --release -- empty-program
```

## Command Line Options

| Option | Short | Description | Default | Values |
|--------|-------|-------------|---------|---------|
| `--execution-client` | `-e` | Execution client to use (stateless-validator only) | Required for stateless-validator | `reth`, `ethrex` |
| `--resource` | `-r` | Choose compute resource type | `cpu` | `cpu`, `gpu` |
| `--action` | `-a` | Select benchmark operation | `execute` | `execute`, `prove` |
| `--input-folder` | `-i` | Input folder containing fixture files (stateless-validator and block-encoding-length) | `zkevm-fixtures-input` | Any valid directory path |
| `--output-folder` | `-o` | Output folder for benchmark results | `zkevm-metrics` | Any valid directory path |
| `--loop-count` | - | Number of times to loop the benchmark (block-encoding-length only) | Required for block-encoding-length | Any positive integer |
| `--format` | `-f` | Encoding format for block-encoding-length benchmark | Required for block-encoding-length | `rlp`, `ssz` |
| `--force-rerun` | - | Rerun benchmarks even if output files already exist | `false` | `true`, `false` |
| `--help` | `-h` | Show help information | - | - |
| `--version` | `-V` | Show version information | - | - |

## Architecture

The benchmarker now uses EreDockerized to manage all zkVMs automatically. No feature flags or manual toolchain setup is required:

- **SP1**: Uses dockerized SP1 environment targeting guest programs
- **Risc Zero**: Uses dockerized Risc Zero environment  
- **OpenVM**: Uses dockerized OpenVM environment
- **Zisk**: Uses dockerized Zisk environment

All supported zkVMs will run sequentially when the benchmarker executes, providing comprehensive comparison data.

For stateless validator benchmarks, the tool supports multiple execution clients:
- **Reth**: Uses the `reth_stateless::validation::stateless_validation` implementation
- **Ethrex**: Uses the Ethrex stateless validation implementation

## Adding New zkVMs

All zkVMs are now managed through the EreDockerized system. To add a new zkVM:

1. Add the zkVM implementation to the EreDockerized library
2. Add guest program implementations to the appropriate directories:
   - For stateless validator support: `ere-guests/stateless-validator/reth/your_zkvm/` and `ere-guests/stateless-validator/ethrex/your_zkvm/`
   - For empty program support: `ere-guests/empty-program/your_zkvm/`
   - For block encoding length support: `ere-guests/block-encoding-length/your_zkvm/`

3. The benchmarker will automatically detect and use the new zkVM through the EreDockerized interface.

## License

Apache and MIT dual licensed

## Contributing

Contributions are welcome.
