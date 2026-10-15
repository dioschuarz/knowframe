---
wiki:
  content_hash: 53e87f334ab20ffa9089f469c302224b152686253049a7dd5c9879a3c8ed9718
  fragments:
    feature-specification-ai-context-observatory-curated-knowledge-ux-002-context-observatory-ux:
      tokens: 6176
      inbound: 0
      outbound: 0
    executive-summary-paradigm-shift:
      tokens: 832
      inbound: 0
      outbound: 0
    user-scenarios-testing-mandatory:
      tokens: 2392
      inbound: 0
      outbound: 0
    user-story-1-calm-status-dashboard-zero-action-duplication-priority-p0:
      tokens: 545
      inbound: 0
      outbound: 0
    user-story-2-contextual-selection-inspector-for-nodes-edges-health-priority-p1:
      tokens: 496
      inbound: 0
      outbound: 0
    user-story-3-noise-management-first-class-graph-exclusions-priority-p1:
      tokens: 559
      inbound: 0
      outbound: 0
    user-story-4-hop-constrained-focus-modes-semantic-visual-hierarchy-priority-p1:
      tokens: 425
      inbound: 0
      outbound: 0
    user-story-5-advanced-settings-progressive-disclosure-layer-priority-p2:
      tokens: 357
      inbound: 0
      outbound: 0
    functional-requirements-fr-046-through-fr-066:
      tokens: 1237
      inbound: 0
      outbound: 0
    information-architecture-action-hierarchy:
      tokens: 275
      inbound: 0
      outbound: 0
    contextual-selection-inspector:
      tokens: 271
      inbound: 0
      outbound: 0
    noise-management-curated-graph-exclusions:
      tokens: 292
      inbound: 0
      outbound: 0
    focus-modes-semantic-sizing:
      tokens: 222
      inbound: 0
      outbound: 0
    progressive-disclosure-settings-drawer:
      tokens: 165
      inbound: 0
      outbound: 0
    success-criteria-sc-008-through-sc-014:
      tokens: 325
      inbound: 0
      outbound: 0
    data-model-configuration-contract-updates:
      tokens: 812
      inbound: 0
      outbound: 0
    1-updated-context-vaultyaml-schema:
      tokens: 315
      inbound: 0
      outbound: 0
    2-json-rpc-protocol-additions:
      tokens: 489
      inbound: 0
      outbound: 0
    edge-cases-defensive-architecture:
      tokens: 323
      inbound: 0
      outbound: 0
    non-functional-security-requirements:
      tokens: 160
      inbound: 0
      outbound: 0
---
# Feature Specification: AI Context Observatory & Curated Knowledge UX (`002-context-observatory-ux`)

**Feature Branch**: `002-context-observatory-ux`  
**Created**: 2026-09-11  
**Status**: Proposed / Under Review  
**Input**: Critical UX Advisory & Architecture Review (`specs/chatgpt_conversation.md`), Constitution v1.3.0, User Requests, PRD v1.0.0

---

## Executive Summary & Paradigm Shift

The original implementation of `llm_wiki_ext4vsc` established a robust technical foundation (Rust AST parsing with `pulldown-cmark`, Rayon parallelism, `petgraph` centrality, `tiktoken-rs` BPE token counting, loopback Git hooks via `prek`, and 60 FPS HTML5 Canvas rendering). However, as identified in architectural advisory review, the user interface currently **exposes implementation machinery instead of serving the user's primary job**. Duplicate controls compete across the left sidebar, floating toolbar, and dropdown menus, while disconnected library files (e.g. package `README.md` files) clutter the graph as orphan noise.

This specification elevates the product from a generic "Obsidian graph visualizer in VS Code" into a calm, world-class **AI Context Observatory**:
1. **Core Axiom**: *"The filesystem is the source. The knowledge graph is a curated representation of what matters."* The graph does not visualize raw filesystem noise; it maps the structural knowledge that powers developer understanding and AI agent context.
2. **Three Core Pillars**:
   - **Knowledge**: Structural topology, hub discovery, and internal link integrity.
   - **AI Context**: Active Prompt Load (Level 0) versus Reachable On-Demand Context (Level 1+), context paths, and BPE token limits.
   - **Health & Issues**: Dead links, missing anchors, unmapped orphans, and background Git synchronization.
