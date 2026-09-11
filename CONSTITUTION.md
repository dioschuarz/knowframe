# LLM Wiki Extension for VS Code (`llm_wiki_ext4vsc`) Constitution

> Canonical Source of Truth: [`.specify/memory/constitution.md`](.specify/memory/constitution.md)

---

<!--
=== SYNC IMPACT REPORT ===
Version Change: 1.2.0 -> 1.3.0 (AI Context Observatory & Progressive Disclosure UX)
Modified Principles:
  - Principle III: Organic Discovery & Curated Representation -> Upgraded with "Filesystem is source, knowledge graph is curated representation", tri-state classification (Connected, Orphan, Excluded), and two-tier exclusion architecture (scan.exclude vs graph.exclude) ensuring excluded nodes do not pollute orphan metrics or canvas space while remaining resolvable link targets.
  - Principle IV: Multi-Tier Context Hierarchy & BPE Token Budgeting -> Formalized the AI Context Observatory paradigm, semantic tier-based node sizing, focus modes (Hub, 1-hop, 2-hops, All), and explicit distinction between Active Prompt Load and Reachable On-Demand Context.
  - Principle VI: Sandboxed Webview & Zero-Plaintext Security Boundary -> Expanded with strict Action Hierarchy and Progressive Disclosure: zero UI action duplication across sidebar/toolbar/menus, state-driven action emergence, 3 core pillars (Knowledge, AI Context, Health & Issues), and selection-driven contextual inspectors.
Added Principles: None
Added Sections: None
Removed Sections: None
Follow-up TODOs: None deferred.
==========================
-->

## Core Principles

### I. Dual-Process Separation & Vertical Optimization (NON-NEGOTIABLE)
The system MUST be architected into two decoupled execution tiers: a lightweight Node.js/TypeScript Extension Host and a standalone Rust Core Engine sidecar (`llm-wiki-core`).
- **Heavy Compute in Rust**: Parsing Markdown AST (`pulldown-cmark`), computing graph topologies and centrality metrics (`petgraph`), evaluating token economics (`tiktoken-rs` with `cl100k_base`), and parsing Git diff deltas MUST be executed exclusively by the Rust sidecar process.
- **Node.js as Lean UI Orchestrator**: The VS Code Extension Host process MUST NOT perform file scraping, heavy AST parsing, or graph math. Its responsibilities are strictly constrained to process lifecycle management, IPC/WebSocket bridging, native VS Code API dispatch, and Webview hosting.
- **Resource Ceiling**: The Rust binary MUST NOT exceed 35 MB of resident RAM in resting state for vaults up to 5,000 Markdown files. Full vault graph calculation for 1,000 notes MUST execute in under 200 ms.

### II. Event-Driven Git Hook Lifecycle with Rust-Native `prek` Orchestration
The extension MUST NOT employ continuous disk file watchers or background keystroke polling that introduces CPU spikes or typing latency in the editor.
- **Architectural Motivations for `prek`**:
  1. *Rust-to-Rust Homogeneity*: Both the Git hook manager (`prek`) and the knowledge graph engine (`llm-wiki-core`) are written in Rust, creating an ultra-lean, zero-runtime systems stack that eliminates Python runtime requirements, virtualenv bootstrapping, and interpreter startup latency.
  2. *True Parallel Hook Execution*: `prek` executes hooks concurrently across a multi-threaded worker pool. The knowledge graph synchronization (`llm-wiki-sync`) runs in parallel alongside other project linters, formatters, and security checks without inducing sequential lock contention or delaying commits.
  3. *Pattern Reusability via Standard `.pre-commit-config.yaml`*: Leverages the universal, declarative `.pre-commit-config.yaml` file pattern as a drop-in replacement, requiring zero proprietary configuration DSLs or workflow migration for existing engineering teams.
  4. *Zero-Overhead File Filtering*: Declarative regex filtering (`files: '(?i)\.md$'`) guarantees that commits modifying only source code bypass hook execution in $< 2\text{ ms}$ without spawning the sidecar.
