#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${1:-/home/ds/projects}"

echo "Compiling extension in dev mode..."
cd "$DIR/extension"
npm run package

echo "Launching Extension Development Host on: $TARGET_DIR"
code -n \
  --extensionDevelopmentPath="$DIR/extension" \
  "$TARGET_DIR"
