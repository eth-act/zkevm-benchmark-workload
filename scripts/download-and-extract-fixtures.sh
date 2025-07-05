#!/usr/bin/env bash
#
# download-and-extract-fixtures.sh
#
# Downloads execution spec test fixtures for zkevm.
# By default, it fetches the latest release tag starting with 'zkevm@'.
# You can optionally provide a tag as the first argument, or use 'latest' to explicitly fetch the latest tag.
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
#   # Download a specific tag to default directory
#   ./scripts/download-and-extract-fixtures.sh zkevm@v0.0.1
#   # Download a specific tag to a custom directory
#   ./scripts/download-and-extract-fixtures.sh zkevm@v0.0.1 /tmp/fixtures
#

set -euo pipefail

REPO="ethereum/execution-spec-tests"
ASSET_NAME="fixtures_zkevm.tar.gz"

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
  # Find the latest tag with 'zkevm@' prefix
  echo "🔎  Finding the latest release tag with prefix 'zkevm@'..."
  LATEST_TAG=$( \
    curl -fsSL "https://api.github.com/repos/${REPO}/tags" | \
    jq -r '.[].name' | \
    grep '^zkevm@' | \
    sed 's/^zkevm@v//' | \
    sort -V | \
    tail -n 1 | \
    sed 's/^/zkevm@v/' \
  )
  if [[ -z "${LATEST_TAG}" ]]; then
    echo "❌  Could not find any release tags with prefix 'zkevm@' in ${REPO}" >&2
    exit 1
  fi
  TAG="${LATEST_TAG}"
  echo "ℹ️  Using latest found tag: ${TAG}"
fi

API_URL="https://api.github.com/repos/${REPO}/releases/tags/${TAG}"

echo "🔎  Getting release info for ${TAG} …"
DOWNLOAD_URL=$(
  curl -fsSL "${API_URL}" |
  jq -r ".assets[] | select(.name==\"${ASSET_NAME}\") | .browser_download_url"
)

if [[ -z "${DOWNLOAD_URL}" || "${DOWNLOAD_URL}" == "null" ]]; then
  echo "❌  Asset ${ASSET_NAME} not found in release ${TAG}" >&2
  exit 1
fi

echo "⬇️  Downloading ${ASSET_NAME} …"
curl -L -o "${ASSET_NAME}" "${DOWNLOAD_URL}"

echo "📂  Extracting to ${DEST_DIR}/"
mkdir -p "${DEST_DIR}"
tar -xzf "${ASSET_NAME}" -C "${DEST_DIR}"

echo "🗑️  Cleaning up ${ASSET_NAME}"
rm -f "${ASSET_NAME}"

echo "✅  Fixtures ready in ${DEST_DIR}"
