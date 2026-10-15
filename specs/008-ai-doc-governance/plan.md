---
wiki:
  content_hash: d6c5e7f89e9e3a206c239eedd076b16c222dc11903091e474e85e8985c6d6601
  fragments:
    implementation-plan-ai-oriented-documentation-governance:
      tokens: 2544
      inbound: 0
      outbound: 1
    summary:
      tokens: 350
      inbound: 0
      outbound: 0
    technical-context:
      tokens: 481
      inbound: 0
      outbound: 0
    constitution-check:
      tokens: 616
      inbound: 0
      outbound: 0
    project-structure:
      tokens: 992
      inbound: 0
      outbound: 0
    documentation-this-feature:
      tokens: 175
      inbound: 0
      outbound: 0
    source-code-repository-root:
      tokens: 813
      inbound: 0
      outbound: 0
    complexity-tracking:
      tokens: 31
      inbound: 0
      outbound: 0
---
# Implementation Plan: AI-Oriented Documentation Governance

**Branch**: `008-ai-doc-governance` | **Date**: 2026-09-29 | **Spec**: [`specs/008-ai-doc-governance/spec.md`](spec.md)

**Input**: Feature specification from `/specs/008-ai-doc-governance/spec.md`

---

## Summary

This feature establishes an authoritative, deterministic documentation governance engine for `AI-Oriented` mode in LLM Wiki. It treats the Harness Entry Point, embedded Knowledge Index, Skills, procedures, Markdown documents, and directory structure as an integrated knowledge architecture. Key capabilities include:

1. **Deterministic Harness Entry Point Discovery**: Identifies configured or standard AI agent entry points (`AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.github/copilot-instructions.md`, or custom paths) and models them as first-class Knowledge Graph objects.
2. **Direct Embedded Knowledge Index & Anti-Secondary Layer Invariant**: Enforces that the Knowledge Index is embedded directly within the harness entry point, detecting and flagging intermediate routing hops (`memory.md`, `hot-index.md`, `docs/index.md`) as structural defects.
3. **Skill Governance & Procedure vs. Capability Distinction**: Validates reusable agent Skills (`SKILL.md`, `scripts/`, `templates/`, `references/`) and detects candidates for new Skills from repeated procedural steps across documents.
4. **Non-Prescriptive Directory Structure Analytics**: Evaluates physical repository layout (depth, concentration entropy, domain mixing, cross-folder coupling) without imposing dogmatic folder taxonomies.
5. **Harness-Anchored Context Trace & 4-Capability Quality Model**: Originates search traces strictly from the Harness Entry Point, measuring Entry, Discovery, Retrieval, and Reuse capabilities locally without external LLM calls.
6. **User-Controlled Structural Evolution**: Recommends evidence-based structural transformations with explicit trade-off matrices, enforced approval gates (`Auto-Apply: OFF`, `Require Approval: ON`), and Before-vs-After re-measurement.

---

## Technical Context

**Language/Version**: Rust 1.75+ (2021/2024 edition) for core sidecar engine; Node.js 20+ and TypeScript 5.4+ for VS Code Extension Host and Webview.

**Primary Dependencies**:
- Rust: `pulldown-cmark` (Markdown AST parser), `petgraph` (graph topologies), `tiktoken-rs` (`cl100k_base` BPE counter), `serde`, `serde_json`, `serde_yaml`, `tokio`, `criterion` (benchmarks).
- TypeScript: VS Code Extension API (`^1.90.0`), `@vscode/webview-ui-toolkit`.

**Storage**: In-memory ephemeral candidate graph, Markdown vault filesystem, `.context-vault.yaml` for configuration. Zero external database.

**Testing**:
- Rust: `cargo test` (unit, integration, and Criterion microbenchmarks).
- TypeScript: `npm test` (Mocha test suites via `@vscode/test-electron`).

**Target Platform**: Linux (x86_64, aarch64), macOS (x86_64, aarch64 Apple Silicon), Windows (x86_64).

