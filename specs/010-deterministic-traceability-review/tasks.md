---
wiki:
  content_hash: 7c8be66be50f10e61eaf0714170bec7df60a38bcfd7b1025ebbee9a1ff49a938
  fragments:
    tasks-breakdown-deterministic-traceability-review-knowledge-consistency:
      tokens: 3741
      inbound: 0
      outbound: 3
    phase-1-setup-foundational-infrastructure:
      tokens: 347
      inbound: 0
      outbound: 0
    phase-2-user-story-1-reverse-code-impact-analysis-traceability-priority-p1-mvp:
      tokens: 412
      inbound: 0
      outbound: 0
    phase-3-user-story-2-prci-review-lifecycle-verifiable-resolution-priority-p1:
      tokens: 433
      inbound: 0
      outbound: 0
    phase-4-user-story-3-deterministic-link-anchor-heading-integrity-verification-priority-p1:
      tokens: 316
      inbound: 0
      outbound: 0
    phase-5-user-story-4-explicit-rule-verification-vs-semantic-drift-separation-priority-p2:
      tokens: 363
      inbound: 0
      outbound: 0
    phase-6-json-rpc-sidecar-api-vs-code-extension-integration-priority-p2:
      tokens: 337
      inbound: 0
      outbound: 0
    phase-7-user-story-5-isolated-graphify-spike-for-observed-architecture-priority-p3:
      tokens: 256
      inbound: 0
      outbound: 0
    phase-8-polish-performance-budgets-documentation-verification:
      tokens: 242
      inbound: 0
      outbound: 0
    requirements-coverage-mapping:
      tokens: 764
      inbound: 0
      outbound: 0
    dependencies-execution-order:
      tokens: 179
      inbound: 0
      outbound: 0
---
# Tasks Breakdown: Deterministic Traceability, Review & Knowledge Consistency

**Feature Branch**: `010-deterministic-traceability-review` | **Date**: 2026-10-13  
**Spec**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md) | **Data Model**: [data-model.md](data-model.md) | **Contracts**: *contracts/traceability_rpc_protocol.json*  

---

## Phase 1: Setup & Foundational Infrastructure

**Purpose**: Core data models, frontmatter parsing extension, and path containment validation prerequisites.

- [x] T001 [P] Extend `crates/core/src/model/mod.rs` with `pub code_dependencies: Vec<CodeDependency>` on `NoteNode` (annotated with `#[serde(default, skip_serializing_if = "Vec::is_empty")]`), define `DeterministicFindingSeverity`, `DependencySource`, `ReviewObligation`, `ReviewStatus`, `ReviewGatingConfig`, `AffectedDocumentsReport`, and `DeterministicFinding` structs per `data-model.md` (FR-001, FR-006, FR-007, FR-011).
- [x] T002 [P] Extend `crates/core/src/fragment/frontmatter.rs` to parse `code_dependencies: Vec<String>` from Markdown YAML frontmatter with error handling (FR-001).
- [x] T003 Implement POSIX path normalization and workspace root containment guard (`PATH_TRAVERSAL_PROHIBITED`) in `crates/core/src/drift/extractor.rs` (FR-002, FR-003, SC-004).
- [x] T004 [P] Update `crates/core/src/drift/index.rs` to record `DependencySource` (`Frontmatter` vs `InlineMarkdownLink`) in `ReverseImpactIndex` (FR-004, FR-006).
- [x] T005 Create unit tests in `crates/core/tests/drift_test.rs` validating frontmatter `code_dependencies` extraction, path normalization, and path traversal rejection (FR-001, FR-002, FR-003, SC-004).

---

## Phase 2: User Story 1 - Reverse Code Impact Analysis & Traceability (Priority: P1) 🎯 MVP

**Goal**: Given modified code files from a Git change, identify all dependent documentation notes with deduplication and origin evidence.  
**Independent Test**: Run `llm-wiki-core trace-impact --paths <code_files>` on fixture repository and verify exact declaring notes returned.