- **Multi-Tier Fallback Hierarchy**: Hook installation and dispatch MUST support a 3-tier discovery hierarchy:
  1. *Tier 1 (Rust-Native / Primary)*: `prek` executing a declarative `.pre-commit-config.yaml` with hook ID `llm-wiki-sync` and stage `post-commit` (or `pre-commit`).
  2. *Tier 2 (Ecosystem Compatibility)*: Python `pre-commit` or `lefthook` if already integrated in the developer's repository.
  3. *Tier 3 (Zero-Dependency Direct Script)*: Standalone `.git/hooks/post-commit` shell dispatcher executing `llm-wiki-core git-delta` directly.
- **Strict Non-Blocking Fail-Open**: All hook invocations MUST enforce a hard non-blocking timeout of $\le 500\text{ ms}$. If the socket is unresponsive, the sidecar is offline, or `prek` encounters an error, the hook MUST immediately exit with code 0 (silent fail-open), never obstructing or delaying developer Git commits.
- **Delta Processing & Loopback Authorization**: The hook transmits only changed Markdown paths directly to `llm-wiki-core`, enabling instantaneous incremental graph updates ($\le 50\text{ ms}$) without re-indexing unchanged workspace files. Loopback `git/commitDelta` notifications on `127.0.0.1` are write-only notifications carrying Git metadata, accepted without requiring `vscode.SecretStorage` credentials so external terminal commits succeed non-blockingly.

### III. Organic Discovery, Curated Representation & Noise Exclusion
The knowledge graph MUST build organically from the user's existing filesystem chaos without demanding YAML frontmatter, rigid directory structures, or canonical entry filenames (`README.md`, `index.md`).
- **Core Knowledge Axiom**: *The filesystem is the source; the knowledge graph is a curated representation of what matters.* The system is not a raw file tree visualizer; its mandate is to distill structural knowledge relevant to developer tasks and AI agent context.
- **Tri-State Node Classification**: Every Markdown document known to the system MUST belong to exactly one of three lifecycle states:
  1. **Connected**: Part of the knowledge graph and reachable via link traversals.
  2. **Orphan**: Part of the knowledge graph but possessing zero inbound and outbound connections.
  3. **Excluded**: Intentionally outside the knowledge graph by user directive or default workspace hygiene rules.
- **Two-Tier Exclusion Architecture**:
  1. *Technical Scan Exclusion (`scan.exclude`)*: Files and directories matching technical ignore globs (e.g. `.git/**`, `node_modules/**`, `target/**`, `dist/**`, `build/**`) are completely bypassed by the AST parser and indexer.
  2. *Curated Knowledge Exclusion (`graph.exclude`)*: Files and directory trees (e.g. package documentation, library `**/README.md`, vendor licenses) that are parsed and indexed to maintain link integrity, but are intentionally excluded from the rendered graph and orphan calculations.
- **Target Resolvability Invariant**: Valid Markdown links or wikilinks targeting a `graph.exclude` document MUST resolve cleanly and MUST NOT be classified as broken links. Excluded targets are valid knowledge destinations that the user has chosen not to visualize.
- **Algorithmic Hub Discovery**: The Rust engine MUST calculate degree centrality (in-degree, out-degree) and betweenness centrality across all unexcluded Markdown files to mathematically deduce central knowledge hubs.
- **User Agency & Consent**: The system MUST propose top centrality nodes as default hubs and propose noisy file clusters for exclusion, but MUST NEVER silently exclude documentation or alter central hubs without explicit user consent.

### IV. Multi-Tier Context Hierarchy, Semantic Sizing, Focus Mode & Token Budgeting
The extension MUST function as an **AI Context Observatory**, providing instant visual clarity on agent context window consumption to prevent context saturation in AI agents (Claude Code, Codex, Antigravity, Roo/Cline).
- **Context Tiers & Opacity Decay Law**:
  - **Level 0 (Active Context / Harness Base)**: Fixed agent prompts and harness files (`AGENTS.md`, `CLAUDE.md`, or selected Hub). Rendered with 100% solid opacity and expanded node radius.
  - **Level 1 (First Hop)**: Files directly referenced by Level 0. Rendered at 60% opacity.
  - **Level 2+ (Deep Hops)**: Transitive reference chains. Rendered at decaying opacity (30% at Level 2, 15% at Level 3+).
  - **Orphan Nodes**: Disconnected notes with zero edges. Rendered in muted gray (`#6B7280`) on the graph perimeter unless custom cluster rules override.
