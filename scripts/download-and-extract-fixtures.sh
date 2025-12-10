#!/usr/bin/env bash
#
# download-and-extract-fixtures.sh
#
# Downloads execution spec test fixtures for zkevm.
# By default, it fetches the benchmark@v0.0.6 release.
# You can optionally provide a tag as the first argument, or use 'latest' to explicitly fetch the latest release.
# The second argument optionally sets the destination directory (default: ./zkevm-fixtures).
#
# Usage:
#   ./scripts/download-and-extract-fixtures.sh [TAG|latest] [DEST_DIR]
#
# Examples:
#   # Download default release (benchmark@v0.0.6) to default directory
#   ./scripts/download-and-extract-fixtures.sh
#   # Download latest official release to default directory
#   ./scripts/download-and-extract-fixtures.sh latest
#   # Download latest official release to a custom directory
#   ./scripts/download-and-extract-fixtures.sh latest /tmp/fixtures
#   # Download a specific release to default directory
#   ./scripts/download-and-extract-fixtures.sh v5.0.0
#   # Download a specific release to a custom directory
#   ./scripts/download-and-extract-fixtures.sh v5.0.0 /tmp/fixtures
#

set -euo pipefail

REPO="ethereum/execution-spec-tests"
ASSET_NAME="fixtures_benchmark.tar.gz"
DEFAULT_TAG="benchmark%40v0.0.6"

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
if [ -z "${1:-}" ]; then
  # No argument provided, use the default tag
  TAG="${DEFAULT_TAG}"
  echo "ℹ️  Using default tag: ${TAG}"
elif [ "${1}" = "latest" ]; then
  # Find the latest official release (not pre-release, standard version format)
  echo "🔎  Finding the latest official release..."
  LATEST_TAG=$( \
    github_api_curl "https://api.github.com/repos/${REPO}/releases" | \
    jq -r '.[] | select(.prerelease == false) | .tag_name' | \
    grep '^v[0-9]\+\.[0-9]\+\.[0-9]\+$' | \
    head -n 1 \
  )
  if [[ -z "${LATEST_TAG}" ]]; then
    echo "❌  Could not find any official releases in ${REPO}" >&2
    exit 1
  fi
  TAG="${LATEST_TAG}"
  echo "ℹ️  Using latest official release: ${TAG}"
else
  # Use the tag provided as the first argument
  TAG="$1"
  echo "ℹ️  Using specified tag: ${TAG}"
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

TMP_DIR=$(mktemp -d)
cleanup() {
  rm -rf "${TMP_DIR}"
}
trap cleanup EXIT
TMP_TAR="${TMP_DIR}/${ASSET_NAME}"

echo "⬇️  Downloading ${ASSET_NAME} to temporary directory …"
curl -L -o "${TMP_TAR}" "${DOWNLOAD_URL}"

echo "📂  Extracting to ${DEST_DIR}/"
mkdir -p "${DEST_DIR}"
tar -xzf "${TMP_TAR}" -C "${DEST_DIR}"

echo "✅  Fixtures ready in ${DEST_DIR}"
