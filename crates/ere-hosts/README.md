# zkVM Benchmarker

A command-line tool for benchmarking different Ere compatible zero-knowledge virtual machines (zkVMs) using pre-generated fixture files. The tool supports multiple guest program types, allowing for comprehensive performance evaluation across different use cases.

## Overview

This benchmarker consumes pre-generated fixture files (created by the `witness-generator-cli` binary) and runs performance benchmarks across various zkVM implementations including SP1, Risc Zero, OpenVM, Pico, and Zisk. You can select which zkVMs to compile using feature flags, choose between different guest program types, select CPU or GPU resources, and either execute or prove.

## Guest Program Types

The benchmarker supports multiple guest program types:

- **`stateless-validator`**: Runs Ethereum stateless block validation logic. Requires input fixture files containing `BlockAndWitness` data.
- **`empty-program`**: Runs minimal programs to measure zkVM overhead without the computational complexity of Ethereum validation.
- **`block-encoding-length`**: Measures the performance of calculating encoded length of Ethereum blocks. Supports both RLP and SSZ encoding formats. Requires input fixture files containing `BlockAndWitness` data and accepts a `--loop-count` parameter to control the number of iterations and a `--format` parameter to specify the encoding format (`rlp` or `ssz`).

## Prerequisites

Before running benchmarks:

1. **For `stateless-validator` and `block-encoding-length` benchmarks:** You must first generate fixture files using the `witness-generator-cli` binary:

   ```bash
   cd ../witness-generator-cli
   cargo run -- tests  # or use rpc source
   cd ../ere-hosts
   ```

   The benchmarker expects fixture files to be available in the input directory (default: `zkevm-fixtures-input/`). Each file should contain a `BlockAndWitness` object.

2. **For `empty-program` benchmarks:** No fixture files are required as these programs don't process external input data.

## Feature Flags

The benchmarker uses Cargo feature flags to control which zkVMs are compiled into the binary. You **must** specify at least one zkVM feature when building or running.

### Available Features

- `sp1` - Enable SP1 zkVM support
- `risc0` - Enable Risc Zero zkVM support
- `openvm` - Enable OpenVM zkVM support
- `pico` - Enable Pico zkVM support
- `zisk` - Enable Zisk zkVM support

### Basic Usage

**Note:** Unlike the previous version, you must:
1. Specify which guest program type to benchmark
2. For `stateless-validator` and `block-encoding-length`: Generate fixture files using the `witness-generator-cli` binary
3. Explicitly specify which zkVMs to include via feature flags

Run stateless validator benchmarks with SP1:

```bash
cargo run --features sp1 -- stateless-validator
```

Run empty program benchmarks with SP1:

```bash
cargo run --features sp1 -- empty-program
```

Run block encoding length benchmarks with SP1 (using RLP format):

```bash
cargo run --features sp1 -- block-encoding-length --loop-count 100 --format rlp
```

Run block encoding length benchmarks with SP1 (using SSZ format):

```bash
cargo run --features sp1 -- block-encoding-length --loop-count 100 --format ssz
```

Build and run with multiple zkVMs for stateless validation:

```bash
cargo run --features "sp1,risc0" -- stateless-validator
```

Run all available zkVMs for empty programs:

```bash
cargo run --features "sp1,risc0,openvm,pico,zisk" -- empty-program
```

### Input Source Configuration

For `stateless-validator` and `block-encoding-length` benchmarks, the tool reads pre-generated fixture files from an input directory:

```bash
# Use default input directory (zkevm-fixtures-input/)
cargo run --features sp1 -- stateless-validator

# Specify custom input directory
cargo run --features sp1 -- stateless-validator --input-folder my-fixtures

# Block encoding length benchmarks also support custom input directories
cargo run --features sp1 -- block-encoding-length --input-folder my-fixtures --loop-count 50 --format rlp
```

For `empty-program` benchmarks, no input files are required:

```bash
# Empty programs don't require input files
cargo run --features sp1 -- empty-program
```

### Resource Configuration

Choose compute resource type:

```bash
# Use CPU resources (default)
cargo run --features sp1 -- stateless-validator --resource cpu

# Use GPU resources
cargo run --features sp1 -- stateless-validator --resource gpu
```

### Action Types

Select benchmark operation:

```bash
# Execute programs (default)
cargo run --features sp1 -- stateless-validator --action execute

# Generate proofs
cargo run --features sp1 -- stateless-validator --action prove
```

### Force Rerun

By default, the benchmarker will skip tests that already have output files in the `zkevm-metrics/` directory to avoid redundant computation. Use `--force-rerun` to override this behavior:

```bash
# Skip tests that already have results (default behavior)
cargo run --features sp1 -- stateless-validator

# Rerun all tests, overwriting existing results
cargo run --features sp1 -- stateless-validator --force-rerun
```

### Output Folder Configuration

By default, benchmark results are saved to the `zkevm-metrics/` directory. You can specify a custom output directory using the `--output-folder` flag:

