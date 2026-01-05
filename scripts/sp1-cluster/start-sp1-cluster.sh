#!/usr/bin/env bash
#
# SP1 Cluster Startup Script
# 
# Usage:
#   ./start-sp1-cluster.sh [OPTIONS]
#
# Options:
#   --gpu-nodes N    Number of GPU worker nodes (0-8, default: 1)
#                    Use 0 for CPU-only mode
#   --mixed          Use mixed worker instead of separate CPU/GPU workers
#   --pull           Force re-pull of Docker images
#   --detach, -d     Run in detached mode
#   --help, -h       Show this help message
#
# Examples:
#   ./start-sp1-cluster.sh                  # 1 GPU worker (default)
#   ./start-sp1-cluster.sh --gpu-nodes 2    # 2 GPU workers
#   ./start-sp1-cluster.sh --gpu-nodes 0    # CPU-only mode
#   ./start-sp1-cluster.sh --gpu-nodes 4 -d # 4 GPU workers, detached
#   ./start-sp1-cluster.sh --mixed -d       # Mixed mode worker

set -euo pipefail

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Default values
GPU_NODES=1
FORCE_PULL=false
DETACH=false
MIXED_MODE=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Show help
show_help() {
    cat << EOF
SP1 Cluster Startup Script

Usage:
  $0 [OPTIONS]

Options:
  --gpu-nodes N    Number of GPU worker nodes (0-8, default: 1)
                   Use 0 for CPU-only mode
  --mixed          Use mixed worker (WORKER_TYPE=ALL) instead of separate workers
  --pull           Force re-pull of Docker images
  --detach, -d     Run in detached mode
  --help, -h       Show this help message

Examples:
  $0                        # 1 GPU worker (default)
  $0 --gpu-nodes 2          # 2 GPU workers (gpu0, gpu1) + cpu-node
  $0 --gpu-nodes 0          # CPU-only mode (cpu-node only)
  $0 --gpu-nodes 4 -d       # 4 GPU workers, detached
  $0 --mixed -d             # Mixed mode worker, detached

Images:
  Uses pre-built images from ghcr.io/succinctlabs/sp1-cluster

Services:
  Core:     redis, postgresql, api, coordinator
  CPU:      cpu-node (or mixed)
  GPU:      gpu0, gpu1, gpu2, gpu3, gpu4, gpu5, gpu6, gpu7

API Endpoints:
  gRPC API:     http://localhost:50051
  
EOF
    exit 0
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --gpu-nodes)
            GPU_NODES="$2"
            shift 2
            ;;
        --mixed)
            MIXED_MODE=true
            shift
            ;;
        --pull)
            FORCE_PULL=true
            shift
            ;;
        --detach|-d)
            DETACH=true
            shift
            ;;
        --help|-h)
            show_help
            ;;
        *)
            log_error "Unknown option: $1"
            show_help
            ;;
    esac
done

# Validate GPU_NODES is a valid integer (0-8)
if ! [[ "$GPU_NODES" =~ ^[0-8]$ ]]; then
    log_error "--gpu-nodes must be an integer between 0 and 8"
    exit 1
fi

# Load environment variables if .env exists
if [[ -f ".env" ]]; then
    log_info "Loading environment from .env"
    set -a
    source .env
    set +a
else
    log_warn "No .env file found. Using defaults. Copy env.example to .env to customize."
fi

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed. Please install Docker first."
        exit 1
    fi
    
    # Check Docker Compose
    if ! docker compose version &> /dev/null; then
        log_error "Docker Compose is not available. Please install Docker Compose v2."
        exit 1
    fi
    
    # Check NVIDIA runtime if GPU nodes requested
    if [[ "$GPU_NODES" -gt 0 ]]; then
        if ! docker info 2>/dev/null | grep -q "nvidia"; then
            log_warn "NVIDIA Docker runtime not detected. GPU workers may not function correctly."
            log_warn "Install nvidia-docker2 or nvidia-container-toolkit for GPU support."
        fi
    fi
    
    log_success "Prerequisites check passed"
}

# Pull Docker images
pull_images() {
    log_info "Pulling Docker images..."
    docker compose -f docker-compose.yml pull
    log_success "Docker images pulled"
}

# Build the list of services to start
get_services() {
    local services="redis postgresql api coordinator"
    
    if [[ "$MIXED_MODE" == true ]]; then
        services="$services mixed"
    else
        # Add CPU node
        services="$services cpu-node"
        
        # Add GPU nodes
        for ((i=0; i<GPU_NODES; i++)); do
            services="$services gpu${i}"
        done
    fi
    
    echo "$services"
}

# Start the cluster
start_cluster() {
    local services
    services=$(get_services)
    
    local detach_flag=""
    if [[ "$DETACH" == true ]]; then
        detach_flag="-d"
    fi
    
    if [[ "$MIXED_MODE" == true ]]; then
        log_info "Starting SP1 Cluster in mixed mode..."
    elif [[ "$GPU_NODES" -gt 0 ]]; then
        log_info "Starting SP1 Cluster with $GPU_NODES GPU worker(s) + cpu-node..."
    else
        log_info "Starting SP1 Cluster in CPU-only mode..."
    fi
    
    log_info "Services: $services"
    
    # Start services
    # shellcheck disable=SC2086
    docker compose -f docker-compose.yml up $detach_flag $services
}

# Wait for services to be healthy
wait_for_health() {
    log_info "Waiting for services to be healthy..."
    
    local max_attempts=30
    local attempt=0
    
    while [[ $attempt -lt $max_attempts ]]; do
        # Check if API container is running and healthy
        if docker compose -f docker-compose.yml ps api 2>/dev/null | grep -q "Up"; then
            log_success "SP1 Cluster API is running"
            return 0
        fi
        
        attempt=$((attempt + 1))
        echo -n "."
        sleep 2
    done
    
    log_warn "Health check timed out. Services may still be starting..."
    return 1
}

# Print cluster information
print_info() {
    local services
    services=$(get_services)
    
    echo ""
    echo "========================================"
    echo -e "${GREEN}SP1 Cluster is running!${NC}"
    echo "========================================"
    echo ""
    echo "gRPC API:        http://localhost:50051"
    echo "Redis:           localhost:6379"
    echo ""
    if [[ "$MIXED_MODE" == true ]]; then
        echo "Worker Mode:     Mixed (ALL)"
    elif [[ "$GPU_NODES" -gt 0 ]]; then
        echo "Worker Mode:     GPU ($GPU_NODES worker(s)) + CPU"
    else
        echo "Worker Mode:     CPU only"
    fi
    echo ""
    echo "Running services: $services"
    echo ""
    echo "Useful commands:"
    echo "  View logs:     cd $SCRIPT_DIR && docker compose logs -f"
    echo "  View status:   cd $SCRIPT_DIR && docker compose ps"
    echo "  Stop cluster:  $SCRIPT_DIR/stop-sp1-cluster.sh"
    echo ""
}

# Main execution
main() {
    echo ""
    echo "========================================"
    echo "       SP1 Cluster Startup Script       "
    echo "========================================"
    echo ""
    
    check_prerequisites
    
    # Pull images if requested or if images not available
    if [[ "$FORCE_PULL" == true ]]; then
        pull_images
    fi
    
    start_cluster
    
    if [[ "$DETACH" == true ]]; then
        wait_for_health || true
        print_info
    fi
}

main
