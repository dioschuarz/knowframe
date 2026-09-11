# Implementation Plan: LLM Wiki Extension for VS Code (`llm_wiki_ext4vsc`)

**Branch**: `001-llm-wiki-ext` | **Date**: 2026-09-11 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `/specs/001-llm-wiki-ext/spec.md` and CTaaS Developer Tooling Knowledge Base (`notes/06_studies/vscode_extensions/`, `GAP-026` through `GAP-029`).

---

## Summary

`llm_wiki_ext4vsc` delivers an Obsidian-style dynamic knowledge graph and context budgeting system inside Visual Studio Code. The product is structured into two decoupled tiers: a high-performance Rust core engine sidecar (`llm-wiki-core`) handling Markdown AST parsing, `petgraph` centrality calculations, and `tiktoken-rs` BPE token counting, paired with a TypeScript Extension Host managing process supervision, sandboxed Webview canvas rendering, and Git hook orchestration via `prek`. The architecture enforces a strict Hue + Opacity composition invariant (cluster hashtags/paths define base hue, hop distance governs alpha channel), multi-threaded hook synchronization with declarative `.pre-commit-config.yaml` reuse, and $\le 500\text{ ms}$ non-blocking fail-open execution.

---

## Technical Context

**Language/Version**: 
- Rust 2021/2024 edition (`rustc ^1.75.0`, active workspace: `1.98.0`)
- TypeScript 5.4+ / Node.js 20+ / VS Code Extension API `^1.90.0`

**Primary Dependencies**:
- *Rust Core*: `pulldown-cmark` (AST link & tag extractor), `petgraph` (directed graph & Brandes centrality), `tiktoken-rs` (`cl100k_base` BPE tokenizer), `tokio` / `tokio-tungstenite` (async WebSocket IPC), `serde` / `serde_json` / `serde_yaml` (data contracts), `clap` (CLI argument parser), `glob` (path matching).
- *Extension Host & Webview*: `ws` (WebSocket client), `d3-force` (force-directed canvas graph physics), `@vscode/webview-ui-toolkit` (VS Code styled web components), `@types/vscode`.

**Storage**:
- Workspace config: `.context-vault.yaml` at workspace root.
- State persistence: VS Code Memento SQLite backend (`ExtensionContext.globalState` / `workspaceState`) (CTaaS GAP-026).
- Secret handling: `vscode.SecretStorage` backed by OS Keychain (CTaaS GAP-026).

**Testing**:
- Rust: `cargo test` (unit, integration, Criterion benchmarks).
- TypeScript: `@vscode/test-electron` (headless extension host integration tests), `mocha`, `esbuild` / `tsc`.

**Target Platform**:
- Linux (x86_64, aarch64), macOS (x86_64, aarch64 Apple Silicon), Windows (x86_64).

**Project Type**:
- Hybrid VS Code Extension with standalone Rust binary sidecar (`crates/core` + `extension`).

**Performance Goals**:
- Full vault parse and graph calculation for 1,000 Markdown files in $\le 200\text{ ms}$.
- Resident memory footprint (RSS) of Rust sidecar $\le 35\text{ MB}$ resting state for 5,000 notes.
- Webview force-directed canvas rendering at consistent 60 FPS.
- Incremental Rust core in-memory graph patch in $\le 50\text{ ms}$; end-to-end Git hook roundtrip $< 150\text{ ms}$.

**Constraints**:
- Strict non-blocking Git hook fail-open $\le 500\text{ ms}$ timeout.
- Packaged `.vsix` size $\le 25\text{ MB}$ per platform target (CTaaS GAP-028).
- Sandboxed Webview with CSP cryptographic nonces and zero plaintext credentials on disk; daemon tokens managed exclusively via `vscode.SecretStorage` (CTaaS GAP-026/GAP-027, Constitution Principle VI).
- No continuous disk watchers or background keystroke polling.

