# Phase 0 Research: Technical Decisions & Architectural Investigations

**Feature**: LLM Wiki Extension for VS Code (`llm_wiki_ext4vsc`)  
**Branch**: `001-llm-wiki-ext`  
**Date**: 2026-09-11  
**Status**: Completed & Resolved  

---

## 1. IPC Protocol & Inter-Process Communication

### Decision
We select **Local WebSocket over `127.0.0.1` on an OS-assigned ephemeral port** secured with a **per-session cryptographic authorization token** passed via CLI flag `--auth-token <hex>` and verified upon connection handshake.

### Rationale
- **Cross-Platform Equivalence**: Named pipes on Windows (`\\.\pipe\...`) behave differently from POSIX Unix Domain Sockets (permissions, socket cleanup on crash, blocking semantics). WebSockets provide identical client and server APIs on Linux, macOS, and Windows.
- **Node.js & Rust Native Support**: Rust provides `tokio-tungstenite` (zero-copy, async, high-performance), while Node.js Extension Host has mature, lightweight implementations (`ws`).
- **Low Latency**: Loopback TCP over `127.0.0.1` achieves round-trip latencies $<0.5\text{ ms}$ for JSON-RPC messages, well within the sub-5ms performance requirement.
- **Robust Reconnection**: When the extension host restarts or reloads, WebSockets support transparent exponential backoff reconnection.
- **Multi-Client Multiplexing**: Allows both the VS Code Extension Host and the `prek` Git hook CLI (`llm-wiki-core git-delta`) to connect to the same background sidecar daemon without file-lock contention.

### Alternatives Considered
- **Stdio (Standard I/O Pipes)**: Standard in Language Server Protocol (LSP), but restricts communication to a 1:1 parent-child relationship. External processes like `prek` Git hooks cannot communicate with a stdio-based sidecar without proxying through VS Code.
- **Unix Domain Sockets / Windows Named Pipes**: Excellent for POSIX, but Windows Named Pipes require platform-specific conditional compilation (`tokio::net::windows::named_pipe`) and custom access control lists (ACLs).

---

## 2. Markdown AST Parsing, Wikilink & Hashtag Extraction

### Decision
We use **`pulldown-cmark`** with custom event-stream processing for CommonMark/GFM syntax, combined with custom tokenizers for Obsidian-style wikilinks (`[[target|label]]`) and hashtags (`#tag`).

### Rationale
- **Pull Parser Efficiency**: `pulldown-cmark` is a streaming, zero-copy pull parser written in pure Rust. It parses large Markdown documents in microseconds and avoids building full DOM tree allocations in RAM.
- **Code Block / Code Span Immunity**: `pulldown-cmark` emits `Start(Tag::CodeBlock)` and `Code(CowStr)` events. By maintaining a parser state machine that suppresses link extraction inside code blocks and inline backticks, we strictly fulfill **FR-003** with zero false-positive links.
- **Unified Tag Extraction**: Inline hashtags (`#architecture`, `#security/audit`) and frontmatter tags are captured during the text stream traversal and normalized into lowercase sets.

### Alternatives Considered
- **Regular Expressions (`regex` crate)**: Regex cannot accurately track Markdown contextual nesting, falsely matching link syntax inside fenced code blocks (` ```markdown [[link]] ``` `) or raw HTML comments.
- **`markdown-rs` (comrak)**: Comrak produces a full AST tree in memory, which consumes $3\times$ to $5\times$ more memory than `pulldown-cmark`'s streaming event iterator.

---

## 3. Graph Topology, Centrality Algorithms & BPE Token Economics

### Decision
We use **`petgraph::graph::DiGraph`** for directed topological mapping, **Brandes' Algorithm** for betweenness centrality, Breadth-First Search (BFS) for shortest-path hop distances, and **`tiktoken-rs` (`cl100k_base`)** for exact BPE token counts.

### Rationale
- **`petgraph` In-Memory Graph**: High-performance adjacency list representation. In-degree and out-degree lookups are $O(1)$.
- **Betweenness Centrality**: Brandes' algorithm computes betweenness centrality for all nodes in $O(V \cdot E)$ time and $O(V + E)$ space. For vaults of 1,000 files and ~5,000 edges, calculation finishes in $<30\text{ ms}$. Parallelized across CPU cores using `rayon` for large vaults ($>1,000$ notes).
- **Hop Distance BFS**: Running a BFS starting from the designated central hub (`Level 0`) assigns discrete hop levels ($0, 1, 2, \dots$) in $O(V + E)$ time, naturally handling circular references without infinite loops.
- **`tiktoken-rs`**: Direct Rust binding to OpenAI's BPE rank table. Parses text at $\sim 20\text{ MB/s}$ per core, giving bit-exact token counts identical to modern LLM tokenizers (Claude, GPT-4o, Codex).

