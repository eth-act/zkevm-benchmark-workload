#!/usr/bin/env bash
#
# run-gas-categorized-benchmarks.sh
#
# Runs ere-hosts benchmarks on each gas-categorized fixtures folder and outputs
# results to metrics folders with gas parameters appended.
#
# Usage:
#   ./scripts/run-gas-categorized-benchmarks.sh [OPTIONS]
#
# Options:
#   --dry-run       Show what would be executed without actually running
#   --help, -h      Show this help message
#   --force-rerun   Force rerun of benchmarks (default: true)
#   --no-force-rerun Disable force rerun
#   --action <ACTION> Benchmark action to run (default: prove)
#   --resource <RESOURCE> Resource type to use (default: gpu)
#   --guest <GUEST> Guest program type (default: stateless-executor)
#   --zkvm <ZKVM> zkVM implementation to use (default: risc0)
#   --execution-client <CLIENT> Execution client to use (default: reth)
#   --input-dir <DIR> Base input directory (default: ./zkevm-fixtures-input)
#   --gas-category <CATEGORY> Run on specific gas category only (e.g., 1M, 10M, 30M, 45M, 60M, 100M, 500M)
#   --memory-tracking <ENABLED> Enable memory tracking as a cargo feature (default: false)
#
# Examples:
#   # Run all gas categories with default settings
#   ./scripts/run-gas-categorized-benchmarks.sh
#   
#   # Run with custom action and resource
#   ./scripts/run-gas-categorized-benchmarks.sh --action execute --resource cpu
#   
#   # Run with specific zkVM and execution client
#   ./scripts/run-gas-categorized-benchmarks.sh --zkvm sp1 --execution-client ethrex
#   
#   # Run with custom input directory
#   ./scripts/run-gas-categorized-benchmarks.sh --input-dir ./my-custom-fixtures
#   
#   # Run on just one gas category
#   ./scripts/run-gas-categorized-benchmarks.sh --gas-category 10M
#   
#   # Preview what would be executed
#   ./scripts/run-gas-categorized-benchmarks.sh --dry-run
#
# Gas Categories:
#   - benchmark-gas-value_1M: 1 million gas limit
#   - benchmark-gas-value_10M: 10 million gas limit
#   - benchmark-gas-value_30M: 30 million gas limit
#   - benchmark-gas-value_45M: 45 million gas limit
#   - benchmark-gas-value_60M: 60 million gas limit
#   - benchmark-gas-value_100M: 100 million gas limit
#   - benchmark-gas-value_150M: 150 million gas limit
#

set -euo pipefail

# Default values
DRY_RUN=false
FORCE_RERUN=true
ACTION="prove"
RESOURCE="gpu"
GUEST="stateless-executor"
ZKVM="risc0"
EXECUTION_CLIENT="reth"
BASE_INPUT_DIR="./zkevm-fixtures-input"
BASE_METRICS_DIR="./zkevm-metrics"
SINGLE_GAS_CATEGORY=""
MEMORY_TRACKING=false

# Gas parameter categories
declare -a GAS_CATEGORIES=(
    "benchmark-gas-value_1M"
    "benchmark-gas-value_10M"
    "benchmark-gas-value_30M"
    "benchmark-gas-value_45M"
    "benchmark-gas-value_60M"
    "benchmark-gas-value_100M"
    "benchmark-gas-value_150M"
)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local color="$1"
    local message="$2"
    echo -e "${color}${message}${NC}"
}

