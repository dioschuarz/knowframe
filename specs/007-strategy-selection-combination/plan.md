---
wiki:
  content_hash: d1b8dfa710c893ffdf2951afa53281da3dc90e1716230fb428dfb2f700cf6096
  fragments:
    implementation-plan-strategy-selection-combination:
      tokens: 2098
      inbound: 0
      outbound: 1
    summary:
      tokens: 250
      inbound: 0
      outbound: 0
    technical-context:
      tokens: 456
      inbound: 0
      outbound: 0
    constitution-check:
      tokens: 609
      inbound: 0
      outbound: 0
    project-structure:
      tokens: 680
      inbound: 0
      outbound: 0
    documentation-this-feature:
      tokens: 173
      inbound: 0
      outbound: 0
    source-code-repository-root:
      tokens: 503
      inbound: 0
      outbound: 0
    complexity-tracking:
      tokens: 31
      inbound: 0
      outbound: 0
---
# Implementation Plan: Strategy Selection & Combination

**Branch**: `007-strategy-selection-combination` | **Date**: 2026-09-23 | **Spec**: [`specs/007-strategy-selection-combination/spec.md`](spec.md)

**Input**: Feature specification from `/specs/007-strategy-selection-combination/spec.md`

---

## Summary

This feature extends the structural evolution subsystem from a single-strategy recommendation model into a cohesive, multi-strategy selection and combination engine. It implements:
1. Deterministic Fit Score calculation ($0\text{--}100$) for 4 candidate strategies (`section_addressing`, `faceted_classification`, `lightweight_index`, `preserve_document`), with a strict eligibility gate ($\text{fit} \ge 60$).
2. Pairwise strategy relationship classification (`COMPLEMENTARY`, `ALTERNATIVE`, `MUTUALLY_EXCLUSIVE`), strictly enforcing that `preserve_document` represents zero transformation and cannot combine with transformation strategies.
3. 9-step Strategy Selection Algorithm generating a structured `StrategySet` cleanly partitioned into `PRIMARY` (top-fit), `SUPPORTING` (complementary high-fit), and `ALTERNATIVES` (competing options).
4. Composite Plan Value model quantifying the net architectural utility of any active strategy combination, accounting for individual fit, synergy bonuses, overlap penalties, and transformation effort.
5. Interactive UI selection in the Management & Settings drawer with checkboxes (`☑` / `☐`) and real-time incompatibility guards preventing contradictory transformations.

---

## Technical Context

**Language/Version**: Rust 1.75+ (2021 edition) for core computation engine; Node.js 20+ and TypeScript 5.4+ for VS Code Extension Host and Webview.

**Primary Dependencies**:
- Rust: `serde`, `serde_json`, `petgraph`, `tiktoken-rs` (`cl100k_base`), `tokio`, `criterion` (benchmarks).
- TypeScript: VS Code Extension API (`^1.90.0`), `@vscode/webview-ui-toolkit`.

**Storage**: In-memory ephemeral candidate graph, Markdown vault filesystem, `.context-vault.yaml` for configuration. Zero external database.

**Testing**:
- Rust: `cargo test` (unit, integration, and Criterion microbenchmarks).
- TypeScript: `npm test` (Mocha test suites via `@vscode/test-electron`).

**Target Platform**: Linux (x86_64, aarch64), macOS (x86_64, aarch64 Apple Silicon), Windows (x86_64).

**Project Type**: Desktop IDE extension (VS Code) with background Rust daemon sidecar.

**Performance Goals**:
- Candidate strategy evaluation, relationship classification, and `StrategySet` synthesis: $\le 10\text{ ms}$ per detected problem (SC-006).
- Composite Plan Value calculation: $\le 1\text{ ms}$.
- Interactive UI checkbox toggle & incompatibility guard response: $< 16.6\text{ ms}$ (60 FPS rendering).

**Constraints**:
- Zero external AI APIs, cloud LLMs, or remote embeddings (Invariants III, IV).
- Raw numeric integer integrity for fit scores and plan values ($0\text{--}100$), formatted strictly at the presentation layer (Invariants IX, X).
- Non-prescriptive organization: `preserve_document` is a first-class strategy and default fallback (Invariant XIII).
- User-controlled downstream automation: explicit checkbox selection and approval gate required before transformation (Invariant XVI).