**Scale/Scope**:
- Repositories and Obsidian vaults up to 5,000 Markdown files, multi-repo and submodule traversal.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Gate Status | Architecture Compliance Summary |
| :--- | :--- | :--- |
| **I. Dual-Process Separation** | **PASS** | Heavy AST parsing, petgraph centrality, tiktoken BPE, and Git diff parsing reside strictly in Rust (`crates/core`). Node.js Extension Host serves strictly as UI orchestrator and lifecycle supervisor. |
| **II. Event-Driven Hook via `prek`** | **PASS** | Adopts `prek` (Rust-native, multi-threaded, `.pre-commit-config.yaml` reuse, regex file filter `files: '(?i)\.md$'`). 3-tier fallback hierarchy. Enforces $\le 500\text{ ms}$ fail-open timeout. Zero keystroke polling. |
| **III. Organic Discovery** | **PASS** | Organic link discovery via `pulldown-cmark`. Computes in/out-degree and betweenness centrality using `petgraph` without requiring frontmatter or canonical entrypoints. Proposes top centrality node as default Hub. |
| **IV. Multi-Tier Context & Color Groups** | **PASS** | Hop distance opacity decay law (Level 0 = 100%, Level 1 = 60%, Level 2 = 30%, Level 3+ = 15%). User-defined cluster groups (`tag:#...`, `path:...`) map to base hues while hop distance strictly dictates alpha channel. Exact `cl100k_base` BPE budgeting with 60k/100k warning thresholds. |
| **V. Link Integrity & Submodule Boundaries** | **PASS** | Enforces 100% internal validation for both Markdown links and wikilinks. Cross-submodule relative links resolve transparently. Dead links flagged with dashed red edges. |
| **VI. Sandboxed Webview & Zero-Plaintext** | **PASS** | Complies with CTaaS GAP-026/GAP-027: CSP cryptographic nonces (`script-src 'nonce-${nonce}'`), `asWebviewUri` asset mapping, typed JSON-RPC with `requestId`, state persistence via `vscode.setState()`, zero plaintext secrets via `vscode.SecretStorage`. |
| **VII. TDD & Multi-Platform Verification** | **PASS** | Red-green-refactor cycle across Rust unit tests and TypeScript e2e tests. Multi-target packaging (`vsce package --target <platform>`). |

---

## Project Structure

### Documentation (`specs/001-llm-wiki-ext/`)

```text
specs/001-llm-wiki-ext/
├── plan.md              # Implementation plan (this document)
├── research.md          # Phase 0 architectural decisions & technical context
├── data-model.md        # Phase 1 domain entities, schemas & state transitions
├── quickstart.md        # Phase 1 runnable validation scenarios & acceptance tests
└── contracts/           # Phase 1 formal interface specifications
    ├── ipc_jsonrpc_protocol.json     # JSON-RPC 2.0 schema for Rust sidecar daemon
    ├── webview_rpc_protocol.json     # Typed RPC protocol between Extension Host & Webview
    ├── context_vault_schema.json     # JSON Schema for .context-vault.yaml
    └── git_delta_contract.json       # Git hook commit delta payload schema
```

### Source Code Layout

