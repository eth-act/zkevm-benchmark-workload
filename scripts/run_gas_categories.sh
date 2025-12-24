#!/bin/bash
# Script to run trace_opcodes.py for each gas category separately
# Usage: ./run_gas_categories.sh [gas1] [gas2] ... [gasN]
#   If no arguments provided, uses default categories
#   Example: ./run_gas_categories.sh 0.1 0.2 0.4 0.6 0.8

# Default gas categories if none provided
DEFAULT_GAS_CATEGORIES=("1M" "10M" "30M" "45M" "60M" "100M" "150M")

# Use provided categories or defaults
if [ $# -gt 0 ]; then
    GAS_CATEGORIES=("$@")
else
    GAS_CATEGORIES=("${DEFAULT_GAS_CATEGORIES[@]}")
fi

# Default fixtures directory (can be overridden with FIXTURES_DIR env var)
FIXTURES_DIR="${FIXTURES_DIR:-fixtures_benchmark/blockchain_tests}"
BASE_OUTPUT_DIR="opcode_traces"

echo "Starting gas category traces..."
echo "Fixtures directory: ${FIXTURES_DIR}"
echo "Categories: ${GAS_CATEGORIES[@]}"
echo ""

for gas in "${GAS_CATEGORIES[@]}"; do
    echo "=========================================="
    echo "Processing gas category: ${gas}"
    echo "=========================================="
    
    # Create separate output directory for each gas category
    # Replace dots with underscores for directory names
    GAS_DIR=$(echo "${gas}" | tr '.' '_')
    OUTPUT_DIR="${BASE_OUTPUT_DIR}_${GAS_DIR}"
    LOG_FILE="trace_${GAS_DIR}.log"
    
    # Add 'M' suffix if not present (for gas filter matching)
    GAS_FILTER="${gas}"
    if [[ ! "${gas}" =~ M$ ]]; then
        GAS_FILTER="${gas}M"
    fi
    
    echo "  Output directory: ${OUTPUT_DIR}"
    echo "  Log file: ${LOG_FILE}"
    echo "  Gas filter: ${GAS_FILTER}"
    
    python3 ./scripts/trace_opcodes.py \
        --method pyevm \
        --fixtures-dir "${FIXTURES_DIR}" \
        --output "${OUTPUT_DIR}" \
        --gas-filter "${GAS_FILTER}" \
        > "${LOG_FILE}" 2>&1
    
    EXIT_CODE=$?
    
    if [ $EXIT_CODE -eq 0 ]; then
        echo "✓ Completed ${gas} gas category"
        echo "  Log: ${LOG_FILE}"
        # Show summary
        if [ -f "${OUTPUT_DIR}/instruction_summary.csv" ]; then
            echo "  Summary: $(wc -l < ${OUTPUT_DIR}/instruction_summary.csv) entries"
        fi
    else
        echo "✗ Failed ${gas} gas category (exit code: $EXIT_CODE)"
        echo "  Check ${LOG_FILE} for details"
    fi
    
    echo ""
done

echo "=========================================="
echo "All gas categories processed!"
echo "=========================================="
