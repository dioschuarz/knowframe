# Quickstart & Validation Guide: `llm_wiki_ext4vsc`

**Feature**: LLM Wiki Extension for VS Code (`llm_wiki_ext4vsc`)  
**Branch**: `001-llm-wiki-ext`  
**Date**: 2026-09-11  
**Status**: Ready for Verification  

---

## 1. Prerequisites

Before running or validating the extension, ensure the following toolchains are installed:
- **Rust Toolchain**: `rustc ^1.75.0` (active: `1.98.0`), `cargo`
- **Node.js & Package Manager**: `node >= 20.0.0`, `npm >= 10.0.0`
- **Git Hook Orchestrator**: `prek >= 0.2.0` (or `pre-commit` as Tier 2 fallback)
- **VS Code**: Visual Studio Code `^1.90.0`

---

## 2. Workspace Setup & Build

### Step 1: Clone and Inspect Workspace
```bash
cd /home/ds/projects/llm_wiki_ext4vsc
```

### Step 2: Build Rust Core Sidecar (`llm-wiki-core`)
```bash
cargo build --release --manifest-path crates/core/Cargo.toml
```
*Expected Outcome*: Generates binary at `target/release/llm-wiki-core`. File size $\le 15\text{ MB}$, zero compilation warnings (`cargo clippy` clean).

### Step 3: Compile TypeScript Extension Host & Webview
```bash
cd extension
npm install
npm run build
```
*Expected Outcome*: Emits compiled bundles into `extension/dist/extension.js` and `extension/dist/webview.js`.

---

## 3. Validation Scenario 1: Headless Vault Discovery & Centrality (P1)

Verify organic AST extraction, wikilink normalization, BPE token computation, and mathematical hub discovery directly from the CLI without launching VS Code.

```bash
# Run standalone CLI scan against a sample documentation vault
target/release/llm-wiki-core scan /home/ds/projects/notes --format json > /tmp/scan_result.json
```

### Verification Criteria:
1. **Execution Time**: Completes in $< 200\text{ ms}$ for 1,000 Markdown files.
2. **Centrality**: Inspect `/tmp/scan_result.json`:
   - `hub_id` identifies the top betweenness centrality node (e.g. `wiki_index.md` or `active_context.md`).
3. **Dead Link Detection**: Broken links in `/tmp/scan_result.json` have `is_broken: true`.
4. **Token Accuracy**: `tiktoken-rs` returns exact `cl100k_base` token counts for each file.

---

## 4. Validation Scenario 2: Git Hook Sync via `prek` (P2)

Validate the event-driven git hook workflow with zero overhead on non-documentation commits and sub-50ms delta processing.

### Setup Hook Configuration
```bash
# In target repo root:
cat << 'EOF' > .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: llm-wiki-sync
        name: LLM Wiki Knowledge Graph Sync
        entry: target/release/llm-wiki-core git-delta
        language: system
        stages: [post-commit]
        files: '(?i)\.md$'
        pass_filenames: true
        always_run: false
EOF

prek install --hook-type post-commit
```

### Scenario 2A: Code-Only Commit (Bypass Test)
```bash
# Stage and commit a Rust or TypeScript file
touch crates/core/src/dummy.rs
git add crates/core/src/dummy.rs
git commit -m "test: code only commit"
```
*Expected Outcome*: `prek` evaluates `files: '(?i)\.md$'`, detects no Markdown files, and finishes in $< 2\text{ ms}$ without running `llm-wiki-core`.

### Scenario 2B: Documentation Commit (Delta Sync Test)
```bash
# Start sidecar daemon in background
target/release/llm-wiki-core daemon --port 8989 &
DAEMON_PID=$!

# Modify a Markdown file
echo "Check out [[new_feature]]" >> docs/sample.md
git add docs/sample.md
git commit -m "docs: add new feature link"
```
*Expected Outcome*: `prek` triggers `llm-wiki-core git-delta docs/sample.md`, which dispatches delta JSON payload to daemon on port 8989. Graph updates in $< 50\text{ ms}$.

### Scenario 2C: Fail-Open Resilience Test (Offline Sidecar)
```bash
# Kill sidecar daemon
kill $DAEMON_PID

# Commit another Markdown file
echo "Another doc note" >> docs/sample.md
git add docs/sample.md
git commit -m "docs: test fail-open commit"
```
*Expected Outcome*: Hook attempts socket connection, hits the 500 ms non-blocking timeout, exits with code 0 (silent fail-open). Git commit succeeds seamlessly.

---

## 5. Validation Scenario 3: Sandboxed Webview & Cluster Coloring (P2)

Launch the VS Code Extension Development Host and verify visual graph behavior.

```bash
cd extension
npm run test:e2e
# Or open in VS Code debug mode (F5)
```

### Visual Verification Checklist:
- [ ] **Level 0 Hub**: Rendered with solid 100% opacity (`rgba(..., 1.0)`) and enlarged radius.
- [ ] **Hop Opacity Decay**: Level 1 nodes at 60% opacity (`0.6`), Level 2 at 30% (`0.3`), Level 3+ at 15% (`0.15`).
- [ ] **Cluster Color Matching**:
  - Notes containing `#architecture` or tagged `architecture` render with purple base hue (`#8B5CF6`).
  - Notes in `docs/tests/**` render with emerald base hue (`#10B981`).
  - Both adhere strictly to their respective hop distance opacity channels.
- [ ] **Dead Link Styling**: Broken references rendered with red dashed lines (`#EF4444`).
- [ ] **Diagnostics Panel**: Active prompt load vs reachable subcall token counter updates dynamically with color indicator (Green $<60\text{k}$, Yellow $60\text{k}\text{--}100\text{k}$, Red $>100\text{k}$).
- [ ] **Node Navigation**: Clicking a node triggers `openFile` RPC and focuses the Markdown file in the active VS Code editor tab.

---

## 6. References & Contracts

- Data Model: [data-model.md](data-model.md)
- JSON-RPC Contract: [contracts/ipc_jsonrpc_protocol.json](contracts/ipc_jsonrpc_protocol.json)
- Webview RPC Contract: [contracts/webview_rpc_protocol.json](contracts/webview_rpc_protocol.json)
- Config Schema: [contracts/context_vault_schema.json](contracts/context_vault_schema.json)
- Git Delta Schema: [contracts/git_delta_contract.json](contracts/git_delta_contract.json)
