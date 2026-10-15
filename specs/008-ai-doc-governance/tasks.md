---
wiki:
  content_hash: eefde7f1be056ca494cc5e4a1ba5188bfab52eb27a450e56244eb0243116ce92
  fragments:
    tasks-ai-oriented-documentation-governance-008-ai-doc-governance:
      tokens: 5106
      inbound: 0
      outbound: 2
    phase-1-setup-shared-infrastructure:
      tokens: 200
      inbound: 0
      outbound: 0
    phase-2-foundational-blocking-prerequisites:
      tokens: 206
      inbound: 0
      outbound: 0
    phase-3-user-story-1-deterministic-harness-entry-point-knowledge-index-discovery-priority-p1-mvp:
      tokens: 640
      inbound: 0
      outbound: 0
    tests-for-user-story-1:
      tokens: 153
      inbound: 0
      outbound: 0
    implementation-for-user-story-1:
      tokens: 353
      inbound: 0
      outbound: 0
    phase-4-user-story-2-harness-entry-point-quality-context-efficiency-audit-priority-p2:
      tokens: 566
      inbound: 0
      outbound: 0
    tests-for-user-story-2:
      tokens: 177
      inbound: 0
      outbound: 0
    implementation-for-user-story-2:
      tokens: 259
      inbound: 0
      outbound: 0
    phase-5-user-story-3-skill-governance-procedure-vs-capability-distinction-priority-p3:
      tokens: 501
      inbound: 0
      outbound: 0
    tests-for-user-story-3:
      tokens: 136
      inbound: 0
      outbound: 0
    implementation-for-user-story-3:
      tokens: 246
      inbound: 0
      outbound: 0
    phase-6-user-story-4-harness-anchored-context-trace-4-capability-quality-model-priority-p4:
      tokens: 524
      inbound: 0
      outbound: 0
    tests-for-user-story-4:
      tokens: 136
      inbound: 0
      outbound: 0
    implementation-for-user-story-4:
      tokens: 253
      inbound: 0
      outbound: 0
    phase-7-user-story-5-evidence-based-structural-evolution-for-ai-governance-priority-p5:
      tokens: 611
      inbound: 0
      outbound: 0
    tests-for-user-story-5:
      tokens: 140
      inbound: 0
      outbound: 0
    implementation-for-user-story-5:
      tokens: 312
      inbound: 0
      outbound: 0
    phase-8-user-story-6-physical-directory-knowledge-organization-analysis-priority-p6:
      tokens: 354
      inbound: 0
      outbound: 0
    tests-for-user-story-6:
      tokens: 95
      inbound: 0
      outbound: 0
    implementation-for-user-story-6:
      tokens: 161
      inbound: 0
      outbound: 0
    phase-9-polish-cross-cutting-concerns:
      tokens: 183
      inbound: 0
      outbound: 0
    dependencies-execution-order:
      tokens: 388
      inbound: 0
      outbound: 0
    phase-dependencies:
      tokens: 118
      inbound: 0
      outbound: 0
    user-story-dependencies:
      tokens: 168
      inbound: 0
      outbound: 0
    parallel-opportunities:
      tokens: 96
      inbound: 0
      outbound: 0
    parallel-example-user-story-1:
      tokens: 130
      inbound: 0
      outbound: 0
    implementation-strategy:
      tokens: 210
      inbound: 0
      outbound: 0
    mvp-first-user-story-1-only:
      tokens: 97
      inbound: 0
      outbound: 0
    incremental-delivery:
      tokens: 109
      inbound: 0
      outbound: 0
    notes:
      tokens: 65
      inbound: 0
      outbound: 0
    phase-10-convergence:
      tokens: 455
      inbound: 0
      outbound: 0
---
# Tasks: AI-Oriented Documentation Governance (`008-ai-doc-governance`)

**Branch**: `008-ai-doc-governance` | **Spec**: [`specs/008-ai-doc-governance/spec.md`](spec.md) | **Plan**: [`specs/008-ai-doc-governance/plan.md`](plan.md)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Module initialization, shared TypeScript definitions, and Rust data structures

