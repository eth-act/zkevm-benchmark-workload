#!/usr/bin/env bash
#
# SP1 Cluster Startup Script
# 
# Usage:
#   ./start-sp1-cluster.sh [OPTIONS]
#
# Options:
#   --gpu-nodes N      Number of GPU worker nodes (0-8, default: 1)
#                      Use 0 for CPU-only mode
#   --mixed            Use mixed worker instead of separate CPU/GPU workers
#   --port PORT        Set API gRPC port (default: 50051)
#   --redis-port PORT  Set Redis port (default: 6379)
#   --pull             Force re-pull of Docker images
#   --detach, -d       Run in detached mode
#   --wait             Wait for all services to be healthy (implies --detach)
#   --skip-gpu-check   Skip NVIDIA runtime verification (use if you know GPU is available)
#   --help, -h         Show this help message
#
# Examples:
#   ./start-sp1-cluster.sh                  # 1 GPU worker (default)
#   ./start-sp1-cluster.sh --gpu-nodes 2    # 2 GPU workers
#   ./start-sp1-cluster.sh --gpu-nodes 0    # CPU-only mode
#   ./start-sp1-cluster.sh --gpu-nodes 4 -d # 4 GPU workers, detached
#   ./start-sp1-cluster.sh --mixed -d       # Mixed mode worker
#   ./start-sp1-cluster.sh --port 50052     # Custom API port (for multiple clusters)
#   ./start-sp1-cluster.sh --wait           # Wait for all services to be healthy

set -euo pipefail

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Default values
GPU_NODES=1
FORCE_PULL=false
DETACH=false
MIXED_MODE=false
SKIP_GPU_CHECK=false
WAIT_HEALTHY=false

# Port configuration (can be overridden via CLI or .env)
# These are set here but may be overridden by .env file or CLI args
CLI_API_PORT=""
CLI_REDIS_PORT=""

# Docker Compose command (will be set by detect_docker_compose)
DOCKER_COMPOSE_CMD=""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
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

log_hint() {
    echo -e "${CYAN}[HINT]${NC} $1"
}

# Show help
show_help() {
    cat << EOF
SP1 Cluster Startup Script

Usage:
  $0 [OPTIONS]

Options:
  --gpu-nodes N      Number of GPU worker nodes (0-8, default: 1)
                     Use 0 for CPU-only mode
  --mixed            Use mixed worker (WORKER_TYPE=ALL) instead of separate workers
  --port PORT        Set API gRPC port (default: 50051, overrides API_PORT env var)
  --redis-port PORT  Set Redis port (default: 6379, overrides REDIS_PORT env var)
  --pull             Force re-pull of Docker images
  --detach, -d       Run in detached mode
  --wait             Wait for all services to be healthy (implies --detach)
  --skip-gpu-check   Skip NVIDIA runtime verification
  --help, -h         Show this help message

Examples:
  $0                        # 1 GPU worker (default)
  $0 --gpu-nodes 2          # 2 GPU workers (gpu0, gpu1) + cpu-node
  $0 --gpu-nodes 0          # CPU-only mode (cpu-node only)
  $0 --gpu-nodes 4 -d       # 4 GPU workers, detached
  $0 --mixed -d             # Mixed mode worker, detached
  $0 --port 50052           # Run on custom API port
  $0 --port 50052 --redis-port 6380  # Custom ports for multiple clusters
  $0 --wait                 # Wait for all services to be healthy

Images:
  Uses pre-built images from ghcr.io/succinctlabs/sp1-cluster

Services:
  Core:     redis, postgresql, api, coordinator
  CPU:      cpu-node (or mixed)
  GPU:      gpu0, gpu1, gpu2, gpu3, gpu4, gpu5, gpu6, gpu7

API Endpoints:
  gRPC API:     http://localhost:\${API_PORT:-50051}
  Redis:        localhost:\${REDIS_PORT:-6379}
  
Configuration:
  Copy env.example to .env to customize resource limits and ports.
  CLI arguments (--port, --redis-port) override .env settings.

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
        --port)
            CLI_API_PORT="$2"
            shift 2
            ;;
        --redis-port)
            CLI_REDIS_PORT="$2"
            shift 2
            ;;
        --pull)
            FORCE_PULL=true
            shift
            ;;
        --detach|-d)
            DETACH=true
            shift
            ;;
        --wait)
            WAIT_HEALTHY=true
            DETACH=true  # --wait implies --detach
            shift
            ;;
        --skip-gpu-check)
            SKIP_GPU_CHECK=true
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

# Apply CLI port overrides (CLI args take precedence over .env)
if [[ -n "$CLI_API_PORT" ]]; then
    export API_PORT="$CLI_API_PORT"
    log_info "Using CLI-specified API port: $API_PORT"
fi