3. **Information Architecture & Deduplication (Zero Action Duplication)**:
   - Eliminates duplicate action buttons ("Open Knowledge Graph", "Set Central Hub", "Rescan Vault", "Fix Broken Links", "Install Git Hooks") from the sidebar.
   - Converts buttons into **state-driven indicators**: actions emerge naturally from state (e.g. `343 broken links → [Review issues]`, `● Synced · 12s ago`, `Central Hub: Workspace Wiki Master → [Change]`).
   - Retains only in-graph controls on the floating toolbar: Token pill, Search, Focus mode, Reset disposition, and More (`⋯`).
4. **Contextual Selection Inspector**:
   - Selecting a node opens a dedicated contextual inspector drawer with token breakdown, context hop level, inbound/outbound links, and contextual actions (`Open file`, `Set as hub`, `Exclude this file/folder`).
   - Selecting a broken edge displays source file, missing target, line number, and one-click fix/open.
   - Clicking the token pill reveals the full prompt budget distribution (Active L0 vs Reachable L1+).
5. **Noise Management & First-Class Graph Exclusions (FR-046 through FR-053)**:
   - Distinguishes **Technical Scan Exclusions** (`scan.exclude`: `.git/**`, `node_modules/**`, `target/**`) from **Curated Knowledge Exclusions** (`graph.exclude`: `packages/**/README.md`, `vendor/**`).
   - Tri-State Node Classification: **Connected** (in graph & reachable), **Orphan** (in graph but disconnected), and **Excluded** (intentionally outside graph).
   - **Target Resolvability Guarantee**: Valid Markdown links or wikilinks targeting `graph.exclude` files resolve cleanly and **MUST NOT** trigger broken link errors.
   - Contextual right-click exclusion ("Exclude this file", "Exclude this folder", "Exclude pattern..."), interactive preview with affected file counts, and automated noise pattern suggestions.
6. **Focus Modes & Semantic Visual Hierarchy**:
   - Hop-constrained focus levels (`Hub`, `1 hop`, `2 hops`, `All`) to prevent visual overwhelm on 500+ node codebases.
   - Semantic node sizing: Level 0 (Hub) prominent, Level 1 medium, Level 2 compact, Level 3+ subtle, orphans muted.
   - Orphan visibility toggle (`Show orphans` checkbox) to clean up exploratory views.

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Calm Status Dashboard & Zero Action Duplication (Priority: P0)

As a developer opening the LLM Wiki panel in VS Code,  
I want the sidebar to serve as a calm status dashboard answering *"What is important about my knowledge base right now?"* without duplicate action buttons or competing controls,  
So that I understand vault health, AI token load, and hub status at a glance without cognitive clutter.

**Why this priority**: Solves the #1 UX defect identified in the architectural review: action duplication across sidebar, top toolbar, and dropdown menus.

**Independent Test**: Load the extension in a test workspace with 500+ notes; verify that the left sidebar displays summary metrics (notes, links, issues, active tokens, central hub) with zero duplicate buttons; verify that the top floating toolbar contains only view-manipulation tools (Token pill, Search, Focus, Reset disposition, More).

**Acceptance Scenarios**:
1. **Given** the Knowledge Graph view is open,  
   **When** viewing the left sidebar,  
   **Then** the obsolete buttons `Open Knowledge Graph`, `Set Central Hub`, `Rescan Vault`, `Fix Broken Links`, and `Install Git Hooks` are completely removed.
2. **Given** the left sidebar,  
   **When** rendering status,  
   **Then** it displays three structured sections:
     - **KNOWLEDGE**: Note count, connection count, and health status badge.
     - **AI CONTEXT**: Active Prompt Load tokens (Level 0), Reachable Context tokens (Level 1+), and percentage progress bar.
     - **CENTRAL HUB**: Current active hub name with a discreet `[Change]` link, and sync status (`● Synced · Xs ago`).