# Function to show help
show_help() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Runs ere-hosts benchmarks on each gas-categorized fixtures folder and outputs"
    echo "results to metrics folders with gas parameters appended."
    echo ""
    echo "Options:"
    echo "  --dry-run              Show what would be executed without actually running"
    echo "  --help, -h             Show this help message"
    echo "  --force-rerun          Force rerun of benchmarks (default: true)"
    echo "  --no-force-rerun       Disable force rerun"
    echo "  --action <ACTION>      Benchmark action to run (default: prove)"
    echo "  --resource <RESOURCE>  Resource type to use (default: gpu)"
    echo "  --guest <GUEST>        Guest program type (default: stateless-executor)"
    echo "  --zkvm <ZKVM>          zkVM implementation to use (default: risc0)"
    echo "  --execution-client <CLIENT> Execution client to use (default: reth)"
    echo "  --input-dir <DIR>      Base input directory (default: ./zkevm-fixtures-input)"
    echo "  --gas-category <CATEGORY> Run on specific gas category only (e.g., 1M, 10M, 30M, 45M, 60M, 100M, 150M)"
    echo "  --memory-tracking <ENABLED> Enable memory tracking as a cargo feature (default: false)"
    echo ""
    echo "Available zkVM Features:"
    echo "  - risc0: RISC0 zkVM implementation (default)"
    echo "  - sp1: SP1 zkVM implementation"
    echo "  - openvm: OpenVM zkVM implementation"
    echo "  - pico: Pico zkVM implementation"
    echo "  - zisk: Zisk zkVM implementation"
    echo "  - airbender: Airbender zkVM implementation"
    echo "  - zkm: ZKM zkVM implementation"
    echo ""
    echo "Available Execution Clients:"
    echo "  - reth: Reth execution client (default)"
    echo "  - ethrex: Ethrex execution client"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Run all gas categories with defaults"
    echo "  $0 --action execute --resource cpu    # Run with custom action and resource"
    echo "  $0 --zkvm sp1 --execution-client ethrex # Run with specific zkVM and client"
    echo "  $0 --input-dir ./my-custom-fixtures   # Run with custom input directory"
    echo "  $0 --gas-category 10M                 # Run on just one gas category"
    echo "  $0 --dry-run                          # Show what would be executed"
    echo ""
    echo "Gas Categories:"
    echo "  - benchmark-gas-value_1M: 1 million gas limit"
    echo "  - benchmark-gas-value_10M: 10 million gas limit"
    echo "  - benchmark-gas-value_30M: 30 million gas limit"
    echo "  - benchmark-gas-value_45M: 45 million gas limit"
    echo "  - benchmark-gas-value_60M: 60 million gas limit"
    echo "  - benchmark-gas-value_100M: 100 million gas limit"
    echo "  - benchmark-gas-value_150M: 150 million gas limit"
    exit 0
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --help|-h)
            show_help
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --force-rerun)
            FORCE_RERUN=true
            shift
            ;;
        --no-force-rerun)
            FORCE_RERUN=false
            shift
            ;;
        --action)
            ACTION="$2"
            shift 2
            ;;
        --resource)
            RESOURCE="$2"
            shift 2
            ;;
        --guest)
            GUEST="$2"
            shift 2
            ;;
        --zkvm)
            ZKVM="$2"
            shift 2
            ;;
        --execution-client)
            EXECUTION_CLIENT="$2"
            shift 2
            ;;
        --input-dir)
            BASE_INPUT_DIR="$2"
            shift 2
            ;;
        --gas-category)
            SINGLE_GAS_CATEGORY="$2"
            shift 2
            ;;
        --memory-tracking)
            if [ "$2" = "true" ]; then
                MEMORY_TRACKING=true
            elif [ "$2" = "false" ]; then
                MEMORY_TRACKING=false
            else
                echo "Error: --memory-tracking must be 'true' or 'false', got: $2"
                exit 1
            fi
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            ;;
    esac
done

# Function to check if cargo is available
check_cargo() {
    if ! command -v cargo &> /dev/null; then
        print_status "$RED" "❌ Error: cargo is not installed or not in PATH"
        exit 1
    fi
}

# Function to validate gas category
validate_gas_category() {
    if [ -n "$SINGLE_GAS_CATEGORY" ]; then
        local valid_categories=("1M" "10M" "30M" "45M" "60M" "100M" "150M")
        local is_valid=false
        
        for valid_cat in "${valid_categories[@]}"; do
            if [ "$SINGLE_GAS_CATEGORY" = "$valid_cat" ]; then
                is_valid=true
                break
            fi
        done
        
        if [ "$is_valid" = false ]; then
            print_status "$RED" "❌ Error: Invalid gas category '$SINGLE_GAS_CATEGORY'"
            print_status "$RED" "   Valid categories: ${valid_categories[*]}"
            exit 1
        fi
    fi
}

# Function to get categories to run
get_categories_to_run() {
    if [ -n "$SINGLE_GAS_CATEGORY" ]; then
        echo "benchmark-gas-value_${SINGLE_GAS_CATEGORY}"
    else
        echo "${GAS_CATEGORIES[@]}"
    fi
}

# Function to check if we're in the right directory
check_workspace() {
    if [ ! -f "Cargo.toml" ]; then
        print_status "$RED" "❌ Error: Cargo.toml not found. This script must be run from the project root directory"
        exit 1
    fi
    
    if [ ! -d "crates/ere-hosts" ]; then
        print_status "$RED" "❌ Error: ere-hosts crate not found. This script must be run from the project root directory"
        exit 1
    fi
    
    print_status "$GREEN" "✅ Project structure verified"
}

# Function to build the project if needed
build_project() {
    local features=""
    if [ "$MEMORY_TRACKING" = true ]; then
        features="memory-tracking"
    fi
    
    if [ -n "$features" ]; then
        print_status "$BLUE" "🔨 Building ere-hosts with features: $features..."
        if cargo build --release --bin ere-hosts --features "$features"; then
            print_status "$GREEN" "✅ Build successful"
        else
            print_status "$RED" "❌ Build failed"
            exit 1
        fi
    else
        print_status "$BLUE" "🔨 Building ere-hosts..."
        if cargo build --release --bin ere-hosts; then
            print_status "$GREEN" "✅ Build successful"
        else
            print_status "$RED" "❌ Build failed"
            exit 1
        fi
    fi
}

