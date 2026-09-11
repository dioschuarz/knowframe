# Feature Specification: LLM Wiki Extension for VS Code (`llm_wiki_ext4vsc`)

**Feature Branch**: `001-llm-wiki-ext`  
**Created**: 2026-09-11  
**Status**: Approved  
**Input**: User PRD for `llm_wiki_ext4vsc` (Dynamic Obsidian-Style Knowledge Graph & Context Budgeting for VS Code with Rust Core Sidecar)

---

## Executive Summary & Alignment with CTaaS Knowledge Base

`llm_wiki_ext4vsc` is a high-performance Visual Studio Code extension paired with an ultra-lean Rust sidecar engine (`llm-wiki-core`). It solves AI context window saturation (50k–100k+ tokens injected blindly into agents like Claude Code, Codex, Antigravity, and Roo/Cline) and eliminates documentation sprawl by mapping workspace Markdown files into a dynamic knowledge graph.

The system incorporates production-grade architectural patterns established in the CTaaS Developer Tooling knowledge base:
- **CTaaS GAP-026**: Persistent state management via `ExtensionContext.globalState` / `workspaceState` and secure credential handling via `vscode.SecretStorage`.
- **CTaaS GAP-027**: Sandboxed Chromium Webview architecture with strict Content Security Policy (CSP), cryptographic nonces (`script-src 'nonce-${nonce}'`), typed JSON-RPC 2.0 message passing, and memory-safe tab lifecycle management without `retainContextWhenHidden`.
- **CTaaS GAP-028**: Open Packaging Conventions (`.vsix`) packaging, multi-target platform distribution, and automated CI/CD via GitHub Actions.
- **CTaaS GAP-029**: Decoupled background process lifecycle management, asynchronous JSON-RPC protocol over local WebSocket/IPC, and real-time diagnostic reporting.

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Organic Vault Discovery & Graph Centrality Computation (Priority: P1)

As an AI-assisted developer opening a disorganized project containing dozens or hundreds of Markdown files,  
I want the extension to automatically discover all documentation files, parse both standard links and wikilinks, calculate mathematical centrality metrics, and detect broken links,  
So that I immediately obtain an accurate topological map and entrypoint without having to manually curate frontmatter, index files, or directory trees.

**Why this priority**: Fundamental core capability. Without organic AST link extraction and graph calculation, no visualizer, token budget, or navigation can function.

**Independent Test**: Can be executed completely headless via the Rust binary CLI (`llm-wiki-core --scan <workspace_dir>`) or via Rust integration unit tests, outputting a complete topological JSON payload with exact degree/betweenness centrality, broken link counts, and BPE token counts.

**Acceptance Scenarios**:
1. **Given** a workspace with 500 Markdown files containing arbitrary links `[text](rel/path.md)` and `[[wikilink]]`,  
   **When** the Rust core scans the directory,  
   **Then** it constructs a directed graph in `petgraph`, calculates in-degree, out-degree, and betweenness centrality, identifies the top-ranked node as the recommended central hub, and completes in under 150 ms.
2. **Given** Markdown files referencing non-existent paths or mistyped wikilinks,  
   **When** the parser processes the files,  
   **Then** it flags all missing targets as dead links (`is_broken: true`) with corresponding source file locations.
3. **Given** Markdown files with code blocks containing link-like text (e.g. inside triple backticks or inline backticks),  
   **When** the AST parser (`pulldown-cmark`) runs,  
   **Then** it ignores code spans and code blocks, extracting only true structural document links.

---

### User Story 2 - Sidecar Process Lifecycle & Typed WebSocket IPC (Priority: P1)

As a VS Code user,  
I want the extension to manage the Rust background process automatically (spawning on extension activation, keeping resident memory low, reconnecting if interrupted, and terminating cleanly on window close),  
So that I experience zero configuration friction, zero zombie background processes, and instant graph query responses.

