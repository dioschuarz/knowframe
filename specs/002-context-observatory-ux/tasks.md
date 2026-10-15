---
wiki:
  content_hash: 643e87dd303b29942e6cdbeae108026c136e10ec73dfee69a3b478b2b9fb1fb2
  fragments:
    tasks-ai-context-observatory-curated-knowledge-ux-002-context-observatory-ux:
      tokens: 9266
      inbound: 0
      outbound: 3
    phase-1-setup-shared-infrastructure:
      tokens: 1063
      inbound: 0
      outbound: 0
    phase-2-foundational-blocking-prerequisites:
      tokens: 273
      inbound: 0
      outbound: 0
    phase-3-user-story-1-p0-calm-status-dashboard-zero-action-duplication-mvp:
      tokens: 545
      inbound: 0
      outbound: 0
    tests-for-user-story-1:
      tokens: 52
      inbound: 0
      outbound: 0
    implementation-for-user-story-1:
      tokens: 369
      inbound: 0
      outbound: 0
    phase-4-user-story-2-p1-contextual-selection-inspector-for-nodes-edges-health:
      tokens: 5138
      inbound: 0
      outbound: 0
    tests-for-user-story-2:
      tokens: 57
      inbound: 0
      outbound: 0
    implementation-for-user-story-2:
      tokens: 4969
      inbound: 0
      outbound: 0
    phase-5-user-story-3-p1-noise-management-first-class-graph-exclusions:
      tokens: 518
      inbound: 0
      outbound: 0
    tests-for-user-story-3:
      tokens: 89
      inbound: 0
      outbound: 0
    implementation-for-user-story-3:
      tokens: 332
      inbound: 0
      outbound: 0
    phase-6-user-story-4-p1-hop-constrained-focus-modes-semantic-visual-sizing:
      tokens: 581
      inbound: 0
      outbound: 0
    tests-for-user-story-4:
      tokens: 47
      inbound: 0
      outbound: 0
    implementation-for-user-story-4:
      tokens: 419
      inbound: 0
      outbound: 0
    phase-7-user-story-5-p2-advanced-settings-drawer-progressive-disclosure:
      tokens: 372
      inbound: 0
      outbound: 0
    implementation-for-user-story-5:
      tokens: 270
      inbound: 0
      outbound: 0
    phase-8-polish-cross-cutting-concerns:
      tokens: 171
      inbound: 0
      outbound: 0
    dependencies-execution-order:
      tokens: 354
      inbound: 0
      outbound: 0
    parallel-opportunities:
      tokens: 156
      inbound: 0
      outbound: 0
    implementation-strategy-mvp-first:
      tokens: 176
      inbound: 0
      outbound: 0
---
# Tasks: AI Context Observatory & Curated Knowledge UX (`002-context-observatory-ux`)

**Input**: [`specs/002-context-observatory-ux/plan.md`](plan.md), [`spec.md`](spec.md), [`data-model.md`](data-model.md), `contracts/observatory_rpc_protocol.json`  
**Status**: Ready for Execution

---

## Phase 1: Setup & Shared Infrastructure

**Purpose**: Scaffolding new modules and schema definitions for v2 Observatory UX.

