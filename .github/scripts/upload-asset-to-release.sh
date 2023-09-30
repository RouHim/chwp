#!/bin/env bash
set -e
# This script uploads a file to the latest release of the repository
# It requires the GitHub Personal Access Token

# The repository name in the format of user/repo
REPO_NAME=RouHim/chwp

# The first argument is the GitHub Personal Access Token
GITHUB_TOKEN=$1

# The second argument is the path to the asset's file
BIN_PATH=$2

# The third argument is the name of the asset's file in the release
NAME=$3


echo "Uploading $BIN_PATH to $NAME"

# Get the release ID from the latest release
RELEASE_ID=$(curl -s https://api.github.com/repos/${REPO_NAME}/releases/latest | jq -r .id)

echo "Release ID: $RELEASE_ID"

# Upload the asset
UPLOAD_URL="https://uploads.github.com/repos/${REPO_NAME}/releases/${RELEASE_ID}/assets?name=${NAME}"
curl -X POST \
  -H "Content-Type: $(file -b --mime-type "$BIN_PATH")" \
  -H "Authorization: token ${GITHUB_TOKEN}" \
  -T "${BIN_PATH}" \
  "${UPLOAD_URL}"