- **Semantic Visual Hierarchy & Sizing**: Node geometry MUST visually communicate context role rather than raw connection volume alone. Level 0 hubs are rendered with prominent radius and bold labels; Level 1 with medium radius; Level 2 with compact radius; Level 3+ with subtle indicators.
- **Focus Modes**: To eliminate visual overwhelm in large vaults (500+ notes), the graph renderer MUST support hop-constrained focus view states (`Hub`, `1 hop`, `2 hops`, `All`). In focused modes, secondary or peripheral nodes are subdued or collapsed to keep cognitive load minimal.
- **User-Defined Cluster Coloring (Hashtags & Paths)**:
  - The system MUST allow users to define custom chromatic cluster groups based on:
    1. *Cluster Hashtags*: Extracted from inline Markdown hashtags (`#tag`) and YAML frontmatter `tags: [...]`.
    2. *Filesystem Paths*: Directory globs matching note physical paths (e.g. `docs/architecture/**`, `notes/05_secrets/**`).
  - *Hue + Opacity Composition Invariant*: User-defined cluster colors determine the **base hue/saturation** of the node. The **alpha/opacity channel is strictly governed by the hop distance** (Level 0 = 1.0, Level 1 = 0.6, Level 2 = 0.3, Level 3+ = 0.15). This guarantees that custom styling enriches semantic clustering without compromising the visual differentiation between active and secondary LLM context.
- **BPE Token Accounting**: The Rust engine MUST compute exact BPE tokens (`cl100k_base`) per file and cumulative branch cost. The UI MUST display **Active Prompt Load (L0)** versus **Reachable On-Demand Context (L1+)**, raising warnings at 60,000 tokens and critical alerts at 100,000 tokens.

### V. Strict Link Integrity & Cross-Submodule Vault Traversal
The engine MUST enforce 100% internal link validation across both standard Markdown links `[text](path)` and wikilinks `[[filename]]`.
- **Dead Link Detection**: Links pointing to missing files, broken anchors, or moved targets MUST be immediately flagged with red dashed edges, diagnostic issue cards, and line-precise coordinates.
- **Multi-Repo & Submodule Boundary Resolution**: In multi-repo and unified vault configurations, relative links crossing Git submodule boundaries MUST be resolved transparently without fragmenting the graph topology.

### VI. Sandboxed Webview, Progressive Disclosure & Zero Action Duplication
All embedded interfaces and runtime interactions MUST adhere to strict information architecture and security principles:
- **Calm Control Center & The Three Pillars**: The user experience MUST be organized around three calm, distinct pillars:
  1. **Knowledge**: Structural topology and note relationships.
  2. **AI Context**: Prompt token load, reachable context horizons, and budget limits.
  3. **Health & Issues**: Broken links, unmapped orphans, and synchronization status.
- **Zero Action Duplication**: Administrative actions MUST NOT be duplicated across competing UI surfaces (e.g. sidebar buttons, top toolbar buttons, and dropdown items simultaneously). Primary controls are reserved exclusively for active graph manipulation (Token status, Search, Filter/Focus, Reset Disposition, More).
- **State-Driven Action Emergence**: High-friction administrative operations MUST emerge naturally from displayed system states rather than existing as permanent clutter:
  - Broken link issues appear as status metrics (`343 broken links → [Review issues]`).
  - Vault synchronization appears as a quiet status indicator (`● Synced · 12s ago`), with full index rebuild relegated to advanced menus.
  - Central Hub selection is displayed as the active entity with a contextual `[Change]` workflow.
  - Git hook management resides in Settings / Integrations, not as a primary dashboard button.
- **Selection-Driven Contextual Inspector**: When an element is selected (node, edge, or issue), the Webview MUST present a contextual inspector drawer containing relevant metadata and targeted actions (`[Open file]`, `[Set as hub]`, `[Exclude file/folder]`, `[Fix reference]`).
- **Webview Sandboxing & CSP**: Webview panels MUST enforce a strict Content Security Policy (CSP) with unique cryptographic nonces (`script-src 'nonce-${nonce}'`). Local assets MUST strictly be loaded via `webview.asWebviewUri` under allowed `localResourceRoots`.
- **Typed Asynchronous RPC**: Communication between the Extension Host and Webview iframe MUST use strongly typed JSON-RPC 2.0 messages with request-response correlation tokens (`requestId`).
- **Memory-Safe State Lifecycle**: Webview panel state MUST be serialized via `vscode.setState()` / `vscode.getState()` and `WebviewPanelSerializer`.
- **Zero Plaintext Secrets**: Any credentials, tokens, or private settings MUST be stored exclusively via `vscode.SecretStorage` (backed by OS Keychains/safeStorage) and NEVER written to plaintext configuration files or disk logs.

