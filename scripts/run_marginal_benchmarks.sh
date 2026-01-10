#!/bin/bash
#
# run_marginal_benchmarks.sh - Multi-sample benchmark runner with automatic witness generation
#
# This script:
# 1. Automatically converts EEST fixtures to StatelessValidationFixture format (if needed)
# 2. Runs benchmarks multiple times for statistical analysis
#
# Usage:
#   ./run_marginal_benchmarks.sh \
#     --zkvms sp1 \
#     --actions both \
#     --execute-resource cpu \
#     --prove-resource cluster \
#     --output-folder /path/to/output \
#     --input-folder /path/to/eest-fixtures \
#     --execution-client reth \
#     --guest stateless-validator \
#     --num-samples 3

set -e

# Default values
NUM_SAMPLES=3
ACTIONS="both"
EXECUTE_RESOURCE="cpu"
PROVE_RESOURCE="cluster"
EXECUTION_CLIENT="reth"
GUEST="stateless-validator"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

usage() {
    cat << EOF
Usage: $(basename "$0") [OPTIONS]

Run marginal benchmarks with multiple samples for statistical analysis.

Required options:
  --zkvms <vm>              ZK VM to use (e.g., sp1, risc0)
  --output-folder <path>    Base output folder for results
  --input-folder <path>     Input fixtures folder (EEST format)

Optional options:
  --actions <actions>       Actions to run: "execute", "prove", or "both" (default: both)
  --execute-resource <r>    Resource for execute mode (default: cpu)
  --prove-resource <r>      Resource for prove mode (default: cluster)
  --execution-client <c>    Execution client (default: reth)
  --guest <guest>           Guest program (default: stateless-validator)
  --num-samples <n>         Number of proving samples for statistics (default: 3)
                            Note: execution always runs once (deterministic)
  --help                    Show this help message

Example:
  $(basename "$0") \\
    --zkvms sp1 \\
    --actions both \\
    --execute-resource cpu \\
    --prove-resource cluster \\
    --output-folder /root/lin/marginal-run/results/sp1 \\
    --input-folder /root/lin/marginal-run/fixtures \\
    --execution-client reth \\
    --guest stateless-validator \\
    --num-samples 3

Notes:
  - The script automatically generates witnesses from EEST fixtures
  - Witnesses are cached in <input-folder>/../zkevm-fixtures
  - Delete the zkevm-fixtures folder to force regeneration
  - Execution runs once (deterministic, no sampling needed)
  - Proving runs N times for statistical analysis

Output structure:
  <output-folder>/
  ├── execute/              # Execution results (run once)
  ├── prove/                # Proving results (sampled)
  │   ├── sample-1/
  │   ├── sample-2/
  │   └── sample-N/
  └── run-metadata.json

EOF
    exit 1
}

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Timing utilities
format_duration() {
    local seconds=$1
    local hours=$((seconds / 3600))
    local minutes=$(((seconds % 3600) / 60))
    local secs=$((seconds % 60))

    if [ $hours -gt 0 ]; then
        printf "%dh %dm %ds" $hours $minutes $secs
    elif [ $minutes -gt 0 ]; then
        printf "%dm %ds" $minutes $secs
    else
        printf "%ds" $secs
    fi
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --zkvms)
            ZKVMS="$2"
            shift 2
            ;;
        --actions)
            ACTIONS="$2"
            shift 2
            ;;
        --execute-resource)
            EXECUTE_RESOURCE="$2"
            shift 2
            ;;
        --prove-resource)
            PROVE_RESOURCE="$2"
            shift 2
            ;;
        --output-folder)
            OUTPUT_FOLDER="$2"
            shift 2
            ;;
        --input-folder)
            INPUT_FOLDER="$2"
            shift 2
            ;;
        --execution-client)
            EXECUTION_CLIENT="$2"
            shift 2
            ;;
        --guest)
            GUEST="$2"
            shift 2
            ;;
        --num-samples)
            NUM_SAMPLES="$2"
            shift 2
            ;;
        --help|-h)
            usage
            ;;
        *)
            log_error "Unknown option: $1"
            usage
            ;;
    esac
done

# Validate required arguments
if [[ -z "$ZKVMS" ]]; then
    log_error "Missing required argument: --zkvms"
    usage
fi

