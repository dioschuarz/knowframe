# Implementation Plan: AI Context Observatory & Curated Knowledge UX (`002-context-observatory-ux`)

**Branch**: `002-context-observatory-ux` | **Date**: 2026-09-11 | **Spec**: [`specs/002-context-observatory-ux/spec.md`](spec.md)

**Input**: Feature specification from `specs/002-context-observatory-ux/spec.md`, Architectural Advisory (`specs/chatgpt_conversation.md`), Constitution v1.3.0

---

## Summary

This plan executes the transition of `llm_wiki_ext4vsc` into an **AI Context Observatory**:
1. **Calm Sidebar Dashboard (P0)**: Eliminates redundant action buttons (`Open Knowledge Graph`, `Set Central Hub`, `Rescan Vault`, `Fix Broken Links`, `Install Git Hooks`), structuring the left sidebar strictly around the 3 pillars: Knowledge, AI Context, and Health.
2. **Contextual Selection Inspector (P1)**: Slide-over drawer displaying note metadata, tokens, hop level, and contextual actions (`[Open file]`, `[Set as hub]`, `[Exclude from graph]`) or broken link diagnostics.
3. **Noise Management & Curated Exclusions (P1)**: Two-tier exclusion architecture (`scan.exclude` vs `graph.exclude`) with `globset` evaluation in Rust core, target resolvability guarantee, preview dialog, and automated noise suggestions (`**/README.md`, `vendor/**`).
4. **Hop-Constrained Focus Modes & Semantic Sizing (P1)**: Hop filtering (`Hub`, `1 hop`, `2 hops`, `All`) and tier-based radius scaling on the Webview Canvas.
5. **Advanced Settings Drawer (P2)**: Progressive disclosure for Git hooks, hub rules, exclusion paths, and sidecar diagnostics.

---

## Technical Context

**Language/Version**:
- Backend / Core Engine: Rust 1.78+ (2021/2024 edition)
- Extension Host & Webview UI: TypeScript 5.4+, Node.js 20+

**Primary Dependencies**:
- Rust: `pulldown-cmark` 0.11, `petgraph` 0.6, `tiktoken-rs` 0.5, `tokio` 1.38, `tokio-tungstenite` 0.23, `globset` 0.4, `serde` / `serde_json` 1.0, `serde_yaml` 0.9.
- TypeScript / Extension: `@types/vscode` ^1.90.0, `ws` ^8.17.1, `yaml` ^2.4.5, `esbuild` ^0.21.5.
- Webview UI: Vanilla Canvas 2D with `d3-force` ^3.0.0.

**Storage**:
- Configuration: `.context-vault.yaml` at workspace root (persisting `scan.exclude`, `graph.exclude`, `hub`, `clusters`).
- Secrets: `vscode.SecretStorage` (zero plaintext disk storage).
- State: `vscode.setState()` / `vscode.getState()` in Webview.

**Testing**:
- Rust: `cargo test` (unit, integration, and benchmark tests).
- Extension & Webview: Node.js native test runner (`node --test --experimental-strip-types test/suite/*.test.ts`).

**Target Platform**:
- Multi-platform: Linux (x86_64, aarch64), macOS (x86_64, Apple Silicon aarch64), Windows (x86_64).

**Project Type**:
- VS Code Extension with native Rust sidecar daemon.

**Performance Goals**:
- Glob exclusion preview matching for 5,000 files in $\le 10\text{ ms}$ via Rust `globset`.
- Focus mode switching on Webview canvas in $\le 16\text{ ms}$ (60 FPS frame rate).
- Contextual Inspector opening latency $\le 30\text{ ms}$.

**Constraints**:
- Rust resident memory resting ceiling $\le 35\text{ MB}$.
- Git hook non-blocking fail-open timeout $\le 500\text{ ms}$.
- Strict Content Security Policy (CSP) with cryptographic nonces.

**Scale/Scope**:
- Repositories up to 5,000 Markdown files, 15,000 links, 2.5M BPE tokens.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