- [X] T001 Update `.context-vault.yaml` deserializer in `crates/core/src/config/mod.rs` to support two-tier `scan.exclude` and `graph.exclude` arrays with default fallback to v1 schema.
*extension/src/types.ts*-*extension/src/types.ts* *extension/src/types.ts*[*extension/src/types.ts*X*extension/src/types.ts*]*extension/src/types.ts* *extension/src/types.ts*T*extension/src/types.ts*0*extension/src/types.ts*0*extension/src/types.ts*2*extension/src/types.ts* *extension/src/types.ts*[*extension/src/types.ts*P*extension/src/types.ts*]*extension/src/types.ts* *extension/src/types.ts*U*extension/src/types.ts*p*extension/src/types.ts*d*extension/src/types.ts*a*extension/src/types.ts*t*extension/src/types.ts*e*extension/src/types.ts* *extension/src/types.ts*T*extension/src/types.ts*y*extension/src/types.ts*p*extension/src/types.ts*e*extension/src/types.ts*S*extension/src/types.ts*c*extension/src/types.ts*r*extension/src/types.ts*i*extension/src/types.ts*p*extension/src/types.ts*t*extension/src/types.ts* *extension/src/types.ts*c*extension/src/types.ts*o*extension/src/types.ts*n*extension/src/types.ts*f*extension/src/types.ts*i*extension/src/types.ts*g*extension/src/types.ts* *extension/src/types.ts*d*extension/src/types.ts*e*extension/src/types.ts*f*extension/src/types.ts*i*extension/src/types.ts*n*extension/src/types.ts*i*extension/src/types.ts*t*extension/src/types.ts*i*extension/src/types.ts*o*extension/src/types.ts*n*extension/src/types.ts*s*extension/src/types.ts* *extension/src/types.ts*i*extension/src/types.ts*n*extension/src/types.ts* *extension/src/types.ts*`*extension/src/types.ts*e*extension/src/types.ts*x*extension/src/types.ts*t*extension/src/types.ts*e*extension/src/types.ts*n*extension/src/types.ts*s*extension/src/types.ts*i*extension/src/types.ts*o*extension/src/types.ts*n*extension/src/types.ts*/*extension/src/types.ts*s*extension/src/types.ts*r*extension/src/types.ts*c*extension/src/types.ts*/*extension/src/types.ts*t*extension/src/types.ts*y*extension/src/types.ts*p*extension/src/types.ts*e*extension/src/types.ts*s*extension/src/types.ts*.*extension/src/types.ts*t*extension/src/types.ts*s*extension/src/types.ts*`*extension/src/types.ts* *extension/src/types.ts*t*extension/src/types.ts*o*extension/src/types.ts* *extension/src/types.ts*m*extension/src/types.ts*i*extension/src/types.ts*r*extension/src/types.ts*r*extension/src/types.ts*o*extension/src/types.ts*r*extension/src/types.ts* *extension/src/types.ts*t*extension/src/types.ts*h*extension/src/types.ts*e*extension/src/types.ts* *extension/src/types.ts*t*extension/src/types.ts*w*extension/src/types.ts*o*extension/src/types.ts*-*extension/src/types.ts*t*extension/src/types.ts*i*extension/src/types.ts*e*extension/src/types.ts*r*extension/src/types.ts* *extension/src/types.ts*e*extension/src/types.ts*x*extension/src/types.ts*c*extension/src/types.ts*l*extension/src/types.ts*u*extension/src/types.ts*s*extension/src/types.ts*i*extension/src/types.ts*o*extension/src/types.ts*n*extension/src/types.ts*,*extension/src/types.ts* *extension/src/types.ts*f*extension/src/types.ts*o*extension/src/types.ts*c*extension/src/types.ts*u*extension/src/types.ts*s*extension/src/types.ts* *extension/src/types.ts*m*extension/src/types.ts*o*extension/src/types.ts*d*extension/src/types.ts*e*extension/src/types.ts* *extension/src/types.ts*s*extension/src/types.ts*c*extension/src/types.ts*h*extension/src/types.ts*e*extension/src/types.ts*m*extension/src/types.ts*a*extension/src/types.ts*s*extension/src/types.ts*,*extension/src/types.ts* *extension/src/types.ts*a*extension/src/types.ts*n*extension/src/types.ts*d*extension/src/types.ts* *extension/src/types.ts*s*extension/src/types.ts*e*extension/src/types.ts*r*extension/src/types.ts*i*extension/src/types.ts*a*extension/src/types.ts*l*extension/src/types.ts*i*extension/src/types.ts*z*extension/src/types.ts*a*extension/src/types.ts*t*extension/src/types.ts*i*extension/src/types.ts*o*extension/src/types.ts*n*extension/src/types.ts*s*extension/src/types.ts*.*extension/src/types.ts*
*extension/src/types.ts*- [X] T003 [P] Add new JSON-RPC message types for exclusion preview, path exclusion, set central hub, and focus mode in `crates/core/src/ipc/protocol.rs`.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core engine link resolution and exclusion matching that MUST be complete before UI stories begin.