if [[ -z "$OUTPUT_FOLDER" ]]; then
    log_error "Missing required argument: --output-folder"
    usage
fi

if [[ -z "$INPUT_FOLDER" ]]; then
    log_error "Missing required argument: --input-folder"
    usage
fi

# Validate actions
case "$ACTIONS" in
    execute|prove|both)
        ;;
    *)
        log_error "Invalid actions: $ACTIONS. Must be 'execute', 'prove', or 'both'"
        exit 1
        ;;
esac

# Convert EEST fixtures to StatelessValidationFixture format if needed
INPUT_PARENT=$(dirname "$INPUT_FOLDER")
ZKEVM_FIXTURES="$INPUT_PARENT/zkevm-fixtures"

if [[ -d "$ZKEVM_FIXTURES" ]]; then
    log_info "Found existing zkevm-fixtures at: $ZKEVM_FIXTURES"
    log_info "Skipping witness generation (delete $ZKEVM_FIXTURES to regenerate)"
    WITNESS_GEN_TIME=0
else
    log_info "Generating witnesses from EEST fixtures..."
    log_info "Input (EEST): $INPUT_FOLDER"
    log_info "Output (zkEVM): $ZKEVM_FIXTURES"

    WITNESS_START=$(date +%s)
    cargo run --release -p witness-generator-cli -- \
        --output-folder "$ZKEVM_FIXTURES" \
        tests \
        --eest-fixtures-path "$INPUT_FOLDER"

    if [[ $? -ne 0 ]]; then
        log_error "Witness generation failed"
        exit 1
    fi

    WITNESS_END=$(date +%s)
    WITNESS_GEN_TIME=$((WITNESS_END - WITNESS_START))
    log_info "Witness generation complete ($(format_duration $WITNESS_GEN_TIME))"
fi

# Use the converted fixtures for benchmarking
INPUT_FOLDER="$ZKEVM_FIXTURES"
log_info "Using zkEVM fixtures: $INPUT_FOLDER"

# Create output folder
mkdir -p "$OUTPUT_FOLDER"

# Write metadata
METADATA_FILE="$OUTPUT_FOLDER/run-metadata.json"
cat > "$METADATA_FILE" << EOF
{
    "start_time": "$(date -Iseconds)",
    "zkvms": "$ZKVMS",
    "actions": "$ACTIONS",
    "execute_resource": "$EXECUTE_RESOURCE",
    "prove_resource": "$PROVE_RESOURCE",
    "execution_client": "$EXECUTION_CLIENT",
    "guest": "$GUEST",
    "num_samples": $NUM_SAMPLES,
    "zkevm_fixtures_folder": "$INPUT_FOLDER",
    "output_folder": "$OUTPUT_FOLDER"
}
EOF

SCRIPT_START=$(date +%s)

log_info "Starting benchmark run"
log_info "ZKVMS: $ZKVMS"
log_info "Actions: $ACTIONS"
log_info "Output folder: $OUTPUT_FOLDER"
log_info "Input folder: $INPUT_FOLDER"

# Run execute action once (deterministic, no need to sample)
EXECUTE_TIME=0
if [[ "$ACTIONS" == "execute" || "$ACTIONS" == "both" ]]; then
    EXECUTE_DIR="$OUTPUT_FOLDER/execute"
    mkdir -p "$EXECUTE_DIR"

    log_info "=========================================="
    log_info "Running execution (deterministic, run once)"
    log_info "=========================================="

    EXECUTE_START=$(date +%s)
    cargo run --release -p ere-hosts -- \
        --zkvms "$ZKVMS" \
        --action execute \
        --resource "$EXECUTE_RESOURCE" \
        --output-folder "$EXECUTE_DIR" \
        "$GUEST" \
        --input-folder "$INPUT_FOLDER" \
        --execution-client "$EXECUTION_CLIENT"

    EXECUTE_END=$(date +%s)
    EXECUTE_TIME=$((EXECUTE_END - EXECUTE_START))
    log_info "Execution complete ($(format_duration $EXECUTE_TIME))"
fi

