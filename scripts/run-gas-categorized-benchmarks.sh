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
#   -n, --dry-run                Show what would be executed without actually running
#   -h, --help                   Show this help message
#   -f, --force-rerun            Force rerun of benchmarks (default: false)
#       --no-force-rerun         Disable force rerun
#   -a, --action <ACTION>        Benchmark action to run (default: prove)
#   -r, --resource <RESOURCE>    Resource type: cpu, gpu, network (default: gpu)
#   -g, --guest <GUEST>          Guest program type (default: stateless-executor)
#   -z, --zkvm <ZKVM>            zkVM implementation to use (default: risc0)
#   -e, --execution-client <CLIENT> Execution client to use (default: reth)
#   -i, --input-dir <DIR>        Base input directory (default: ./zkevm-fixtures-input)
#   -o, --output-dir <DIR>       Base metrics output directory (default: ./zkevm-metrics)
#   -c, --gas-categories <LIST>  Comma-separated list of gas categories (e.g., 0.5M,1M,2.5M,10M)
#   -m, --memory-tracking        Enable memory tracking as a cargo feature
#
# Examples:
#   # Run all gas categories with default settings
#   ./scripts/run-gas-categorized-benchmarks.sh
#   
#   # Run with custom action and resource (short form)
#   ./scripts/run-gas-categorized-benchmarks.sh -a execute -r cpu
#   
#   # Run with specific zkVM and execution client
#   ./scripts/run-gas-categorized-benchmarks.sh -z sp1 -e ethrex
#   
#   # Run with SP1 network proving (NETWORK_PRIVATE_KEY env var is optional)
#   ./scripts/run-gas-categorized-benchmarks.sh -z sp1 -r network
#   
#   # Run with custom input and output directories
#   ./scripts/run-gas-categorized-benchmarks.sh -i ./my-fixtures -o ./my-metrics
#   
#   # Run on specific gas categories (supports rational numbers like 0.5M, 2.5M)
#   ./scripts/run-gas-categorized-benchmarks.sh -c 10M
#   ./scripts/run-gas-categorized-benchmarks.sh -c 0.5M,1M,2.5M,10M
#   
#   # Preview what would be executed
#   ./scripts/run-gas-categorized-benchmarks.sh -n
#
# Gas Categories:
#   Format: xM where x is a rational number (e.g., 0.5M, 1M, 2.5M, 10M, 100M)
#   Default categories: 1M, 10M, 30M, 45M, 60M, 100M, 150M
#

set -euo pipefail

# SP1 Network proving environment variables
# Set default NETWORK_RPC_URL if not already set
export NETWORK_RPC_URL="${NETWORK_RPC_URL:-http://127.0.0.1:50051/}"
# NETWORK_PRIVATE_KEY is optional - if set, it will be used for authenticated proving

# Default values
DRY_RUN=false
FORCE_RERUN=false
ACTION="prove"
RESOURCE="gpu"
GUEST="stateless-executor"
ZKVM="risc0"
EXECUTION_CLIENT="reth"
BASE_INPUT_DIR="./zkevm-fixtures-input"
BASE_METRICS_DIR="./zkevm-metrics"
CUSTOM_GAS_CATEGORIES=""
MEMORY_TRACKING=false

# Default gas parameter categories (used when no custom categories are specified)
declare -a DEFAULT_GAS_CATEGORIES=(
    "1M"
    "10M"
    "30M"
    "45M"
    "60M"
    "100M"
    "150M"
)

