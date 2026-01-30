# Running Marginal Per-Opcode Proving Time Benchmarks

This guide explains how to run the marginal per-opcode proving time benchmarks, which measure the isolated proving cost of individual EVM opcodes and precompiles in zkVMs.

For background and methodology, see the [Ethereum Research post: Measuring Per-Opcode Proving Time](https://ethresear.ch/t/measuring-per-opcode-proving-time/23955).

## Overview

The marginal benchmarking pipeline consists of four main steps:

1. **Generate EEST Fixtures** - Create test fixtures using the marginal methodology
2. **Generate zkEVM Witnesses** - Convert fixtures to witness inputs for proving
3. **Run Benchmarks** - Execute and prove with SP1 or RISC0
4. **Generate Reports** - Analyze results and create regression reports

## Prerequisites

- **Rust toolchain** (stable, via rustup)
- **Docker** with NVIDIA runtime (for GPU proving)
- **Python 3** with uv (for fixture generation)
- **Git**

### Install Dependencies

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Install uv (Python package manager)
curl -LsSf https://astral.sh/uv/install.sh | sh
source "$HOME/.local/bin/env"
```

## Step 1: Generate EEST Fixtures

Clone the execution-specs fork with marginal tests and generate test fixtures:

```bash
# Clone the execution-specs repository with marginal benchmarks
git clone https://github.com/linoscope/execution-specs.git
cd execution-specs
git checkout marginal-zk-benchmarks

# Generate fixtures (adjust output path as needed)
export FIXTURES_DIR="$HOME/marginal-run/fixtures"
uv run fill -m benchmark \
  tests/benchmark/marginal/ \
  --fork Prague \
  -n 100 \
  -v \
  --clean \
  --output "$FIXTURES_DIR"

cd ..
```

This creates EEST fixtures following the marginal methodology, which isolates each opcode's contribution by varying only the target operation count while holding all other execution context constant.

## Step 2: Generate zkEVM Witness Files

Clone (or update) this repository and generate witness inputs:

```bash
# Clone the zkevm-benchmark-workload repository
git clone https://github.com/NethermindEth/zkevm-benchmark-workload.git
cd zkevm-benchmark-workload

# Build the witness generator
cargo build --release -p witness-generator-cli

# Generate zkEVM witness files
export ZKEVM_FIXTURES="$HOME/marginal-run/zkevm-fixtures"
RAYON_NUM_THREADS=4 cargo run --release -p witness-generator-cli -- \
  -o "$ZKEVM_FIXTURES" \
  tests \
  --eest-fixtures-path "$FIXTURES_DIR" \
  --include "marginal"
```

## Step 3: Run Benchmarks

### Benchmark Script Options

| Option | Description | Default |
|--------|-------------|---------|
| `--zkvms` | zkVM to use (`sp1`, `risc0`) | Required |
| `--actions` | `execute`, `prove`, or `both` | `both` |
| `--execute-resource` | Resource for execution (`cpu`) | `cpu` |
| `--prove-resource` | Resource for proving (`cpu`, `gpu`, `cluster`) | `cluster` |
| `--output-folder` | Directory for results | Required |
| `--input-folder` | Directory with zkEVM fixtures | Required |
| `--execution-client` | Execution client (`reth`, `ethrex`) | `reth` |
| `--num-samples` | Number of proving samples for statistics | `3` |

### Option A: RISC0 (Multiple GPUs)

```bash
export OUTPUT_DIR="$HOME/marginal-run/results/risc0"

# Note: RISC0_KECCAK_PO2=15 prevents frequent prover crashes
RISC0_KECCAK_PO2=15 ./scripts/run_marginal_benchmarks.sh \
  --zkvms risc0 \
  --actions both \
  --execute-resource cpu \
  --prove-resource gpu \
  --output-folder "$OUTPUT_DIR" \
  --input-folder "$ZKEVM_FIXTURES" \
  --execution-client reth \
  --num-samples 1
```

### Option B: SP1 with Single GPU

```bash
export OUTPUT_DIR="$HOME/marginal-run/results/sp1"

./scripts/run_marginal_benchmarks.sh \
  --zkvms sp1 \
  --actions both \
  --execute-resource cpu \
  --prove-resource gpu \
  --output-folder "$OUTPUT_DIR" \
  --input-folder "$ZKEVM_FIXTURES" \
  --execution-client reth \
  --num-samples 1
```

### Option C: SP1 with Multi-GPU Cluster

SP1 requires cluster setup to run on multiple GPUs.

```bash
# Start the SP1 cluster (e.g., 4 GPU nodes)
./scripts/sp1-cluster/start-sp1-cluster.sh --gpu-nodes 4 -d

# Wait for cluster to be ready, then run benchmarks
export OUTPUT_DIR="$HOME/marginal-run/results/sp1"

./scripts/run_marginal_benchmarks.sh \
  --zkvms sp1 \
  --actions both \
  --execute-resource cpu \
  --prove-resource cluster \
  --output-folder "$OUTPUT_DIR" \
  --input-folder "$ZKEVM_FIXTURES" \
  --execution-client reth \
  --num-samples 1

# Stop the cluster when done
./scripts/sp1-cluster/stop-sp1-cluster.sh
```

## Step 4: Generate Reports

After benchmarks complete, generate the analysis report:

```bash
python3 scripts/generate_zk_gas_report.py \
  --execution-input "$OUTPUT_DIR/execute" \
  --proving-input "$OUTPUT_DIR/prove" \
  --output "$OUTPUT_DIR/report"
```

This generates:
- **Markdown report** with regression analysis and charts
- **HTML report** for easy viewing
- **CSV files** with raw data and regression results

### Report Contents

The report includes:
- **Proving time per gas** for each opcode/precompile
- **ZK cycles per gas** regression analysis
- **Time per ZK cycle** analysis (to evaluate if ZK cycles are a good proxy for proving time)
- **Regression plots** for each opcode
- **R² values** indicating the quality of linear fit

## Example: Full Run

Here's a complete example running benchmarks on a multi-GPU machine:

```bash
# Setup
export WORK_DIR="$HOME/marginal-$(date +%Y%m%d)"
mkdir -p "$WORK_DIR"

# Step 1: Generate fixtures
git clone https://github.com/linoscope/execution-specs.git "$WORK_DIR/execution-specs"
cd "$WORK_DIR/execution-specs"
git checkout marginal-zk-benchmarks
uv run fill -m benchmark tests/benchmark/marginal/ --fork Prague -n 100 -v --clean --output "$WORK_DIR/fixtures"

# Step 2: Generate witnesses
cd /path/to/zkevm-benchmark-workload
cargo build --release -p witness-generator-cli
RAYON_NUM_THREADS=4 cargo run --release -p witness-generator-cli -- \
  -o "$WORK_DIR/zkevm-fixtures" tests \
  --eest-fixtures-path "$WORK_DIR/fixtures" \
  --include "marginal"

# Step 3: Run SP1 benchmarks with 4 GPUs
./scripts/sp1-cluster/start-sp1-cluster.sh --gpu-nodes 4 -d
sleep 30  # Wait for cluster to initialize

./scripts/run_marginal_benchmarks.sh \
  --zkvms sp1 \
  --actions both \
  --execute-resource cpu \
  --prove-resource cluster \
  --output-folder "$WORK_DIR/results/sp1" \
  --input-folder "$WORK_DIR/zkevm-fixtures" \
  --execution-client reth \
  --num-samples 1

./scripts/sp1-cluster/stop-sp1-cluster.sh

# Step 4: Generate report
python3 scripts/generate_zk_gas_report.py \
  --execution-input "$WORK_DIR/results/sp1/execute" \
  --proving-input "$WORK_DIR/results/sp1/prove" \
  --output "$WORK_DIR/results/sp1/report"

echo "Report available at: $WORK_DIR/results/sp1/report/"
```

## Output Structure

After running, you'll have:

```
$OUTPUT_DIR/
├── execute/                    # Execution results (deterministic, run once)
│   └── reth/
│       └── <zkvm-version>/
│           └── *.json          # Per-fixture execution metrics
├── prove/                      # Proving results (sampled)
│   └── sample-1/
│       └── reth/
│           └── <zkvm-version>/
│               └── *.json      # Per-fixture proving metrics
├── run-metadata.json           # Run configuration and timing
└── report/                     # Generated reports (if step 4 completed)
    ├── *.md                    # Markdown report
    ├── *.html                  # HTML report
    ├── *.csv                   # Raw data exports
    └── plots/                  # Regression plots
```

## Troubleshooting

### RISC0 Prover Crashes

If RISC0 crashes on certain inputs (especially keccak-heavy tests), use:

```bash
RISC0_KECCAK_PO2=15 ./scripts/run_marginal_benchmarks.sh ...
```

### SP1 Cluster Not Responding

Check cluster status:

```bash
cd scripts/sp1-cluster
docker compose ps
docker compose logs -f
```

## References

- [Measuring Per-Opcode Proving Time (ethresear.ch)](https://ethresear.ch/t/measuring-per-opcode-proving-time/23955)
- [Execution-specs marginal tests PR](https://github.com/linoscope/execution-specs/pull/1/changes)