if [[ -n "$CLI_REDIS_PORT" ]]; then
    export REDIS_PORT="$CLI_REDIS_PORT"
    log_info "Using CLI-specified Redis port: $REDIS_PORT"
fi

# Validate port numbers if specified
validate_port() {
    local port="$1"
    local name="$2"
    if [[ -n "$port" ]] && ! [[ "$port" =~ ^[0-9]+$ && "$port" -ge 1 && "$port" -le 65535 ]]; then
        log_error "Invalid $name: $port (must be 1-65535)"
        exit 1
    fi
}

validate_port "${API_PORT:-}" "API port"
validate_port "${REDIS_PORT:-}" "Redis port"

# Ensure SP1 circuits cache directory exists
# This directory is mounted into containers for caching compiled circuits
# Without this directory, Docker will create it as root, causing permission issues
ensure_circuits_dir() {
    local circuits_dir="${SP1_CIRCUITS_DIR:-${HOME}/.sp1/circuits}"
    
    if [[ ! -d "$circuits_dir" ]]; then
        log_info "Creating SP1 circuits cache directory: $circuits_dir"
        if mkdir -p "$circuits_dir"; then
            log_success "Created circuits directory"
        else
            log_error "Failed to create circuits directory: $circuits_dir"
            log_hint "Create it manually: mkdir -p $circuits_dir"
            log_hint "Or set SP1_CIRCUITS_DIR in .env to a different location"
            return 1
        fi
    else
        log_info "SP1 circuits cache directory exists: $circuits_dir"
    fi
    
    # Check if directory is writable
    if [[ ! -w "$circuits_dir" ]]; then
        log_warn "Circuits directory is not writable: $circuits_dir"
        log_hint "Fix permissions: chmod u+w $circuits_dir"
    fi
    
    # Export for use in docker-compose
    export SP1_CIRCUITS_DIR="$circuits_dir"
    return 0
}

