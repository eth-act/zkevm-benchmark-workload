#!/usr/bin/env bash
# generate-gas-categorized-fixtures.sh
# Generates zkevm-fixtures-input categorized by gas parameters into different folders.
#
# Usage: ./scripts/generate-gas-categorized-fixtures.sh [EEST_TAG] [BASE_OUTPUT_DIR]

set -euo pipefail

# Colors
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; BLUE='\033[0;34m'; NC='\033[0m'
print_status() { echo -e "${1}${2}${NC}"; }

# Gas parameter categories
declare -a GAS_CATEGORIES=(
    "benchmark-gas-value_1M" "benchmark-gas-value_10M" "benchmark-gas-value_30M"
    "benchmark-gas-value_45M" "benchmark-gas-value_60M" "benchmark-gas-value_100M"
    "benchmark-gas-value_150M"
)

# Help message
show_help() {
    cat << EOF
Usage: $0 [EEST_TAG] [BASE_OUTPUT_DIR] [OPTIONS]

Generates zkevm-fixtures-input categorized by gas parameters.

Arguments:
  EEST_TAG        EEST release tag (e.g., v0.1.0). Default: latest
  BASE_OUTPUT_DIR Base output directory (default: ./zkevm-fixtures-input)

Options:
  --dry-run       Show what would be executed without running
  --help, -h      Show this help message

Gas Categories: 1M, 10M, 30M, 45M, 60M, 100M, 150M
EOF
    exit 0
}

# Parse arguments
DRY_RUN=false
[[ "${1:-}" =~ ^(-h|--help)$ ]] && show_help
[[ "${1:-}" == "--dry-run" ]] && { DRY_RUN=true; shift; }
EEST_TAG="${1:-}"
BASE_OUTPUT_DIR="${2:-./zkevm-fixtures-input}"

# Validation functions
validate_gas_categories() {
    print_status "$BLUE" "🔍 Validating gas categories..."
    for category in "${GAS_CATEGORIES[@]}"; do
        [[ "$category" =~ ^benchmark-gas-value_[0-9]+[KMG]$ ]] || {
            print_status "$RED" "❌ Invalid format: $category"
            return 1
        }
    done
    print_status "$GREEN" "✅ All gas categories validated"
}

check_cargo() {
    command -v cargo &> /dev/null || {
        print_status "$RED" "❌ cargo not installed or not in PATH"
        exit 1
    }
}

check_workspace() {
    [[ -f "Cargo.toml" && -d "crates/witness-generator-cli" ]] || {
        print_status "$RED" "❌ Must be run from project root directory"
        exit 1
    }
    print_status "$GREEN" "✅ Project structure verified"
}

build_project() {
    print_status "$BLUE" "🔨 Building witness-generator-cli..."
    cargo build --release --bin witness-generator-cli && {
        print_status "$GREEN" "✅ Build successful"
    } || {
        print_status "$RED" "❌ Build failed"
        exit 1
    }
}

# Generate fixtures for a specific gas category
generate_fixtures() {
    local gas_category="$1" output_dir="$2"
    local eest_tag_arg=""
    [[ -n "$EEST_TAG" ]] && eest_tag_arg="--tag $EEST_TAG"
    
    print_status "$BLUE" "🚀 Generating: $gas_category → $output_dir"
    mkdir -p "$output_dir"
    
    cargo run --release --bin witness-generator-cli -- \
        --output-folder "$output_dir" tests $eest_tag_arg \
        --include "$gas_category" --include "Prague" && {
        local file_count=$(find "$output_dir" -type f 2>/dev/null | wc -l)
        print_status "$GREEN" "✅ $gas_category: $file_count files"
    } || {
        print_status "$RED" "❌ Failed: $gas_category"
        return 1
    }
}

# Show summary
show_summary() {
    print_status "$GREEN" "\n🎉 Fixture generation completed!"
    print_status "$BLUE" "\n📊 Summary:"
    for category in "${GAS_CATEGORIES[@]}"; do
        local gas_value="${category#benchmark-gas-value_}"
        local output_dir="${BASE_OUTPUT_DIR}-${gas_value}"
        if [[ -d "$output_dir" ]]; then
            local file_count=$(find "$output_dir" -type f 2>/dev/null | wc -l)
            print_status "$GREEN" "  ✅ $category: $file_count files"
        else
            print_status "$RED" "  ❌ $category: Failed"
        fi
    done
    print_status "$BLUE" "\n📁 Fixtures location: $BASE_OUTPUT_DIR-*"
}

# Main execution
main() {
    if [[ "$DRY_RUN" == true ]]; then
        print_status "$YELLOW" "🔍 DRY RUN MODE"
        print_status "$BLUE" "EEST Tag: ${EEST_TAG:-latest}"
        print_status "$BLUE" "Base Output: $BASE_OUTPUT_DIR\n"
        for category in "${GAS_CATEGORIES[@]}"; do
            local gas_value="${category#benchmark-gas-value_}"
            local output_dir="${BASE_OUTPUT_DIR}-${gas_value}"
            local tag_arg=""; [[ -n "$EEST_TAG" ]] && tag_arg="--tag $EEST_TAG"
            echo "  cargo run --release --bin witness-generator-cli -- --output-folder \"$output_dir\" tests $tag_arg --include \"$category\" --include \"Prague\""
        done
        print_status "$GREEN" "\n✅ Dry run completed"
        exit 0
    fi
    
    print_status "$BLUE" "🚀 Starting zkEVM fixture generation"
    print_status "$BLUE" "EEST Tag: ${EEST_TAG:-latest} | Base Dir: $BASE_OUTPUT_DIR"
    
    check_cargo && check_workspace
    validate_gas_categories || exit 1
    build_project
    
    local failed_categories=()
    for category in "${GAS_CATEGORIES[@]}"; do
        local gas_value="${category#benchmark-gas-value_}"
        local output_dir="${BASE_OUTPUT_DIR}-${gas_value}"
        generate_fixtures "$category" "$output_dir" || failed_categories+=("$category")
        echo ""
    done
    
    show_summary
    
    if [[ ${#failed_categories[@]} -gt 0 ]]; then
        print_status "$YELLOW" "\n⚠️  Failed categories:"
        printf '%s\n' "${failed_categories[@]}" | sed 's/^/  - /'
        exit 1
    fi
    
    print_status "$GREEN" "\n🎯 All gas categories completed successfully!"
}

main "$@"