# Active gas categories (will be populated from defaults or user input)
declare -a GAS_CATEGORIES=()

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
    echo "  -n, --dry-run                    Show what would be executed without running"
    echo "  -h, --help                       Show this help message"
    echo "  -f, --force-rerun                Force rerun of benchmarks"
    echo "      --no-force-rerun             Disable force rerun (default)"
    echo "  -a, --action <ACTION>            Benchmark action (default: prove)"
    echo "  -r, --resource <RESOURCE>        Resource type: cpu, gpu, network (default: gpu)"
    echo "                                   Note: 'network' requires SP1 zkVM"
    echo "                                   Default NETWORK_RPC_URL: http://127.0.0.1:50051/"
    echo "                                   NETWORK_PRIVATE_KEY env var is optional"
    echo "  -g, --guest <GUEST>              Guest program (default: stateless-executor)"
    echo "  -z, --zkvm <ZKVM>                zkVM implementation (default: risc0)"
    echo "  -e, --execution-client <CLIENT>  Execution client (default: reth)"
    echo "  -i, --input-dir <DIR>            Base input directory (default: ./zkevm-fixtures-input)"
    echo "  -o, --output-dir <DIR>           Base metrics output directory (default: ./zkevm-metrics)"
    echo "  -c, --gas-categories <LIST>      Comma-separated gas categories (e.g., 0.5M,1M,10M)"
    echo "  -m, --memory-tracking            Enable memory tracking cargo feature"
    echo ""
    echo "Available zkVMs:"
    echo "  risc0 (default), sp1, openvm, pico, zisk, airbender, zkm"
    echo "  Note: SP1 is required when using 'network' resource for proving on SP1 Network"
    echo ""
    echo "Available Resources:"
    echo "  cpu, gpu (default), network (SP1 only)"
    echo ""
    echo "Available Execution Clients:"
    echo "  reth (default), ethrex"
    echo ""
    echo "Examples:"
    echo "  $0                               # Run all default gas categories"
    echo "  $0 -a execute -r cpu             # Custom action and resource"
    echo "  $0 -z sp1 -e ethrex              # Specific zkVM and client"
    echo "  $0 -z sp1 -r network             # SP1 with network proving"
    echo "  $0 -i ./fixtures -o ./metrics    # Custom input/output directories"
    echo "  $0 -c 10M                        # Single gas category"
    echo "  $0 -c 0.5M,1M,2.5M,10M           # Multiple custom categories"
    echo "  $0 -n                            # Dry run preview"
    echo "  $0 -f -m -c 30M                  # Force rerun with memory tracking"
    echo ""
    echo "Gas Categories:"
    echo "  Format: xM where x is a rational number (e.g., 0.5M, 1M, 2.5M, 10M)"
    echo "  Default: 1M, 10M, 30M, 45M, 60M, 100M, 150M"
    exit 0
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            ;;
        -n|--dry-run)
            DRY_RUN=true
            shift
            ;;
        -f|--force-rerun)
            FORCE_RERUN=true
            shift
            ;;
        --no-force-rerun)
            FORCE_RERUN=false
            shift
            ;;
        -a|--action)
            ACTION="$2"
            shift 2
            ;;
        -r|--resource)
            RESOURCE="$2"
            shift 2
            ;;
        -g|--guest)
            GUEST="$2"
            shift 2
            ;;
        -z|--zkvm)
            ZKVM="$2"
            shift 2
            ;;
        -e|--execution-client)
            EXECUTION_CLIENT="$2"
            shift 2
            ;;
        -i|--input-dir)
            BASE_INPUT_DIR="$2"
            shift 2
            ;;
        -o|--output-dir)
            BASE_METRICS_DIR="$2"
            shift 2
            ;;
        -c|--gas-categories)
            CUSTOM_GAS_CATEGORIES="$2"
            shift 2
            ;;
        -m|--memory-tracking)
            MEMORY_TRACKING=true
            shift
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

# Function to validate a single gas category format (xM where x is a rational number)
validate_gas_category_format() {
    local category="$1"
    # Match pattern: optional digits, optional decimal point with digits, followed by M
    # Valid examples: 1M, 10M, 0.5M, 2.5M, 100M, 0.25M
    if [[ "$category" =~ ^[0-9]*\.?[0-9]+M$ ]]; then
        return 0
    else
        return 1
    fi
}