**Why this priority**: Necessary runtime bridge connecting the Node.js Extension Host to the Rust compute engine.

**Independent Test**: Run extension in a VS Code Development Host (`@vscode/test-electron`), trigger extension activation, verify sidecar process spawns, completes handshake over WebSocket/IPC, exchanges ping/pong messages within 5 ms, and cleanly shuts down when VS Code terminates.

**Acceptance Scenarios**:
1. **Given** VS Code opens a workspace containing Markdown files,  
   **When** the extension activates,  
   **Then** it selects the platform-specific pre-compiled Rust binary (`llm-wiki-core`), launches it with an ephemeral port / IPC socket, and establishes a bidirectional WebSocket connection within 100 ms.
2. **Given** the workspace does not yet have a `.context-vault.yaml`,  
   **When** the sidecar initializes,  
   **Then** it auto-generates `.context-vault.yaml` with discovered defaults (suggested hub, standard include/exclude globs, and token warning thresholds) without overwriting existing user settings.
3. **Given** an unexpected termination of the sidecar process,  
   **When** the Extension Host detects process exit,  
   **Then** it restarts the sidecar up to 3 times with exponential backoff and informs the user with a discreet status bar notification.

---

### User Story 3 - Interactive Force-Directed Webview with Hop Opacity, Cluster Hashtag / Path Coloring & Token Economics (Priority: P2)

As a prompt architect or developer organizing workspace knowledge,  
I want to define custom node colors based on cluster hashtags inside Markdown notes (e.g. `#architecture`, `#security`) or filesystem paths (e.g. `docs/tests/**`), while preserving hop-distance opacity decay for LLM context tiers,  
So that I can visually group semantic domains at a glance without losing visibility into active prompt load versus secondary reference depth.

**Why this priority**: Core user-facing deliverable and primary visual interface of the product, combining domain clustering with context window economics.

**Independent Test**: Configure color groups in `.context-vault.yaml` (or via Webview color manager), verify Webview panel renders a 60 FPS force-directed canvas/WebGL graph where nodes matching `#security` take base color `#EF4444` and nodes matching `docs/tests/**` take `#10B981`, with Level 0 rendered at 100% opacity, Level 1 at 60%, Level 2 at 30%, and orphan notes in muted gray on the periphery.

**Acceptance Scenarios**:
1. **Given** an active central hub (e.g. `AGENTS.md` or `CLAUDE.md`),  
   **When** the graph renders in the Webview,  
   **Then** Level 0 is drawn with 100% solid opacity and enlarged radius, Level 1 nodes are rendered with 60% opacity, Level 2 nodes with 30% opacity, and Level 3+ nodes with 15% opacity.
2. **Given** notes tagged with `#architecture` and a color rule `tag:#architecture -> #8B5CF6`,  
   **When** the graph renders,  
   **Then** those nodes inherit the purple base hue (`#8B5CF6`), modulated by their specific hop distance opacity (e.g. 60% if at Level 1, 30% if at Level 2).
3. **Given** notes residing in `docs/tests/**` and a path rule `path:docs/tests/** -> #10B981`,  
   **When** the graph renders,  
   **Then** those nodes inherit the emerald base hue (`#10B981`), modulated by their specific hop distance opacity.
4. **Given** notes that have no incoming or outgoing connections and no overriding color rule,  
   **When** the graph renders,  
   **Then** they are styled in muted gray (`#6B7280`) and positioned on the graph periphery as orphan nodes.
5. **Given** the diagnostic sidebar in the Webview,  
   **When** viewing the active context metrics and color groups panel,  
   **Then** it displays:
     - **Active Harness Tokens**: Exact sum of Level 0 files.
     - **Subcall / On-Demand Tokens**: Cumulative token budget of Level 1 and Level 2 reachable files.
     - **Token Threshold Status**: Normal (<60k), Warning (60k–100k, yellow), or Critical (>100k, red).
     - **Cluster Groups Manager**: Interactive list allowing users to add, edit, or toggle hashtag and path color rules with immediate live preview and persistence to `.context-vault.yaml`.
