# LLM Wiki Extension for VS Code (`llm_wiki_ext4vsc`) Constitution

> Canonical Source of Truth: [`.specify/memory/constitution.md`](.specify/memory/constitution.md)

---

<!--
=== SYNC IMPACT REPORT ===
Version Change: 1.3.0 -> 1.4.0 (Knowledge Quality Admission Control, Git State Boundary & Brownfield Debt Lifecycle)
Modified Principles:
  - Principle I: Dual-Process Separation & Vertical Optimization -> Reinforced with strict Local Determinism & Zero External AI Invariant (no LLM APIs, remote embeddings, external search, or rerankers).
  - Principle II: Event-Driven Git Hook Lifecycle -> Upgraded to "Git Knowledge State Boundary, Ephemeral Candidate Graph & Dual Hook Lifecycle" with two non-negotiable invariants:
      1. The Knowledge Graph MUST represent only committed repository state; working-tree changes analyzed in candidate graph in-memory only without mutating the persistent graph.
      2. Pre-commit validation MUST prevent new regressions while preserving and exposing legacy debt.
    Formally established dual hooks: Hook 1 (`llm-wiki-validate`, pre-commit) and Hook 2 (`llm-wiki-sync`, post-commit).
Added Principles:
  - Principle VI: Brownfield Debt Baseline & Continuous Regression-Based Admission Control (Baseline creation, legacy debt vs new regressions, PASS/WARN/BLOCK decision engine, debt reduction convergence).
  - Principle VII: Deterministic Knowledge Drift, Reverse-Index Impact & Semantic Collision Engine (Technical drift via code-to-doc reverse indexes, semantic drift via multi-signal deterministic fingerprints without N×N search, candidate canonical ownership).
  - Principle VIII: Context Trace & Network-Aware Knowledge Quality Score (Graph-constrained traversal from entry points like AGENTS.md, token accounting, depth & path quality, topology-weighted health scoring).
Renumbered Principles:
  - Previous Principle VI (Sandboxed Webview) -> Principle IX.
  - Previous Principle VII (TDD & Quality Gates) -> Principle X.
Follow-up TODOs: None deferred.
==========================
-->

## Core Principles

### I. Dual-Process Separation, Local Determinism & Zero External AI (NON-NEGOTIABLE)
The system MUST be architected into two decoupled execution tiers: a lightweight Node.js/TypeScript Extension Host and a standalone Rust Core Engine sidecar (`llm-wiki-core`).
- **Heavy Compute in Rust**: Parsing Markdown AST (`pulldown-cmark`), computing graph topologies and centrality metrics (`petgraph`), evaluating token economics (`tiktoken-rs` with `cl100k_base`), maintaining reverse code-doc indexes, evaluating deterministic similarity, and parsing Git diff deltas MUST be executed exclusively by the Rust sidecar process.
- **Node.js as Lean UI Orchestrator**: The VS Code Extension Host process MUST NOT perform file scraping, heavy AST parsing, or graph math. Its responsibilities are strictly constrained to process lifecycle management, IPC/WebSocket bridging, native VS Code API dispatch, and Webview hosting.
- **Zero External AI / LLM Invariant**: The product MUST NOT use external LLM APIs, remote embeddings, external cloud semantic search, remote rerankers, or external AI services. All analysis, drift detection, and candidate evaluation MUST be executed locally and deterministically.
- **Resource Ceiling**: The Rust binary MUST NOT exceed 35 MB of resident RAM in resting state for vaults up to 5,000 Markdown files. Full vault graph calculation for 1,000 notes MUST execute in under 200 ms.

### II. Git Knowledge State Boundary, Ephemeral Candidate Graph & Dual Hook Lifecycle (NON-NEGOTIABLE)
The system MUST NOT continuously monitor file saves or keystrokes. The boundary of persistent knowledge is strictly the Git commit.
- **The Knowledge Graph State Boundary Invariant**:
  > *The Knowledge Graph MUST represent only committed repository state. Working-tree changes MAY be analyzed in an ephemeral candidate graph during pre-commit validation, but MUST NOT mutate the persistent Knowledge Graph until the corresponding Git commit succeeds.*
  A file saved in the editor is NOT a knowledge graph update (`file saved ≠ knowledge graph update`).
- **Dual Analytical Modes**:
  1. *Mode A (Admission Control)*: Executed during `pre-commit` against an in-memory candidate graph. Answers: *"Can this change enter the knowledge base?"* Potentially blocks or warns.
  2. *Mode B (Knowledge State Analysis)*: Executed against the persistent committed graph. Answers: *"What is the current state of my knowledge base?"* Never mutates until commit succeeds.