- [x] T006 [P] [US1] Create unit tests in `crates/core/tests/trace_test.rs` verifying reverse code impact lookups across multiple code files (FR-004, FR-006, SC-001).
- [x] T007 [US1] Implement Git branch diff parser (`git diff base...head` and commit range `A..B`) in `crates/core/src/delta/updater.rs` (FR-005).
- [x] T008 [US1] Implement affected documents calculation and obligation deduplication in `crates/core/src/drift/review.rs` (FR-005, FR-006, SC-001).
- [x] T009 [US1] Implement code file rename and deletion impact analysis emitting `OBSOLETE_CODE_REFERENCE` and `DELETED_CODE_FILE` findings in `crates/core/src/drift/detector.rs` (FR-010).
- [x] T010 [US1] Implement CLI command `llm-wiki-core trace-impact` in `crates/core/src/cli/trace_impact.rs` (and wire into `crates/core/src/main.rs`) with `--paths` and `--range` options (FR-005, FR-006).
- [x] T011 [US1] Add integration test in `crates/core/tests/cli_delta_test.rs` validating `trace-impact` CLI output format against `data-model.md` and verifying bit-exact JSON idempotence across repeated runs on unmodified repositories (NFR-004, SC-001).

---

## Phase 3: User Story 2 - PR/CI Review Lifecycle & Verifiable Resolution (Priority: P1)

**Goal**: Support PR review obligations with resolution states (`updated`, `reviewed_unchanged`, `pending`, `blocked`) and multi-tier verifiable receipts.  
**Independent Test**: Run pre-commit/PR check on modified code; verify status transitions to `reviewed_unchanged` when review receipt is provided.

- [x] T012 [P] [US2] Create unit tests in `crates/core/tests/candidate_test.rs` testing review status transitions (`updated`, `reviewed_unchanged`, `pending`, `blocked`) (FR-007, SC-002).
- [x] T013 [US2] Implement review receipt store (`.context-vault/reviews.json`) and commit trailer parser (`Reviewed-Docs: ...`) in `crates/core/src/drift/review.rs` (FR-008).
- [x] T014 [US2] Implement CLI command `llm-wiki-core review resolve <doc_id>` in `crates/core/src/cli/review_cmd.rs` (and wire into `crates/core/src/main.rs`) (FR-008).
- [x] T015 [US2] Extend pre-commit candidate gate in `crates/core/src/candidate/evaluator.rs` and `crates/core/src/cli/validate.rs` (supporting `--paths <files>` and `--json`) to verify PR review obligations and apply repository gating policy (FR-007, FR-009, SC-002).
- [x] T016 [US2] Integrate brownfield debt baseline diffing in `crates/core/src/debt/baseline.rs` to isolate pre-existing unreviewed debt from new PR regressions (FR-009).
- [x] T017 [US2] Add integration test in `crates/core/tests/cli_validate_test.rs` verifying PR block policy when `pending` reviews exist and pass when receipts are recorded (FR-007, FR-008, FR-009, SC-002).

---

## Phase 4: User Story 3 - Deterministic Link, Anchor & Heading Integrity Verification (Priority: P1)

**Goal**: Detect broken internal links, non-existent heading anchors, cross-file heading mutation impacts, and obsolete citations.  
**Independent Test**: Modify heading in target note and verify referencing notes emit `BROKEN_FRAGMENT_ANCHOR`.

- [x] T018 [P] [US3] Create unit tests in `crates/core/tests/anchor_normalization_test.rs` for cross-file heading rename impacts (FR-010).
- [x] T019 [US3] Extend AST visitor in `crates/core/src/ast/visitor.rs` to track cross-document heading anchor resolution (FR-010).
- [x] T020 [US3] Implement cross-file fragment validation in `crates/core/src/graph/resolver.rs` emitting `BROKEN_FRAGMENT_ANCHOR` findings (FR-010).
- [x] T021 [US3] Implement cross-file heading and markdown anchor mutation resolution in `crates/core/src/ast/visitor.rs` and `crates/core/src/graph/resolver.rs` (updating target anchor slugs purely in-memory on candidate graph structures to emit `BROKEN_FRAGMENT_ANCHOR` or transformation proposals, never automatically modifying Markdown files on disk per Invariant XVI) (FR-010).
- [x] T022 [US3] Add regression test in `crates/core/tests/ast_test.rs` verifying heading anchor stability and normalized slug generation (FR-010).

---

## Phase 5: User Story 4 - Explicit Rule Verification vs. Semantic Drift Separation (Priority: P2)