3. **Given** the vault contains broken links (e.g. 343 broken links),  
   **When** the user views the sidebar,  
   **Then** an attention card `343 broken links` is displayed with a single contextual action `[Review issues]`.
4. **Given** the floating toolbar inside the Webview,  
   **When** rendered,  
   **Then** it consists strictly of:
     - `[4.9k active]` Token pill with budget health color.
     - `[🔍]` Multi-mode search button.
     - `[🎯 Focus: All ▾]` Focus mode dropdown (`Hub`, `1 hop`, `2 hops`, `All`).
     - `[📌]` Reset Disposition button with dynamic pinned count badge.
     - `[⋯]` More / Advanced dropdown menu.

---

### User Story 2 - Contextual Selection Inspector for Nodes, Edges & Health (Priority: P1)

As a developer exploring the knowledge graph,  
I want clicking any node or broken link to reveal a rich contextual inspector with targeted metadata and actions,  
So that the interface adapts dynamically to my intent instead of crowding permanent buttons onto the screen.

**Why this priority**: Core interaction design principle: actions emerge contextually from selection rather than existing as static clutter.

**Independent Test**: Click a normal node, a hub node, a broken link edge, and the token badge; verify the inspector panel opens with precise metadata, tokens, hop level, and actionable buttons.

**Acceptance Scenarios**:
1. **Given** a normal node selected in the graph,  
   **When** the inspector opens,  
   **Then** it displays note title, relative file path, context tier (`Level 1` or `Level 2`), exact BPE tokens, incoming connection count, outgoing connection count, and two primary action buttons: `[Open file]` and `[Set as hub]`, plus a `⋯ More` menu with `[Exclude from graph]`.
2. **Given** a user clicks `[Open file]`,  
   **When** executed,  
   **Then** VS Code opens the target Markdown document in the editor beside the graph (`ViewColumn.Beside`) with zero graph reload or context loss.
3. **Given** a red dashed broken link edge selected in the graph,  
   **When** the inspector opens,  
   **Then** it displays source file path, line number, missing target name, error type (`Missing file`, `Missing anchor`, or `Unresolved wikilink`), and actions `[Open source]` and `[Fix reference]`.
4. **Given** the user clicks the floating `[4.9k active]` token pill,  
   **When** triggered,  
   **Then** the Token Economics popover displays:
     - Active Context (Level 0 files): token sum and file list.
     - Reachable Context (Level 1 & Level 2 files): cumulative tokens and file count.
     - Budget status: `Healthy (<60k)`, `Warning (60k-100k)`, or `Critical (>100k)`.
     - An `[Explore context]` button to filter the graph strictly to the active prompt load.

---

### User Story 3 - Noise Management & First-Class Graph Exclusions (Priority: P1)

As a developer working in a monorepo with nested package `README.md` files, vendored code, and generated docs,  
I want to exclude noisy or irrelevant files and folders from appearing in the knowledge graph without deleting them or having them clutter orphan metrics,  
So that my knowledge graph represents only curated, meaningful project architecture.

**Why this priority**: Solves the primary pain point of automatic graph discovery in real repositories: technical correctness paired with cognitive noise.

**Independent Test**: Configure `graph.exclude: ["packages/**/README.md"]` in `.context-vault.yaml`; verify that all matching notes disappear from the graph canvas and are subtracted from orphan counts, while valid links pointing to those files remain 100% resolvable and do NOT trigger broken link alerts.

**Acceptance Scenarios**:
1. **Given** a project with 923 total notes including 105 package `README.md` files,  
   **When** the user right-clicks a node or clicks `⋯ More` in the inspector and selects `Exclude this folder` or `Exclude matching files...`,  
   **Then** an exclusion preview modal appears displaying the pattern (e.g. `packages/**/README.md`), the count of affected files (`Matches 105 files`), and representative paths before applying.
2. **Given** an exclusion pattern is applied,  
   **When** the graph and summary update,  
   **Then**:
     - The excluded notes are removed from the canvas force layout.
     - The sidebar metrics display: `781 connected`, `37 orphans`, `105 excluded`.
     - Excluded notes do NOT increment the orphan counter.