- **Ephemeral Candidate Graph Construction**:
  During pre-commit, Rust constructs an ephemeral candidate graph:
  $$\text{Candidate Graph} = \text{Committed Graph} + \text{Staged Git Delta}$$
  The candidate graph exists solely in memory. If validation passes, the commit proceeds, and post-commit incorporates the delta into the persistent index. If validation blocks or the user aborts, the candidate graph is discarded without contaminating persistent state.
- **Dual Hook Architecture via `prek`**:
  1. *Hook 1 (`llm-wiki-validate`, stage: `pre-commit`)*: Analyzes staged files (Markdown and code), performs reverse impact analysis, checks for regressions against the debt baseline, and returns `PASS`, `WARN`, or `BLOCK`.
  2. *Hook 2 (`llm-wiki-sync`, stage: `post-commit`)*: Applies the committed Git delta to the persistent Rust Knowledge Graph via incremental patching ($\le 50\text{ ms}$).
- **Hook Discovery & Execution Fallbacks**:
  1. *Tier 1 (Rust-Native / Primary)*: `prek` executing declarative `.pre-commit-config.yaml` with parallel hook execution.
  2. *Tier 2 (Ecosystem Compatibility)*: Python `pre-commit` or `lefthook` if already present in repository.
  3. *Tier 3 (Zero-Dependency Script)*: Standalone `.git/hooks/` shell dispatchers executing `llm-wiki-core validate` and `llm-wiki-core git-delta`.
- **Strict Non-Blocking Fail-Open**: All hook invocations MUST enforce a hard non-blocking timeout of $\le 500\text{ ms}$. If the socket is unresponsive, the sidecar is offline, or the daemon encounters an internal error, the hook MUST immediately exit with code 0 (silent fail-open), never obstructing or delaying developer Git commits.
- **Loopback Authorization**: Loopback `git/validate` and `git/commitDelta` notifications on `127.0.0.1` are write-only notifications carrying Git metadata, accepted without requiring `vscode.SecretStorage` credentials so external terminal commits succeed non-blockingly.

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
  - *Neutral Grey Baseline*: Unstyled nodes default to neutral grey (`#64748B` for connected, `#6B7280` for orphans). Grey strictly signifies that no custom cluster color rule has been assigned.
- **BPE Token Accounting**: The Rust engine MUST compute exact BPE tokens (`cl100k_base`) per file and cumulative branch cost. The UI MUST display **Active Prompt Load (L0)** versus **Reachable On-Demand Context (L1+)**, raising warnings at 60,000 tokens and critical alerts at 100,000 tokens.

### V. Strict Link Integrity & Cross-Submodule Vault Traversal
The engine MUST enforce 100% internal link validation across both standard Markdown links `[text](path)` and wikilinks `[[filename]]`.
- **Dead Link Detection**: Links pointing to missing files, broken anchors, or moved targets MUST be immediately flagged with red dashed edges, diagnostic issue cards, and line-precise coordinates.
- **Multi-Repo & Submodule Boundary Resolution**: In multi-repo and unified vault configurations, relative links crossing Git submodule boundaries MUST be resolved transparently without fragmenting the graph topology.

### VI. Brownfield Debt Baseline & Continuous Regression-Based Admission Control (NON-NEGOTIABLE)
A repository adopted by LLM Wiki may start with legacy broken links, orphans, collisions, and stale documents. The product MUST NOT make adoption impossible by immediately blocking existing problems.
- **Regression-Based Enforcement Invariant**:
  > *Pre-commit validation MUST prevent the introduction of new high-confidence knowledge-quality regressions while preserving and exposing pre-existing brownfield debt.*
- **Knowledge Debt Baseline (`baseline.json`)**:
  - The Rust Core establishes a baseline of known findings upon initialization or initial scan (`baseline_commit = <sha>`), recording counts of legacy broken links, orphans, collisions, and drift findings.
  - Existing debt is reported, tracked, and prioritized for remediation, but NEVER blocks developer commits.
- **Continuous Enforcement Lifecycle**:
  - New issues introduced by a commit (new broken links, new high-confidence collisions, new technical contradictions) are classified as **New Regressions** and trigger pre-commit `BLOCK`.
  - When developers fix legacy problems, the baseline progressively ratchets down toward zero ($\Delta < 0$).
  - The system MUST track five distinct accounting dimensions: $\text{Total Debt}$, $\text{Legacy Debt}$, $\text{New Debt}$, $\text{Resolved Debt}$, and $\text{Regressions}$.
- **Decision Engine Thresholds**:
  - `BLOCK`: High-confidence regressions that must not enter the repo (new broken internal references, new invalid heading anchors, high-confidence duplicate with canonical collision).
  - `WARN`: Ambiguous or potential problems requiring human judgment (moderate semantic similarity $\ge 75\%$, potential fragmentation, potential drift with incomplete evidence).
  - `PASS`: No regressions introduced, or only improvements/remediation committed.