# Function to parse and validate gas categories
parse_gas_categories() {
    if [ -n "$CUSTOM_GAS_CATEGORIES" ]; then
        # Split by comma and validate each category
        IFS=',' read -ra categories <<< "$CUSTOM_GAS_CATEGORIES"
        
        for category in "${categories[@]}"; do
            # Trim whitespace
            category=$(echo "$category" | xargs)
            
            if ! validate_gas_category_format "$category"; then
                print_status "$RED" "❌ Error: Invalid gas category format '$category'"
                print_status "$RED" "   Expected format: xM where x is a rational number (e.g., 0.5M, 1M, 2.5M, 10M)"
                exit 1
            fi
            
            GAS_CATEGORIES+=("$category")
        done
        
        if [ ${#GAS_CATEGORIES[@]} -eq 0 ]; then
            print_status "$RED" "❌ Error: No valid gas categories provided"
            exit 1
        fi
    else
        # Use default categories
        GAS_CATEGORIES=("${DEFAULT_GAS_CATEGORIES[@]}")
    fi
}

# Function to get categories to run (returns the gas values without prefix)
get_categories_to_run() {
    echo "${GAS_CATEGORIES[@]}"
}

# Function to validate resource and zkVM combination
validate_resource_zkvm() {
    if [[ "$RESOURCE" == "network" && "$ZKVM" != "sp1" ]]; then
        print_status "$RED" "❌ Error: Network resource is only supported for SP1 zkVM"
        print_status "$RED" "   Use: -z sp1 -r network"
        exit 1
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
    
    for gas_value in "${GAS_CATEGORIES[@]}"; do
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
    local gas_value="$1"
    local input_dir="${BASE_INPUT_DIR}-${gas_value}"
    local metrics_dir="${BASE_METRICS_DIR}-${ZKVM}-${gas_value}"
    
    # Check if input directory exists
    if [ ! -d "$input_dir" ]; then
        print_status "$YELLOW" "⚠️  Skipping $gas_value: Input directory $input_dir not found"
        return 1
    fi
    
    print_status "$BLUE" "🚀 Running benchmark for gas category: $gas_value"
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
        print_status "$GREEN" "✅ Successfully completed benchmark for $gas_value"
        
        # Count the generated metric files
        local file_count=$(find "$metrics_dir" -type f 2>/dev/null | wc -l)
        print_status "$GREEN" "📊 Generated $file_count metric files in $metrics_dir"
    else
        print_status "$RED" "❌ Failed to complete benchmark for $gas_value"
        return 1
    fi
}

# Function to show summary
show_summary() {
    print_status "$GREEN" "\n🎉 Benchmark execution completed!"
    print_status "$BLUE" "\n📊 Summary of benchmark results:"
    
    for gas_value in "${GAS_CATEGORIES[@]}"; do
        local input_dir="${BASE_INPUT_DIR}-${gas_value}"
        local metrics_dir="${BASE_METRICS_DIR}-${ZKVM}-${gas_value}"
        
        if [ -d "$input_dir" ] && [ -d "$metrics_dir" ]; then
            local metric_file_count=$(find "$metrics_dir" -type f 2>/dev/null | wc -l)
            print_status "$GREEN" "  ✅ $gas_value: $metric_file_count metric files in $metrics_dir"
        elif [ ! -d "$input_dir" ]; then
            print_status "$YELLOW" "  ⚠️  $gas_value: Input directory $input_dir not found"
        else
            print_status "$RED" "  ❌ $gas_value: Failed or no metrics generated"
        fi
    done
    
    print_status "$BLUE" "\n📁 All metrics are located in: $BASE_METRICS_DIR-$ZKVM-*"
}

# Main execution
main() {
    # Parse gas categories early so we can display them
    parse_gas_categories
    
    # Validate resource and zkVM combination early
    validate_resource_zkvm
    
    if [ "$DRY_RUN" = true ]; then
        print_status "$YELLOW" "🔍 DRY RUN MODE - No actual execution will occur"
        print_status "$BLUE" "🚀 Would start ere-hosts benchmarks for gas categories..."
        print_status "$BLUE" "📊 Action: $ACTION"
        print_status "$BLUE" "🖥️  Resource: $RESOURCE"
        print_status "$BLUE" "🎯 Guest: $GUEST"
        print_status "$BLUE" "🔧 zkVM: $ZKVM"
        print_status "$BLUE" "⚙️  Execution Client: $EXECUTION_CLIENT"
        print_status "$BLUE" "📁 Input: $BASE_INPUT_DIR"
        print_status "$BLUE" "📂 Output: $BASE_METRICS_DIR"
        if [ -n "$CUSTOM_GAS_CATEGORIES" ]; then
            print_status "$BLUE" "⛽ Gas Categories: ${GAS_CATEGORIES[*]} (custom)"
        else
            print_status "$BLUE" "⛽ Gas Categories: ${GAS_CATEGORIES[*]} (defaults)"
        fi
        print_status "$BLUE" "🔄 Force Rerun: $FORCE_RERUN"
        print_status "$BLUE" "🧠 Memory Tracking: $MEMORY_TRACKING"
        print_status "$BLUE" "\n📋 Would execute the following commands:"
        
        for gas_value in "${GAS_CATEGORIES[@]}"; do
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
    print_status "$BLUE" "📁 Input: $BASE_INPUT_DIR"
    print_status "$BLUE" "📂 Output: $BASE_METRICS_DIR"
    if [ -n "$CUSTOM_GAS_CATEGORIES" ]; then
        print_status "$BLUE" "⛽ Gas Categories: ${GAS_CATEGORIES[*]} (custom)"
    else
        print_status "$BLUE" "⛽ Gas Categories: ${GAS_CATEGORIES[*]} (defaults)"
    fi
    print_status "$BLUE" "🔄 Force Rerun: $FORCE_RERUN"
    print_status "$BLUE" "🧠 Memory Tracking: $MEMORY_TRACKING"
    
    # Pre-flight checks
    check_cargo
    check_workspace
    
    # Check input fixtures
    check_input_fixtures
    
    # Build the project
    build_project
    
    # Run benchmarks for each gas category
    local failed_categories=()
    
    for gas_value in "${GAS_CATEGORIES[@]}"; do
        if run_benchmark "$gas_value"; then
            print_status "$GREEN" "✅ Completed: $gas_value"
        else
            print_status "$RED" "❌ Failed: $gas_value"
            failed_categories+=("$gas_value")
        fi
        
        echo "" # Add spacing between categories
    done
    
    # Show summary
    show_summary
    
    # Exit with error if any categories failed
    if [ ${#failed_categories[@]} -gt 0 ]; then
        print_status "$YELLOW" "\n⚠️  Some gas categories failed to complete:"
        for gas_value in "${failed_categories[@]}"; do
            print_status "$YELLOW" "  - $gas_value"
        done
        exit 1
    fi
    
    print_status "$GREEN" "\n🎯 All gas categories completed successfully!"
}

# Run main function
main "$@"