**Project Type**: Desktop IDE extension (VS Code) with background Rust daemon sidecar.

**Performance Goals**:
- Harness detection & embedded index parsing: $\le 15\text{ ms}$.
- Context Trace anchored to harness: $\le 50\text{ ms}$ for query beam search (SC-006).
- Full vault governance audit (harness, skills, directories): $\le 100\text{ ms}$ for 1,000 notes.
- Webview UI rendering: 60 FPS during inspection and preview.

**Constraints**:
- Zero external AI APIs, cloud LLMs, or remote embeddings (Invariants III, IV).
- Raw numeric integer integrity for capability scores ($0\text{--}100$) (Invariants IX, X).
- Non-prescriptive organization: size is a signal, not a verdict (Invariant XIII).
- User-controlled downstream automation: explicit approval gate required before applying any structural transformation (Invariant XVI).

**Scale/Scope**: Vaults up to 5,000 Markdown documents, multiple concurrent harness entry points, up to 100 skills, and deep directory trees.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Invariant | Status | Verification in Design |
| :--- | :--- | :--- |
| **Invariant I — Committed Truth** | **PASS** | Governance evaluations run against committed repository state; working-tree changes evaluated only in ephemeral candidate graphs. |
| **Invariant II — Candidate Isolation** | **PASS** | Staged harness changes and transformation plan previews execute in memory without mutating persistent state. |
| **Invariant III — Determinism** | **PASS** | 100% deterministic local path resolution, AST parsing, and metric calculations. Zero randomness. |
| **Invariant IV — Locality** | **PASS** | All analysis, BPE token counting, and Context Trace executions run locally in Rust core. Zero network or external AI APIs. |
| **Invariant V — Document-Centric Graph** | **PASS** | Harness entry points and secondary layers are document nodes. Index routes and sections are addressable metadata attached to document nodes. |
| **Invariant VI — Section Addressability** | **PASS** | Index routes target normalized Markdown heading anchors (`#heading-anchor`). |
| **Invariant VII — AST Authority** | **PASS** | All routes, section headings, and code references derived strictly from `pulldown-cmark` Markdown AST. |
| **Invariant VIII — Materialized Metadata** | **PASS** | Frontmatter metadata updated during transformation execution. |
| **Invariant IX — Raw Numeric Integrity** | **PASS** | Capability scores ($0\text{--}100$) and token counts modeled as raw floats/integers in Rust/TS structs and RPC contracts. |
| **Invariant X — Presentation Separation** | **PASS** | Percentages and compact numbers formatted strictly at presentation layer in UI. |
| **Invariant XI — Brownfield Tolerance** | **PASS** | Pre-existing missing routes or unindexed files are reported as structural debt without blocking unrelated commits. |
| **Invariant XII — Quality Gate** | **PASS** | Pre-commit validation blocks high-confidence regressions (e.g. broken index routes introduced in staged commit). |
| **Invariant XIII — Non-Prescriptive Organization** | **PASS** | Monolithic entry points ($>30\text{k}$ tokens) are NOT penalized if structurally cohesive. No universal directory schema imposed. |
| **Invariant XIV — Unified Optimization Profile** | **PASS** | Governance evaluations compile into the active `OptimizationProfile` struct. |
| **Invariant XV — Transparent Trade-Offs & Evidence** | **PASS** | Every transformation recommendation itemizes expected benefits and countervailing trade-offs explicitly. |
| **Invariant XVI — User-Controlled Downstream Automation** | **PASS** | All transformations require user approval (`Auto-Apply: OFF`, `Require Approval: ON`). Re-measurement verifies Before-vs-After deltas. |

*Verdict: 16/16 Invariants Passed. Zero violations.*

---

## Project Structure

### Documentation (this feature)

```text
specs/008-ai-doc-governance/
├── spec.md              # Feature specification
├── plan.md              # This file (implementation plan)
├── research.md          # Phase 0 output (architectural research & decisions)
├── data-model.md        # Phase 1 output (entities, enums, state machine)
├── quickstart.md        # Phase 1 output (runnable validation scenarios)
├── contracts/           # Phase 1 output (JSON-RPC protocol schema)
│   └── ai_governance_rpc_protocol.json
├── checklists/          # Requirements validation checklist
│   └── requirements.md
└── tasks.md             # Phase 2 output (to be generated by /speckit-tasks)
```