6. **Given** a user clicks on any graph node,  
   **When** the node is clicked,  
   **Then** the Webview sends an RPC message to the Extension Host, which opens the target file in the active editor tab (`vscode.window.showTextDocument`).

---

### User Story 4 - Event-Driven Git Hook Synchronization with Rust-Native `prek` Orchestration (Priority: P2)

As a developer committing documentation changes via Git,  
I want the knowledge graph to synchronize automatically on Git commits using a modern, Rust-native hook orchestrator (`prek`), with regex file filtering so that non-documentation commits incur zero overhead,  
So that my editor stays snappy and my graph stays current without blocking my commits or requiring continuous disk file watchers.

**Why this priority**: Replaces heavy filesystem watchers with high-performance, event-driven Git synchronization. Leverages `prek` for three primary architectural advantages:
- **Rust Homogeneity**: Pairs a Rust-based hook orchestrator with the Rust core engine (`llm-wiki-core`), eliminating Python runtime dependencies and interpreter cold-start delays.
- **Parallel Execution**: Uses `prek`'s multi-threaded worker pool to execute `llm-wiki-sync` in parallel with other codebase linters, formatters, and security checks without blocking the commit queue.
- **Declarative Pattern Reusability**: Reuses the industry-standard `.pre-commit-config.yaml` file format, ensuring drop-in compatibility without proprietary DSLs or team retraining.

**Independent Test**: Configure `.pre-commit-config.yaml` with `prek`, commit a `.rs` file (verify `prek` skips hook instantly); commit a `.md` file (verify `prek` invokes `llm-wiki-core git-delta`, sidecar updates graph in <50 ms); kill the sidecar and commit (verify hook exits with code 0 in <500 ms without blocking commit).

**Acceptance Scenarios**:
1. **Given** an initialized workspace with Git,  
   **When** the user runs `LLM Wiki: Install Git Hooks`,  
   **Then** the extension detects `prek` (or falls back to `pre-commit`, `lefthook`, or standalone script), writes the declarative `.pre-commit-config.yaml` configuration with `files: '(?i)\.md$'`, and runs `prek install --hook-type post-commit`.
2. **Given** a commit that modifies only source code files (e.g. `.ts`, `.rs`, `.py`),  
   **When** `git commit` executes,  
   **Then** `prek` evaluates the `files: '(?i)\.md$'` regex, determines no documentation was modified, and skips hook execution in under 2 ms without launching the sidecar dispatcher.
3. **Given** a commit that modifies Markdown documentation files,  
   **When** `git commit` executes,  
   **Then** `prek` invokes `llm-wiki-core git-delta <changed_files>`, which dispatches the delta JSON payload (`{"event": "git_commit", "changed_files": [...]}`) to the running sidecar, updating graph edges and BPE tokens in under 50 ms.
4. **Given** the VS Code extension or Rust sidecar daemon is offline during a terminal commit,  
   **When** the hook dispatcher attempts to reach the socket,  
   **Then** it enforces a strict 500 ms non-blocking timeout and exits with code 0 (silent fail-open), allowing the developer's Git commit to proceed without interruption.
5. **Given** a development environment without `prek` or Python installed,  
   **When** the user installs hooks,  
   **Then** the system transparently falls back to a standalone zero-dependency `.git/hooks/post-commit` shell wrapper that executes `llm-wiki-core git-delta` directly.

---

### User Story 5 - Multi-Repo, Unified Vault & Git Submodule Resolution (Priority: P3)

As an engineer working in an enterprise multi-repo workspace or an Obsidian vault containing Git submodules,  
I want relative markdown links and wikilinks to resolve across submodule and repository boundaries,  
So that cross-project architecture references do not generate false-positive dead links.

**Why this priority**: Ensures high utility in enterprise and complex monorepo/polyrepo workspaces.

