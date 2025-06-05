# zkVM Benchmarker

A command-line tool for benchmarking different Ere compatible zero-knowledge virtual machines (zkVMs).

## Overview

This benchmarker allows you to compare the performance of various zkVM implementations including SP1, Risc Zero, OpenVM, and Pico. You can select which zkVMs to compile using feature flags, choose between CPU and GPU resources, and either execute or prove.

## Feature Flags

The benchmarker uses Cargo feature flags to control which zkVMs are compiled into the binary. You **must** specify at least one zkVM feature when building or running.

### Available Features

- `sp1` - Enable SP1 zkVM support
- `risc0` - Enable Risc Zero zkVM support  
- `openvm` - Enable OpenVM zkVM support
- `pico` - Enable Pico zkVM support

### Basic Usage

**Note:** Unlike the previous version, you must explicitly specify which zkVMs to include via feature flags.

Build and run with SP1 only:

```bash
cargo run --features sp1 -- tests
```

Build and run with multiple zkVMs:

```bash
cargo run --features "sp1,risc0" -- tests
```

Run all available zkVMs:

```bash
cargo run --features "sp1,risc0,openvm,pico" -- tests
```

### Data Sources

The benchmarker supports two data sources:

#### Test Files (Default)

Uses pre-generated test files from the workspace:

```bash
# Uses default test directory: <workspace>/zkevm-fixtures/fixtures/blockchain_tests
cargo run --features sp1 -- tests

# Specify custom test directory
cargo run --features sp1 -- tests --directory-path /path/to/test/files
```

#### Rpc Data

Pull blocks directly from mainnet via RPC:

```bash
cargo run --features sp1 -- rpc \
  --last-n-blocks 10 \
  --rpc-url "https://mainnet.infura.io/v3/YOUR_KEY"

# With custom headers
cargo run --features sp1 -- rpc \
  --last-n-blocks 5 \
  --rpc-url "https://mainnet.infura.io/v3/YOUR_KEY" \
  --rpc-header "Authorization:Bearer TOKEN"
```

### Resource Configuration

Choose compute resource type:

```bash
# Use CPU resources (default)
cargo run --features sp1 -- --resource cpu tests

# Use GPU resources
cargo run --features sp1 -- --resource gpu tests
```

### Action Types

Select benchmark operation:

```bash
# Execute programs (default)
cargo run --features sp1 -- --action execute tests

# Generate proofs
cargo run --features sp1 -- --action prove tests
```

### Combined Examples

Run SP1 and OpenVM with GPU proving on RPC data:

```bash
cargo run --features "sp1,openvm" -- \
  --resource gpu \
  --action prove \
  rpc --last-n-blocks 5 --rpc-url "https://mainnet.infura.io/v3/YOUR_KEY"
```

Run all zkVMs with CPU execution on test files:

```bash
cargo run --features "sp1,risc0,openvm,pico" -- \
  --resource cpu \
  --action execute \
  tests --directory-path ./my-test-files
```

## Command Line Options

| Option | Short | Description | Default | Values |
|--------|-------|-------------|---------|---------|
| `--resource` | `-r` | Choose compute resource type | `cpu` | `cpu`, `gpu` |
| `--action` | `-a` | Select benchmark operation | `execute` | `execute`, `prove` |
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
