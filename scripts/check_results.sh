#!/bin/bash

# Script to analyze benchmark results for success/crashed status
# Usage: ./check_results.sh <directory> [-d]
#   -d flag: delete crashed JSON files after displaying info

set -euo pipefail

# Check if directory is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <directory> [-d]"
    echo "  -d: Delete crashed JSON files after analysis"
    exit 1
fi

DIR="$1"
DELETE_FLAG=false

# Check for -d flag
if [ "${2:-}" == "-d" ]; then
    DELETE_FLAG=true
fi

# Check if directory exists
if [ ! -d "$DIR" ]; then
    echo "Error: Directory '$DIR' does not exist"
    exit 1
fi

# Initialize counters
SUCCESS_COUNT=0
CRASHED_COUNT=0

# Arrays to store crashed files and their reasons
declare -a CRASHED_FILES
declare -a CRASHED_REASONS

echo "Analyzing results in: $DIR"
echo "----------------------------------------"

# Process all JSON files in the directory
for json_file in "$DIR"/*.json; do
    if [ ! -f "$json_file" ]; then
        continue
    fi

    # Check if file contains "success" or "crashed"
    if grep -q '"success"' "$json_file"; then
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    elif grep -q '"crashed"' "$json_file"; then
        CRASHED_COUNT=$((CRASHED_COUNT + 1))

        # Extract the reason using jq or grep/sed
        if command -v jq &> /dev/null; then
            reason=$(jq -r '.proving.crashed.reason' "$json_file" 2>/dev/null || echo "Unknown reason")
        else
            # Fallback to grep/sed if jq is not available
            reason=$(grep -o '"reason": *"[^"]*"' "$json_file" | sed 's/"reason": *"\(.*\)"/\1/' || echo "Unknown reason")
        fi

        CRASHED_FILES+=("$json_file")
        CRASHED_REASONS+=("$reason")
    fi
done

# Display summary
echo ""
echo "SUMMARY:"
echo "  Success: $SUCCESS_COUNT"
echo "  Crashed: $CRASHED_COUNT"
echo "  Total:   $((SUCCESS_COUNT + CRASHED_COUNT))"
echo ""

# Display crashed files and reasons
if [ $CRASHED_COUNT -gt 0 ]; then
    echo "CRASHED FILES:"
    echo "----------------------------------------"
    for i in "${!CRASHED_FILES[@]}"; do
        filename=$(basename "${CRASHED_FILES[$i]}")
        echo "[$((i+1))] $filename"
        echo "    Reason: ${CRASHED_REASONS[$i]}"
        echo ""
    done

    # Delete crashed files if -d flag is set
    if [ "$DELETE_FLAG" = true ]; then
        echo "----------------------------------------"
        echo "Deleting crashed JSON files..."
        for crashed_file in "${CRASHED_FILES[@]}"; do
            rm -f "$crashed_file"
            echo "  Deleted: $(basename "$crashed_file")"
        done
        echo ""
        echo "Deleted $CRASHED_COUNT crashed files."
    fi
else
    echo "No crashed files found."
fi

echo "----------------------------------------"
echo "Analysis complete."