**Scale/Scope**: Vaults up to 5,000 Markdown documents and 50 structural opportunities; 4 core candidate strategies evaluated across 10 structural evidence dimensions.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Invariant | Status | Verification in Design |
| :--- | :--- | :--- |
| **Invariant I — Committed Truth** | **PASS** | Multi-strategy evaluation runs against committed vault state; working tree changes only evaluated in candidate graphs. |
| **Invariant II — Candidate Isolation** | **PASS** | Plan generation produces preview diffs on ephemeral models without mutating persistent state. |
| **Invariant III — Determinism** | **PASS** | Pure algebraic calculation for Fit Scores ($0\text{--}100$) and Total Plan Value without stochastic elements or unseeded heuristics. |
| **Invariant IV — Locality** | **PASS** | 100% local computation inside Rust sidecar. Zero network calls or external AI services. |
| **Invariant V — Document-Centric Graph** | **PASS** | Strategies operate at document or section metadata level. Fragments are never created as standalone graph nodes. |
| **Invariant VI — Section Addressability** | **PASS** | `section_addressing` generates normalized Markdown heading anchors (`#heading-anchor`). |
| **Invariant VII — AST Authority** | **PASS** | All heading fragments and document anchors derive strictly from `pulldown-cmark` Markdown AST. |
| **Invariant VIII — Materialized Metadata** | **PASS** | Materialized `wiki.fragments` frontmatter updated during transformation execution. |
| **Invariant IX — Raw Numeric Integrity** | **PASS** | Fit scores ($0\text{--}100$) and plan values are modeled as complete raw integers in Rust/TS structs and RPC contracts. |
| **Invariant X — Presentation Separation** | **PASS** | Percentage signs and badges are applied solely in Webview HTML/CSS rendering. |
| **Invariant XI — Brownfield Tolerance** | **PASS** | Structural debts are baselined; recommendations guide intentional evolution without blocking unrelated commits. |
| **Invariant XII — Quality Gate** | **PASS** | Transformation plans validate that link integrity is maintained after restructuring. |
| **Invariant XIII — Non-Prescriptive Organization** | **PASS** | `preserve_document` is fully supported as an equal candidate and serves as the automatic fallback when $\text{fit} < 60$. |
| **Invariant XIV — Unified Optimization Profile** | **PASS** | Fit scores are calculated against the active `OptimizationProfile` parameters. |
| **Invariant XV — Transparent Trade-Offs & Evidence** | **PASS** | Every recommendation explicitly itemizes expected benefits, countervailing trade-offs, synergy bonuses, and effort costs. |
| **Invariant XVI — User-Controlled Downstream Automation** | **PASS** | Interactive checkboxes give users granular control over active strategies; requires explicit approval before applying changes. |

*Verdict: 16/16 Invariants Passed. Zero violations.*

---

## Project Structure

### Documentation (this feature)

```text
specs/007-strategy-selection-combination/
├── spec.md              # Feature specification
├── plan.md              # This file (implementation plan)
├── research.md          # Phase 0 output (architectural research & decisions)
├── data-model.md        # Phase 1 output (entities, enums, state machine)
├── quickstart.md        # Phase 1 output (runnable validation scenarios)
├── contracts/           # Phase 1 output (JSON-RPC protocol schema)
│   └── strategy_selection_rpc_protocol.json
├── checklists/          # Requirements validation checklist
│   └── requirements.md
└── tasks.md             # Phase 2 output (to be generated by /speckit-tasks)
```

### Source Code (repository root)

```text
crates/core/
├── src/
│   ├── evolution/
│   │   ├── mod.rs                  # Module exports
│   │   ├── strategies.rs           # Candidate generation, Fit Score (0-100), pairwise relationships, 9-step selection
│   │   ├── opportunity.rs          # StructuralOpportunity integration with StrategySet
│   │   ├── plan_value.rs           # Composite Plan Value valuation engine (synergy, overlap, effort)
│   │   ├── transformation.rs       # Multi-strategy atomic transformation plan generator
│   │   └── summary.rs              # Opportunity summary reporting
│   └── ipc/
│       └── handlers.rs             # RPC handlers: evolution/getStrategySet, calculatePlanValue, validateStrategySelection, generatePlan
└── tests/
    ├── strategy_combination_test.rs # Integration test suite for multi-strategy selection & guards
    └── plan_value_test.rs          # Test suite for composite plan valuation model

extension/
├── src/
│   ├── types/
│   │   └── evolution.ts            # TypeScript interfaces for StrategySet, StrategyRole, CompositePlanValue
│   ├── evolution/
│   │   └── evolutionManager.ts     # Extension lifecycle & IPC coordinator
│   └── webview/
│       └── panelManager.ts         # IPC message dispatcher to webview
├── webview-ui/
│   ├── src/
│   │   ├── components/
│   │   │   └── evolutionTab.ts     # UI rendering for PRIMARY, SUPPORTING, ALTERNATIVES, checkboxes, incompatibility guards
│   │   └── rpc/
│   │       └── webviewRpc.ts       # Typed RPC client for strategy selection
│   └── public/
│       └── theme.css               # CSS styling for role badges, synergy pill, conflict warning banner
└── test/
    └── suite/
        └── strategySelection.test.ts # Webview & extension unit test suite
```

**Structure Decision**: Monorepo split between Rust sidecar (`crates/core`) and VS Code extension (`extension/` and `extension/webview-ui/`). Core analytical algorithms execute in Rust for microsecond performance and deterministic safety, while interactive checkboxes and real-time incompatibility guards live in the TypeScript Webview UI.

---

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

*No constitutional violations; all 16 invariants strictly upheld.*