### VII. Test-Driven Development (TDD) & Multi-Platform Binary Verification
Quality and cross-platform stability are guaranteed through strict automated test gates before code merge:
- **Rust Core Test Suite**: 100% test coverage for Markdown AST parsing, wikilink normalization, petgraph centrality computation, and BPE tokenization.
- **Extension Host Integration Tests**: Headless VS Code extension tests (`@vscode/test-electron`) verifying sidecar process spawning, IPC socket resilience, and Webview message roundtrips.
- **Multi-Platform Support**: Binaries MUST be built and tested for Linux (x86_64, aarch64), macOS (x86_64, Apple Silicon aarch64), and Windows (x86_64).

---

## Technology Stack & Performance Standards

### 1. Technology Matrix
- **Core Engine (Sidecar)**: Rust 2021/2024 edition, `pulldown-cmark` (AST Markdown parser), `petgraph` (graph data structure and centrality algorithms), `tiktoken-rs` (cl100k BPE token counter), `tokio` (async runtime), `tokio-tungstenite` / Unix Domain Sockets / Windows Named Pipes (IPC/WebSocket transport), `serde` / `serde_json` / `serde_yaml` (data serialization).
- **Extension Host**: Node.js 20+, TypeScript 5.4+, VS Code Extension API (`^1.90.0`), `ws` / IPC client.
- **Webview UI**: High-performance 2D HTML5 Canvas force-directed graph renderer (`d3-force`), `@vscode/webview-ui-toolkit` Web Components, CSS custom properties adhering to active VS Code theme tokens.
- **Configuration Contract**: `.context-vault.yaml` placed at workspace root.

### 2. Non-Functional Performance Budgets
| Metric | Threshold | Verification Method |
| :--- | :--- | :--- |
| **Rust Sidecar Memory (Rest)** | $\le$ 35 MB resident RSS for 5,000 files | Valgrind / Heap profiling in CI |
| **Graph Analysis Latency** | $\le$ 200 ms for 1,000 Markdown files | Criterion microbenchmarks |
| **Webview Rendering Rate** | Consistent 60 FPS during pan/zoom/drag | Chrome DevTools Frame Meter |
| **Git Hook Timeout Ceiling** | Strict 500 ms non-blocking fail-open | Unit test with hanging socket |
| **Packaging Size** | $\le$ 25 MB per platform-specific VSIX | `@vscode/vsce package` artifact audit |

---

## Development Workflow, Review Process & Quality Gates

### 1. Code Quality Gates
1. **Red-Green-Refactor Cycle**: All new parsers, graph algorithms, or IPC handlers must start with failing unit tests prior to implementation.
2. **Formatting & Linter Compliance**:
   - Rust: `cargo fmt --check` and `cargo clippy -- -D warnings`.
   - TypeScript: `eslint` with strict type checking and `prettier --check`.
3. **Packaging Discipline**: Production `.vsix` packaging MUST use platform-specific targets (`vsce package --target <platform>`) and exclude unnecessary build artifacts via `.vscodeignore`.

### 2. Review Protocol
- Any PR introducing filesystem watchers or continuous background polling MUST be rejected.
- Any PR modifying IPC contracts MUST include synchronized updates and tests in both the Rust engine and TypeScript extension.
- Every architectural change MUST reference this Constitution and update the decisions log (`decisions_log.md`).

---

## Governance

This Constitution represents the supreme architectural and technical policy for the `llm_wiki_ext4vsc` project.
- **Supremacy**: In any conflict between feature requests, quick fixes, and these principles, this Constitution governs.
- **Amendments**: Amendments require documented rationale, impact assessment on existing performance budgets, and consensus approval.
- **Versioning Policy**:
  - **MAJOR**: Incompatible principle changes or fundamental architecture modifications.
  - **MINOR**: Addition of new principles, sections, or materially expanded guidelines.
  - **PATCH**: Clarifications, wording adjustments, or typo fixes.

**Version**: 1.3.0 | **Ratified**: 2026-09-11 | **Last Amended**: 2026-09-11