# Run prove action with sampling (for statistical analysis)
PROVE_TIMES=()
TOTAL_PROVE_TIME=0
if [[ "$ACTIONS" == "prove" || "$ACTIONS" == "both" ]]; then
    log_info "=========================================="
    log_info "Running proving with $NUM_SAMPLES samples"
    log_info "=========================================="

    for sample in $(seq 1 "$NUM_SAMPLES"); do
        PROVE_DIR="$OUTPUT_FOLDER/prove/sample-$sample"
        mkdir -p "$PROVE_DIR"

        log_info "Running prove sample $sample of $NUM_SAMPLES..."

        PROVE_START=$(date +%s)
        cargo run --release -p ere-hosts -- \
            --zkvms "$ZKVMS" \
            --action prove \
            --resource "$PROVE_RESOURCE" \
            --output-folder "$PROVE_DIR" \
            "$GUEST" \
            --input-folder "$INPUT_FOLDER" \
            --execution-client "$EXECUTION_CLIENT"

        PROVE_END=$(date +%s)
        PROVE_TIME=$((PROVE_END - PROVE_START))
        PROVE_TIMES+=($PROVE_TIME)
        TOTAL_PROVE_TIME=$((TOTAL_PROVE_TIME + PROVE_TIME))
        log_info "Prove sample $sample complete ($(format_duration $PROVE_TIME))"
    done

    log_info "All prove samples complete (total: $(format_duration $TOTAL_PROVE_TIME))"
fi

# Calculate total time
SCRIPT_END=$(date +%s)
TOTAL_TIME=$((SCRIPT_END - SCRIPT_START))

# Update metadata with end time and timing info
END_TIME=$(date -Iseconds)

# Build prove_times array for JSON
PROVE_TIMES_JSON="["
for i in "${!PROVE_TIMES[@]}"; do
    if [ $i -gt 0 ]; then
        PROVE_TIMES_JSON+=","
    fi
    PROVE_TIMES_JSON+="${PROVE_TIMES[$i]}"
done
PROVE_TIMES_JSON+="]"

python3 - << EOF
import json

with open("$METADATA_FILE", "r") as f:
    data = json.load(f)

data["end_time"] = "$END_TIME"
data["status"] = "complete"
data["timing"] = {
    "witness_generation_seconds": $WITNESS_GEN_TIME,
    "execution_seconds": $EXECUTE_TIME,
    "proving_samples_seconds": $PROVE_TIMES_JSON,
    "total_proving_seconds": $TOTAL_PROVE_TIME,
    "total_runtime_seconds": $TOTAL_TIME
}

with open("$METADATA_FILE", "w") as f:
    json.dump(data, f, indent=2)
EOF

log_info "=========================================="
log_info "Benchmark run complete!"
log_info ""

# Timing summary
log_info "Timing Summary:"
if [[ $WITNESS_GEN_TIME -gt 0 ]]; then
    log_info "  Witness generation: $(format_duration $WITNESS_GEN_TIME)"
fi
if [[ "$ACTIONS" == "execute" || "$ACTIONS" == "both" ]]; then
    log_info "  Execution:          $(format_duration $EXECUTE_TIME)"
fi
if [[ "$ACTIONS" == "prove" || "$ACTIONS" == "both" ]]; then
    log_info "  Proving ($NUM_SAMPLES samples):"
    for i in "${!PROVE_TIMES[@]}"; do
        sample_num=$((i + 1))
        log_info "    Sample $sample_num:        $(format_duration ${PROVE_TIMES[$i]})"
    done
    if [[ $NUM_SAMPLES -gt 1 ]]; then
        AVG_PROVE_TIME=$((TOTAL_PROVE_TIME / NUM_SAMPLES))
        log_info "    Average:          $(format_duration $AVG_PROVE_TIME)"
        log_info "    Total:            $(format_duration $TOTAL_PROVE_TIME)"
    fi
fi
log_info ""
log_info "  Total runtime:      $(format_duration $TOTAL_TIME)"
log_info ""

# Output locations
log_info "Output Locations:"
if [[ "$ACTIONS" == "execute" || "$ACTIONS" == "both" ]]; then
    log_info "  Execution: $OUTPUT_FOLDER/execute"
fi
if [[ "$ACTIONS" == "prove" || "$ACTIONS" == "both" ]]; then
    log_info "  Proving:   $OUTPUT_FOLDER/prove"
fi
log_info "  Metadata:  $METADATA_FILE"
log_info "=========================================="



