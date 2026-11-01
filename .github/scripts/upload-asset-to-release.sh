#!/usr/bin/env bash
set -euo pipefail

# Uploads an asset to an explicit release tag.
# Usage: upload-asset-to-release.sh <token> <tag> <path> <asset-name>

REPO_NAME="${GITHUB_REPOSITORY:-RouHim/chwp}"

TOKEN="${1:?GitHub token is required}"
TAG="${2:?Release tag is required}"
BIN_PATH="${3:?Asset path is required}"
NAME="${4:?Asset name is required}"

if [[ ! -f "$BIN_PATH" ]]; then
  echo "error: asset not found: $BIN_PATH" >&2
  exit 1
fi

export GH_TOKEN="$TOKEN"

echo "Uploading $BIN_PATH as $NAME to release $TAG"

# Fail loudly if the release does not exist instead of uploading to a bogus ID.
gh release view "$TAG" \
  --repo "$REPO_NAME" \
  --json tagName \
  --jq '.tagName' > /dev/null

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

# Upload under the requested asset name and replace an existing asset on retries.
cp "$BIN_PATH" "$TMP_DIR/$NAME"
gh release upload "$TAG" "$TMP_DIR/$NAME" --repo "$REPO_NAME" --clobber

echo "Uploaded $NAME to $TAG"