3. **Given** an architecture document `docs/api.md` containing a link to an excluded file `[Auth API](packages/auth/README.md)`,  
   **When** link validation executes,  
   **Then** the target is recognized as an existing file on disk and marked as `resolved_excluded`; it MUST NOT be flagged as a broken link.
4. **Given** the user opens the Filter popover,  
   **When** they check the optional diagnostic checkbox `Show excluded`,  
   **Then** excluded nodes appear in faint, ghosted dotted outlines (`⋯`) without pulling on the force simulation.
5. **Given** the system discovers high-frequency disconnected file patterns (e.g. 84 `**/README.md` files with zero connections),  
   **When** the graph health check runs,  
   **Then** the sidebar displays a suggested clean-up card: *"84 disconnected README files detected. [Review suggestions]"*, allowing one-click batch exclusion.

---

### User Story 4 - Hop-Constrained Focus Modes & Semantic Visual Hierarchy (Priority: P1)

As a prompt architect analyzing complex dependency chains in a large vault,  
I want to restrict the visible graph to 1 or 2 hops from the central hub and have node sizes clearly communicate context importance,  
So that I can study active LLM context without being blinded by hundreds of distant peripheral nodes.

**Why this priority**: Prevents visual saturation and brings semantic meaning to the 2D canvas visualization.

**Acceptance Scenarios**:
1. **Given** a vault with 923 notes and an active hub (`AGENTS.md`),  
   **When** the user selects `Focus: 1 hop` from the toolbar dropdown,  
   **Then** only Level 0 (Hub) and Level 1 (direct references) are rendered; Level 2+ nodes are hidden.
2. **Given** `Focus: 2 hops` is selected,  
   **When** rendered,  
   **Then** Level 0, Level 1, and Level 2 transitive references are rendered, while Level 3+ nodes are hidden.
3. **Given** any focus mode,  
   **When** nodes are rendered on canvas,  
   **Then** node geometry reflects semantic role:
     - Level 0 (Hub): Radius = 14px, bold label, 100% opacity.
     - Level 1: Radius = 9px, medium label, 60% opacity.
     - Level 2: Radius = 6px, small label on hover, 30% opacity.
     - Level 3+: Radius = 4px, minimal label on hover, 15% opacity.
     - Orphans: Radius = 4px, muted gray (`#6B7280`), 40% opacity.
4. **Given** the Filter menu,  
   **When** the user unchecks `Show orphans`,  
   **Then** all orphan notes are hidden from the canvas, leaving only connected knowledge topologies.

---

### User Story 5 - Advanced Settings & Progressive Disclosure Layer (Priority: P2)

As a developer configuring vault preferences,  
I want administrative tools (Git hook management, cluster color groups, exclusion rules, and manual index rebuilding) organized in an Advanced Settings slide-over drawer,  
So that configuration tools are accessible when needed but never clutter the primary graph experience.

**Why this priority**: Implements Apple-grade progressive disclosure by cleanly separating operational configuration from exploratory visualization.

**Acceptance Scenarios**:
1. **Given** the `[⋯]` button on the floating toolbar,  
   **When** clicked,  
   **Then** a clean dropdown menu opens containing:
      - **Graph**: `Reset view`, `Export graph (SVG/PNG)`.
      - **Knowledge**: `Rebuild index`, `Review broken links`.
      - **Configuration**: `Central hub rules`, `Excluded paths`, `Cluster color groups`.
      - **Integrations**: `Git synchronization (prek)`.
      - **Diagnostics**: `Sidecar metrics`, `Copy graph state`.
2. **Given** the user selects `Git synchronization (prek)`,  
   **When** opened,  
   **Then** a modal or drawer displays current hook status (`● Installed via prek`, `Last commit delta: 14s ago`), hook latency, and a `[Reinstall]` button.