# Function to check if input fixtures exist
check_input_fixtures() {
    local missing_folders=()
    
    for category in "${GAS_CATEGORIES[@]}"; do
        local gas_value=$(echo "$category" | sed 's/benchmark-gas-value_//')
        local input_dir="${BASE_INPUT_DIR}-${gas_value}"
        
        if [ ! -d "$input_dir" ]; then
            missing_folders+=("$input_dir")
        fi
    done
    
    if [ ${#missing_folders[@]} -gt 0 ]; then
        print_status "$YELLOW" "⚠️  Warning: Some input fixture folders are missing:"
        for folder in "${missing_folders[@]}"; do
            print_status "$YELLOW" "  - $folder"
        done
        print_status "$YELLOW" "  Run './scripts/generate-gas-categorized-fixtures.sh' first to generate fixtures."
        print_status "$YELLOW" "  Continuing with available folders..."
    fi
}

# Function to run benchmark for a specific gas category
run_benchmark() {
    local category="$1"
    local gas_value=$(echo "$category" | sed 's/benchmark-gas-value_//')
    local input_dir="${BASE_INPUT_DIR}-${gas_value}"
    local metrics_dir="${BASE_METRICS_DIR}-${ZKVM}-${gas_value}"
    
    # Check if input directory exists
    if [ ! -d "$input_dir" ]; then
        print_status "$YELLOW" "⚠️  Skipping $category: Input directory $input_dir not found"
        return 1
    fi
    
    print_status "$BLUE" "🚀 Running benchmark for gas category: $category"
    print_status "$BLUE" "📁 Input directory: $input_dir"
    print_status "$BLUE" "📊 Metrics output: $metrics_dir"
    
    # Create metrics directory if it doesn't exist
    mkdir -p "$metrics_dir"
    
    # Build force-rerun argument
    local force_arg=""
    if [ "$FORCE_RERUN" = true ]; then
        force_arg="--force-rerun"
    fi
    
    # Build features list (memory-tracking is a cargo feature, not a CLI arg)
    local features=""
    if [ "$MEMORY_TRACKING" = true ]; then
        features="memory-tracking"
    fi
    
    # Run the benchmark
    local cmd_args=(
        --release
        --bin ere-hosts
    )
    
    if [ -n "$features" ]; then
        cmd_args+=(--features "$features")
    fi
    
    cmd_args+=(
        --
        --zkvms "$ZKVM"
        -a "$ACTION"
        -r "$RESOURCE"
    )
    
    if [ -n "$force_arg" ]; then
        cmd_args+=("$force_arg")
    fi
    
    cmd_args+=(
        -o "$metrics_dir"
        "$GUEST"
        --input-folder "$input_dir"
        --execution-client "$EXECUTION_CLIENT"
    )
    
    if cargo run "${cmd_args[@]}"; then
        print_status "$GREEN" "✅ Successfully completed benchmark for $category"
        
        # Count the generated metric files
        local file_count=$(find "$metrics_dir" -type f 2>/dev/null | wc -l)
        print_status "$GREEN" "📊 Generated $file_count metric files in $metrics_dir"
    else
        print_status "$RED" "❌ Failed to complete benchmark for $category"
        return 1
    fi
}

# Function to show summary
show_summary() {
    print_status "$GREEN" "\n🎉 Benchmark execution completed!"
    print_status "$BLUE" "\n📊 Summary of benchmark results:"
    
    local categories_to_run=($(get_categories_to_run))
    for category in "${categories_to_run[@]}"; do
        local gas_value=$(echo "$category" | sed 's/benchmark-gas-value_//')
        local input_dir="${BASE_INPUT_DIR}-${gas_value}"
        local metrics_dir="${BASE_METRICS_DIR}-${ZKVM}-${gas_value}"
        
        if [ -d "$input_dir" ] && [ -d "$metrics_dir" ]; then
            local metric_file_count=$(find "$metrics_dir" -type f 2>/dev/null | wc -l)
            print_status "$GREEN" "  ✅ $category: $metric_file_count metric files in $metrics_dir"
        elif [ ! -d "$input_dir" ]; then
            print_status "$YELLOW" "  ⚠️  $category: Input directory $input_dir not found"
        else
            print_status "$RED" "  ❌ $category: Failed or no metrics generated"
        fi
    done
    
    print_status "$BLUE" "\n📁 All metrics are located in: $BASE_METRICS_DIR-$ZKVM-*"
}

# Main execution
main() {
    if [ "$DRY_RUN" = true ]; then
        print_status "$YELLOW" "🔍 DRY RUN MODE - No actual execution will occur"
        print_status "$BLUE" "🚀 Would start ere-hosts benchmarks for gas categories..."
        print_status "$BLUE" "📊 Action: $ACTION"
        print_status "$BLUE" "🖥️  Resource: $RESOURCE"
        print_status "$BLUE" "🎯 Guest: $GUEST"
        print_status "$BLUE" "🔧 zkVM: $ZKVM"
        print_status "$BLUE" "⚙️  Execution Client: $EXECUTION_CLIENT"
        print_status "$BLUE" "📁 Input Directory: $BASE_INPUT_DIR"
        if [ -n "$SINGLE_GAS_CATEGORY" ]; then
            print_status "$BLUE" "🎯 Gas Category: $SINGLE_GAS_CATEGORY (single category mode)"
        else
            print_status "$BLUE" "🎯 Gas Categories: All available categories"
        fi
        print_status "$BLUE" "🔄 Force Rerun: $FORCE_RERUN"
        print_status "$BLUE" "🧠 Memory Tracking: $MEMORY_TRACKING"
        print_status "$BLUE" "\n📋 Would execute the following commands:"
        
        local categories_to_run=($(get_categories_to_run))
        for category in "${categories_to_run[@]}"; do
            local gas_value=$(echo "$category" | sed 's/benchmark-gas-value_//')
            local input_dir="${BASE_INPUT_DIR}-${gas_value}"
            local metrics_dir="${BASE_METRICS_DIR}-${ZKVM}-${gas_value}"
            
            # Build features list
            local features=""
            if [ "$MEMORY_TRACKING" = true ]; then
                features="memory-tracking"
            fi
            
            local force_arg=""
            if [ "$FORCE_RERUN" = true ]; then
                force_arg="--force-rerun"
            fi
            
            local cmd_preview="cargo run --release --bin ere-hosts"
            if [ -n "$features" ]; then
                cmd_preview="$cmd_preview --features $features"
            fi
            cmd_preview="$cmd_preview -- --zkvms $ZKVM -a $ACTION -r $RESOURCE"
            if [ -n "$force_arg" ]; then
                cmd_preview="$cmd_preview $force_arg"
            fi
            cmd_preview="$cmd_preview -o \"$metrics_dir\" $GUEST --input-folder \"$input_dir\" --execution-client $EXECUTION_CLIENT"
            
            print_status "$BLUE" "  $cmd_preview"
            print_status "$BLUE" "  # Input: $input_dir"
        done
        
        print_status "$GREEN" "\n✅ Dry run completed. Use without --dry-run to execute."
        exit 0
    fi
    
    print_status "$BLUE" "🚀 Starting ere-hosts benchmarks for gas categories..."
    print_status "$BLUE" "📊 Action: $ACTION"
    print_status "$BLUE" "🖥️  Resource: $RESOURCE"
    print_status "$BLUE" "🎯 Guest: $GUEST"
    print_status "$BLUE" "🔧 zkVM: $ZKVM"
    print_status "$BLUE" "⚙️  Execution Client: $EXECUTION_CLIENT"
    print_status "$BLUE" "📁 Input Directory: $BASE_INPUT_DIR"
    if [ -n "$SINGLE_GAS_CATEGORY" ]; then
        print_status "$BLUE" "🎯 Gas Category: $SINGLE_GAS_CATEGORY (single category mode)"
    else
        print_status "$BLUE" "🎯 Gas Categories: All available categories"
    fi
    print_status "$BLUE" "🔄 Force Rerun: $FORCE_RERUN"
    print_status "$BLUE" "🧠 Memory Tracking: $MEMORY_TRACKING"
    
    # Pre-flight checks
    check_cargo
    check_workspace
    validate_gas_category
    
    # Check input fixtures
    check_input_fixtures
    
    # Build the project
    build_project
    
    # Run benchmarks for each gas category
    local failed_categories=()
    local categories_to_run=($(get_categories_to_run))
    
    for category in "${categories_to_run[@]}"; do
        if run_benchmark "$category"; then
            print_status "$GREEN" "✅ Completed: $category"
        else
            print_status "$RED" "❌ Failed: $category"
            failed_categories+=("$category")
        fi
        
        echo "" # Add spacing between categories
    done
    
    # Show summary
    show_summary
    
    # Exit with error if any categories failed
    if [ ${#failed_categories[@]} -gt 0 ]; then
        print_status "$YELLOW" "\n⚠️  Some gas categories failed to complete:"
        for category in "${failed_categories[@]}"; do
            print_status "$YELLOW" "  - $category"
        done
        exit 1
    fi
    
    print_status "$GREEN" "\n🎯 All gas categories completed successfully!"
}

# Run main function
main "$@"