**⚠️ CRITICAL**: Blocks all user stories.

- [X] T004 Implement `ExclusionEngine` in `crates/core/src/graph/exclusions.rs` using `globset::GlobSet` for compiled high-speed glob evaluation over workspace paths.
- [X] T005 [P] Extend `crates/core/src/graph/resolver.rs` to validate link targets against `graph.exclude` files, emitting `LinkResolutionStatus::ResolvedExcluded` with `is_broken: false` to uphold the Target Resolvability Invariant.
- [X] T006 Update `crates/core/src/graph/mod.rs` to compute `ObservatorySummary` with distinct `connectedNotes`, `orphanNotes`, and `excludedNotes` metrics, ensuring excluded notes never increment the orphan counter.
- [X] T007 [P] Implement RPC handlers for `graph/previewExclusion`, `graph/excludePath`, `graph/setCentralHub`, and `graph/suggestExclusions` in `crates/core/src/ipc/handlers.rs`.

**Checkpoint**: Core engine handles exclusions, hub updates, and summary metrics; user story implementations can now proceed.

---

## Phase 3: User Story 1 (P0) — Calm Status Dashboard & Zero Action Duplication 🎯 MVP

**Goal**: Redesign the left sidebar into a calm status dashboard answering *"What is important about my knowledge base right now?"* and remove all 5 duplicate action buttons.

**Independent Test**: Load the Webview in VS Code; verify the left sidebar displays Knowledge, AI Context, and Central Hub & Health cards with zero duplicate action buttons (`Open Knowledge Graph`, `Set Central Hub`, `Rescan Vault`, `Fix Broken Links`, `Install Git Hooks` are gone).

### Tests for User Story 1
- [X] T008 [P] [US1] Create unit test in `extension/test/suite/statusDashboard.test.ts` verifying `StatusDashboard` renders correct note/token metrics and omits obsolete action buttons.

### Implementation for User Story 1
- [X] T009 [US1] Create `StatusDashboard` component in `extension/webview-ui/src/components/statusDashboard.ts` rendering the 3 pillars: KNOWLEDGE (notes, links), AI CONTEXT (L0 tokens, reachable tokens, progress bar), and CENTRAL HUB / HEALTH (`[Change]` hub link, `● Synced` status).
- [X] T010 [US1] Streamline `extension/webview-ui/src/components/actionsToolbar.ts` to strictly contain view-manipulation tools: Token status pill (`4.9k active`), Search (`🔍`), Focus mode dropdown (`🎯 Focus: All ▾`), Reset Disposition (`📌`), and More (`⋯`).
- [X] T011 [US1] Remove obsolete buttons and references from `extension/webview-ui/index.html` and clean up `extension/package.json` contributes menus to prevent action duplication in the editor context menu.
- [X] T012 [US1] Implement state-driven attention card in `extension/webview-ui/src/components/statusDashboard.ts` that conditionally renders `⚠ X broken links [Review issues]` opening Settings Drawer broken links tab when broken links $> 0$.
- [X] T013 [US1] Wire `StatusDashboard` into `extension/webview-ui/src/main.ts` updating metrics dynamically on graph delta payloads.
- [X] T014 [US1] Add responsive layout styles in `extension/webview-ui/src/theme.css` for the calm dashboard, status indicators, and streamlined toolbar.

**Checkpoint**: User Story 1 (MVP) complete. The user sees a calm, uncluttered dashboard and toolbar with zero action duplication.

---

## Phase 4: User Story 2 (P1) — Contextual Selection Inspector for Nodes, Edges & Health

**Goal**: Provide a slide-over Contextual Inspector drawer that displays relevant metadata and actionable operations whenever a node, edge, or token status metric is selected.

**Independent Test**: Click a note node, a broken link edge, and the token badge; verify the inspector slides open with line-precise coordinates, token weights, and working `[Open file]`, `[Set as hub]`, and `[Fix reference]` buttons.