```text
/home/ds/projects/llm_wiki_ext4vsc/
├── Cargo.toml                        # Cargo workspace definition
├── crates/
│   └── core/                         # Rust Core Sidecar (llm-wiki-core)
│       ├── Cargo.toml
│       ├── src/
│       │   ├── main.rs               # CLI entrypoint (scan, daemon, git-delta)
│       │   ├── lib.rs                # Library root for unit testing
│       │   ├── types.rs              # Core domain types & enums (NodeStatus, LinkType)
│       │   ├── error.rs              # Domain error handling & result types
│       │   ├── models/               # Domain entity models
│       │   │   └── mod.rs            # NoteNode, NoteEdge structs & serialization
│       │   ├── cli/                  # Standalone CLI sub-commands
│       │   │   ├── mod.rs
│       │   │   ├── scan.rs           # Workspace scanner CLI handler
│       │   │   └── delta.rs          # Git commit delta CLI dispatcher
│       │   ├── ast/                  # Markdown AST & pulldown-cmark pull parser
│       │   │   ├── mod.rs
│       │   │   ├── parser.rs         # Link and text event extractor
│       │   │   ├── wikilink.rs       # [[target|label]] tokenizer
│       │   │   └── tags.rs           # Inline #hashtag and frontmatter extractor
│       │   ├── graph/                # Petgraph topology & centrality
│       │   │   ├── mod.rs
│       │   │   ├── builder.rs        # Graph construction from parsed notes
│       │   │   ├── centrality.rs     # In-degree, out-degree, Brandes betweenness
│       │   │   ├── hops.rs           # BFS shortest-path hop resolver
│       │   │   └── resolver.rs       # Path, section anchor & submodule link resolution
│       │   ├── tokens/               # BPE token economics
│       │   │   ├── mod.rs
│       │   │   └── counter.rs        # tiktoken-rs cl100k_base accounting
│       │   ├── config/               # .context-vault.yaml loader & generator
│       │   │   ├── mod.rs
│       │   │   ├── schema.rs         # Serde structures & color groups
│       │   │   └── generator.rs      # Default auto-discovery writer
│       │   ├── ipc/                  # WebSocket JSON-RPC 2.0 server
│       │   │   ├── mod.rs
│       │   │   ├── server.rs         # tokio-tungstenite daemon
│       │   │   ├── handlers.rs       # workspace/scan & graph/setHub request handlers
│       │   │   ├── protocol.rs       # Typed request/response/delta handlers
│       │   │   └── client.rs         # Ephemeral client for git-delta sub-command
│       │   └── delta/                # Incremental commit processor
│       │       ├── mod.rs
│       │       └── updater.rs        # In-memory graph patching (<=50 ms)
│       └── tests/
│           ├── ast_test.rs           # Unit tests for code-block link immunity
│           ├── graph_test.rs         # Petgraph centrality & circular link BFS
│           ├── token_test.rs         # BPE exact token counting tests
│           └── delta_test.rs         # Incremental update tests
│
├── extension/                        # VS Code Extension Host & Webview UI
│   ├── package.json                  # Extension manifest & command contributions
│   ├── tsconfig.json
│   ├── esbuild.js                    # Fast bundle pipeline
│   ├── src/
│   │   ├── extension.ts              # Extension activation & deactivation lifecycle
│   │   ├── types/
│   │   │   └── protocol.ts           # Shared TypeScript contracts & RPC interfaces
│   │   ├── lifecycle/
│   │   │   ├── binaryResolver.ts     # Multi-platform Rust binary discovery
│   │   │   ├── processManager.ts     # Rust binary spawner, supervisor & auto-restart
│   │   │   ├── secretManager.ts      # vscode.SecretStorage IPC session token manager
│   │   │   └── healthMonitor.ts      # Ping/pong socket health check & backoff
│   │   ├── ipc/
│   │   │   ├── client.ts             # WebSocket JSON-RPC 2.0 client
│   │   │   └── messageRouter.ts      # Request/response correlation via requestId
│   │   ├── webview/
│   │   │   ├── panelManager.ts       # WebviewPanel lifecycle & serializer (GAP-027)
│   │   │   └── cspHelper.ts          # Cryptographic nonce & asWebviewUri generator
│   │   ├── hooks/
│   │   │   ├── prekInstaller.ts      # .pre-commit-config.yaml writer & prek CLI
│   │   │   └── fallbackInstaller.ts  # Tier 2 (pre-commit) & Tier 3 shell script
│   │   └── commands/
│   │       ├── showGraph.ts          # Open knowledge graph command
│   │       ├── setHub.ts             # Designate active hub command
│   │       ├── installHooks.ts       # Setup git hooks command
│   │       ├── rescanVault.ts        # Manual refresh command
│   │       └── fixBrokenLinks.ts     # QuickPick broken link navigation
│   ├── webview-ui/                   # Sandboxed Webview frontend
│   │   ├── index.html                # CSP-compliant template
│   │   ├── src/
│   │   │   ├── main.ts               # Webview entrypoint & vscode.getState()
│   │   │   ├── graph/
│   │   │   │   ├── canvasRenderer.ts # 60 FPS HTML5 Canvas force-directed renderer
│   │   │   │   ├── physics.ts        # d3-force simulation manager
│   │   │   │   └── colorResolver.ts  # Hue + Opacity composition invariant
│   │   │   ├── components/
│   │   │   │   ├── tokenCounter.ts   # Active harness vs on-demand token display
│   │   │   │   ├── colorManager.ts   # Interactive cluster hashtag & path manager
│   │   │   │   └── actionsToolbar.ts # Quick action buttons (Hub, Rescan, Fix Links)
│   │   │   └── rpc/
│   │   │       └── webviewRpc.ts     # Typed JSON-RPC bridge to Extension Host
│   └── test/
│       ├── runTest.ts                # Headless Electron test launcher
│       └── suite/
│           ├── extension.test.ts     # Activation and command test
│           ├── webview.test.ts       # Webview RPC and CSP test
│           ├── hooks.test.ts         # Hook installation test
│           └── ipc.test.ts           # Sidecar handshake and ping/pong test
```

