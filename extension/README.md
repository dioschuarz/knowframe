# KnowFrame — Knowledge Management Framework

[![VS Code Marketplace Version](https://img.shields.io/visual-studio-marketplace/v/dscorpsolutions.knowframe?style=flat-square&label=Marketplace&color=38bdf8)](https://marketplace.visualstudio.com/items?itemName=dscorpsolutions.knowframe)
[![Visual Studio Marketplace Installs](https://img.shields.io/visual-studio-marketplace/i/dscorpsolutions.knowframe?style=flat-square&color=22c55e)](https://marketplace.visualstudio.com/items?itemName=dscorpsolutions.knowframe)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![Rust 2021](https://img.shields.io/badge/Rust-2021_Edition-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![VS Code](https://img.shields.io/badge/VS_Code-%E2%89%A51.90.0-007ACC.svg?style=flat-square&logo=visual-studio-code)](https://code.visualstudio.com/)

**The open-source knowledge governance compiler and dynamic graph for VS Code.**  
*Prevents AI agent context window saturation, eliminates markdown drift, and slashes prompt token costs through deterministic quality scoring, BPE token budgeting, and automated structural evolution.*

---

## 🚀 Why KnowFrame?

When modern developers and autonomous AI coding agents (Claude Code, Antigravity, Codex, Cursor) navigate extensive documentation repositories and markdown knowledge bases, four systemic failures emerge:

1. **Context Window Saturation**: Agents ingest 50k+ token monolithic documents during initial context traces, driving token costs up and causing attention degradation.
2. **Silent Link Decay & Drift**: Renamed headers, broken wikilinks, and orphaned notes silently accumulate, leaving agents with hallucinations and broken code references.
3. **Knowledge Debt Blindness**: Teams lack metrics on documentation cohesion, coupling, and cyclicity.
4. **Git Regressions**: Broken links and unanchored references get committed into `main` without automated CI/CD gating.

**KnowFrame solves this at compile time.** Built on a high-performance **Rust core engine (`knowframe-core`)** with a JSON-RPC daemon and an interactive VS Code webview, KnowFrame turns markdown repositories into a living, governed knowledge graph.

---

## 🌟 Key Capabilities

### 1. 🕸️ Interactive Dynamic Knowledge Graph
- **Force-Directed Graph Simulation**: Real-time D3 physics simulation with smooth layout convergence.
- **Radial Hub Pinning**: Click any document to make it the central radial hub and explore concentric hop neighborhoods (1-hop direct connections, 2-hop peripheral nodes).
- **Cluster Color-Coding**: Automatic grouping and coloring by directory hierarchy or tags with customizable opacity falloffs.
- **Instant Search & Focus**: Fast filter by file path, tag, or connection count with hotkey navigation.

### 2. 🛡️ Deterministic 5-Pillar Quality Scoring (0–100)
Sub-second, mathematical evaluation of documentation health without non-deterministic LLM hallucinations:
- **Pillar 1: Integrity (Weight: 25%)**: Detects broken wikilinks, circular file paths, unanchored section references, and missing target documents.
- **Pillar 2: Structure (Weight: 25%)**: Identifies God Documents (>3,000 tokens), taxonomy cycles, reference concentration, and AI harness route drifts.
- **Pillar 3: Consistency (Weight: 20%)**: Flags homograph term collisions, synonym ring drift, and metadata schema inconsistencies.
- **Pillar 4: Discoverability (Weight: 15%)**: Discovers isolated silos, unindexed leaves, and high-degree bridge cut vertices.
- **Pillar 5: Traceability & Freshness (Weight: 15%)**: Audits Git commit decay, unreviewed documentation obligations, and stale AST code dependencies.

### 3. 📊 Active Prompt Load & BPE Token Observatory
- **Real-Time BPE Token Counting**: Exact byte-pair encoding (cl100k_base) token calculations for every document in the vault.
- **Active Prompt Load Gauge**: Visualizes total prompt burden with configurable warning thresholds (default: 60k tokens) and critical limits (100k tokens).
- **Short Context Tracing**: Constrained beam-search route discovery allowing agents to trace only relevant subtrees.

### 4. 🔒 Knowledge Debt Baselining & Pre-Commit Git Admission Control
- **Zero-Tolerance Baseline Ratchet**: Record existing legacy debt in `.context-vault/baseline.json`. Future commits are guaranteed to never introduce new regressions (`knowframe-core validate-candidate`).
- **Sub-500ms Pre-Commit Admission Hooks**: Seamless integration with [prek](https://github.com/jdx/prek) and standard Git hooks to validate file changes before committing.

### 5. 🧬 18 KOS Structural Evolution Strategies
- Automated detection of structural debt signatures: **God Document**, **Fragmented Concept**, **Isolated Silo**, **Implicit Router**.
- Evaluates candidate remediation strategies: Topic Decomposition, Section Addressing, Faceted Classification, Thesaurus Relational Reciprocity, Lightweight Routing, and Metadata Materialization.
- Interactive **Unified Diff Preview** and atomic one-click filesystem transformation with rollback guarantees.

### 6. ⚡ Dual-Interface Parity (Deterministic Fast-Path + AI Review)
- **Deterministic Split-Button `[ Review | ▾ ]`**: Executes native algorithmic fixes instantly with zero LLM tokens.
- **Dropdown Menu `✨ Review with AI`**: Delegates complex semantic reasoning to configured AI coding agents (Claude Code, Antigravity, Codex, Cursor) with isolated, targeted prompt envelopes.

---

## 📦 Installation

### From Visual Studio Marketplace
1. Open VS Code.
2. Press `Ctrl+P` (or `Cmd+P` on macOS) and run:
   ```bash
   ext install dscorpsolutions.knowframe
   ```
3. Alternatively, search for **KnowFrame** in the Extensions view (`Ctrl+Shift+X`).

### From VSIX Package
Download the latest `knowframe-1.0.0.vsix` from [GitHub Releases](https://github.com/dioschuarz/knowframe/releases) and install via CLI:
```bash
code --install-extension knowframe-1.0.0.vsix
```

---

## 🛠️ Quickstart

1. **Open your Markdown repository or Obsidian vault** in VS Code.
2. Open the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`) and type:
   - `KnowFrame: Show Graph` — Opens the dynamic interactive knowledge graph.
   - `KnowFrame: Open Management & Settings` — Opens the Health, Governance, Exclusions, and Evolution drawers.
   - `KnowFrame: Install Git Hooks` — Automatically configures the sub-500ms pre-commit admission control hook.
   - `KnowFrame: Fix Broken Links` — Automatically scans and fixes broken references across the vault.

---

## 💻 CLI Tools

KnowFrame ships with standalone Rust CLI binaries for CI/CD pipelines and headless environments:

### `knowframe-core`
```bash
# Full repository health audit and defect report
knowframe-core issues

# Validate Git commit candidates against baseline debt
knowframe-core validate-candidate --staged

# Update baseline snapshot
knowframe-core baseline-update

# Run headless background JSON-RPC daemon
knowframe-core daemon --root /path/to/repo --port 8765
```

### `knowframe-cli`
```bash
# Quick quality score summary
knowframe-cli score

# Trace minimal context path between documents
knowframe-cli trace --from docs/index.md --to docs/api.md
```

---

## ⚙️ Configuration Settings

Configure KnowFrame via VS Code Settings (`settings.json`):

| Setting | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `llmWiki.daemonPort` | `number` | `0` | Port for the background JSON-RPC daemon (`0` = OS-assigned ephemeral). |
| `llmWiki.gitHookEngine` | `string` | `"prek"` | Hook orchestrator preference (`prek`, `pre-commit`, or `script`). |
| `llmWiki.tokenBudgetWarning` | `number` | `60000` | Warning threshold for Active Prompt Load tokens. |
| `llmWiki.tokenBudgetCritical` | `number` | `100000` | Critical threshold for Active Prompt Load tokens. |

---

## 🏗️ Repository Architecture

KnowFrame is organized as a Cargo workspace with an integrated VS Code extension:

```
knowframe/
├── Cargo.toml                  # Cargo Workspace configuration (v1.0.0)
├── crates/
│   ├── core/                   # High-performance Rust quality engine & graph compiler
│   ├── cli/                    # Headless CLI driver for CI/CD gates and scripting
│   └── engine/                 # JSON-RPC 2.0 daemon and WebSocket server
├── extension/                  # VS Code Extension Host & Webview UI
│   ├── package.json            # Extension manifest (dscorpsolutions.knowframe)
│   ├── src/                    # TypeScript extension host (lifecycle, RPC, admission)
│   ├── webview-ui/             # D3 Force graph canvas, management drawer, and components
│   └── bin/                    # Platform-specific native binary distribution
└── scripts/
    └── distribute.sh           # Clean build, package & local installation pipeline
```

---

## 🤝 Contributing

Contributions are warmly welcome! Please feel free to submit issues and pull requests to our [GitHub Repository](https://github.com/dioschuarz/knowframe).

1. Fork the repository.
2. Create your feature branch (`git checkout -b feat/amazing-feature`).
3. Ensure all tests pass:
   ```bash
   cargo test
   npm test --prefix extension
   ```
4. Commit your changes (`git commit -m 'feat: add amazing feature'`).
5. Push to the branch (`git push origin feat/amazing-feature`).
6. Open a Pull Request.

---

## 📄 License

Distributed under the MIT License. See [`LICENSE`](https://github.com/dioschuarz/knowframe/blob/main/LICENSE) for more information.

---

*Authored by **[DS Corp Solutions](https://github.com/dioschuarz)** — Knowledge Management & Architecture for Autonomous AI Systems.*