```bash
# Use default output folder (zkevm-metrics/)
cargo run --features sp1 -- stateless-validator

# Use custom output folder
cargo run --features sp1 -- stateless-validator --output-folder my-custom-results

# Use absolute path
cargo run --features sp1 -- stateless-validator --output-folder /tmp/benchmark-results
```

The benchmark results will be organized by zkVM type within the specified folder (e.g., `my-custom-results/sp1/`, `my-custom-results/risc0/`, etc.).

### Combined Examples

Run SP1 and OpenVM stateless validator with GPU proving:

```bash
cargo run --features "sp1,openvm" -- stateless-validator \
  --resource gpu \
  --action prove
```

Run all zkVMs for empty program benchmarks with CPU execution:

```bash
cargo run --features "sp1,risc0,openvm,pico" -- empty-program \
  --resource cpu \
  --action execute
```

Force rerun all stateless validator benchmarks for SP1 and RISC0, overwriting existing results:

```bash
cargo run --features "sp1,risc0" -- stateless-validator \
  --force-rerun \
  --action execute
```

Run SP1 stateless validator with custom input and output directories:

```bash
cargo run --features sp1 -- stateless-validator \
  --input-folder custom-fixtures \
  --output-folder custom-benchmarks \
  --action execute
```

Run block encoding length benchmarks with different encoding formats:

```bash
# Test RLP encoding performance
cargo run --features sp1 -- block-encoding-length \
  --loop-count 100 \
  --format rlp

# Test SSZ encoding performance  
cargo run --features sp1 -- block-encoding-length \
  --loop-count 100 \
  --format ssz
```

Compare zkVM overhead by running empty programs across all platforms:

```bash
cargo run --features "sp1,risc0,openvm,pico,zisk" -- empty-program
```

Compare encoding format performance for block encoding length:

```bash
# Test RLP encoding performance
cargo run --features sp1 -- block-encoding-length --loop-count 50 --format rlp

# Test SSZ encoding performance on the same data
cargo run --features sp1 -- block-encoding-length --loop-count 50 --format ssz
```

## Command Line Options

| Option | Short | Description | Default | Values |
|--------|-------|-------------|---------|---------|
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

The benchmarker uses conditional compilation via feature flags to include only the selected zkVMs:

- **SP1**: Uses `RV32_IM_SUCCINCT_ZKVM_ELF` compiler targeting `ere-guests/sp1`
- **Risc Zero**: Uses `RV32_IM_RISCZERO_ZKVM_ELF` compiler targeting `ere-guests/risc0`  
- **OpenVM**: Uses `OPENVM_TARGET` compiler targeting `ere-guests/openvm`
- **Pico**: Uses `PICO_TARGET` compiler targeting `ere-guests/pico`

Each enabled zkVM will run sequentially when the benchmarker executes.

## Adding New zkVMs

To add a new zkVM to the benchmarker:

1. Add a new feature flag in `Cargo.toml`:

   ```toml
   [features]
   your_zkvm = ["dep:ere-your-zkvm"]

   [dependencies]
   ere-your-zkvm = { workspace = true, optional = true }
   ```

2. Add conditional imports in `main.rs`:

   ```rust
   #[cfg(feature = "your_zkvm")]
   use ere_your_zkvm::{EreYourZkVM, YOUR_ZKVM_TARGET};
   ```

3. Add a conditional block in the main function:

   ```rust
   #[cfg(feature = "your_zkvm")]
   {
       run_cargo_patch_command("your_zkvm")?;
       let your_zkvm = new_your_zkvm(resource)?;
       run_benchmark_ere("your_zkvm", your_zkvm, action, &block_witness_gen).await?;
       ran_any = true;
   }
   ```

4. Add the constructor function to the `get_zkvm_instances` function:

   ```rust
   #[cfg(feature = "your_zkvm")]
   {
       run_cargo_patch_command("your_zkvm")?;
       let program = YOUR_ZKVM_TARGET::compile(&guest_program_folder.join("your_zkvm"))?;
       let zkvm = EreYourZkVM::new(program, resource.clone());
       name_zkvms.push(zkVMInstance {
           name: zkvm_fullname(zkvm.name(), zkvm.sdk_version()),
           instance: Box::new(zkvm),
       });
   }
   ```

   Note: `guest_program_folder` is passed as a parameter to `get_zkvm_instances` and will automatically point to the correct guest program type directory (e.g., `ere-guests/stateless-validator/` or `ere-guests/empty-program/`).

5. Add your zkVM's guest program implementations into the appropriate guest program directories:
   - For stateless validator support: `ere-guests/stateless-validator/your_zkvm/`
   - For empty program support: `ere-guests/empty-program/your_zkvm/`
   - For block encoding length support: `ere-guests/block-encoding-length/your_zkvm/`
   
   You can implement one, some, or all depending on what guest program types you want to support.

## Error Handling

If you try to run without any feature flags enabled, you'll get a helpful error message:

```
Error: please enable one of the zkVM's using the appropriate feature flag
```

Always specify at least one zkVM feature when building or running the benchmarker.

## License

Apache and MIT dual licensed

## Contributing

Contributions are welcome.