### VII. Deterministic Knowledge Drift, Reverse-Index Impact & Semantic Collision Engine
The engine MUST detect divergence between code and documentation and eliminate duplicate conceptual authority without utilizing external LLMs or vector embeddings.
- **Code-Doc Reverse Impact Index**:
  - The Rust engine maintains reverse indexes mapping:
    1. Code file paths $\to$ documentation references citing those symbols/paths.
    2. Document paths $\to$ related code paths and APIs.
    3. Document $\to$ inbound and outbound knowledge links.
  - Staged code changes (e.g. `src/auth/*`) immediately identify the affected documentation surface (`authentication.md`, `oauth.md`) without repository-wide recomputation.
- **Technical Drift Detection (at Pre-Commit)**:
  - Technical drift occurs when documented entities (classes, methods, file references) are deleted, renamed, or modified in code while corresponding documentation remains unchanged.
  - Technical drift MUST be detected at pre-commit so that out-of-sync documentation can be flagged before entering the repository.
- **Deterministic Semantic Collision Engine (at Pre-Commit)**:
  - Avoids $O(N^2)$ exhaustive comparison by generating multi-signal search fingerprints: title fingerprint, heading tokens, BM25/TF-IDF term vectors, tag sets, and structural n-grams.
  - Staged documents retrieve candidate match sets (top 10–50 plausible matches) for detailed deterministic scoring.
  - Similarity combines title matching, heading overlap, term overlap, n-gram overlap, and reference intersection into a normalized similarity score $[0.0, 1.0]$.
- **Canonical Knowledge Candidate Identification**:
  - For overlapping knowledge, the engine proposes canonical candidates based on deterministic signals: Git creation chronology, in-degree centrality, reference by harness entrypoints (`AGENTS.md`), and heading completeness.
  - *Chronology Invariant*: Git creation date is evidence, NOT infallible authority. The system suggests canonical ownership; the developer confirms.

### VIII. Context Trace & Network-Aware Knowledge Quality Score
The engine MUST evaluate both structural graph health and agent discoverability to guide developers toward optimal knowledge architecture.
- **Context Trace Formulation**:
  - Evaluates: *"Given a concrete task query, how discoverable is the relevant knowledge starting from the project's instruction entry point (e.g. `AGENTS.md`)?"*
  - Executes deterministically: $\text{Query} \to \text{BM25 Term Match} \to \text{Graph-Constrained Traversal} \to \text{Top-K Candidate Paths} \to \text{Path Scoring \& BPE Token Accounting}$.
  - Outputs: Discovery path hops, path traversal depth, total relevant BPE tokens, and Path Quality score ($0\text{--}100\%$).
- **Topological Quality Scoring**:
  - Overall Knowledge Quality Score ($0\text{--}100$) aggregates four balanced dimensions:
    1. **Integrity** (broken links, missing anchors, unmapped targets).
    2. **Consistency** (collisions, canonical conflict, fragmentation).
    3. **Discoverability** (Context Trace path depth, graph centrality distribution, orphan ratio).
    4. **Freshness** (technical drift, code-doc staleness).
  - *Topology Weighting*: Quality deductions are weighted by node centrality (betweenness and degree). A broken link or collision in `AGENTS.md` or a central hub causes significantly higher quality penalties than one in an isolated leaf note.
  - *Debt vs Regression Clarity*: The Quality Score UI MUST explicitly distinguish between Legacy Debt and New Regressions (e.g. `Quality: 74/100 · 0 new regressions · 42 legacy issues`).

### IX. Sandboxed Webview, Progressive Disclosure & Zero Action Duplication
All embedded interfaces and runtime interactions MUST adhere to strict information architecture and security principles:
- **Calm Control Center & The Four Pillars**: The user experience MUST be organized around four calm, distinct pillars:
  1. **Knowledge**: Structural topology, cluster groups, and note relationships.
  2. **AI Context**: Prompt token load, reachable context horizons, and budget limits.
  3. **Health & Issues**: Broken links, unmapped orphans, and synchronization status.
  4. **Quality & Admission**: Quality Score, debt baseline, and pre-commit regression audit.