### Alternatives Considered
- **Ad-hoc Adjacency HashMaps**: Lacks verified graph traversal algorithms and requires manual cycle detection, DFS/BFS implementations, and memory management.
- **Character/Word Heuristic Token Estimator (e.g. chars / 4)**: Unacceptable for technical documentation containing code snippets, YAML frontmatter, and URLs, where character-to-token ratios deviate by up to 40%.

---

## 4. Git Hook Lifecycle & Rust-Native `prek` Orchestration

### Decision
We adopt **`prek`** as the primary Git hook engine, utilizing declarative `.pre-commit-config.yaml` integration with a 3-tier discovery fallback hierarchy and a strict $500\text{ ms}$ non-blocking fail-open timeout.

### Rationale
- **Rust-to-Rust Homogeneity**: `prek` is a single static binary written in Rust. It requires no Python interpreter, no `pip`, no `venv`, and has negligible startup latency ($<5\text{ ms}$ vs $150\text{--}300\text{ ms}$ for Python `pre-commit`).
- **Parallel Hook Execution**: `prek` executes hooks concurrently across a worker thread pool. The `llm-wiki-sync` hook executes in parallel with code linters (`cargo clippy`, `eslint`, `ruff`), eliminating commit bottlenecks.
- **Pattern Reusability via `.pre-commit-config.yaml`**: Drop-in replacement for standard repositories without needing proprietary configuration DSLs.
- **Zero-Overhead File Filtering**: Pre-filtering via `files: '(?i)\.md$'` ensures non-Markdown commits exit in $<2\text{ ms}$ without spawning `llm-wiki-core`.
- **Fail-Open Policy**: If `llm-wiki-core git-delta` cannot connect to the local WebSocket within $500\text{ ms}$, it exits with code 0, guaranteeing developers are never blocked from committing code.

### Alternatives Considered
- **VS Code `vscode.workspace.onDidChangeTextDocument`**: Triggers continuous parsing on every keystroke, causing editor stuttering, battery drain, and race conditions.
- **Direct Shell Script Only (`.git/hooks/post-commit`)**: Lacks declarative regex filtering, parallel hook concurrency, and team-wide portability across Windows/macOS/Linux. Kept as Tier 3 fallback.

---

## 5. Sandboxed Webview Graph Rendering & Hop Opacity Invariant

### Decision
We implement a **High-Performance 2D HTML5 Canvas Force-Directed Visualizer** using **`d3-force`** physics simulation, wrapped inside a strictly sandboxed VS Code Webview with CSP cryptographic nonces. Custom colors apply to node **base hues**, while the **alpha channel is strictly governed by hop distance**.

### Rationale
- **Rendering Performance**: 2D Canvas easily renders 1,000 to 5,000 nodes at 60 FPS on standard modern laptops without the overhead, context loss risks, and GPU driver discrepancies of WebGL inside an iframe.
- **Hue + Opacity Composition Invariant**:
  $$\text{Color} = \text{rgba}(R_{\text{cluster}}, G_{\text{cluster}}, B_{\text{cluster}}, \alpha_{\text{hop}})$$
  - Level 0 (Active Context / Harness): $\alpha = 1.0$ (100% opacity)
  - Level 1 (Direct Reference): $\alpha = 0.6$ (60% opacity)
  - Level 2 (Transitive Reference): $\alpha = 0.3$ (30% opacity)
  - Level 3+ (Deep Reference): $\alpha = 0.15$ (15% opacity)
  - Orphan (Disconnected): $\alpha = 0.4$ (Muted gray `#6B7280` or subdued cluster color)
  This invariant ensures that user-defined cluster coloring never obscures active versus inactive LLM context boundaries.
- **Security & Memory Compliance (CTaaS GAP-026/GAP-027)**:
  - Nonced CSP: `script-src 'nonce-${nonce}'`
  - Zero Plaintext Disk Secrets: All credentials managed via `vscode.SecretStorage`
  - Memory-safe lifecycle: No `retainContextWhenHidden`; state persisted via `vscode.setState()` and restored via `WebviewPanelSerializer`.

### Alternatives Considered
- **SVG-Based Graph (D3 SVG DOM elements)**: Drops below 15 FPS when rendering $>500$ nodes with multiple links due to excessive DOM node overhead.
- **Three.js / 3D Force Graph**: High memory usage ($>80\text{ MB}$ Webview RAM), slow initialization, and poor usability for 2D documentation navigation.