**Independent Test**: Configure `.context-vault.yaml` with `mode: multi-repo` or `mode: vault`, run graph scan, verify cross-submodule links `../submodule-a/docs/spec.md` resolve accurately to target nodes.

**Acceptance Scenarios**:
1. **Given** a parent vault containing two git submodules `services/auth` and `services/billing`,  
   **When** `services/auth/README.md` links to `../billing/README.md`,  
   **Then** the resolver resolves the path relative to the vault root and connects the edge without error.

---

## Edge Cases & Failure Modes

1. **Circular Link Dependencies**: If Note A links to Note B and Note B links to Note A, the hop distance resolver must use Breadth-First Search (BFS) with visited sets to prevent infinite loops; both nodes retain their minimum shortest-path hop distance from Level 0.
2. **Self-Referencing Links**: Nodes referencing themselves (`[self](file.md)`) must not increment degree centrality artificially or distort layout physics.
3. **Empty or Malformed Markdown**: Files with zero bytes or broken formatting must be registered as valid nodes with 0 tokens and 0 edges, without panicking the AST parser.
4. **OS Case-Sensitivity Discrepancies**: On Linux (case-sensitive) vs macOS/Windows (case-insensitive), wikilinks `[[my_note]]` matching `My_Note.md` must be normalized gracefully using case-folded dictionary index lookups. If multiple case-insensitive files match on a case-sensitive OS without an exact match, the resolver selects the first file sorted lexicographically and emits a diagnostic warning.
5. **Dangling Anchors in Existing Files**: Links pointing to `file.md#non-existent-header` must verify file existence and validate section anchors against target document headings (`# Heading`), immediately flagging missing section targets as broken links (`is_broken: true`) with red dashed styling, in strict accordance with Constitution Principle V.
6. **Webview Tab Hidden & Restored**: When the Webview panel is hidden, VS Code destroys its iframe DOM. The Webview script must invoke `vscode.setState()` before disposal and recover via `vscode.getState()` upon reactivation without triggering redundant Rust re-scans.
7. **Detached HEAD, Rebase, or Merge Conflict States**: If `.git/MERGE_HEAD`, `.git/rebase-merge`, or `.git/rebase-apply` exists, or during merge conflicts, the Git hook must immediately exit with code 0 without attempting delta updates.

---

## Requirements *(mandatory)*

### Functional Requirements

#### 1. Core Parsing & AST Link Extraction
- **FR-001**: System MUST parse standard CommonMark and GFM links `[label](target.md)` using `pulldown-cmark` AST events.
- **FR-002**: System MUST parse Obsidian-style wikilinks `[[target_file]]` and aliased wikilinks `[[target_file|Display Label]]`.
- **FR-003**: System MUST ignore markdown link syntax occurring inside inline code spans (` `...` `) and fenced code blocks (` ```...``` `).
- **FR-004**: System MUST resolve relative file paths relative to the directory of the source markdown file.
- **FR-005**: System MUST detect dead links pointing to non-existent files or directories and flag them with `is_broken: true`.
- **FR-006**: System MUST extract document headers (`# Title`) to derive default human-readable labels when YAML frontmatter is absent.
- **FR-007**: System MUST extract inline hashtags (`#tag`) and YAML frontmatter tags (`tags: [...]`) from Markdown AST, normalizing them into a tags collection on the node entity.

#### 2. Graph Topology & Centrality Resolver
- **FR-008**: System MUST construct a directed graph in memory using `petgraph::Directed`.
- **FR-009**: System MUST compute in-degree and out-degree metrics for every node.
- **FR-010**: System MUST calculate betweenness centrality to identify critical documentation bottlenecks and hubs.
- **FR-011**: System MUST automatically suggest the highest centrality node as the default Level 0 Hub if none is configured.
- **FR-012**: System MUST compute shortest-path hop distance (Level 0, 1, 2, ...) from the designated Hub using BFS.
- **FR-013**: System MUST identify disconnected nodes (in-degree = 0, out-degree = 0) and categorize them as `orphan` nodes.

