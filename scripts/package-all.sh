#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "=== Packaging LLM Wiki Extension for VS Code ==="

# 1. Build Rust core engine in release mode
echo "--> Building Rust core engine in release mode..."
cargo build --release --manifest-path "${ROOT_DIR}/crates/core/Cargo.toml"

# 2. Inject target binary into extension/bin/
mkdir -p "${ROOT_DIR}/extension/bin"
cp "${ROOT_DIR}/target/release/llm-wiki-core" "${ROOT_DIR}/extension/bin/llm-wiki-core"
chmod +x "${ROOT_DIR}/extension/bin/llm-wiki-core"
echo "--> Injected sidecar binary to extension/bin/llm-wiki-core"

# 3. Compile extension and webview bundle
echo "--> Compiling TypeScript extension and webview..."
cd "${ROOT_DIR}/extension"
npm run package

# 4. Package VSIX with vsce
echo "--> Packaging VSIX..."
if command -v vsce &> /dev/null; then
    vsce package --no-dependencies --out "${ROOT_DIR}/"
elif npx --yes @vscode/vsce --version &> /dev/null; then
    npx --yes @vscode/vsce package --no-dependencies --out "${ROOT_DIR}/"
else
    echo "Notice: @vscode/vsce not installed globally. VSIX packaging ready."
fi

echo "=== Packaging Completed Successfully ==="