- [X] T001 Register `harness` module and export submodules in `crates/core/src/harness/mod.rs` and `crates/core/src/lib.rs`
- [X] T002 [P] Define TypeScript domain types (`HarnessEntryPoint`, `HarnessType`, `KnowledgeIndexRoute`, `SecondaryRoutingLayer`, `SkillDefinition`, `ProcedureCandidate`, `AIOrientedCapabilityMatrix`, `DirectoryStructureMetrics`, `GovernanceStrategyType`) in `extension/src/types/governance.ts` matching `contracts/ai_governance_rpc_protocol.json`
- [X] T003 [P] Define core Rust data models and Serde structs (`HarnessEntryPoint`, `HarnessType`, `EmbeddedIndexMetadata`, `KnowledgeIndexRoute`, `SecondaryRoutingLayer`, `GovernanceStrategyType`) in `crates/core/src/harness/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared structural traits, governance error definitions, and template generator that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T004 Implement `HarnessType` enum, traits, and threshold constants (`MAX_NAVIGATION_DEPTH_THRESHOLD`, `CONCENTRATION_ENTROPY_THRESHOLD`, `LOW_CONTEXT_EFFICIENCY_THRESHOLD`) in `crates/core/src/harness/mod.rs`
- [X] T005 [P] Define governance error types, diagnostic logging, and validation failure reasons in `crates/core/src/harness/mod.rs`
- [X] T006 [P] Implement template `AGENTS.md` initializer generator with default `## Knowledge Index` for unconfigured repositories (FR-002) in `crates/core/src/harness/discovery.rs`

**Checkpoint**: Foundation ready — user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Deterministic Harness Entry Point & Knowledge Index Discovery (Priority: P1) 🎯 MVP

**Goal**: Detect configured AI Harness Entry Points (`AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.github/copilot-instructions.md`, custom) and verify that the Knowledge Index is incorporated directly into the entry point without intermediate routing layers.

**Independent Test**: Create a workspace with `AGENTS.md` containing `## Knowledge Index`, verify that the engine detects the harness, parses embedded index routes, resolves document and fragment targets, and surfaces the harness status with 0 intermediary routing hops.

### Tests for User Story 1 ⚠️
> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T007 [P] [US1] Unit test for harness detection (`AGENTS.md`, `CLAUDE.md`, custom paths) in `crates/core/tests/harness_governance_test.rs`
- [X] T008 [P] [US1] Unit test for embedded Knowledge Index AST parsing and route resolution in `crates/core/tests/harness_governance_test.rs`
- [X] T009 [P] [US1] Contract test for `governance/getHarnessStatus` JSON-RPC handler in `crates/core/tests/harness_governance_test.rs`

### Implementation for User Story 1

- [X] T010 [US1] Implement `HarnessDetector` locating `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.github/copilot-instructions.md`, and custom paths from `.context-vault.yaml` in `crates/core/src/harness/discovery.rs`
- [X] T011 [US1] Implement `IndexParser` extracting `## Knowledge Index` AST sections, parsing domain categories, labels, and routes in `crates/core/src/harness/index_parser.rs`
- [X] T012 [US1] Implement route resolution checking target files and fragment anchors against `ResolverIndex` via `resolve_relative_path` in `crates/core/src/harness/index_parser.rs`
- [X] T013 [US1] Implement `SecondaryLayerDetector` flagging intermediate routing hops (`docs/index.md`, `memory.md`, `hot-index.md`) with penalty points in `crates/core/src/harness/secondary_layer.rs`
- [X] T014 [US1] Implement JSON-RPC handler for `governance/getHarnessStatus` in `crates/core/src/ipc/handlers.rs`
- [X] T015 [US1] Implement GovernanceManager in `extension/src/governance/governanceManager.ts` and dispatch to webview in `extension/src/webview/panelManager.ts`
- [X] T016 [US1] Wire client RPC method `getHarnessStatus` and command `llmWiki.initializeHarness` in `extension/webview-ui/src/rpc/webviewRpc.ts`

**Checkpoint**: At this point, User Story 1 is fully functional and testable independently (MVP ready!)

---

