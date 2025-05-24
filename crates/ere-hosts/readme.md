# zkVM Benchmarker

A command-line tool for benchmarking different Ere compatible zero-knowledge virtual machines (zkVMs).

## Overview

This benchmarker allows you to compare the performance of various zkVM implementations including SP1, Risc Zero, and OpenVM. You can run benchmarks on all zkVMs or select specific ones, choose between CPU and GPU resources, and either execute or prove.

## Usage

### Basic Usage

Run all zkVMs with default settings (execute on CPU):

```bash
cargo run --release
```

### Select Specific zkVMs

Run only SP1:

```bash
cargo run -- --zkvm sp1
```

Run multiple specific zkVMs:

```bash
cargo run -- --zkvm sp1 --zkvm risc0
```

### Choose Resource Type

Use GPU resources:

```bash
cargo run -- --resource gpu
```

Use CPU resources (default):

```bash
cargo run -- --resource cpu
```

### Select Action Type

Execute programs (default):

```bash
cargo run -- --action execute
```

Generate proofs:

```bash
cargo run -- --action prove
```

### Combined Examples

Run SP1 and OpenVM with GPU proving:

```bash
cargo run -- --zkvm sp1 --zkvm openvm --resource gpu --action prove
```

Run all zkVMs with CPU execution:

```bash
cargo run -- --resource cpu --action execute
```

## Command Line Options

| Option | Short | Description | Default | Values |
|--------|-------|-------------|---------|---------|
| `--zkvm` | `-z` | Select specific zkVMs to benchmark | All zkVMs | `sp1`, `risc0`, `openvm` |
| `--resource` | `-r` | Choose compute resource type | `cpu` | `cpu`, `gpu` |
| `--action` | `-a` | Select benchmark operation | `execute` | `execute`, `prove` |
| `--help` | `-h` | Show help information | - | - |
| `--version` | `-V` | Show version information | - | - |

## Architecture

The benchmarker relies heavily on the modular architecture of `Ere` where each zkVM is split into a `Ere-Compiler` and `Ere-zkVM`:

- **SP1**: Uses `RV32_IM_SUCCINCT_ZKVM_ELF` compiler targeting `ere-guests/sp1`
- **Risc Zero**: Uses `RV32_IM_RISCZERO_ZKVM_ELF` compiler targeting `ere-guests/risc0`  
- **OpenVM**: Uses `OPENVM_TARGET` compiler targeting `ere-guests/openvm`

Each zkVM implementation compiles guest programs from their respective directories and creates zkVM instances with the specified resource configuration.

## Adding New zkVMs

To add a new zkVM to the benchmarker:

1. Add the new variant to the `zkVM` enum:

   ```rust
   enum zkVM {
       Sp1,
       Risc0,
       Openvm,
       YourNewZkVM,  // Add here
   }
   ```

2. Add a match arm in the main loop:

   ```rust
   zkVM::YourNewZkVM => {
       let your_zkvm = new_your_zkvm(resource)?;
       run_benchmark_ere("your_zkvm_name", your_zkvm, action)?;
   }
   ```

3. Add ypour zkVMs guest program into the `ere-guests` directory.

The zkVM will automatically be included when running all zkVMs thanks to the `strum::EnumIter` derive macro.

## License

Apache and MIT dual licensed

## Contributing

Contributions are welcome.
