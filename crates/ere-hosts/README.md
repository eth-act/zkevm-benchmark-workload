# zkVM Benchmarker

A command-line tool for benchmarking different Ere compatible zero-knowledge virtual machines (zkVMs) using pre-generated fixture files.

## Overview

This benchmarker consumes pre-generated fixture files (created by the `witness-generator-cli` binary) and runs performance benchmarks across various zkVM implementations including SP1, Risc Zero, OpenVM, and Pico. You can select which zkVMs to compile using feature flags, choose between CPU and GPU resources, and either execute or prove.

## Prerequisites

Before running benchmarks, you must first generate fixture files using the `witness-generator-cli` binary:

```bash
cd ../witness-generator-cli
cargo run -- tests  # or use rpc source
cd ../ere-hosts
```

The benchmarker expects fixture files to be available in the input directory (default: `zkevm-fixtures-input/`). Each file should contain a `BlockAndWitness` object.

## Feature Flags

The benchmarker uses Cargo feature flags to control which zkVMs are compiled into the binary. You **must** specify at least one zkVM feature when building or running.

### Available Features

- `sp1` - Enable SP1 zkVM support
- `risc0` - Enable Risc Zero zkVM support  
- `openvm` - Enable OpenVM zkVM support
- `pico` - Enable Pico zkVM support

### Basic Usage

**Note:** Unlike the previous version, you must:
1. First generate fixture files using the `witness-generator-cli` binary
2. Explicitly specify which zkVMs to include via feature flags

Build and run with SP1 only:

```bash
cargo run --features sp1
```

Build and run with multiple zkVMs:

```bash
cargo run --features "sp1,risc0"
```

Run all available zkVMs:

```bash
cargo run --features "sp1,risc0,openvm,pico"
```

### Input Source Configuration

The benchmarker reads pre-generated fixture files from an input directory:

```bash
# Use default input directory (zkevm-fixtures-input/)
cargo run --features sp1

# Specify custom input directory
cargo run --features sp1 -- --input-folder my-fixtures
```

### Resource Configuration

Choose compute resource type:

```bash
# Use CPU resources (default)
cargo run --features sp1 -- --resource cpu

# Use GPU resources
cargo run --features sp1 -- --resource gpu
```

### Action Types

Select benchmark operation:

```bash
# Execute programs (default)
cargo run --features sp1 -- --action execute

# Generate proofs
cargo run --features sp1 -- --action prove
```

### Force Rerun

By default, the benchmarker will skip tests that already have output files in the `zkevm-metrics/` directory to avoid redundant computation. Use `--force-rerun` to override this behavior:

```bash
# Skip tests that already have results (default behavior)
cargo run --features sp1

# Rerun all tests, overwriting existing results
cargo run --features sp1 -- --force-rerun
```

### Output Folder Configuration

By default, benchmark results are saved to the `zkevm-metrics/` directory. You can specify a custom output directory using the `--output-folder` flag:

```bash
# Use default output folder (zkevm-metrics/)
cargo run --features sp1

# Use custom output folder
cargo run --features sp1 -- --output-folder my-custom-results

# Use absolute path
cargo run --features sp1 -- --output-folder /tmp/benchmark-results
```

The benchmark results will be organized by zkVM type within the specified folder (e.g., `my-custom-results/sp1/`, `my-custom-results/risc0/`, etc.).

### Combined Examples

Run SP1 and OpenVM with GPU proving:

```bash
cargo run --features "sp1,openvm" -- \
  --resource gpu \
  --action prove
```

Run all zkVMs with CPU execution:

```bash
cargo run --features "sp1,risc0,openvm,pico" -- \
  --resource cpu \
  --action execute
```

Force rerun all benchmarks for SP1 and RISC0, overwriting existing results:

```bash
cargo run --features "sp1,risc0" -- \
  --force-rerun \
  --action execute
```

Run SP1 with custom input and output directories:

```bash
cargo run --features sp1 -- \
  --input-folder custom-fixtures \
  --output-folder custom-benchmarks \
  --action execute
```

## Command Line Options

| Option | Short | Description | Default | Values |
|--------|-------|-------------|---------|---------|
| `--resource` | `-r` | Choose compute resource type | `cpu` | `cpu`, `gpu` |
| `--action` | `-a` | Select benchmark operation | `execute` | `execute`, `prove` |
| `--input-folder` | `-i` | Input folder containing fixture files | `zkevm-fixtures-input` | Any valid directory path |
| `--output-folder` | `-o` | Output folder for benchmark results | `zkevm-metrics` | Any valid directory path |
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

4. Add the constructor function:

   ```rust
   #[cfg(feature = "your_zkvm")]
   fn new_your_zkvm(prover_resource: ProverResourceType) -> Result<EreYourZkVM, Box<dyn std::error::Error>> {
       let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "/ere-guests/your_zkvm");
       let program = YOUR_ZKVM_TARGET::compile(&PathBuf::from(guest_dir))?;
       Ok(EreYourZkVM::new(program, prover_resource))
   }
   ```

5. Add your zkVM's guest program into the `ere-guests/your_zkvm` directory.

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