## Phase 4: User Story 2 - Harness Entry Point Quality & Context Efficiency Audit (Priority: P2)

**Goal**: Comprehensive auditing of the Harness Entry Point across Integrity, Structure, Consistency, Freshness, and Context Efficiency (ratio of useful routing tokens vs boilerplate), enforcing Invariant XIII (size is a signal, not a verdict), and supporting active harness toggling (`setActiveHarness`).

**Independent Test**: Verify that broken links, dead anchors, and stale routes in `AGENTS.md` are flagged with exact line numbers and penalty points, while a 40k-token cohesive entry point passes without arbitrary size penalties.

### Tests for User Story 2 ⚠️

- [X] T017 [P] [US2] Unit test for Harness Entry Point 5-pillar and context efficiency scoring in `crates/core/tests/harness_governance_test.rs`
- [X] T018 [P] [US2] Integration test verifying large cohesive harness does not trigger size penalty (Invariant XIII) in `crates/core/tests/harness_governance_test.rs`
- [X] T019 [P] [US2] Contract test for `governance/setActiveHarness` JSON-RPC handler in `crates/core/tests/harness_governance_test.rs`
- [X] T020 [P] [US2] Pre-commit admission control test asserting staged broken index routes block commit (Invariant XII) in `crates/core/tests/cli_validate_test.rs`

### Implementation for User Story 2

- [X] T021 [US2] Implement `HarnessQualityAuditor` evaluating Integrity (broken links/anchors), Structure, Consistency, Freshness, and Context Efficiency in `crates/core/src/harness/mod.rs`
- [X] T022 [US2] Calculate context efficiency ratio (useful routing tokens vs total tokens) and non-prescriptive size evaluation (< 40% triggers degradation) in `crates/core/src/harness/mod.rs`
- [X] T023 [US2] Integrate harness governance issues into `extract_all_issues_with_baseline` in `crates/core/src/quality/scorer.rs`
- [X] T024 [US2] Implement JSON-RPC handler for `governance/setActiveHarness` in `crates/core/src/ipc/handlers.rs`
- [X] T025 [US2] Render Harness Entry Point status card, health indicators, and active harness selector in `extension/webview-ui/src/components/settingsDrawer.ts`
- [X] T026 [US2] Add CSS styling for harness status badges and secondary layer alert cards in `extension/webview-ui/src/theme.css`

**Checkpoint**: User Stories 1 AND 2 are functional and fully integrated

---

## Phase 5: User Story 3 - Skill Governance & Procedure vs. Capability Distinction (Priority: P3)

**Goal**: Distinguish between one-off procedures and reusable Skills (`SKILL.md` + scripts/templates/references), audit Skill assets for broken paths/manifests, and detect candidates for new Skills from repeated procedural steps across documents.

**Independent Test**: Add valid skills, broken skills, and a repeated 4-step procedure across 3 documents, verifying that valid skills are cataloged, broken script references are flagged, and the procedure is surfaced as a `ProcedureCandidate`.

### Tests for User Story 3 ⚠️

- [X] T027 [P] [US3] Unit test for Skill structure validation (`SKILL.md`, scripts, templates, references) in `crates/core/tests/skill_governance_test.rs`
- [X] T028 [P] [US3] Unit test for procedure extraction and `ProcedureCandidate` generation in `crates/core/tests/skill_governance_test.rs`
- [X] T029 [P] [US3] Contract test for `governance/getSkillCatalog` JSON-RPC handler in `crates/core/tests/skill_governance_test.rs`

### Implementation for User Story 3

- [X] T030 [US3] Implement `SkillAuditor` validating `SKILL.md` presence, script/template disk existence, and capability overlap in `crates/core/src/harness/skill.rs`
- [X] T031 [US3] Implement `ProcedureExtractor` detecting repeated ordered step sequences across documents in `crates/core/src/harness/procedure.rs`
- [X] T032 [US3] Implement candidacy scoring (recurrence score >= 0.70) for promoting procedures to Skills in `crates/core/src/harness/procedure.rs`
- [X] T033 [US3] Implement JSON-RPC handler for `governance/getSkillCatalog` in `crates/core/src/ipc/handlers.rs`
- [X] T034 [US3] Render Skills Catalog and Procedure Candidates in `extension/webview-ui/src/components/settingsDrawer.ts`
- [X] T035 [US3] Wire client RPC method `getSkillCatalog` in `extension/webview-ui/src/rpc/webviewRpc.ts`