### Source Code (repository root)

```text
crates/core/
├── src/
│   ├── harness/
│   │   ├── mod.rs                  # Module declarations, public types, and GovernanceStrategyType enum
│   │   ├── discovery.rs            # Deterministic multi-harness detector & template AGENTS.md generator
│   │   ├── index_parser.rs         # Embedded Knowledge Index AST parser & route resolution via ResolverIndex
│   │   ├── secondary_layer.rs      # Intermediate router anti-pattern detector (memory.md, hot-index.md)
│   │   ├── skill.rs                # Skill structure validator (SKILL.md, scripts, templates, overlap)
│   │   ├── procedure.rs            # Procedural step sequence signature extractor & candidate generator
│   │   ├── directory_metrics.rs    # Non-prescriptive directory depth, concentration entropy & coupling
│   │   └── matrix.rs               # 4-capability matrix calculation (Entry, Discovery, Retrieval, Reuse)
│   ├── quality/
│   │   └── scorer.rs               # Integrates harness governance findings into 5-pillar scoring
│   ├── trace/
│   │   └── traversal.rs            # Harness-anchored Context Trace root locking
│   ├── evolution/
│   │   ├── profile.rs              # Compiles AI-Oriented governance weights into OptimizationProfile (Invariant XIV)
│   │   ├── strategies.rs           # All 7 governance strategies (EmbedKnowledgeIndex, EliminateSecondaryRoutingLayer, CreateSkillFromProcedure, ConsolidateDuplicateSkills, DecomposeHarness, IntroduceSectionAddressing, RestructureDirectoryDomain)
│   │   └── transformation.rs       # Atomic transformation plan generation with approval_token validation
│   └── ipc/
│       └── handlers.rs             # RPC handlers: governance/getHarnessStatus, setActiveHarness, getSkillCatalog, analyzeDirectoryStructure, traceFromHarness, generateGovernancePlan, applyGovernancePlan
└── tests/
    ├── harness_governance_test.rs  # Test suite for harness detection, embedded index, and secondary layers
    ├── skill_governance_test.rs    # Test suite for skill validation and procedure candidacy
    ├── context_trace_harness_test.rs # Test suite for harness-anchored Context Trace
    └── directory_metrics_test.rs   # Test suite for non-prescriptive directory analytics

extension/
├── src/
│   ├── types/
│   │   └── governance.ts           # TypeScript interfaces for HarnessEntryPoint, SkillDefinition, CapabilityMatrix
│   ├── governance/
│   │   └── governanceManager.ts    # Extension lifecycle & IPC coordinator
│   └── webview/
│       └── panelManager.ts         # IPC message dispatcher to webview
├── webview-ui/
│   ├── src/
│   │   ├── components/
│   │   │   ├── settingsDrawer.ts   # Governance diagnostic views, harness health badges, skill catalog
│   │   │   └── evolutionTab.ts     # Governance structural transformation proposals
│   │   └── rpc/
│   │       └── webviewRpc.ts       # Typed RPC client for governance methods
│   └── src/
│       └── theme.css               # Styling for harness health indicators, skill cards, secondary layer alerts
└── test/
    └── suite/
        └── governance.test.ts      # Webview & extension unit test suite
```

**Structure Decision**: Monorepo split between Rust sidecar (`crates/core`) and VS Code extension (`extension/` and `extension/webview-ui/`). Heavy AST analysis, multi-harness discovery, route validation, procedure extraction, and Context Trace execution are implemented exclusively in Rust for microsecond performance and local determinism. The TypeScript layer coordinates IPC and renders interactive diagnostic cards, skill catalogs, and user approval gates in the UI.

---

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

*No constitutional violations; all 16 invariants strictly upheld.*