#### 3. Token Economics Engine
- **FR-014**: System MUST compute exact BPE token counts for every Markdown note using `tiktoken-rs` with the `cl100k_base` encoding (compatible with modern GPT-4o/Claude/Gemini/Codex models).
- **FR-015**: System MUST calculate cumulative branch token costs (node tokens + downstream transitive reachable tokens).
- **FR-016**: System MUST compute total Active Prompt Load (sum of Level 0 files) versus Accessible Subcall Context (sum of Level 1+ files).
- **FR-017**: System MUST flag warnings when Active Prompt Load exceeds `token_budget.warning_threshold` (default: 60,000) or `critical_threshold` (default: 100,000).

#### 4. Sidecar Architecture & IPC Protocol
- **FR-018**: Rust core MUST compile as a standalone binary (`llm-wiki-core`) supporting CLI invocation and daemon/IPC mode.
- **FR-019**: Sidecar communication MUST operate over local WebSocket (`ws://127.0.0.1:<ephemeral_port>`) or local domain sockets. The active daemon ephemeral port MUST be recorded in `.git/llm-wiki.port` (with fallback to `8989`) upon binding, and loopback write-only `git/commitDelta` notifications on `127.0.0.1` MUST be accepted without exposing read state.
- **FR-020**: All IPC messages MUST conform to typed JSON-RPC 2.0 contracts with matching `requestId` fields.
- **FR-021**: Extension Host MUST monitor sidecar process health, auto-restarting on unexpected termination with exponential backoff.
- **FR-022**: Extension Host MUST cleanly terminate the sidecar process during VS Code `deactivate()`.
- **FR-045**: Extension Host MUST store IPC daemon authentication credentials exclusively via `vscode.SecretStorage` and pass them securely to the sidecar daemon in-memory or via environment variables (`LLM_WIKI_AUTH_TOKEN`) without writing plaintext tokens to disk (CTaaS GAP-026, Constitution Principle VI).

#### 5. Configuration Contract (`.context-vault.yaml`)
- **FR-023**: System MUST automatically generate `.context-vault.yaml` at workspace root upon initial scan if not present.
- **FR-024**: System MUST read and respect include/exclude glob patterns (e.g. excluding `.git/**`, `node_modules/**`, `target/**`).
- **FR-025**: System MUST allow users to specify custom designated hubs under `vault.hubs` in `.context-vault.yaml`.
- **FR-026**: System MUST support `mode: single-repo`, `mode: multi-repo`, and `mode: vault`.

#### 6. Git Hook Integration & Delta Processing with `prek`
- **FR-027**: System MUST adopt `prek` (Rust-native git hook manager) as the primary Git hook engine, generating or updating `.pre-commit-config.yaml` with the `llm-wiki-sync` hook definition.
- **FR-028**: System MUST configure declarative regex file filtering (`files: '(?i)\.md$'`) in `.pre-commit-config.yaml` so non-documentation commits incur zero execution overhead.
- **FR-029**: System MUST support automated installation via command `llmWiki.installGitHooks`, invoking `prek install --hook-type post-commit` (and optional `--hook-type pre-commit`).
- **FR-030**: System MUST support a 3-tier fallback hierarchy if `prek` is unavailable: Tier 1 (`prek`), Tier 2 (Python `pre-commit` / `lefthook`), Tier 3 (standalone `.git/hooks/post-commit` shell script wrapper).
- **FR-031**: System MUST provide CLI sub-command `llm-wiki-core git-delta [files...]` capable of receiving changed paths from `prek` and dispatching JSON-RPC notifications to the daemon socket.
- **FR-032**: Rust core MUST process delta updates incrementally, re-parsing only altered Markdown files and updating graph edges and BPE tokens in memory within 50 ms.
- **FR-033**: The Git hook dispatcher MUST enforce a strict non-blocking timeout of $\le 500\text{ ms}$ and fail open (exit code 0) if the socket or daemon is unreachable.