**Checkpoint**: User Stories 1, 2, and 3 work independently and cohesively

---

## Phase 6: User Story 4 - Harness-Anchored Context Trace & 4-Capability Quality Model (Priority: P4)

**Goal**: Originate Context Trace traversals strictly from the Harness Entry Point down through embedded index routes, evaluating the 4 AI capabilities (Entry, Discovery, Retrieval, Reuse) locally in Rust and compiling into the active `OptimizationProfile` (Invariant XIV).

**Independent Test**: Execute a simulated query trace from `AGENTS.md` to a specific section fragment, verifying that trace depth, accumulated BPE tokens, and the 4-capability matrix are computed in $\le 50	ext{ ms}$.

### Tests for User Story 4 ⚠️

- [X] T036 [P] [US4] Unit test for harness-anchored Context Trace traversal in `crates/core/tests/context_trace_harness_test.rs`
- [X] T037 [P] [US4] Unit test for 4-capability matrix computation (Entry, Discovery, Retrieval, Reuse) in `crates/core/tests/context_trace_harness_test.rs`
- [X] T038 [P] [US4] Contract test for `governance/traceFromHarness` JSON-RPC handler in `crates/core/tests/context_trace_harness_test.rs`

### Implementation for User Story 4

- [X] T039 [US4] Implement `trace_from_harness` locking traversal root to active Harness Entry Point in `crates/core/src/trace/traversal.rs`
- [X] T040 [US4] Implement `AIOrientedCapabilityMatrix` evaluator synthesizing Entry, Discovery, Retrieval, and Reuse in `crates/core/src/harness/matrix.rs`
- [X] T041 [US4] Compile `AI-Oriented` documentation governance evaluation directly into `OptimizationProfile::ai_oriented()` (Invariant XIV) in `crates/core/src/evolution/profile.rs`
- [X] T042 [US4] Implement JSON-RPC handler for `governance/traceFromHarness` in `crates/core/src/ipc/handlers.rs`
- [X] T043 [US4] Render 4-capability scorecard (Entry, Discovery, Retrieval, Reuse) in `extension/webview-ui/src/components/settingsDrawer.ts`
- [X] T044 [US4] Update status bar tooltip to display AI Governance Score in `extension/src/extension.ts`

**Checkpoint**: 4 user stories functional and integrated

---

## Phase 7: User Story 5 - Evidence-Based Structural Evolution for AI Governance (Priority: P5)

**Goal**: Propose user-controlled structural transformations across all 7 governance strategies (`EmbedKnowledgeIndex`, `EliminateSecondaryRoutingLayer`, `CreateSkillFromProcedure`, `ConsolidateDuplicateSkills`, `DecomposeHarness`, `IntroduceSectionAddressing`, `RestructureDirectoryDomain`) with trade-off matrices, unified diffs, approval gates (`Auto-Apply: OFF`, `approval_token` verification), and Before-vs-After re-measurement.

**Independent Test**: Generate a transformation plan with a valid `approval_token`, verify that applying without the token fails, and verify Before-vs-After delta metrics upon execution of `governance/applyGovernancePlan`.

### Tests for User Story 5 ⚠️

- [X] T045 [P] [US5] Unit test for all 7 governance transformation strategies in `crates/core/tests/transformation_governance_test.rs`
- [X] T046 [P] [US5] Unit test verifying Invariant XVI user approval token enforcement in `crates/core/tests/transformation_governance_test.rs`
- [X] T047 [P] [US5] Contract test for `governance/generateGovernancePlan` and `governance/applyGovernancePlan` JSON-RPC handlers in `crates/core/tests/transformation_governance_test.rs`

### Implementation for User Story 5

