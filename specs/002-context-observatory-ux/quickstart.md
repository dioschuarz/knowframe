---
wiki:
  content_hash: eb3db137e203d72e6e942eca987631f491b59c5ff292beff9b444ea75a5fe8fd
  fragments:
    quickstart-runnable-validation-scenarios-ai-context-observatory-002-context-observatory-ux:
      tokens: 1024
      inbound: 0
      outbound: 0
    prerequisites:
      tokens: 75
      inbound: 0
      outbound: 0
    scenario-1-calm-status-dashboard-zero-action-duplication:
      tokens: 212
      inbound: 0
      outbound: 0
    execution-steps:
      tokens: 160
      inbound: 0
      outbound: 0
    expected-outcome:
      tokens: 18
      inbound: 0
      outbound: 0
    scenario-2-contextual-selection-inspector:
      tokens: 206
      inbound: 0
      outbound: 0
    execution-steps-1:
      tokens: 154
      inbound: 0
      outbound: 0
    expected-outcome-1:
      tokens: 25
      inbound: 0
      outbound: 0
    scenario-3-curated-knowledge-exclusions-target-resolvability:
      tokens: 262
      inbound: 0
      outbound: 0
    execution-steps-2:
      tokens: 194
      inbound: 0
      outbound: 0
    expected-outcome-2:
      tokens: 33
      inbound: 0
      outbound: 0
    scenario-4-hop-constrained-focus-modes-semantic-sizing:
      tokens: 194
      inbound: 0
      outbound: 0
    execution-steps-3:
      tokens: 137
      inbound: 0
      outbound: 0
    expected-outcome-3:
      tokens: 25
      inbound: 0
      outbound: 0
---
# Quickstart & Runnable Validation Scenarios: AI Context Observatory (`002-context-observatory-ux`)

**Feature Branch**: `002-context-observatory-ux`  
**Date**: 2026-09-11  
**Status**: Completed  
**Source Inputs**: `spec.md`, `data-model.md`, `contracts/observatory_rpc_protocol.json`

---

## Prerequisites

- VS Code $\ge 1.90.0$ installed with `llm-wiki-0.1.0` extension loaded.
- Workspace containing Markdown files (e.g. `/home/ds/projects` with `notes/`).
- Rust toolchain (`cargo`) and Node.js ($\ge 20$) with npm.

---

## Scenario 1: Calm Status Dashboard & Zero Action Duplication

**Objective**: Verify the left sidebar functions as a calm observatory dashboard without redundant action buttons.

### Execution Steps:
1. Open the command palette (`Ctrl+Shift+P`) and run **`LLM Wiki: Show Knowledge Graph`**.
2. Inspect the left sidebar panel.
3. Verify that the obsolete buttons (`Open Knowledge Graph`, `Set Central Hub`, `Rescan Vault`, `Fix Broken Links`, `Install Git Hooks`) are **not present**.
4. Verify the three structured sections:
   - **KNOWLEDGE**: Note count, connection count.
   - **AI CONTEXT**: Active prompt load (Level 0), reachable context, and token capacity progress bar.
   - **CENTRAL HUB & HEALTH**: Active hub name with `[Change]` link, sync indicator (`● Synced`), and broken link attention card if errors exist.

### Expected Outcome:
Zero duplicate action buttons; the sidebar communicates state and health calmly.

---

## Scenario 2: Contextual Selection Inspector

**Objective**: Verify that clicking graph nodes and broken edges reveals an interactive contextual inspector.

### Execution Steps:
1. In the Knowledge Graph webview, click on any connected note (e.g. `notes/00_meta/wiki_index.md`).
2. Verify the Contextual Inspector drawer slides open from the right margin.
3. Confirm displayed fields: Note title, relative path, context tier (`Level 0`), exact BPE tokens, and in/out connections.
4. Click the `[Open file]` button in the inspector; verify the file opens in a side-by-side editor tab (`ViewColumn.Beside`) without resetting the graph view.
5. Click on a red dashed broken link edge; verify the inspector displays the source note, line number, missing destination, and `[Open source]` / `[Fix reference]` buttons.

### Expected Outcome:
Inspector opens in $\le 30\text{ ms}$ with accurate metadata and functional contextual actions.

---

## Scenario 3: Curated Knowledge Exclusions & Target Resolvability

**Objective**: Verify that excluding noisy files reduces orphan noise while preserving link integrity for valid references.

### Execution Steps:
1. In the inspector or via right-click on an orphan note (e.g. `packages/foo/README.md`), select **`Exclude this folder`** or click **`⋯ More → Exclude matching files...`**.
2. An Exclusion Preview modal appears: enter `packages/**/README.md`.
3. Verify the preview indicates exact match count (e.g. `Matches 84 files`) with sample paths.
4. Click **`Apply Exclusion`**.
5. Observe the graph canvas:
   - Matching notes disappear from the simulation force layout.
   - Sidebar metric updates: `Orphan notes` decreases by matching count; `Excluded notes` increases.
6. Open an architecture note that links to an excluded file (e.g. `[Foo README](packages/foo/README.md)`).
7. Verify that this link is **NOT** marked as broken (Target Resolvability Invariant).

### Expected Outcome:
Orphan noise decreases by $\ge 70\%$; valid links targeting excluded files remain fully resolved with zero false broken link alerts.

---

## Scenario 4: Hop-Constrained Focus Modes & Semantic Sizing

**Objective**: Verify that Focus modes restrict visualization to relevant context tiers without visual jitter.

### Execution Steps:
1. Locate the **`🎯 Focus`** dropdown on the top floating toolbar.
2. Select **`Focus: Hub`**: verify only Level 0 (Central Hub) is rendered ($r=14\text{px}$).
3. Select **`Focus: 1 hop`**: verify Level 0 and Level 1 nodes are rendered; all deeper nodes are hidden.
4. Select **`Focus: 2 hops`**: verify Level 0, 1, and 2 nodes are rendered.
5. Toggle the **`Show orphans`** checkbox in the filter popover; verify orphan nodes disappear completely from the canvas.

### Expected Outcome:
Focus switches execute in $\le 50\text{ ms}$; node sizes visually differentiate context tiers.