#### 7. Webview Graph UI, Visual Hierarchy & Cluster Color Groups
- **FR-034**: Webview MUST render a force-directed interactive graph running on high-performance 2D HTML5 Canvas (`d3-force`).
- **FR-035**: Level 0 active nodes MUST be rendered with 100% solid opacity and expanded radius based on centrality.
- **FR-036**: Level 1 nodes MUST be rendered at 60% opacity; Level 2 at 30% opacity; Level 3+ at 15% opacity.
- **FR-037**: Dead links MUST be rendered as dashed red edges (`#EF4444`) with warning icons.
- **FR-038**: Orphan nodes MUST be rendered in muted gray (`#6B7280`) on the graph perimeter unless custom cluster rules override.
- **FR-039**: System MUST support backend configuration schema and parsing of user-defined chromatic cluster groups in `.context-vault.yaml` based on hashtag queries (`tag:#name`) and path globs (`path:folder/**`).
- **FR-040**: Webview renderer MUST apply cluster colors to node base hues while strictly preserving the hop-distance opacity decay law defined in FR-035 and FR-036.
- **FR-041**: Webview UI MUST provide an interactive Cluster Groups panel allowing users to define, edit, and toggle hashtag and path color rules with immediate live rendering and persistence to `.context-vault.yaml`.
- **FR-042**: Diagnostic sidebar MUST provide quick action toolbar buttons: "Set as Central Hub", "Fix Broken Links" (which queries all edges where `is_broken: true` and opens a VS Code QuickPick displaying broken links with file and line coordinates for immediate jump-to-editor navigation), and "Rescan Vault".
- **FR-043**: Webview MUST implement strict CSP with cryptographic nonces and asset loading via `webview.asWebviewUri` (CTaaS GAP-027).
- **FR-044**: Webview state MUST be persisted via `vscode.setState()` and restored via `WebviewPanelSerializer`.

---

## Key Entities & Data Contracts

### 1. Workspace Configuration Schema (`.context-vault.yaml`)
```yaml
version: "1.0"
vault:
  name: "Workspace Context"
  mode: "single-repo" # single-repo | multi-repo | vault
  auto_discovered_hub: "AGENTS.md"
  hubs:
    - "AGENTS.md"
    - "CLAUDE.md"
include:
  - "**/*.md"
exclude:
  - "node_modules/**"
  - ".git/**"
  - "target/**"
  - "dist/**"
token_budget:
  warning_threshold: 60000
  critical_threshold: 100000
color_groups:
  - query: "tag:#architecture"
    color: "#8B5CF6"
    label: "Architecture"
  - query: "tag:#security"
    color: "#EF4444"
    label: "Security & Audits"
  - query: "path:docs/tests/**"
    color: "#10B981"
    label: "Testing Standards"
  - query: "path:notes/05_secrets/**"
    color: "#F59E0B"
    label: "Secrets & Keys"
```

### 2. Declarative Git Hook Configuration Schema (`.pre-commit-config.yaml` for `prek`)
```yaml
repos:
  - repo: local
    hooks:
      - id: llm-wiki-sync
        name: LLM Wiki Knowledge Graph Sync
        entry: llm-wiki-core git-delta
        language: system
        stages: [post-commit]
        files: '(?i)\.md$'
        pass_filenames: true
        always_run: false
```

### 3. Git Hook Delta Payload (JSON over IPC)
```json
{
  "jsonrpc": "2.0",
  "method": "git/commitDelta",
  "params": {
    "repo_path": "/home/user/project",
    "commit_hash": "a1b2c3d4e5f67890",
    "branch": "main",
    "changed_files": [
      {
        "path": "docs/architecture/system-design.md",
        "status": "modified"
      },
      {
        "path": "docs/tests/unit-testing.md",
        "status": "added"
      }
    ]
  }
}
```