- [X] T048 [US5] Implement the 7 governance transformation strategies (`EmbedKnowledgeIndex`, `EliminateSecondaryRoutingLayer`, `CreateSkillFromProcedure`, `ConsolidateDuplicateSkills`, `DecomposeHarness`, `IntroduceSectionAddressing`, `RestructureDirectoryDomain`) in `crates/core/src/evolution/strategies.rs`
- [X] T049 [US5] Implement atomic filesystem transformation plan generator with deterministic `approval_token` and unified diff preview in `crates/core/src/evolution/transformation.rs`
- [X] T050 [US5] Implement JSON-RPC handler for `governance/generateGovernancePlan` in `crates/core/src/ipc/handlers.rs`
- [X] T051 [US5] Implement JSON-RPC handler for `governance/applyGovernancePlan` with `approval_token` validation in `crates/core/src/ipc/handlers.rs`
- [X] T052 [US5] Wire governance transformation plan preview and user approval gate in `extension/webview-ui/src/components/evolutionTab.ts`
- [X] T053 [US5] Implement post-apply empirical re-measurement reporting $\Delta\text{Tokens}$, $\Delta\text{Hops}$, and $\Delta\text{Score}$ in `crates/core/src/evolution/re_measure.rs`

**Checkpoint**: 5 user stories functional with user-controlled transformation safety

---

## Phase 8: User Story 6 - Physical Directory & Knowledge Organization Analysis (Priority: P6)

**Goal**: Analyze physical directory layout for excessive depth ($> 5$), document overconcentration (entropy $< 0.35$), scattered related knowledge, or mixed domains without imposing dogmatic folder taxonomies.

**Independent Test**: Place notes in overconcentrated or deeply nested directories and verify that non-prescriptive organizational signals are raised with domain grouping recommendations.

### Tests for User Story 6 ⚠️

- [X] T054 [P] [US6] Unit test for directory depth, concentration entropy, domain mixing, and cross-folder coupling metrics in `crates/core/tests/directory_metrics_test.rs`
- [X] T055 [P] [US6] Contract test for `governance/analyzeDirectoryStructure` JSON-RPC handler in `crates/core/tests/directory_metrics_test.rs`

### Implementation for User Story 6

- [X] T056 [US6] Implement non-prescriptive directory metrics calculator (depth, entropy, domain mixing, coupling) in `crates/core/src/harness/directory_metrics.rs`
- [X] T057 [US6] Implement organizational signal generator flagging overconcentration and scattered knowledge in `crates/core/src/harness/directory_metrics.rs`
- [X] T058 [US6] Implement JSON-RPC handler for `governance/analyzeDirectoryStructure` in `crates/core/src/ipc/handlers.rs`
- [X] T059 [US6] Render Directory Organization insights card in `extension/webview-ui/src/components/settingsDrawer.ts`

**Checkpoint**: All 6 user stories functional and complete

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: System documentation, Criterion benchmarks, quickstart scenario validation, and test suite green-gate

- [X] T060 [P] Update documentation and active workspace context in `notes/01_projects/llm_wiki_ext4vsc.md` and `notes/02_memory/active_context.md`
- [X] T061 [P] Add Criterion performance benchmark verifying full governance audit $\le 100\text{ ms}$ for 1,000 files in `crates/core/benches/governance_benchmark.rs`
- [X] T062 Execute end-to-end quickstart validation scenarios from `specs/008-ai-doc-governance/quickstart.md`
- [X] T063 Verify full test suite passing (`cargo test --all` and `npm test` in `extension/`)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User Story 1 (P1) is the MVP and should be implemented first.
  - User Stories 2–6 can proceed incrementally or in parallel once Foundational is done.
- **Polish (Phase 9)**: Depends on all user stories being complete.

### User Story Dependencies

- **User Story 1 (P1) [MVP]**: Depends on Phase 2. Provides harness detection, index parsing, and secondary layer detection.
- **User Story 2 (P2)**: Depends on US1 (evaluates quality of the detected entry point).
- **User Story 3 (P3)**: Depends on Phase 2. Can be implemented in parallel with US1/US2.
- **User Story 4 (P4)**: Depends on US1 (requires harness entry point to anchor Context Trace).
- **User Story 5 (P5)**: Depends on US1, US2, US3 (generates transformations for detected problems).
- **User Story 6 (P6)**: Depends on Phase 2. Independent of other stories.

