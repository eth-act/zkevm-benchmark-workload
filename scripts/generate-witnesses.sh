#!/bin/bash

# Witness Generation Script
# This script generates execution witnesses for all test cases and saves them
# to the zkevm-fixtures-with-witnesses directory for faster benchmark runs.

set -e

echo "========================================"
echo "zkEVM Witness Generation"
echo "========================================"

# Check if we're in the workspace root
if [ ! -f "Cargo.toml" ] || [ ! -d "crates/witness-generator" ]; then
    echo "Error: This script must be run from the workspace root directory"
    echo "Please cd to the zkevm-benchmark-workload directory and try again"
    exit 1
fi

echo "Starting witness generation..."
echo "This may take several minutes depending on the number of test cases..."
echo ""

# Run the witness generator
cargo run --bin witness-generator

echo ""
echo "========================================"
echo "Witness generation completed!"
echo "========================================"
echo ""
echo "Generated witnesses are saved in: zkevm-fixtures-with-witnesses/"
echo ""
echo "You can now run benchmarks without waiting for witness generation:"
echo "  cargo run --bin ere-hosts -- --zkvm sp1 --action execute"
echo "  cargo run --bin ere-hosts -- --zkvm sp1 --action prove"
echo ""
echo "To regenerate witnesses (e.g., after updating test fixtures):"
echo "  ./scripts/generate-witnesses.sh" 