#!/usr/bin/env bash
#
# SP1 Cluster Stop Script
#
# Usage:
#   ./stop-sp1-cluster.sh [OPTIONS]
#
# Options:
#   --remove-volumes    Remove persistent volumes (database, redis data)
#   --remove-images     Remove Docker images
#   --all               Remove everything (volumes + images)
#   --help, -h          Show this help message

set -euo pipefail

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Default values
REMOVE_VOLUMES=false
REMOVE_IMAGES=false

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
SP1 Cluster Stop Script

Usage:
  $0 [OPTIONS]

Options:
  --remove-volumes    Remove persistent volumes (database, redis data)
  --remove-images     Remove Docker images
  --all               Remove everything (volumes + images)
  --help, -h          Show this help message

Examples:
  $0                      # Stop services, keep data
  $0 --remove-volumes     # Stop and remove all data
  $0 --all                # Complete cleanup

EOF
    exit 0
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --remove-volumes)
            REMOVE_VOLUMES=true
            shift
            ;;
        --remove-images)
            REMOVE_IMAGES=true
            shift
            ;;
        --all)
            REMOVE_VOLUMES=true
            REMOVE_IMAGES=true
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

# Stop all services
stop_services() {
    log_info "Stopping SP1 Cluster services..."
    
    # Build down command with appropriate flags
    local down_flags=""
    
    if [[ "$REMOVE_VOLUMES" == true ]]; then
        down_flags="$down_flags -v"
        log_info "Will remove volumes"
    fi
    
    if [[ "$REMOVE_IMAGES" == true ]]; then
        down_flags="$down_flags --rmi all"
        log_info "Will remove images"
    fi
    
    # Stop all services from docker-compose.yml
    # shellcheck disable=SC2086
    docker compose -f docker-compose.yml down $down_flags 2>/dev/null || true
    
    log_success "Services stopped"
}

# Show status
show_status() {
    echo ""
    
    # Check if any containers are still running
    local running_containers
    running_containers=$(docker compose -f docker-compose.yml ps -q 2>/dev/null | wc -l)
    
    if [[ "$running_containers" -eq 0 ]]; then
        log_success "All SP1 Cluster containers have been stopped"
    else
        log_warn "Some containers may still be running. Check with: docker compose ps"
    fi
}

# Main execution
main() {
    echo ""
    echo "========================================"
    echo "        SP1 Cluster Stop Script         "
    echo "========================================"
    echo ""
    
    stop_services
    show_status
    
    echo ""
    if [[ "$REMOVE_VOLUMES" == true ]]; then
        log_info "Persistent data has been removed"
    else
        log_info "Persistent data preserved. Use --remove-volumes to delete."
    fi
    
    if [[ "$REMOVE_IMAGES" == true ]]; then
        log_info "Docker images have been removed"
    else
        log_info "Docker images preserved. Use --remove-images to delete."
    fi
    echo ""
}

main
