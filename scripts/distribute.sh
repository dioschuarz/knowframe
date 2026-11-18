#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "=========================================================="
echo "🚀 KnowFrame Clean Build, Package & Installation Pipeline"
echo "=========================================================="

# 1. Compile Rust engine & CLI in release mode
echo "--> [1/6] Compiling Rust binaries (knowframe-core, knowframe-cli)..."
cargo build --release --manifest-path "${ROOT_DIR}/Cargo.toml"

# 2. Sync binaries into extension and user PATH
echo "--> [2/6] Syncing binaries to extension/bin and ~/.local/bin..."
mkdir -p "${ROOT_DIR}/extension/bin"
cp "${ROOT_DIR}/target/release/knowframe-core" "${ROOT_DIR}/extension/bin/knowframe-core"
cp "${ROOT_DIR}/target/release/knowframe-cli" "${ROOT_DIR}/extension/bin/knowframe-cli"
chmod +x "${ROOT_DIR}/extension/bin/knowframe-core" "${ROOT_DIR}/extension/bin/knowframe-cli"

mkdir -p "${HOME}/.local/bin"
rm -f "${HOME}/.local/bin/knowframe-core" "${HOME}/.local/bin/knowframe-cli" "${HOME}/.local/bin/knowframe"
ln -sf "${ROOT_DIR}/target/release/knowframe-core" "${HOME}/.local/bin/knowframe-core"
ln -sf "${ROOT_DIR}/target/release/knowframe-cli" "${HOME}/.local/bin/knowframe-cli"
ln -sf "${HOME}/.local/bin/knowframe-core" "${HOME}/.local/bin/knowframe"

# 3. Build Extension Host and Webview bundles
echo "--> [3/6] Bundling Extension Host & Webview UI with esbuild..."
cd "${ROOT_DIR}/extension"
npm run package

# 4. Package VSIX archive
VERSION=$(node -p "require('./package.json').version")
echo "--> [4/6] Packaging knowframe-${VERSION}.vsix with vsce..."
npx --yes @vscode/vsce package --no-dependencies -o "${ROOT_DIR}/extension/knowframe-${VERSION}.vsix"

# 5. Terminate lingering/stale background daemon processes
echo "--> [5/6] Terminating stale background daemons..."
pkill -9 -f "knowframe-core daemon" 2>/dev/null || true
pkill -9 -f "llm-wiki-core daemon" 2>/dev/null || true
sleep 0.5

# 6. Reinstall clean VSIX in VS Code
echo "--> [6/6] Installing extension into VS Code..."
if command -v code &> /dev/null; then
    code --install-extension "${ROOT_DIR}/extension/knowframe-${VERSION}.vsix" --force
    echo "=========================================================="
    echo "✅ Successfully installed fresh KnowFrame extension!"
    echo "💡 Trigger 'Developer: Reload Window' in VS Code to activate."
    echo "=========================================================="
else
    echo "⚠️ 'code' CLI not found on PATH. VSIX ready at ${ROOT_DIR}/extension/knowframe-${VERSION}.vsix"
fi