### Parallel Opportunities

- All Setup tasks marked [P] (T002, T003) can run in parallel.
- All Foundational tasks marked [P] (T005, T006) can run in parallel once T004 is started.
- All test tasks marked [P] can run in parallel before implementation.
- User Story 3 (Skills) and User Story 6 (Directory Metrics) can proceed in parallel with User Story 1 and 2.

---

## Parallel Example: User Story 1

```bash
# Launch test tasks for User Story 1 together:
Task: "Unit test for harness detection in crates/core/tests/harness_governance_test.rs"
Task: "Unit test for embedded Knowledge Index parsing in crates/core/tests/harness_governance_test.rs"
Task: "Contract test for governance/getHarnessStatus in crates/core/tests/harness_governance_test.rs"

# Launch model tasks in parallel:
Task: "Define TypeScript domain types in extension/src/types/governance.ts"
Task: "Define core Rust data models in crates/core/src/harness/mod.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (`harness` module, TypeScript types, Rust data models).
2. Complete Phase 2: Foundational (harness types, error types, template generator).
3. Complete Phase 3: User Story 1 (Harness & Embedded Index Discovery).
4. **STOP and VALIDATE**: Run `cargo test --test harness_governance_test`.
5. Verify MVP end-to-end.

### Incremental Delivery

1. Setup + Foundational ready.
2. US1: Deterministic Harness & Index Discovery (MVP!).
3. US2: Harness 5-Pillar & Context Efficiency Quality Audit.
4. US3: Skill Governance & Procedure Detection.
5. US4: Harness-Anchored Context Trace & 4-Capability Model.
6. US5: User-Controlled Structural Evolution & Re-Measurement.
7. US6: Directory Structure Analytics.
8. Polish & Criterion benchmarks.

---

## Notes

- Strict checklist format `- [ ] TXXX [P] [US?] Description with file path` enforced across all 63 tasks.
- Tests precede implementation in each story phase per TDD guidelines.
- Invariants III, IV, IX, XII, XIII, XIV, and XVI strictly upheld.

---

## Phase 10: Convergence

**Purpose**: Close gaps identified during specification convergence review: direct Knowledge Index embedding into harness entry points eliminating secondary routing layers, structured multi-harness configuration and persistence (`.context-vault.yaml`), and multi-CLI parity (`CLAUDE.md`, `GEMINI.md`, `AGENTS.md`).

- [X] T064 [CRITICAL] Directly incorporate the 8 feature specification and architectural routes into `AGENTS.md` under `## Knowledge Index` and eliminate the intermediate `index.md` hop per FR-012, FR-013, FR-014 (contradicts)
- [X] T065 [HIGH] Add structured `HarnessConfig` (`harness.active`, `harness.entry_points`) to `ContextVaultConfig` in `crates/core/src/config/mod.rs` and persist active harness updates to `.context-vault.yaml` per FR-001, FR-003, US1/AC3 (missing)
- [X] T066 [HIGH] Implement multi-harness generator and synchronizer in `crates/core/src/harness/discovery.rs` and `extension/src/governance/governanceManager.ts` supporting `CLAUDE.md` and `GEMINI.md` alongside `AGENTS.md` with shared index routes and conventions per FR-001, FR-003 (partial)
- [X] T067 [HIGH] Update `EmbedKnowledgeIndex` and `EliminateSecondaryRoutingLayer` transformation generators in `crates/core/src/evolution/transformation.rs` to extract real routes from existing documents instead of inserting pointers to secondary layers per FR-012, FR-019, FR-031 (contradicts)
- [X] T068 [MEDIUM] Add UI controls in `extension/webview-ui/src/components/settingsDrawer.ts` to initialize and synchronize `CLAUDE.md` and `GEMINI.md` harnesses per FR-002, FR-033 (partial)
- [X] T069 [MEDIUM] Ensure `EliminateSecondaryRoutingLayer` fully re-routes pointers and eliminates secondary index files to achieve zero secondary layer defects and 100% clean harness score per SC-003, FR-013 (partial)
