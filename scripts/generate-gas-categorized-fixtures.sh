#!/usr/bin/env bash
# generate-gas-categorized-fixtures.sh
# Generates zkevm-fixtures-input categorized by gas parameters into different folders.
#
# Usage: ./scripts/generate-gas-categorized-fixtures.sh [OPTIONS] [EEST_TAG] [BASE_OUTPUT_DIR]

set -euo pipefail

# Default Rayon thread count (can be overridden by environment)
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-4}"

# Colors
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; BLUE='\033[0;34m'; NC='\033[0m'
print_status() { echo -e "${1}${2}${NC}"; }

# Default gas values (in millions, without suffix)
DEFAULT_GAS_VALUES="1M,10M,30M,45M,60M,100M,150M"

# Gas parameter categories (will be populated from user input or defaults)
declare -a GAS_CATEGORIES=()

# Help message
show_help() {
    cat << EOF
Usage: $0 [OPTIONS] [EEST_TAG] [BASE_OUTPUT_DIR]

Generates zkevm-fixtures-input categorized by gas parameters.

Arguments:
  EEST_TAG        EEST release tag (e.g., v0.1.0). Default: latest (ignored when using --eest-fixtures-path)
  BASE_OUTPUT_DIR Base output directory (default: ./zkevm-fixtures-input)

Options:
  -g, --gas LIST            Comma-separated list of gas values in format xM where x is a rational number
                            (e.g., "0.1M,1M,10M" or "0.1,1,10" - M suffix added if omitted)
                            Default: $DEFAULT_GAS_VALUES
  -e, --eest-fixtures-path  Path to local EEST fixtures directory (mutually exclusive with EEST_TAG)
  --dry-run                 Show what would be executed without running
  --help, -h                Show this help message

Examples:
  $0                                              # Use default gas categories, download latest EEST
  $0 -g 1M,10M,30M                                # Custom gas categories, download latest EEST
  $0 -g 1M,10M v0.1.0                             # Custom gas categories with specific EEST tag
  $0 -e ./local-eest-fixtures -g 0.1M,1M,10M      # Use local EEST fixtures path
  $0 --eest-fixtures-path /path/to/eest           # Use local EEST fixtures with defaults
EOF
    exit 0
}

# Normalize gas value to format xM where x is a rational number
normalize_gas_value() {
    local value="$1"
    # Remove any whitespace
    value="${value// /}"
    # Check if value is in format xM (rational number with M suffix)
    if [[ "$value" =~ ^[0-9]+(\.[0-9]+)?M$ ]]; then
        echo "$value"
    elif [[ "$value" =~ ^[0-9]+(\.[0-9]+)?$ ]]; then
        # No suffix, add M
        echo "${value}M"
    else
        echo "ERROR:$value" # Signal error to caller
        return 1
    fi
}

# Parse gas categories from comma-separated list
parse_gas_categories() {
    local gas_input="$1"
    IFS=',' read -ra values <<< "$gas_input"
    for value in "${values[@]}"; do
        local normalized
        normalized=$(normalize_gas_value "$value")
        if [[ "$normalized" == ERROR:* ]]; then
            local bad_value="${normalized#ERROR:}"
            print_status "$RED" "❌ Invalid gas value format: $bad_value (expected xM where x is a rational number, e.g., 0.1M, 1M, 10M)"
            return 1
        fi
        GAS_CATEGORIES+=("benchmark-gas-value_${normalized}")
    done
}

# Parse arguments
DRY_RUN=false
GAS_INPUT=""
EEST_FIXTURES_PATH=""
POSITIONAL_ARGS=()

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        -g|--gas)
            GAS_INPUT="$2"
            shift 2
            ;;
        -e|--eest-fixtures-path)
            EEST_FIXTURES_PATH="$2"
            shift 2
            ;;
        *)
            POSITIONAL_ARGS+=("$1")
            shift
            ;;
    esac
done

# Set positional arguments
EEST_TAG="${POSITIONAL_ARGS[0]:-}"
BASE_OUTPUT_DIR="${POSITIONAL_ARGS[1]:-./zkevm-fixtures-input}"