1. **Principle I (Dual-Process Separation)**: PASS. All glob matching and link resolution math remain in Rust; Node.js coordinates UI and VS Code APIs.
2. **Principle II (Event-Driven Git Hook Lifecycle)**: PASS. Git hook delta notifications via `prek` remain non-blocking $\le 500\text{ ms}$ fail-open.
3. **Principle III (Organic Discovery, Curated Representation & Exclusions)**: PASS. Curated exclusions (`graph.exclude`) remove visual noise without altering disk files or breaking valid link targets.
4. **Principle IV (Context Hierarchy, Semantic Sizing & Focus Modes)**: PASS. Semantic radius sizing ($14\text{px}, 9\text{px}, 6\text{px}, 4\text{px}$) and hop focus modes (`Hub`, `1 hop`, `2 hops`, `All`) strictly enforce context clarity.
5. **Principle V (Strict Link Integrity)**: PASS. Excluded targets resolve cleanly as `resolved_excluded`; broken links remain flagged with red dashed edges and diagnostic coordinates.
6. **Principle VI (Sandboxed Webview, Zero Action Duplication & Progressive Disclosure)**: PASS. Sidebar duplicate action buttons removed; primary controls constrained to view manipulation; actions emerge from states; Contextual Inspector provides selection-driven utility.
7. **Principle VII (TDD & Verification)**: PASS. All new exclusion resolvers, preview RPCs, and UI modules require unit and integration tests.

---

## Project Structure

### Documentation (this feature)

```text
specs/002-context-observatory-ux/
├── plan.md              # This file
├── research.md          # Phase 0 research & decisions
├── data-model.md        # Phase 1 domain entities & constraints
├── quickstart.md        # Phase 1 runnable validation scenarios
├── contracts/           # Phase 1 JSON-RPC contracts
│   └── observatory_rpc_protocol.json
└── tasks.md             # Phase 2 task breakdown (by speckit-tasks)
```

### Source Code Tree Layout

```text
crates/core/
├── Cargo.toml
├── src/
│   ├── ast/
│   │   └── parser.rs             # Markdown link and frontmatter extractor
│   ├── config/
│   │   ├── mod.rs                # Configuration loader
│   │   └── schema.rs             # Updated schema with scan.exclude & graph.exclude
│   ├── graph/
│   │   ├── mod.rs                # Graph builder and centrality engine
│   │   ├── exclusions.rs         # NEW: Globset matcher & noise pattern suggester
│   │   └── resolver.rs           # Enhanced link resolver with resolved_excluded status
│   ├── ipc/
│   │   ├── handlers.rs           # RPC dispatch (previewExclusion, excludePath, suggestExclusions)
│   │   └── protocol.rs           # Strongly-typed JSON-RPC envelopes
│   └── main.rs

extension/
├── package.json                  # Extension manifest (clean commands & menus)
├── src/
│   ├── extension.ts              # Entry point & command registrations
│   ├── sidecar/
│   │   └── client.ts             # IPC client communicating with Rust sidecar
│   └── webview/
│       ├── panel.ts              # Webview panel coordinator
│       └── rpc.ts                # Typed JSON-RPC bridge
├── webview-ui/
│   ├── src/
│   │   ├── components/
│   │   │   ├── actionsToolbar.ts # Streamlined floating toolbar
│   │   │   ├── inspectorDrawer.ts# NEW: Slide-over contextual inspector
│   │   │   ├── exclusionModal.ts # NEW: Preview & confirmation dialog
│   │   │   ├── statusDashboard.ts# NEW: Calm 3-pillar sidebar dashboard
│   │   │   └── settingsDrawer.ts # NEW: Advanced settings & integrations
│   │   ├── graph/
│   │   │   ├── canvas.ts         # 2D Canvas renderer with semantic sizing
│   │   │   ├── focusEngine.ts    # NEW: Hop-constrained focus subgraphs
│   │   │   └── physics.ts        # D3 force simulation controller
│   │   ├── main.ts               # Webview bootstrap & state synchronization
│   │   └── theme.css             # Styles for dashboard, inspector, and focus controls
│   └── index.html
└── test/
    └── suite/
        ├── exclusions.test.ts    # NEW: Glob matching & target resolvability tests
        ├── focusEngine.test.ts   # NEW: Hop filter & focus tests
        └── inspector.test.ts     # NEW: Inspector state & action tests
```

---

## Complexity Tracking

*No constitutional violations identified. 100% compliant with Constitution v1.3.0.*