**Goal**: Enforce explicit architectural rules (`prohibited_dependencies`, `mandatory_documentation`, `path_exists`) without false semantic drift verdicts.  
**Independent Test**: Introduce prohibited code-doc dependency and verify deterministic `rule_violation` with exact rule ID.

- [x] T023 [P] [US4] Create unit tests in `crates/core/tests/drift_test.rs` testing architectural rule evaluation from `.context-vault.yaml` (FR-012, FR-013, SC-003).
- [x] T024 [US4] Implement `ArchitecturalRule` schema and parser in `crates/core/src/config/mod.rs` (updating `ContextVaultConfig` with `pub review_policy: Option<ReviewGatingConfig>` and `pub architecture: Option<ArchitectureConfig>`) (FR-013).
- [x] T025 [US4] Implement rule checking engine in `crates/core/src/architecture/rules.rs` distinguishing `change_detected`, `review_required`, and `rule_violation` (FR-012, FR-013, SC-003).
- [x] T026 [US4] Enforce Epistemic Tier D (`not_verifiable`) for freeform prose checks without explicit rules (FR-012).
- [x] T027 [US4] Add end-to-end test in `crates/core/tests/freshness_evidence_test.rs` verifying that implementation changes without rule violations emit `review_required`, never `rule_violation`, and asserting that ordinary code diffs never mutate architectural documentation notes without explicit user consent (FR-012, FR-014, SC-003).

---

## Phase 6: JSON-RPC Sidecar API & VS Code Extension Integration (Priority: P2)

**Goal**: Expose traceability and review resolution methods over JSON-RPC 2.0 and surface review obligations in the extension UI.  
**Independent Test**: Send `trace/getAffectedDocuments` and `review/resolveDocumentReview` RPC requests and verify response schemas.

- [x] T028 [P] Implement `trace/getAffectedDocuments` handler in `crates/core/src/ipc/handlers.rs` conforming to `specs/010-deterministic-traceability-review/contracts/traceability_rpc_protocol.json` (FR-016).
- [x] T029 [P] Implement `review/resolveDocumentReview` handler in `crates/core/src/ipc/handlers.rs` conforming to `specs/010-deterministic-traceability-review/contracts/traceability_rpc_protocol.json` (FR-016).
- [x] T030 Add TypeScript interfaces in `extension/src/types/engine.ts` matching JSON-RPC protocol contracts (FR-011, FR-016).
- [x] T031 Extend IPC client in `extension/src/rpc/` to support `trace/getAffectedDocuments` and `review/resolveDocumentReview` (FR-016).
- [x] T032 Update VS Code Health and Evolution drawers in `extension/webview-ui/src/components/` to render review obligation badges and `[Mark Reviewed]` action button (FR-007, FR-008).
- [x] T033 Add extension integration tests in `extension/test/suite/traceability.test.ts` (FR-016).

---

## Phase 7: User Story 5 - Isolated Graphify Spike for Observed Architecture (Priority: P3)

**Goal**: Evaluate Graphify vs Understand Anything in an isolated example harness without polluting canonical graph truth.  
**Independent Test**: Run `cargo run --example graphify_spike` and verify empirical report generation in `target/graphify_evaluation_report.md`.

- [x] T034 [US5] Create standalone example harness `crates/core/examples/graphify_spike.rs` importing Graphify JSON graph exports (FR-015, SC-006).
- [x] T035 [US5] Implement comparison metrics in `crates/core/examples/graphify_spike.rs` measuring precision, recall against manual citations, runtime latency, and memory RSS (FR-015, SC-006).
- [x] T036 [US5] Verify that imported relations are strictly tagged with `provenance: "external_observation"` and cannot mutate canonical `NoteNode` or `NoteEdge` state (FR-014, FR-015, SC-005, SC-006).
- [x] T037 [US5] Generate Markdown summary report in `target/graphify_evaluation_report.md` (FR-015, SC-006).

---

## Phase 8: Polish, Performance Budgets & Documentation Verification

**Purpose**: Cross-cutting benchmarks, packaging, and validation against Constitution v1.6.0.