# Validate mutually exclusive options
if [[ -n "$EEST_TAG" && -n "$EEST_FIXTURES_PATH" ]]; then
    print_status "$RED" "❌ Cannot use both EEST_TAG and --eest-fixtures-path. They are mutually exclusive."
    exit 1
fi

# Parse gas categories (use default if not provided)
parse_gas_categories "${GAS_INPUT:-$DEFAULT_GAS_VALUES}" || exit 1

# Validation functions
validate_gas_categories() {
    print_status "$BLUE" "🔍 Validating gas categories..."
    if [[ ${#GAS_CATEGORIES[@]} -eq 0 ]]; then
        print_status "$RED" "❌ No gas categories specified"
        return 1
    fi
    for category in "${GAS_CATEGORIES[@]}"; do
        [[ "$category" =~ ^benchmark-gas-value_[0-9]+(\.[0-9]+)?M$ ]] || {
            print_status "$RED" "❌ Invalid format: $category"
            return 1
        }
    done
    print_status "$GREEN" "✅ ${#GAS_CATEGORIES[@]} gas categories validated: ${GAS_CATEGORIES[*]}"
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

# Validate EEST fixtures path exists
check_eest_fixtures_path() {
    if [[ ! -d "$EEST_FIXTURES_PATH" ]]; then
        print_status "$RED" "❌ EEST fixtures directory not found: $EEST_FIXTURES_PATH"
        exit 1
    fi
    print_status "$GREEN" "✅ EEST fixtures directory verified: $EEST_FIXTURES_PATH"
}

# Generate fixtures for a specific gas category
generate_fixtures() {
    local gas_category="$1" output_dir="$2"
    local source_arg=""
    
    # Use either --eest-fixtures-path or --tag (mutually exclusive)
    if [[ -n "$EEST_FIXTURES_PATH" ]]; then
        source_arg="--eest-fixtures-path $EEST_FIXTURES_PATH"
    elif [[ -n "$EEST_TAG" ]]; then
        source_arg="--tag $EEST_TAG"
    fi
    
    print_status "$BLUE" "🚀 Generating: $gas_category → $output_dir"
    mkdir -p "$output_dir"
    
    cargo run --release --bin witness-generator-cli -- \
        --output-folder "$output_dir" tests $source_arg \
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
    # Determine source description for display
    local source_desc="EEST Tag: ${EEST_TAG:-latest}"
    [[ -n "$EEST_FIXTURES_PATH" ]] && source_desc="EEST Path: $EEST_FIXTURES_PATH"
    
    if [[ "$DRY_RUN" == true ]]; then
        print_status "$YELLOW" "🔍 DRY RUN MODE"
        print_status "$BLUE" "$source_desc"
        print_status "$BLUE" "Base Output: $BASE_OUTPUT_DIR"
        print_status "$BLUE" "Gas Categories: ${GAS_CATEGORIES[*]}\n"
        
        for category in "${GAS_CATEGORIES[@]}"; do
            local gas_value="${category#benchmark-gas-value_}"
            local output_dir="${BASE_OUTPUT_DIR}-${gas_value}"
            local source_arg=""
            [[ -n "$EEST_FIXTURES_PATH" ]] && source_arg="--eest-fixtures-path \"$EEST_FIXTURES_PATH\""
            [[ -n "$EEST_TAG" ]] && source_arg="--tag $EEST_TAG"
            echo "  cargo run --release --bin witness-generator-cli -- --output-folder \"$output_dir\" tests $source_arg --include \"$category\" --include \"Prague\""
        done
        print_status "$GREEN" "\n✅ Dry run completed"
        exit 0
    fi
    
    print_status "$BLUE" "🚀 Starting zkEVM fixture generation"
    print_status "$BLUE" "$source_desc | Base Dir: $BASE_OUTPUT_DIR"
    print_status "$BLUE" "Gas Categories: ${GAS_CATEGORIES[*]}"
    
    check_cargo && check_workspace
    validate_gas_categories || exit 1
    [[ -n "$EEST_FIXTURES_PATH" ]] && check_eest_fixtures_path
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