- **Zero Action Duplication**: Administrative actions MUST NOT be duplicated across competing UI surfaces. Primary controls are reserved exclusively for active graph manipulation.
- **State-Driven Action Emergence**: High-friction administrative operations MUST emerge naturally from displayed system states rather than existing as permanent clutter.
- **Selection-Driven Contextual Inspector**: When an element is selected (node, edge, collision, or issue), the Webview MUST present a contextual inspector drawer containing relevant metadata and targeted actions (`[Open file]`, `[Set as hub]`, `[Exclude file/folder]`, `[Fix reference]`, `[Review collision]`).
- **Webview Sandboxing & CSP**: Webview panels MUST enforce a strict Content Security Policy (CSP) with unique cryptographic nonces (`script-src 'nonce-${nonce}'`). Local assets MUST strictly be loaded via `webview.asWebviewUri` under allowed `localResourceRoots`.
- **Typed Asynchronous RPC**: Communication between the Extension Host and Webview iframe MUST use strongly typed JSON-RPC 2.0 messages with request-response correlation tokens (`requestId`).
- **Memory-Safe State Lifecycle**: Webview panel state MUST be serialized via `vscode.setState()` / `vscode.getState()` and `WebviewPanelSerializer`.
- **Zero Plaintext Secrets**: Any credentials, tokens, or private settings MUST be stored exclusively via `vscode.SecretStorage` (backed by OS Keychains/safeStorage) and NEVER written to plaintext configuration files or disk logs.

### X. Test-Driven Development (TDD) & Multi-Platform Binary Verification
Quality and cross-platform stability are guaranteed through strict automated test gates before code merge:
- **Rust Core Test Suite**: 100% test coverage for Markdown AST parsing, wikilink normalization, petgraph centrality computation, BPE tokenization, baseline debt diffing, reverse code-doc indexing, and deterministic similarity fingerprints.
- **Pre-Commit Hook Simulation Suite**: Automated end-to-end tests validating candidate graph evaluation, `BLOCK` on new broken links/collisions, and silent fail-open $\le 500\text{ ms}$.
- **Extension Host Integration Tests**: Headless VS Code extension tests (`@vscode/test-electron`) verifying sidecar process spawning, IPC socket resilience, and Webview message roundtrips.
- **Multi-Platform Support**: Binaries MUST be built and tested for Linux (x86_64, aarch64), macOS (x86_64, Apple Silicon aarch64), and Windows (x86_64).

---

## Technology Stack & Performance Standards

### 1. Technology Matrix
- **Core Engine (Sidecar)**: Rust 2021/2024 edition, `pulldown-cmark` (AST Markdown parser), `petgraph` (graph data structure and centrality algorithms), `tiktoken-rs` (cl100k BPE token counter), `tokio` (async runtime), `tokio-tungstenite` / Unix Domain Sockets / Windows Named Pipes (IPC/WebSocket transport), `serde` / `serde_json` / `serde_yaml` (data serialization), deterministic n-gram and BM25 tokenizers.
- **Extension Host**: Node.js 20+, TypeScript 5.4+, VS Code Extension API (`^1.90.0`), `ws` / IPC client.
- **Webview UI**: High-performance 2D HTML5 Canvas force-directed graph renderer (`d3-force`), `@vscode/webview-ui-toolkit` Web Components, CSS custom properties adhering to active VS Code theme tokens.
- **Configuration Contract**: `.context-vault.yaml` placed at workspace root.

### 2. Non-Functional Performance Budgets
| Metric | Threshold | Verification Method |
| :--- | :--- | :--- |
| **Rust Sidecar Memory (Rest)** | $\le$ 35 MB resident RSS for 5,000 files | Valgrind / Heap profiling in CI |
| **Graph Analysis Latency** | $\le$ 200 ms for 1,000 Markdown files | Criterion microbenchmarks |
| **Pre-Commit Candidate Validation** | $\le$ 150 ms for typical staged commit | Criterion microbenchmarks |
| **Git Hook Timeout Ceiling** | Strict 500 ms non-blocking fail-open | Unit test with hanging socket |
| **Incremental Graph Patching** | $\le$ 50 ms for single-note delta | Criterion microbenchmarks |
| **Webview Rendering Rate** | Consistent 60 FPS during pan/zoom/drag | Chrome DevTools Frame Meter |
| **Packaging Size** | $\le$ 25 MB per platform-specific VSIX | `@vscode/vsce package` artifact audit |

---

## Development Workflow, Review Process & Quality Gates

### 1. Code Quality Gates
1. **Red-Green-Refactor Cycle**: All new parsers, graph algorithms, drift estimators, or IPC handlers must start with failing unit tests prior to implementation.
2. **Formatting & Linter Compliance**:
   - Rust: `cargo fmt --check` and `cargo clippy -- -D warnings`.
   - TypeScript: `eslint` with strict type checking and `prettier --check`.
3. **Packaging Discipline**: Production `.vsix` packaging MUST use platform-specific targets (`vsce package --target <platform>`) and exclude unnecessary build artifacts via `.vscodeignore`.

### 2. Review Protocol
- Any PR introducing external LLM API calls, cloud embeddings, or remote AI dependencies MUST be immediately rejected.
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

**Version**: 1.4.0 | **Ratified**: 2026-09-11 | **Last Amended**: 2026-09-12