### 4. Graph Render State Schema (Extension Host to Webview)
```json
{
  "hub_id": "AGENTS.md",
  "token_summary": {
    "active_harness_tokens": 4200,
    "subcall_accessible_tokens": 48300,
    "warning_threshold": 60000,
    "critical_threshold": 100000,
    "status": "normal"
  },
  "nodes": [
    {
      "id": "AGENTS.md",
      "label": "Agent Operating System",
      "file_path": "/home/user/project/AGENTS.md",
      "tags": ["harness", "system"],
      "color": "#3B82F6",
      "status": "active",
      "tokens": 4200,
      "hop_distance": 0,
      "in_degree": 12,
      "out_degree": 6,
      "betweenness": 0.45,
      "broken_links": 0
    },
    {
      "id": "docs/tests/unit-testing.md",
      "label": "Unit Testing Standards",
      "file_path": "/home/user/project/docs/tests/unit-testing.md",
      "tags": ["tests", "quality"],
      "color": "#10B981",
      "status": "connected_inactive",
      "tokens": 2800,
      "hop_distance": 1,
      "in_degree": 3,
      "out_degree": 2,
      "betweenness": 0.12,
      "broken_links": 0
    },
    {
      "id": "notes/todo-random.md",
      "label": "Orphan Note",
      "file_path": "/home/user/project/notes/todo-random.md",
      "tags": ["scratch"],
      "color": "#6B7280",
      "status": "orphan",
      "tokens": 450,
      "hop_distance": -1,
      "in_degree": 0,
      "out_degree": 0,
      "betweenness": 0.0,
      "broken_links": 1
    }
  ],
  "edges": [
    {
      "id": "AGENTS.md->docs/tests/unit-testing.md",
      "source": "AGENTS.md",
      "target": "docs/tests/unit-testing.md",
      "is_broken": false,
      "link_type": "markdown"
    },
    {
      "id": "notes/todo-random.md->missing-file.md",
      "source": "notes/todo-random.md",
      "target": "missing-file.md",
      "is_broken": true,
      "link_type": "wikilink"
    }
  ]
}
```

---

## Measurable Success Criteria *(mandatory)*

- **SC-001**: Graph parsing, link resolution, and centrality computation of 1,000 Markdown files completes in under **200 ms**.
- **SC-002**: Resident memory footprint (RSS) of the Rust sidecar remains strictly below **35 MB** in resting state for vaults up to 5,000 notes.
- **SC-003**: The Webview graph visualizer renders smoothly at **60 FPS** during pan, zoom, node drag, and physics force stabilization.
- **SC-004**: Incremental in-memory graph patching in the Rust core completes in $\le 50\text{ ms}$; total end-to-end Git commit hook delta processing (process launch, file filtering, and IPC roundtrip) completes in under **150 ms** under normal conditions, and strictly aborts/fails open within **500 ms** if the socket is unreachable.
- **SC-005**: Dead link detection achieves **100% accuracy** on broken internal paths, dangling wikilinks, and missing section anchors (`#heading`).
- **SC-006**: Token estimation matches the reference OpenAI/Tiktoken `cl100k_base` BPE tokenizer with **0% deviation**.
- **SC-007**: Packaged `.vsix` extension size remains below **25 MB** per target architecture, passing automated `@vscode/vsce package` checks.

---

## Assumptions & Technical Dependencies

1. **Target Platforms**: Linux (x86_64, aarch64), macOS (x86_64, Apple Silicon aarch64), Windows (x86_64).
2. **Editor Requirement**: Visual Studio Code version `^1.90.0` or higher.
3. **Distribution Model**: Pre-compiled Rust sidecar binaries bundled within platform-specific `.vsix` packages published to Visual Studio Marketplace and Open VSX Registry (CTaaS GAP-028).
4. **No External Network Dependency**: All AST parsing, graph computations, token calculations, and UI rendering execute 100% locally and offline.
