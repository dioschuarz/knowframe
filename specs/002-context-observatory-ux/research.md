---
wiki:
  content_hash: 3cc041b916980b3edb3efc49f3bce2983f302c441513769e78c7a74c1443b659
  fragments:
    phase-0-research-ai-context-observatory-curated-knowledge-ux-002-context-observatory-ux:
      tokens: 1214
      inbound: 0
      outbound: 0
    technical-context-questions-resolved:
      tokens: 1123
      inbound: 0
      outbound: 0
    question-1-how-should-the-system-separate-technical-file-parsing-from-knowledge-graph-visualization:
      tokens: 283
      inbound: 0
      outbound: 0
    question-2-how-to-guarantee-the-target-resolvability-invariant-without-graph-pollution:
      tokens: 201
      inbound: 0
      outbound: 0
    question-3-how-should-the-contextual-inspector-be-embedded-in-the-webview:
      tokens: 188
      inbound: 0
      outbound: 0
    question-4-how-should-hop-constrained-focus-modes-be-executed:
      tokens: 236
      inbound: 0
      outbound: 0
    question-5-how-should-noise-pattern-suggestions-be-discovered:
      tokens: 207
      inbound: 0
      outbound: 0
---
# Phase 0 Research: AI Context Observatory & Curated Knowledge UX (`002-context-observatory-ux`)

**Feature Branch**: `002-context-observatory-ux`  
**Date**: 2026-09-11  
**Status**: Completed  
**Source Inputs**: Architectural & UX Review (`specs/chatgpt_conversation.md`), Constitution v1.3.0, PRD v1.0.0

---

## Technical Context & Questions Resolved

### Question 1: How should the system separate technical file parsing from knowledge graph visualization?
- **Decision**: Introduce a Two-Tier Exclusion Architecture in `.context-vault.yaml`:
  1. `scan.exclude`: Technical directory ignore (`.git/**`, `node_modules/**`, `target/**`, `dist/**`, `build/**`). The Rust AST parser completely skips these directories during filesystem traversal.
  2. `graph.exclude`: Curated knowledge exclusion (`packages/**/README.md`, `vendor/**`, `generated/**`). The Rust core parses these Markdown files and indexes their headings and IDs so internal link references remain 100% resolvable, but they are **omitted from the active graph topology and orphan metrics**.
- **Rationale**: Real repositories contain documentation (e.g. package `README.md` files) that is technically valid and referenced by architecture docs, but clutters the visual graph as hundreds of disconnected orphan nodes. Treating them as non-existent breaks valid links; treating them as normal nodes floods the visualization. Two-tier separation cleanly resolves this tension.
- **Alternatives Considered**:
  - *Single ignore list*: Rejected because ignoring a file means links to it become false-positive broken links.
  - *Client-side CSS opacity hiding*: Rejected because sending 500+ hidden nodes over WebSocket wastes memory and CPU in the D3 simulation.

---

### Question 2: How to guarantee the Target Resolvability Invariant without graph pollution?
- **Decision**: Extend the Rust link resolver (`crates/core/src/graph/resolver.rs`) with a tri-state classification:
  - `Resolved`: Target exists and is in the active graph. Normal edge drawn.
  - `ResolvedExcluded`: Target exists on disk but matches `graph.exclude`. Edge is **not** drawn on canvas by default, and link is **not** flagged as broken.
  - `Broken`: Target does not exist on disk or anchor heading is missing. Red dashed edge drawn and issue card created.
- **Rationale**: Preserves Constitution Principle V (link integrity) while obeying Constitution Principle III (curated signal over raw noise).
- **Alternatives Considered**:
  - *Render edges to nowhere*: Confuses the user with dangling arrows.
  - *Render ghost nodes permanently*: Re-introduces graph noise that the exclusion was meant to eliminate.

---

### Question 3: How should the Contextual Inspector be embedded in the Webview?
- **Decision**: Implement a slide-over Contextual Inspector drawer inside the Webview iframe anchored to the right margin, triggered by node or edge selection, alongside the calm left sidebar status dashboard.
- **Rationale**: Opening native VS Code dialogs or separate webview panels introduces window fragmentation and context switching. An embedded slide-over drawer keeps the user in flow while providing instant access to file metadata, token weights, and actions (`[Open file]`, `[Set as hub]`, `[Exclude from graph]`).
- **Alternatives Considered**:
  - *Tooltip hover modal*: Tooltips cannot support interactive buttons like `[Open file]` or `[Exclude]` without awkward hover lock interactions.
  - *VS Code QuickPick / InputBox*: Limited layout, cannot display multi-metric token bars or incoming/outgoing connection lists simultaneously.

---

### Question 4: How should Hop-Constrained Focus Modes be executed?
- **Decision**: Execute focus mode filtering on the Webview UI layer using pre-calculated BFS hop distances from the Rust engine (`Node.hop_distance`). When a focus mode is active (`hub`, `1_hop`, `2_hops`), nodes beyond the horizon are filtered from the active simulation nodes array, and the D3 simulation is smoothly re-heated with $\alpha=0.3$.
- **Rationale**: The Rust sidecar already computes exact BFS hop distances from the central hub during each scan. Client-side filtering allows instantaneous ($\le 16\text{ ms}$) UI switching without requiring a round-trip RPC call or full graph recomputation.
- **Alternatives Considered**:
  - *Re-query Rust core for subgraph*: Adds 50–100 ms network roundtrip latency for an operation that can be executed in 2 ms on client state.
  - *Visual opacity fading only*: Distant nodes still collide and exert force on the active layout, causing jitter. Filtering from the active force simulation keeps the focused nodes rock-solid and calm.

---

### Question 5: How should Noise Pattern Suggestions be discovered?
- **Decision**: In `llm-wiki-core`, after computing graph topology, inspect all orphan nodes (degree = 0). Group them by common filename stems (e.g. `README.md`), common directory prefixes (e.g. `vendor/**`), and common extensions. Any cluster containing $\ge 5$ disconnected files generates a `NoiseSuggestion` payload.
- **Rationale**: Developers rarely write glob patterns manually; discovering noisy clusters automatically and offering one-click approval embodies the core principle *"System decides/suggests $\rightarrow$ User confirms"*.
- **Alternatives Considered**:
  - *Hardcoded static pattern list*: Fails to adapt to custom repository layouts (e.g. `packages/`, `modules/`, `apps/`).
  - *AI/LLM-based classification*: Heavyweight, introduces token cost and latency; algorithmic directory clustering runs in $< 5\text{ ms}$ in Rust.