### Tests for User Story 2
- [X] T015 [P] [US2] Create unit test in `extension/test/suite/inspector.test.ts` verifying inspector model populates correct tier, token count, in/out links, and token pill context breakdown.

### Implementation for User Story 2
- [X] T016 [US2] Create `InspectorDrawer` component in `extension/webview-ui/src/components/inspectorDrawer.ts` with slide-over container, close button, and dynamic content slots for nodes and edges.
- [X] T017 [US2] Implement node inspection view in `extension/webview-ui/src/components/inspectorDrawer.ts`: display note title, path, context hop tier, exact BPE tokens, incoming/outgoing link lists, and primary actions `[Open file]`, `[Set as hub]`.
- [X] T018 [US2] Implement broken link edge inspection view in `extension/webview-ui/src/components/inspectorDrawer.ts`: display source file, line number, missing destination, error type, and actions `[Open source]` and `[Fix reference]`.
- [X] T019 [US2] Implement Token Economics breakdown view in `extension/webview-ui/src/components/inspectorDrawer.ts` displaying Active Prompt Load (L0) vs Reachable Context (L1+) with budget health alerts, and wire click events from the toolbar token pill (FR-053).
*extension/webview-ui/src/graph/canvas.ts*-*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*[*extension/webview-ui/src/graph/canvas.ts*X*extension/webview-ui/src/graph/canvas.ts*]*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*T*extension/webview-ui/src/graph/canvas.ts*0*extension/webview-ui/src/graph/canvas.ts*2*extension/webview-ui/src/graph/canvas.ts*0*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*[*extension/webview-ui/src/graph/canvas.ts*U*extension/webview-ui/src/graph/canvas.ts*S*extension/webview-ui/src/graph/canvas.ts*2*extension/webview-ui/src/graph/canvas.ts*]*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*W*extension/webview-ui/src/graph/canvas.ts*i*extension/webview-ui/src/graph/canvas.ts*r*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*s*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*l*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*c*extension/webview-ui/src/graph/canvas.ts*t*extension/webview-ui/src/graph/canvas.ts*i*extension/webview-ui/src/graph/canvas.ts*o*extension/webview-ui/src/graph/canvas.ts*n*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*v*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*n*extension/webview-ui/src/graph/canvas.ts*t*extension/webview-ui/src/graph/canvas.ts*s*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*i*extension/webview-ui/src/graph/canvas.ts*n*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*`*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*x*extension/webview-ui/src/graph/canvas.ts*t*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*n*extension/webview-ui/src/graph/canvas.ts*s*extension/webview-ui/src/graph/canvas.ts*i*extension/webview-ui/src/graph/canvas.ts*o*extension/webview-ui/src/graph/canvas.ts*n*extension/webview-ui/src/graph/canvas.ts*/*extension/webview-ui/src/graph/canvas.ts*w*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*b*extension/webview-ui/src/graph/canvas.ts*v*extension/webview-ui/src/graph/canvas.ts*i*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*w*extension/webview-ui/src/graph/canvas.ts*-*extension/webview-ui/src/graph/canvas.ts*u*extension/webview-ui/src/graph/canvas.ts*i*extension/webview-ui/src/graph/canvas.ts*/*extension/webview-ui/src/graph/canvas.ts*s*extension/webview-ui/src/graph/canvas.ts*r*extension/webview-ui/src/graph/canvas.ts*c*extension/webview-ui/src/graph/canvas.ts*/*extension/webview-ui/src/graph/canvas.ts*g*extension/webview-ui/src/graph/canvas.ts*r*extension/webview-ui/src/graph/canvas.ts*a*extension/webview-ui/src/graph/canvas.ts*p*extension/webview-ui/src/graph/canvas.ts*h*extension/webview-ui/src/graph/canvas.ts*/*extension/webview-ui/src/graph/canvas.ts*c*extension/webview-ui/src/graph/canvas.ts*a*extension/webview-ui/src/graph/canvas.ts*n*extension/webview-ui/src/graph/canvas.ts*v*extension/webview-ui/src/graph/canvas.ts*a*extension/webview-ui/src/graph/canvas.ts*s*extension/webview-ui/src/graph/canvas.ts*.*extension/webview-ui/src/graph/canvas.ts*t*extension/webview-ui/src/graph/canvas.ts*s*extension/webview-ui/src/graph/canvas.ts*`*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*o*extension/webview-ui/src/graph/canvas.ts*n*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*n*extension/webview-ui/src/graph/canvas.ts*o*extension/webview-ui/src/graph/canvas.ts*d*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*c*extension/webview-ui/src/graph/canvas.ts*l*extension/webview-ui/src/graph/canvas.ts*i*extension/webview-ui/src/graph/canvas.ts*c*extension/webview-ui/src/graph/canvas.ts*k*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*a*extension/webview-ui/src/graph/canvas.ts*n*extension/webview-ui/src/graph/canvas.ts*d*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*d*extension/webview-ui/src/graph/canvas.ts*g*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*c*extension/webview-ui/src/graph/canvas.ts*l*extension/webview-ui/src/graph/canvas.ts*i*extension/webview-ui/src/graph/canvas.ts*c*extension/webview-ui/src/graph/canvas.ts*k*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*t*extension/webview-ui/src/graph/canvas.ts*o*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*d*extension/webview-ui/src/graph/canvas.ts*i*extension/webview-ui/src/graph/canvas.ts*s*extension/webview-ui/src/graph/canvas.ts*p*extension/webview-ui/src/graph/canvas.ts*a*extension/webview-ui/src/graph/canvas.ts*t*extension/webview-ui/src/graph/canvas.ts*c*extension/webview-ui/src/graph/canvas.ts*h*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*s*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*l*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*c*extension/webview-ui/src/graph/canvas.ts*t*extension/webview-ui/src/graph/canvas.ts*i*extension/webview-ui/src/graph/canvas.ts*o*extension/webview-ui/src/graph/canvas.ts*n*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*p*extension/webview-ui/src/graph/canvas.ts*a*extension/webview-ui/src/graph/canvas.ts*y*extension/webview-ui/src/graph/canvas.ts*l*extension/webview-ui/src/graph/canvas.ts*o*extension/webview-ui/src/graph/canvas.ts*a*extension/webview-ui/src/graph/canvas.ts*d*extension/webview-ui/src/graph/canvas.ts*s*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*t*extension/webview-ui/src/graph/canvas.ts*o*extension/webview-ui/src/graph/canvas.ts* *extension/webview-ui/src/graph/canvas.ts*`*extension/webview-ui/src/graph/canvas.ts*I*extension/webview-ui/src/graph/canvas.ts*n*extension/webview-ui/src/graph/canvas.ts*s*extension/webview-ui/src/graph/canvas.ts*p*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*c*extension/webview-ui/src/graph/canvas.ts*t*extension/webview-ui/src/graph/canvas.ts*o*extension/webview-ui/src/graph/canvas.ts*r*extension/webview-ui/src/graph/canvas.ts*D*extension/webview-ui/src/graph/canvas.ts*r*extension/webview-ui/src/graph/canvas.ts*a*extension/webview-ui/src/graph/canvas.ts*w*extension/webview-ui/src/graph/canvas.ts*e*extension/webview-ui/src/graph/canvas.ts*r*extension/webview-ui/src/graph/canvas.ts*`*extension/webview-ui/src/graph/canvas.ts*.*extension/webview-ui/src/graph/canvas.ts*
*extension/webview-ui/src/graph/canvas.ts**extension/src/webview/rpc.ts*-*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*[*extension/src/webview/rpc.ts*X*extension/src/webview/rpc.ts*]*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*T*extension/src/webview/rpc.ts*0*extension/src/webview/rpc.ts*2*extension/src/webview/rpc.ts*1*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*[*extension/src/webview/rpc.ts*U*extension/src/webview/rpc.ts*S*extension/src/webview/rpc.ts*2*extension/src/webview/rpc.ts*]*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*W*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*r*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts*[*extension/src/webview/rpc.ts*O*extension/src/webview/rpc.ts*p*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*f*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*l*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*]*extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*a*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*d*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts*[*extension/src/webview/rpc.ts*O*extension/src/webview/rpc.ts*p*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*u*extension/src/webview/rpc.ts*r*extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*]*extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*a*extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*h*extension/src/webview/rpc.ts*a*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*d*extension/src/webview/rpc.ts*l*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*r*extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*x*extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*/*extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts*r*extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*/*extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*b*extension/src/webview/rpc.ts*v*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts*/*extension/src/webview/rpc.ts*r*extension/src/webview/rpc.ts*p*extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*.*extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*a*extension/src/webview/rpc.ts*l*extension/src/webview/rpc.ts*l*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts*v*extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*d*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*.*extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*d*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts*.*extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts*h*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts*T*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*x*extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts*D*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*u*extension/src/webview/rpc.ts*m*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts*h*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts*V*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts*C*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*l*extension/src/webview/rpc.ts*u*extension/src/webview/rpc.ts*m*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*.*extension/src/webview/rpc.ts*B*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*d*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts*,*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*a*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*d*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*r*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts*[*extension/src/webview/rpc.ts*F*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*x*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*r*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*f*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*r*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*]*extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts*h*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts*v*extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*d*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*.*extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*d*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts*.*extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts*h*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts*Q*extension/src/webview/rpc.ts*u*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*k*extension/src/webview/rpc.ts*P*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*k*extension/src/webview/rpc.ts*`*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*a*extension/src/webview/rpc.ts*l*extension/src/webview/rpc.ts*l*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*w*extension/src/webview/rpc.ts*i*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*g*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*u*extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*r*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*s*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*l*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*a*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*r*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*p*extension/src/webview/rpc.ts*l*extension/src/webview/rpc.ts*a*extension/src/webview/rpc.ts*c*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*m*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*n*extension/src/webview/rpc.ts*o*extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts*a*extension/src/webview/rpc.ts*r*extension/src/webview/rpc.ts*g*extension/src/webview/rpc.ts*e*extension/src/webview/rpc.ts*t*extension/src/webview/rpc.ts* *extension/src/webview/rpc.ts*(*extension/src/webview/rpc.ts*F*extension/src/webview/rpc.ts*R*extension/src/webview/rpc.ts*-*extension/src/webview/rpc.ts*0*extension/src/webview/rpc.ts*5*extension/src/webview/rpc.ts*2*extension/src/webview/rpc.ts*)*extension/src/webview/rpc.ts*.*extension/src/webview/rpc.ts*
*extension/src/webview/rpc.ts*- [X] T022 [US2] Wire `[Set as hub]` action handler in *extension/src/webview/rpc.ts* and `crates/core/src/config/mod.rs` to persist `hub.primary_hub` into `.context-vault.yaml` and recalculate BFS hops (FR-051).
- [X] T023 [US2] Add CSS styles and slide animations in `extension/webview-ui/src/theme.css` for `.inspector-drawer`, token breakdown bars, and action button groups.

**Checkpoint**: User Story 2 complete. The UI adapts dynamically to user selection with zero static clutter.

---

## Phase 5: User Story 3 (P1) — Noise Management & First-Class Graph Exclusions

**Goal**: Enable developers to exclude noisy documentation (e.g. package `README.md` files) via right-click or contextual menus, with preview matching and automated noise suggestions.

**Independent Test**: Open exclusion preview for `packages/**/README.md`; verify matching file count; apply exclusion and verify matching notes disappear from the canvas and orphan count without causing any broken links.

### Tests for User Story 3
- [X] T024 [P] [US3] Create unit test in `extension/test/suite/exclusions.test.ts` verifying exclusion preview counts, path exclusions, and Target Resolvability Invariant.
- [X] T025 [P] [US3] Create Rust unit test in `crates/core/tests/exclusions_test.rs` validating glob pattern matching and noise clustering algorithms.

### Implementation for User Story 3
- [X] T026 [US3] Create `ExclusionModal` component in `extension/webview-ui/src/components/exclusionModal.ts` displaying evaluated glob pattern, affected file count, sample path chips, and `[Cancel]` / `[Apply Exclusion]` buttons.
- [X] T027 [US3] Add contextual exclusion actions (`Exclude this file`, `Exclude this folder`, `Exclude pattern...`) to `InspectorDrawer` more menu and canvas context menu.
- [X] T028 [US3] Wire exclusion preview request `graph/previewExclusion` through `extension/src/ipc/client.ts` to `ExclusionModal`.
- [X] T029 [US3] Wire `graph/excludePath` execution in `crates/core/src/config/mod.rs` to persist new rules into `.context-vault.yaml` and trigger incremental graph broadcast.
- [X] T030 [US3] Implement automated noise detection in `crates/core/src/graph/exclusions.rs` clustering disconnected orphan files by stem (`README.md`) or directory (`vendor/**`).
- [X] T031 [US3] Add `Suggested Exclusions` notification card in `StatusDashboard` with one-click `[Review suggestions]` batch approval workflow.
- [X] T032 [US3] Implement optional `Show excluded` diagnostic toggle in Filter popover, rendering excluded nodes as faint, ghosted dotted outlines.

**Checkpoint**: User Story 3 complete. Monorepo orphan noise is reduced by $\ge 70\%$ while link integrity remains 100% preserved.

---

## Phase 6: User Story 4 (P1) — Hop-Constrained Focus Modes & Semantic Visual Sizing

**Goal**: Allow developers to restrict visualization to `Hub`, `1 hop`, `2 hops`, or `All`, and render node sizes according to their context tier.

**Independent Test**: Cycle focus modes on a 500+ note graph; verify canvas filters nodes beyond the hop horizon within 16 ms and re-heats simulation smoothly; verify node sizes follow semantic tier hierarchy ($r=14, 9, 6, 4$).

### Tests for User Story 4
- [X] T033 [P] [US4] Create unit test in `extension/test/suite/focusEngine.test.ts` verifying hop-distance subgraph filtering and orphan toggling.

### Implementation for User Story 4
- [X] T034 [US4] Create `FocusEngine` module in `extension/webview-ui/src/graph/focusEngine.ts` filtering node and edge arrays by `focusMode` (`hub`, `1_hop`, `2_hops`, `all`) and `showOrphans` boolean.
- [X] T035 [US4] Add `Focus Mode` selector dropdown (`🎯 Focus: All ▾`) to the floating toolbar in `extension/webview-ui/src/components/actionsToolbar.ts`.
- [X] T036 [US4] Update *extension/webview-ui/src/graph/canvas.ts* to calculate semantic node radius based on `node.hopDistance` ($r_0=14\text{px}, r_1=9\text{px}, r_2=6\text{px}, r_{\ge 3}=4\text{px}, r_{\text{orphan}}=4\text{px}$) and enforce the Opacity Decay Law ($\alpha_0=1.0, \alpha_1=0.6, \alpha_2=0.3, \alpha_{\ge 3}=0.15, \alpha_{\text{orphan}}=0.40$) composed with cluster hues (FR-064, Constitution Principle IV).
- [X] T037 [US4] Update `extension/webview-ui/src/graph/physics.ts` to re-simulate filtered subgraphs with gentle reheat ($\alpha=0.3$) when focus mode changes.
- [X] T038 [US4] Add `Show orphans` checkbox toggle in Filter popover in `extension/webview-ui/src/components/actionsToolbar.ts` to toggle unlinked node visibility.
- [X] T039 [US4] Style focus dropdown and semantic labels in `extension/webview-ui/src/theme.css`.

**Checkpoint**: User Story 4 complete. Graph visualization is calm, focused, and semantically sized.

---

## Phase 7: User Story 5 (P2) — Advanced Settings Drawer & Progressive Disclosure

**Goal**: House administrative tasks (Git hooks, hub rules, exclusion paths manager, cluster color groups, and diagnostics) in an Advanced Settings slide-over drawer accessed via `⋯`.

**Independent Test**: Click `⋯ More → Settings`; verify slide-over drawer opens with sections for Hub, Excluded paths, Git synchronization (`prek`), and Diagnostics, with working rule management.

### Implementation for User Story 5
- [X] T040 [US5] Create `SettingsDrawer` component in `extension/webview-ui/src/components/settingsDrawer.ts` with tabbed navigation: `Knowledge`, `Excluded Paths`, `Cluster Groups`, `Git Sync`, and `Diagnostics` (FR-066).
- [X] T041 [US5] Implement `Excluded Paths Manager` in `SettingsDrawer` with rule list, match counts, test preview input, and `[+ Add]` / `[Delete]` buttons.
- [X] T042 [US5] Move `Git synchronization` controls (prek hook status, latency, reinstall button) from old sidebar into `SettingsDrawer` under `Git Sync`.
- [X] T043 [US5] Move `Cluster Groups Manager` into `SettingsDrawer` under `Cluster Groups`.
- [X] T044 [US5] Wire `⋯ More` menu item `Settings` in `extension/webview-ui/src/components/actionsToolbar.ts` to open `SettingsDrawer`.
- [X] T045 [US5] Add CSS styles for tabbed settings drawer and forms in `extension/webview-ui/src/theme.css`.

**Checkpoint**: User Story 5 complete. Full administrative power is preserved without cluttering daily workflows.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: End-to-end verification, performance optimization, and packaging.

- [X] T046 [P] Run `cargo clippy -- -D warnings` and `cargo fmt --check` across `crates/core/`.
- [X] T047 [P] Run `npm run lint` and `npm test` across `extension/`.
- [X] T048 Benchmark glob exclusion evaluation in Criterion to guarantee $\le 10\text{ ms}$ for 5,000 files.
- [X] T049 Execute all 4 runnable validation scenarios in `specs/002-context-observatory-ux/quickstart.md`.
- [X] T050 Re-package extension with *./scripts/package-all.sh* and install verified VSIX in VS Code.

---

## Dependencies & Execution Order

```mermaid
graph TD
    Phase1["Phase 1: Setup (T001-T003)"] --> Phase2["Phase 2: Foundational (T004-T007)"]
    Phase2 --> US1["Phase 3: US1 - Calm Dashboard (T008-T014) [MVP]"]
    Phase2 --> US2["Phase 4: US2 - Contextual Inspector (T015-T023)"]
    Phase2 --> US3["Phase 5: US3 - Noise Management (T024-T032)"]
    Phase2 --> US4["Phase 6: US4 - Focus Modes (T033-T039)"]
    US1 & US2 & US3 & US4 --> US5["Phase 7: US5 - Settings Drawer (T040-T045)"]
    US5 --> Polish["Phase 8: Polish & Packaging (T046-T050)"]
```

### Parallel Opportunities:
- **Phase 1**: T002 and T003 can execute in parallel.
- **Phase 2**: T004, T005, and T007 can execute in parallel.
- **User Stories (Phase 3-6)**: Once Foundational (Phase 2) is complete, US1, US2, US3, and US4 can proceed in parallel.
- **Shared File Coordination Note**: When implementing US1, US4, and US5 concurrently, coordinate edits to `extension/webview-ui/src/components/actionsToolbar.ts` and `extension/webview-ui/src/theme.css` to avoid merge conflicts.
- **Tests**: T008, T015, T024, T025, and T033 can all be written concurrently.

---

## Implementation Strategy (MVP First)

1. **Step 1 (MVP)**: Implement Phase 1, Phase 2, and Phase 3 (US1). Stop and validate the clean dashboard in VS Code. All duplicate buttons are gone.
2. **Step 2 (Interactivity)**: Implement Phase 4 (US2 - Contextual Inspector). Clicking nodes, broken edges, and the token pill reveals rich metadata and line navigation.
3. **Step 3 (Signal over Noise)**: Implement Phase 5 (US3 - Curated Exclusions) and Phase 6 (US4 - Focus Modes). Clutter is eliminated; large repositories become calm.
4. **Step 4 (Polish & Settings)**: Implement Phase 7 (US5 - Settings Drawer) and Phase 8 (Benchmarks & VSIX packaging).