3. **Given** the user selects `Excluded paths`,  
   **When** opened,  
   **Then** a list of all active exclusion globs is displayed with file match counts, a test preview input, and `[+ Add pattern]` / `[Remove]` buttons, persisting changes to `.context-vault.yaml`.

---

## Functional Requirements (FR-046 through FR-066)

### Information Architecture & Action Hierarchy
- **FR-046**: The Webview interface MUST organize all presentation around three distinct pillars: **Knowledge** (topology), **AI Context** (tokens & reachability), and **Health & Issues** (broken links, orphans, sync).
- **FR-047**: The left sidebar MUST NOT contain duplicate action buttons that also exist in the floating toolbar or context menus. Specifically, `Open Knowledge Graph`, `Set Central Hub`, `Rescan Vault`, `Fix Broken Links`, and `Install Git Hooks` MUST be removed as standalone primary sidebar buttons.
- **FR-048**: Administrative and remediation actions MUST emerge contextually from displayed system states:
  - Broken link issues MUST appear as a status metric with a single `[Review issues]` action opening the Settings Drawer `Knowledge` tab highlighting all broken links.
  - Background synchronization MUST appear as a quiet status badge (`● Synced · 12s ago`), with index rebuild accessible via secondary menus.
  - Central Hub MUST be displayed as an active entity with a contextual `[Change]` workflow.
- **FR-049**: The floating toolbar inside the Webview canvas MUST be constrained strictly to view-manipulation tools: Token status pill, Multi-mode search, Focus mode selector, Reset disposition button, and More (`⋯`) dropdown.

### Contextual Selection Inspector
- **FR-050**: The Webview MUST provide a collapsible Contextual Inspector drawer that activates whenever a node, edge, or token status metric is selected.
- **FR-051**: When a node is selected, the inspector MUST display: note title, relative file path, context tier (Level 0 through 3+ or Orphan), exact BPE token count, incoming links count, outgoing links count, and primary actions `[Open file]` (opening side-by-side tab via `ViewColumn.Beside`), `[Set as hub]` (persisting `hub.primary_hub` in `.context-vault.yaml` and triggering BFS hop recomputation), and `[Exclude from graph]` (opening exclusion preview modal).
- **FR-052**: When a broken link edge is selected, the inspector MUST display: source file path, line number, missing link text, error classification, and actions `[Open source]` (opening editor to line number) and `[Fix reference]` (opening VS Code QuickPick note picker to replace target link destination).
- **FR-053**: When the token pill is activated, the inspector MUST display a breakdown of Active Prompt Load (Level 0) versus Reachable On-Demand Context (Level 1+), along with warning/critical threshold indicators.

### Noise Management & Curated Graph Exclusions
- **FR-054**: The system MUST separate configuration of technical file scan exclusions (`scan.exclude`) from curated knowledge graph exclusions (`graph.exclude`) in `.context-vault.yaml`.
- **FR-055**: The system MUST classify every discovered Markdown file into exactly one of three states: **Connected**, **Orphan**, or **Excluded**.
- **FR-056**: Files matching `graph.exclude` globs MUST NOT be rendered on the default graph canvas and MUST NOT contribute to orphan counts or centrality calculations.
- **FR-057**: Valid links pointing to files matching `graph.exclude` MUST resolve cleanly to the filesystem and MUST NOT be flagged as dead or broken links (`resolved_excluded`).
- **FR-058**: The Webview MUST provide contextual exclusion actions from node selection (`Exclude file`, `Exclude containing folder`, `Exclude pattern...`).
- **FR-059**: Before applying an exclusion glob, the system MUST display a preview dialog showing the count of affected files and sample file paths.
- **FR-060**: The system MUST automatically detect high-frequency disconnected file patterns (e.g. `**/README.md`, `vendor/**`) and present suggested exclusion recommendations for one-click user approval.
- **FR-061**: The Webview Filter menu MUST provide an optional `Show excluded` diagnostic mode that renders excluded notes as faint, non-interactive ghost nodes.