- [x] T038 Implement Criterion microbenchmark in `crates/core/benches/traceability_benchmark.rs` verifying reverse impact calculation $\le 50\text{ ms}$ for 50 files (NFR-002).
- [x] T039 Verify pre-commit hook ceiling $\le 500\text{ ms}$ fail-open guarantee (NFR-003).
- [x] T040 Verify sidecar resident RSS $\le 35\text{ MB}$ for 5,000 files via heap profiling (NFR-005).
- [x] T041 Run quickstart scenarios 1–5 from `quickstart.md` validating end-to-end user journeys (SC-001, SC-002, SC-003, SC-004, SC-005).
- [x] T042 Recompile release binary, package VSIX, and verify package size $\le 25\text{ MB}$ with 100% native Rust logic and zero external AI calls (NFR-001).

---

## Requirements Coverage Mapping

| Requirement Key | Description | Mapped Task IDs |
| :--- | :--- | :--- |
| **FR-001** | Frontmatter `code_dependencies` YAML parsing | T001, T002, T005 |
| **FR-002** | Inline Markdown code citation normalization | T003, T005 |
| **FR-003** | Strict path containment (`PATH_TRAVERSAL_PROHIBITED`) | T003, T005 |
| **FR-004** | Bidirectional `ReverseImpactIndex` | T004, T006 |
| **FR-005** | Multiple Git diff strategies support | T007, T008, T010 |
| **FR-006** | ReviewObligation generation and deduplication | T001, T004, T006, T008, T010 |
| **FR-007** | Four document resolution states (`updated`, `reviewed_unchanged`, `pending`, `blocked`) | T001, T012, T015, T017, T032 |
| **FR-008** | Multi-tier review receipt ingestion | T013, T014, T017, T032 |
| **FR-009** | Baseline debt isolation and configurable PR gating policy | T015, T016, T017 |
| **FR-010** | Broken links, anchors, renames, and deletions detection | T009, T018, T019, T020, T021, T022 |
| **FR-011** | Standardized JSON diagnostic finding serialization | T001, T030 |
| **FR-012** | Strict category separation (`change_detected`, `review_required`, `rule_violation`, `not_verifiable`) | T023, T025, T026, T027 |
| **FR-013** | Explicit architectural rules verification (`prohibited_dependencies`, etc.) | T023, T024, T025 |
| **FR-014** | No auto-updating of intended architecture without consent | T027, T036 |
| **FR-015** | Isolated Graphify research spike harness | T034, T035, T036, T037 |
| **FR-016** | JSON-RPC methods (`trace/getAffectedDocuments`, `review/resolveDocumentReview`) | T028, T029, T030, T031, T033 |
| **NFR-001** | Zero External AI local execution | T042 |
| **NFR-002** | Impact calculation latency $\le 50\text{ ms}$ | T038 |
| **NFR-003** | Pre-commit fail-open ceiling $\le 500\text{ ms}$ | T039 |
| **NFR-004** | Idempotence and bit-exact reproducibility | T011 |
| **NFR-005** | Resident memory RSS $\le 35\text{ MB}$ | T040 |
| **SC-001** | 100% Impact recall for modified code | T006, T008, T011, T041 |
| **SC-002** | PR/CI gating reliability within $\le 100\text{ ms}$ | T012, T015, T017, T041 |
| **SC-003** | Zero false positives on architectural drift | T023, T025, T027, T041 |
| **SC-004** | Strict path safety (100% traversal caught) | T003, T005, T041 |
| **SC-005** | Graph integrity guarantee (zero mutation by diffs) | T036, T041 |
| **SC-006** | Spike isolation (external observation provenance) | T034, T035, T036, T037 |

---

## Dependencies & Execution Order

- **Foundational (Phase 1)**: Prerequisites for all subsequent phases.
- **US1 Reverse Impact (Phase 2)**: Depends on Phase 1; establishes MVP.
- **US2 Review Lifecycle (Phase 3)**: Depends on US1; integrates PR/CI receipt workflow.
- **US3 Link & Anchor Integrity (Phase 4)**: Can proceed in parallel with US2 after Phase 1.
- **US4 Architectural Rules (Phase 5)**: Depends on Phase 1 and US1.
- **JSON-RPC & UI (Phase 6)**: Depends on US1, US2, US3.
- **Graphify Spike (Phase 7)**: Independent research track; can proceed in parallel.
- **Polish & Benchmarks (Phase 8)**: Depends on all implementation phases.