# Detect and set Docker Compose command
# Supports both Docker Compose v2 (docker compose) and v1 (docker-compose)
detect_docker_compose() {
    # Try Docker Compose v2 first (preferred)
    if docker compose version &> /dev/null; then
        DOCKER_COMPOSE_CMD="docker compose"
        local version
        version=$(docker compose version --short 2>/dev/null || docker compose version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
        log_info "Using Docker Compose v2 ($version)"
        return 0
    fi
    
    # Fall back to Docker Compose v1 (legacy)
    if command -v docker-compose &> /dev/null; then
        DOCKER_COMPOSE_CMD="docker-compose"
        local version
        version=$(docker-compose version --short 2>/dev/null || docker-compose --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
        log_warn "Using Docker Compose v1 ($version) - legacy version"
        log_warn "Consider upgrading to Docker Compose v2 for better performance and features"
        log_hint "Upgrade guide: https://docs.docker.com/compose/migrate/"
        return 0
    fi
    
    # Neither found
    log_error "Docker Compose is not installed."
    log_hint "Install Docker Compose:"
    log_hint "  - Docker Desktop (includes Compose v2): https://docs.docker.com/desktop/"
    log_hint "  - Linux standalone: https://docs.docker.com/compose/install/linux/"
    return 1
}

# Check NVIDIA GPU availability
check_nvidia_gpu() {
    local gpu_available=false
    local nvidia_smi_available=false
    local docker_nvidia_runtime=false
    
    # Check if nvidia-smi is available and working
    if command -v nvidia-smi &> /dev/null; then
        nvidia_smi_available=true
        if nvidia-smi &> /dev/null; then
            gpu_available=true
        fi
    fi
    
    # Check if Docker has NVIDIA runtime
    if docker info 2>/dev/null | grep -q "Runtimes.*nvidia"; then
        docker_nvidia_runtime=true
    fi
    
    # Report findings
    if [[ "$gpu_available" == true && "$docker_nvidia_runtime" == true ]]; then
        log_success "NVIDIA GPU detected and Docker NVIDIA runtime available"
        return 0
    fi
    
    # Detailed error reporting
    echo ""
    log_error "NVIDIA GPU support is not properly configured"
    echo ""
    
    if [[ "$nvidia_smi_available" == false ]]; then
        log_error "  - nvidia-smi command not found"
        log_hint "  Install NVIDIA drivers: https://docs.nvidia.com/datacenter/tesla/tesla-installation-notes/"
    elif [[ "$gpu_available" == false ]]; then
        log_error "  - nvidia-smi failed - GPU may not be accessible"
        log_hint "  Check GPU status: nvidia-smi"
        log_hint "  Verify driver installation: cat /proc/driver/nvidia/version"
    fi
    
    if [[ "$docker_nvidia_runtime" == false ]]; then
        log_error "  - Docker NVIDIA runtime not detected"
        log_hint "  Install NVIDIA Container Toolkit:"
        log_hint "    https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/install-guide.html"
        echo ""
        log_hint "  Quick install (Ubuntu/Debian):"
        log_hint "    curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg"
        log_hint "    curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list | \\"
        log_hint "      sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \\"
        log_hint "      sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list"
        log_hint "    sudo apt-get update && sudo apt-get install -y nvidia-container-toolkit"
        log_hint "    sudo nvidia-ctk runtime configure --runtime=docker"
        log_hint "    sudo systemctl restart docker"
    fi
    
    echo ""
    log_hint "Alternatives:"
    log_hint "  - Use CPU-only mode: $0 --gpu-nodes 0"
    log_hint "  - Skip this check:   $0 --skip-gpu-check"
    echo ""
    
    return 1
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed."
        log_hint "Install Docker: https://docs.docker.com/engine/install/"
        exit 1
    fi
    
    # Check if Docker daemon is running
    if ! docker info &> /dev/null; then
        log_error "Docker daemon is not running."
        log_hint "Start Docker:"
        log_hint "  - Linux: sudo systemctl start docker"
        log_hint "  - macOS: Open Docker Desktop application"
        log_hint "  - Windows: Start Docker Desktop"
        exit 1
    fi
    
    # Detect Docker Compose (v1 or v2)
    if ! detect_docker_compose; then
        exit 1
    fi
    
    # Ensure SP1 circuits cache directory exists
    if ! ensure_circuits_dir; then
        exit 1
    fi
    
    # Check NVIDIA runtime if GPU nodes requested
    if [[ "$GPU_NODES" -gt 0 && "$SKIP_GPU_CHECK" == false ]]; then
        if ! check_nvidia_gpu; then
            exit 1
        fi
    elif [[ "$GPU_NODES" -gt 0 && "$SKIP_GPU_CHECK" == true ]]; then
        log_warn "Skipping NVIDIA GPU check (--skip-gpu-check specified)"
        log_warn "GPU workers may fail if NVIDIA runtime is not properly configured"
    fi
    
    log_success "Prerequisites check passed"
}

# Pull Docker images
pull_images() {
    log_info "Pulling Docker images..."
    $DOCKER_COMPOSE_CMD -f docker-compose.yml pull
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
    $DOCKER_COMPOSE_CMD -f docker-compose.yml up $detach_flag $services
}

# Wait for a specific service to be healthy
wait_for_service() {
    local service="$1"
    local max_attempts="${2:-30}"
    local attempt=0
    
    while [[ $attempt -lt $max_attempts ]]; do
        local status
        status=$($DOCKER_COMPOSE_CMD -f docker-compose.yml ps "$service" 2>/dev/null | tail -n +2 | head -1)
        
        # Check for "healthy" status or "Up" status
        if echo "$status" | grep -qE "(healthy|Up)"; then
            return 0
        fi
        
        attempt=$((attempt + 1))
        sleep 2
    done
    
    return 1
}

# Wait for all critical services to be healthy
wait_for_health() {
    local strict="${1:-false}"
    
    log_info "Waiting for services to be healthy..."
    echo ""
    
    local services=("redis" "postgresql" "api" "coordinator")
    local all_healthy=true
    
    for service in "${services[@]}"; do
        printf "  Waiting for %-15s " "$service..."
        if wait_for_service "$service" 60; then
            echo -e "${GREEN}✓ healthy${NC}"
        else
            echo -e "${RED}✗ not ready${NC}"
            all_healthy=false
        fi
    done
    
    echo ""
    
    if [[ "$all_healthy" == true ]]; then
        log_success "All core services are healthy"
        return 0
    else
        if [[ "$strict" == true ]]; then
            log_error "Some services failed to become healthy"
            return 1
        else
            log_warn "Some services may still be starting. Check logs for details."
            log_hint "View logs: $DOCKER_COMPOSE_CMD logs -f"
            return 0
        fi
    fi
}

# Print cluster information
print_info() {
    local services
    services=$(get_services)
    
    local api_port="${API_PORT:-50051}"
    local redis_port="${REDIS_PORT:-6379}"
    
    echo ""
    echo "========================================"
    echo -e "${GREEN}SP1 Cluster is running!${NC}"
    echo "========================================"
    echo ""
    echo "gRPC API:        http://localhost:${api_port}"
    echo "Redis:           localhost:${redis_port}"
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
    echo "  View logs:     cd $SCRIPT_DIR && $DOCKER_COMPOSE_CMD logs -f"
    echo "  View status:   cd $SCRIPT_DIR && $DOCKER_COMPOSE_CMD ps"
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
        # If --wait was specified, use strict health checking
        if [[ "$WAIT_HEALTHY" == true ]]; then
            if ! wait_for_health true; then
                log_error "Cluster failed to start properly"
                exit 1
            fi
        else
            wait_for_health false || true
        fi
        print_info
    fi
}

main