### Focus Modes & Semantic Sizing
- **FR-062**: The Webview MUST provide a Focus mode dropdown offering four discrete view scopes: `Hub (Level 0)`, `1 Hop (Level 0 + 1)`, `2 Hops (Level 0 + 1 + 2)`, and `All (Full Vault)`.
- **FR-063**: In focused view modes, nodes outside the selected hop horizon MUST be hidden from the force simulation to maximize clarity.
- **FR-064**: Node rendering geometry MUST scale semantically by context tier: Level 0 hubs rendered largest ($r=14\text{px}$), Level 1 medium ($r=9\text{px}$), Level 2 compact ($r=6\text{px}$), Level 3+ subtle ($r=4\text{px}$), and Orphans muted ($r=4\text{px}$).
- **FR-065**: The Filter popover MUST include a `Show orphans` toggle that allows users to completely hide unlinked notes from the visualization.

### Progressive Disclosure & Settings Drawer
- **FR-066**: The Webview MUST provide an Advanced Settings slide-over drawer accessible via `⋯ More → Settings` containing dedicated configuration tabs:
  1. `Knowledge`: Broken links list with line coordinates and one-click jump.
  2. `Excluded Paths`: Active rule list, match counts, glob testing input, and `[+ Add pattern]` / `[Delete]` actions persisting to `.context-vault.yaml`.
  3. `Cluster Groups`: Chromatic group configuration and hashtag/path color rules.
  4. `Git Sync`: `prek` hook installation status, last commit delta latency, and reinstall action.
  5. `Diagnostics`: Sidecar RSS memory, IPC socket status, and graph state export.

---

## Success Criteria (SC-008 through SC-014)

- **SC-008 (Zero Action Duplication)**: 100% of actions previously duplicated across sidebar and toolbar are consolidated into single, contextually appropriate locations.
- **SC-009 (Dashboard Calmness)**: Primary left sidebar height and visual clutter are reduced by $\ge 50\%$, presenting clear metrics across the three pillars (Knowledge, AI Context, Health).
- **SC-010 (Noise Reduction on Monorepos)**: Applying suggested exclusions on repositories with multiple package READMEs reduces orphan note count by $\ge 70\%$ without generating any false-positive broken links.
- **SC-011 (Exclusion Link Integrity)**: 100% of internal links targeting `graph.exclude` files resolve successfully with zero spurious broken link notifications.
- **SC-012 (Focus Mode Responsiveness)**: Client-side subgraph array filtering for Focus modes (`Hub`, `1 hop`, `2 hops`, `All`) executes in $\le 16\text{ ms}$ (60 FPS single frame), with smooth D3 simulation force relaxation completing within $\le 50\text{ ms}$.
- **SC-013 (Inspector Opening Latency)**: Clicking any graph node or broken edge opens the Contextual Inspector with file metadata and token stats in $\le 30\text{ ms}$.
- **SC-014 (Exclusion Preview Accuracy)**: The exclusion preview dialog computes exact matching file counts across 5,000 files in $\le 10\text{ ms}$ using Rust `globset`.

---

## Data Model & Configuration Contract Updates

### 1. Updated `.context-vault.yaml` Schema

```yaml
# .context-vault.yaml - Version 2.0.0
version: "2.0.0"

# Technical file scanning boundaries (completely ignored by parser)
scan:
  include:
    - "**/*.md"
  exclude:
    - ".git/**"
    - "node_modules/**"
    - "target/**"
    - "dist/**"
    - "build/**"
    - ".vscode/**"

# Curated Knowledge Graph exclusions (parsed for link resolution, but excluded from graph & orphans)
graph:
  exclude:
    - "packages/**/README.md"
    - "vendor/**"
    - "coverage/**"
  focus_mode: "all"            # "hub" | "1_hop" | "2_hops" | "all"
  show_orphans: true
  show_excluded: false

# Central hub configuration
hub:
  strategy: "auto"             # "auto" | "manual"
  primary_hub: "notes/00_meta/wiki_index.md"

# Token budget thresholds
token_budget:
  warning_threshold: 60000
  critical_threshold: 100000

# Chromatic cluster rules
clusters:
  - rule: "tag:#architecture"
    color: "#8B5CF6"
    label: "Architecture"
  - rule: "path:notes/05_secrets/**"
    color: "#EF4444"
    label: "Secrets"
```

