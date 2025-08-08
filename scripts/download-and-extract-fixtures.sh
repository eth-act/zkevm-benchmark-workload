#!/usr/bin/env bash
#
# download-and-extract-fixtures.sh
#
# Downloads execution spec test fixtures for zkevm.
# By default, it fetches the latest release starting with 'benchmark@'.
# You can optionally provide a tag as the first argument, or use 'latest' to explicitly fetch the latest release.
# The second argument optionally sets the destination directory (default: ./zkevm-fixtures).
#
# Usage:
#   ./scripts/download-and-extract-fixtures.sh [TAG|latest] [DEST_DIR]
#
# Examples:
#   # Download latest release to default directory
#   ./scripts/download-and-extract-fixtures.sh
#   # Download latest release to a custom directory
#   ./scripts/download-and-extract-fixtures.sh latest /tmp/fixtures
#   # Download a specific release to default directory
#   ./scripts/download-and-extract-fixtures.sh benchmark@v0.0.1
#   # Download a specific release to a custom directory
#   ./scripts/download-and-extract-fixtures.sh benchmark@v0.0.1 /tmp/fixtures
#

set -euo pipefail

REPO="ethereum/execution-spec-tests"
ASSET_NAME="fixtures_benchmark.tar.gz"

# Helper function to make authenticated GitHub API calls
github_api_curl() {
  local url="$1"
  if [ -n "${GITHUB_TOKEN:-}" ]; then
    curl -fsSL -H "Authorization: Bearer ${GITHUB_TOKEN}" "${url}"
  else
    curl -fsSL "${url}"
  fi
}

# Show authentication status
if [ -n "${GITHUB_TOKEN:-}" ]; then
  echo "🔑  Using GitHub token for API authentication"
fi

# Set DEST_DIR from second argument, or default
if [ -n "${2:-}" ]; then
  DEST_DIR="$2"
else
  DEST_DIR="./zkevm-fixtures"
fi

# Determine the tag to use
if [ -n "${1:-}" ] && [ "${1}" != "latest" ]; then
  # Use the tag provided as the first argument (unless it's 'latest')
  TAG="$1"
  echo "ℹ️  Using specified tag: ${TAG}"
else
  # Find the latest release with 'benchmark@' prefix
  echo "🔎  Finding the latest release with prefix 'benchmark@'..."
  LATEST_TAG=$( \
    github_api_curl "https://api.github.com/repos/${REPO}/releases" | \
    jq -r '.[].tag_name' | \
    grep '^benchmark@' | \
    sed 's/^benchmark@v//' | \
    sort -V | \
    tail -n 1 | \
    sed 's/^/benchmark@v/' \
  )
  if [[ -z "${LATEST_TAG}" ]]; then
    echo "❌  Could not find any releases with prefix 'benchmark@' in ${REPO}" >&2
    exit 1
  fi
  TAG="${LATEST_TAG}"
  echo "ℹ️  Using latest found release: ${TAG}"
fi

API_URL="https://api.github.com/repos/${REPO}/releases/tags/${TAG}"

echo "🔎  Getting release info for ${TAG} …"
DOWNLOAD_URL=$(
  github_api_curl "${API_URL}" |
  jq -r ".assets[] | select(.name==\"${ASSET_NAME}\") | .browser_download_url"
)

if [[ -z "${DOWNLOAD_URL}" || "${DOWNLOAD_URL}" == "null" ]]; then
  echo "❌  Asset ${ASSET_NAME} not found in release ${TAG}" >&2
  exit 1
fi

# Check if fixtures already exist locally
if [ -d "${DEST_DIR}" ] && [ "$(ls -A "${DEST_DIR}" 2>/dev/null)" ]; then
  echo "✅  Fixtures already exist in ${DEST_DIR}, skipping download"
  echo "✅  Fixtures ready in ${DEST_DIR}"
  exit 0
fi

TMP_DIR=$(mktemp -d)
cleanup() {
  rm -rf "${TMP_DIR}"
}
trap cleanup EXIT
TMP_TAR="${TMP_DIR}/${ASSET_NAME}"

echo "⬇️  Downloading ${ASSET_NAME} to temporary directory …"
# Try multiple download strategies with retries
download_success=false

# Strategy 1: HTTP/1.1 with retries
echo "🔄  Trying HTTP/1.1 download..."
if curl -L --http1.1 --retry 3 --retry-delay 2 --retry-max-time 300 -o "${TMP_TAR}" "${DOWNLOAD_URL}"; then
  download_success=true
fi

# Strategy 2: HTTP/2 with different options if HTTP/1.1 failed
if [ "$download_success" = false ]; then
  echo "🔄  HTTP/1.1 failed, trying HTTP/2 with different options..."
  if curl -L --http2 --retry 3 --retry-delay 5 --retry-max-time 300 --max-time 600 -o "${TMP_TAR}" "${DOWNLOAD_URL}"; then
    download_success=true
  fi
fi

# Strategy 3: wget as fallback if curl fails
if [ "$download_success" = false ]; then
  echo "🔄  curl failed, trying wget as fallback..."
  if command -v wget >/dev/null 2>&1; then
    if wget --timeout=300 --tries=3 --waitretry=5 -O "${TMP_TAR}" "${DOWNLOAD_URL}"; then
      download_success=true
    fi
  fi
fi

# Check if any download strategy succeeded
if [ "$download_success" = false ]; then
  echo "❌  All download strategies failed"
  exit 1
fi

echo "📂  Extracting to ${DEST_DIR}/"
mkdir -p "${DEST_DIR}"
tar -xzf "${TMP_TAR}" -C "${DEST_DIR}"

echo "✅  Fixtures ready in ${DEST_DIR}"