**Structure Decision**: Monorepo configuration separating Rust systems code (`crates/core`) from the VS Code TypeScript host and Webview bundle (`extension/`). This guarantees independent testing, platform-specific compilation, and clean artifact boundaries adhering to CTaaS GAP-028.

---

## Complexity Tracking

> **Constitutional Compliance**: All 7 Constitutional Principles pass without violations. No complexity concessions or non-compliant shortcuts required.

| Aspect | Justification | Alternatives Evaluated & Rejected |
| :--- | :--- | :--- |
| **WebSocket IPC vs Stdio LSP** | Allows `prek` Git hook CLI sub-commands to dispatch commit deltas directly to the sidecar daemon independently of VS Code UI focus. | LSP Stdio rejected because external Git hooks cannot access the pipe without invasive editor IPC proxies. |
| **Canvas 2D vs WebGL** | 2D Canvas delivers rock-solid 60 FPS for 1,000–5,000 nodes without WebGL context loss issues when VS Code hides/restores Webview tabs. | Three.js / WebGL rejected due to high Webview RAM footprint ($>80\text{ MB}$) and driver flakiness in Linux containers. |
| **`prek` Git Hook vs Disk Watcher** | Eliminates continuous CPU polling, battery drain, and typing stutter in the editor by synchronizing strictly upon explicit Git commit events. | Keystroke / file watcher rejected as prohibited by Constitution Principle II. |

---

## Next Steps (Transition to Tasks)

With the completion of Phase 0 research, Phase 1 data modeling, interface contracts, and this implementation plan, the project is ready for task breakdown (`/speckit-tasks`):
1. **Cargo Workspace & Rust Core (`crates/core`)**: AST parser, wikilink tokenizer, hashtag extractor, petgraph centrality, tiktoken BPE, and CLI interface.
2. **Sidecar Daemon & IPC Server**: `tokio-tungstenite` JSON-RPC server and `git-delta` client.
3. **Extension Host Infrastructure (`extension/`)**: Sidecar process manager, WebSocket client, and `.context-vault.yaml` auto-generation.
4. **`prek` Git Hook Orchestration**: `.pre-commit-config.yaml` generator, 3-tier fallback, and fail-open timeout.
5. **Sandboxed Webview & UI**: 2D Canvas renderer, hop opacity decay, cluster hashtag/path color resolver, and token economics dashboard.