### 2. JSON-RPC Protocol Additions

```json
// Request: graph/excludePath
{
  "jsonrpc": "2.0",
  "id": "req-101",
  "method": "graph/excludePath",
  "params": {
    "pattern": "packages/**/README.md",
    "scope": "graph"
  }
}

// Response: graph/excludePath
{
  "jsonrpc": "2.0",
  "id": "req-101",
  "result": {
    "success": true,
    "affectedFilesCount": 84,
    "updatedSummary": {
      "totalNotes": 923,
      "connectedNotes": 781,
      "orphanNotes": 37,
      "excludedNotes": 105
    }
  }
}

// Request: graph/previewExclusion
{
  "jsonrpc": "2.0",
  "id": "req-102",
  "method": "graph/previewExclusion",
  "params": {
    "pattern": "packages/**/README.md"
  }
}

// Response: graph/previewExclusion
{
  "jsonrpc": "2.0",
  "id": "req-102",
  "result": {
    "matchingCount": 84,
    "samplePaths": [
      "packages/core/README.md",
      "packages/api/README.md",
      "packages/auth/README.md",
      "packages/client/README.md"
    ]
  }
}

// Request: graph/suggestExclusions
{
  "jsonrpc": "2.0",
  "id": "req-103",
  "method": "graph/suggestExclusions",
  "params": {}
}

// Response: graph/suggestExclusions
{
  "jsonrpc": "2.0",
  "id": "req-103",
  "result": {
    "suggestions": [
      {
        "pattern": "**/README.md",
        "matchingCount": 84,
        "reason": "84 disconnected README files found across subdirectories",
        "recommended": true
      },
      {
        "pattern": "vendor/**",
        "matchingCount": 16,
        "reason": "16 files in third-party vendor directory",
        "recommended": true
      }
    ]
  }
}
```

---

## Edge Cases & Defensive Architecture

1. **Self-Referential Exclusions**: If a user attempts to exclude the active Central Hub (e.g. `AGENTS.md`), the system MUST reject the exclusion with an informative error modal: *"Cannot exclude active Central Hub. Please select a different hub first."*
2. **Exclusion Pattern Collision**: If a pattern in `graph.exclude` overlaps with an existing `scan.exclude` pattern, `scan.exclude` takes precedence and the duplicate is deduplicated automatically.
3. **Dangling Link to Excluded Note**: When note A links to note B, and note B is in `graph.exclude`:
   - If note B exists on disk $\rightarrow$ link status is `resolved_excluded`. Edge is not rendered in graph; no broken link warning is created.
   - If note B does NOT exist on disk $\rightarrow$ link status is `broken`. Edge is rendered red dashed; broken link issue card is created with file/line coordinates.
4. **Empty Graph in Focus Mode**: In a freshly initialized repository where the hub has no connections yet and the user selects `Focus: 1 hop`, the Webview displays the hub node centered with a calm empty state tip: *"This hub has no outgoing connections yet. Add links using `[[note-name]]` to expand your context graph."*
5. **Dynamic Vault Rescan**: When the user triggers `Rebuild index` from the advanced menu, the sidecar performs a clean re-scan and emits an incremental update event, ensuring node positions are preserved without visual jumping.

---

## Non-Functional & Security Requirements

1. **Performance Ceiling**:
   - Glob pattern evaluation for 5,000 files using Rust `globset` MUST complete in $\le 10\text{ ms}$.
   - Focus mode filtering on the Webview Canvas MUST execute in $\le 16\text{ ms}$ (within a single animation frame).
2. **Memory Ceiling**:
   - Memory overhead of tracking `excluded` file paths in Rust core MUST NOT exceed 2 MB of resident memory.
3. **Security & Sandboxing**:
   - Contextual Inspector actions that open editor tabs MUST pass target file paths through VS Code URI sanitization to prevent arbitrary command execution.
   - SecretStorage credentials and tokens are NEVER exposed in exclusion preview payloads or IPC logs.
