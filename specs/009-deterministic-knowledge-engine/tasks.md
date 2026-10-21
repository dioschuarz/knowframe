---
wiki:
  content_hash: 1e98256c935727b277e84d9121099a58beae55aa9226531297dca76140405ca9
  fragments:
    tasks-deterministic-knowledge-engineering-engine-009-deterministic-knowledge-engine:
      tokens: 5197
      inbound: 0
      outbound: 2
    phase-1-setup-shared-infrastructure:
      tokens: 290
      inbound: 0
      outbound: 0
    phase-2-foundational-blocking-prerequisites:
      tokens: 316
      inbound: 0
      outbound: 0
    phase-3-user-story-1-deterministic-repository-observation-epistemic-audit-priority-p1-mvp:
      tokens: 618
      inbound: 0
      outbound: 0
    tests-for-user-story-1:
      tokens: 204
      inbound: 0
      outbound: 0
    implementation-for-user-story-1:
      tokens: 297
      inbound: 0
      outbound: 0
    phase-4-user-story-2-profile-relative-health-assessment-across-4-evolution-modes-priority-p2:
      tokens: 654
      inbound: 0
      outbound: 0
    tests-for-user-story-2:
      tokens: 220
      inbound: 0
      outbound: 0
    implementation-for-user-story-2:
      tokens: 304
      inbound: 0
      outbound: 0
    phase-5-user-story-3-comprehensive-18-methodology-fit-evaluation-antipattern-diagnosis-priority-p3:
      tokens: 1069
      inbound: 0
      outbound: 0
    tests-for-user-story-3:
      tokens: 421
      inbound: 0
      outbound: 0
    implementation-for-user-story-3:
      tokens: 514
      inbound: 0
      outbound: 0
    phase-6-user-story-4-evidence-based-strategy-selection-with-explicit-trade-offs-priority-p4:
      tokens: 526
      inbound: 0
      outbound: 0
    tests-for-user-story-4:
      tokens: 174
      inbound: 0
      outbound: 0
    implementation-for-user-story-4:
      tokens: 232
      inbound: 0
      outbound: 0
    phase-7-user-story-5-user-controlled-downstream-automation-beforeafter-re-measurement-priority-p5:
      tokens: 707
      inbound: 0
      outbound: 0
    tests-for-user-story-5:
      tokens: 197
      inbound: 0
      outbound: 0
    implementation-for-user-story-5:
      tokens: 399
      inbound: 0
      outbound: 0
    phase-8-user-story-6-high-performance-vs-code-extension-host-management-experience-ui-priority-p6:
      tokens: 539
      inbound: 0
      outbound: 0
    tests-for-user-story-6:
      tokens: 136
      inbound: 0
      outbound: 0
    implementation-for-user-story-6:
      tokens: 293
      inbound: 0
      outbound: 0
    phase-9-polish-performance-benchmarks-quality-gate:
      tokens: 406
      inbound: 0
      outbound: 0
---
# Tasks: Deterministic Knowledge Engineering Engine (`009-deterministic-knowledge-engine`)

**Branch**: `009-deterministic-knowledge-engine` | **Spec**: [`specs/009-deterministic-knowledge-engine/spec.md`](spec.md) | **Plan**: [`specs/009-deterministic-knowledge-engine/plan.md`](plan.md)

---

## Phase 1: Setup & Shared Infrastructure

**Purpose**: Module initialization, shared TypeScript definitions, and Rust crate hierarchy

- [x] T001 Initialize crate workspace structure for `crates/core`, `crates/cli`, and `crates/engine` in `Cargo.toml` with `tiktoken-rs` (`cl100k_base`), `pulldown-cmark`, `petgraph`, `tantivy`, `blake3` (FR-001, NFR-001)
- [x] T002 [P] Define core TypeScript domain interfaces (`OptimizationProfile`, `EvolutionMode`, `EpistemicTier`, `Methodology`, `StructuralFinding`, `HealthScore`, `StrategyRecommendation`, `TransformationPlan`, `ContextTrace`, `GraphTopology`) in `extension/src/types/engine.ts` matching `contracts/knowledge_engine_protocol.json` (FR-003, FR-006, FR-041, FR-046)
- [x] T003 [P] Define core Rust enums and Serde data models (`EpistemicTier`, `EvolutionMode`, `Methodology`, `OptimizationProfile`, `QualityPillar`, `FindingSeverity`, `StrategyType`, `StrategyRole`, `GraphTopology`, `ContextTrace`) in `crates/core/src/model/mod.rs` (FR-003, FR-006, FR-007, FR-046)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Cryptographic determinism bedrock, AST parsing, and ephemeral candidate isolation

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Implement BLAKE3 cryptographic digest engine deriving canonical `analysis_id` from Git commit hash, canonical configuration bytes, profile bytes, and methodology version in `crates/core/src/epistemic/analysis_id.rs` (FR-002, NFR-002)
- [x] T005 [P] Implement CommonMark AST visitor extracting headings, normalized fragment anchors, wikilinks, code citations, and BPE token counts (`tiktoken-rs` with `cl100k_base`) using `pulldown-cmark` in `crates/core/src/ast/visitor.rs` (FR-004, FR-005)
- [x] T006 [P] Implement `CandidateGraph` managing in-memory ephemeral candidate graph isolation without mutating persistent state in `crates/core/src/graph/candidate.rs` (FR-039)
- [x] T007 Implement unified `OptimizationProfile` compiler transforming `AI-Oriented`, `Human-Oriented`, `Balanced`, and `Custom` presets into concrete weights and hard/soft constraints in `crates/core/src/model/profile.rs` (FR-006, FR-007, FR-008, FR-009)

**Checkpoint**: Foundation ready — user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Deterministic Repository Observation & Epistemic Audit (Priority: P1) 🎯 MVP

**Goal**: Full deterministic scanning of Markdown corpora, bit-exact BLAKE3 digest verification, zero external AI calls, and strict epistemic tier classification (Tiers A, B, C, D).

**Independent Test**: Run analysis on a test repository twice with identical inputs, verify byte-identical JSON outputs, matching `analysis_id`, zero network calls, and 100% adherence to epistemic tier boundaries.

### Tests for User Story 1 ⚠️
> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T008 [P] [US1] Unit test verifying bit-exact BLAKE3 `analysis_id` across repeated runs and platforms in `crates/core/tests/determinism_test.rs` (FR-002, NFR-002, SC-002)
- [x] T009 [P] [US1] Unit test verifying zero network requests and strict Epistemic Tier tagging (Tiers A–D) in `crates/core/tests/epistemic_tier_test.rs` (FR-001, FR-003, SC-001, SC-007)
- [x] T010 [P] [US1] Contract test for `engine/analyzeRepository` and `engine/getGraphTopology` JSON-RPC handlers in `crates/engine/tests/rpc_analyze_test.rs` (FR-041, FR-046)

### Implementation for User Story 1

- [x] T011 [US1] Implement repository filesystem scanner with canonical path sorting, UTF-8 normalization, and excluded path filtering in `crates/core/src/observation/scanner.rs` (FR-001, FR-002)
- [x] T012 [US1] Implement `EpistemicClassifier` strictly assigning Tier A, B, C, or D to every calculated property and finding in `crates/core/src/epistemic/classifier.rs` (FR-003, SC-001)
- [x] T013 [US1] Implement `DocumentNode` and `FragmentMetadata` constructors attaching sections as metadata rather than independent graph nodes in `crates/core/src/observation/document.rs` (FR-004, FR-005)
- [x] T014 [US1] Implement CLI command `llm-wiki-core analyze` emitting bit-exact JSON with `analysis_id` and graph topology in `crates/cli/src/commands/analyze.rs` (FR-002, FR-046, SC-002)
- [x] T015 [US1] Implement JSON-RPC methods `engine/analyzeRepository` and `engine/getGraphTopology` in `crates/engine/src/rpc/handlers.rs` (FR-041, FR-046)

**Checkpoint**: At this point, User Story 1 is fully functional and testable independently (MVP ready!)

---

## Phase 4: User Story 2 - Profile-Relative Health Assessment across 4 Evolution Modes (Priority: P2)

**Goal**: Evaluate repository health across 5 pillars (Integrity, Structure, Consistency, Discoverability, Freshness) relative to active `OptimizationProfile` (`AI-Oriented`, `Human-Oriented`, `Balanced`, `Custom`), with brownfield debt isolation and monolith protection guards.

**Independent Test**: Evaluate a repository under `AI-Oriented` and `Human-Oriented` modes, asserting that scores and findings adapt to profile weights while raw underlying integer metrics remain invariant.

### Tests for User Story 2 ⚠️

- [x] T016 [P] [US2] Unit test for 5-pillar profile-relative health scoring in `crates/core/tests/health_scoring_test.rs` (FR-024, FR-028, FR-029, SC-006)
- [x] T017 [P] [US2] Unit test for Invariant XIII guard (protecting cohesive monoliths from size penalties) in `crates/core/tests/monolith_guard_test.rs` (FR-009, SC-006)
- [x] T018 [P] [US2] Unit test for brownfield baseline isolation (*.context-vault/baseline.json*) in `crates/core/tests/brownfield_debt_test.rs` (FR-028, SC-003)
- [x] T019 [P] [US2] Contract test for `engine/setProfileMode` and `engine/updateCustomProfile` in `crates/engine/tests/rpc_profile_test.rs` (FR-006, FR-041)

### Implementation for User Story 2

- [x] T020 [US2] Implement 5-pillar health scoring aggregator computing profile-weighted composites in `crates/core/src/health/scorer.rs` (FR-024, FR-028, FR-029)
- [x] T021 [US2] Implement brownfield debt baseline manager loading and updating *.context-vault/baseline.json* in `crates/core/src/health/baseline.rs` and CLI command `llm-wiki-core baseline` in `crates/cli/src/commands/baseline.rs` (FR-028, FR-039)
- [x] T022 [US2] Implement monolith cohesion guard verifying that high-token documents with clean internal structure are not penalized in `crates/core/src/health/monolith_guard.rs` (FR-009, FR-028)
- [x] T023 [US2] Implement raw integer numeric model preserving 64-bit integrity across all metric structs using `tiktoken-rs` in `crates/core/src/model/metrics.rs` (FR-030, NFR-007)
- [x] T024 [US2] Implement JSON-RPC handlers for `engine/setProfileMode` and `engine/updateCustomProfile` in `crates/engine/src/rpc/handlers.rs` (FR-006, FR-007, FR-041)

**Checkpoint**: User Stories 1 AND 2 are functional and fully integrated

---

## Phase 5: User Story 3 - Comprehensive 18-Methodology Fit Evaluation & Antipattern Diagnosis (Priority: P3)

**Goal**: Full diagnostic coverage across all 18 KOS methodologies: Classification, Taxonomy, Facets, Controlled Vocabulary, Thesaurus, Semantic Networks, Concept Maps, Ontologies, Domain Analysis, DCAP Metadata, Information Architecture, BM25F IR, Terminology Management, Network Analysis, Knowledge Quality, Provenance, Knowledge Drift, and Context Routing.

**Independent Test**: Introduce targeted structural defects matching each methodology, asserting that the engine detects each corresponding diagnostic rule with exact AST file and line pointers.

### Tests for User Story 3 ⚠️

- [x] T025 [P] [US3] Unit tests for Classification ($C_{\text{cov}}$, $C_{\text{con}}$), Taxonomy ($I_{\text{tax}}$, Tarjan SCC), and Faceted Classification (PMI, $O_{\text{fac}}$) in `crates/core/tests/kos_structural_test.rs` (FR-010, FR-011, FR-012, SC-008)
- [x] T026 [P] [US3] Unit tests for Controlled Vocabulary (Levenshtein), Thesaurus reciprocity (ISO 25964), and Semantic Networks in `crates/core/tests/kos_semantic_test.rs` (FR-013, FR-014, FR-015, SC-008)
- [x] T027 [P] [US3] Unit tests for Concept Maps ($P_{\text{dens}}$), Ontology SHACL shapes, and DCAP Metadata in `crates/core/tests/kos_schema_test.rs` (FR-016, FR-017, FR-019)
- [x] T028 [P] [US3] Unit tests for Information Architecture (FR-020: Dijkstra shortest path, scent proxy, dead-ends), Network Centrality (Brandes $C_B$, PageRank $PR$, Bridges), and BM25F IR in `crates/core/tests/kos_graph_ir_test.rs` (FR-020, FR-021, FR-023, SC-008)
- [x] T029 [P] [US3] Unit tests for Terminology (Aho-Corasick), Provenance (Git co-change), Knowledge Drift (Tennis ontogeny), and Context Trace routing in `crates/core/tests/kos_provenance_routing_test.rs` (FR-018, FR-022, FR-025, FR-026, FR-027, SC-004, SC-008)

### Implementation for User Story 3

- [x] T030 [US3] Implement Classification and Taxonomy evaluators with Tarjan SCC cycle detection and path-to-class mapping in `crates/core/src/methodologies/taxonomy.rs` (FR-010, FR-011, SC-008)
- [x] T031 [US3] Implement Faceted Classification evaluator computing PMI and facet orthogonality in `crates/core/src/methodologies/facets.rs` (FR-012)
- [x] T032 [US3] Implement Controlled Vocabulary and Thesaurus evaluators (reciprocal link verification, FST dictionary) in `crates/core/src/methodologies/thesaurus.rs` (FR-013, FR-014, SC-008)
- [x] T033 [US3] Implement Semantic Networks and Conceptual Maps evaluators in `crates/core/src/methodologies/semantic_networks.rs` (FR-015, FR-016)
- [x] T034 [US3] Implement Ontology and DCAP Metadata application profile shape validator in `crates/core/src/methodologies/metadata_profiles.rs` (FR-017, FR-019)
- [x] T035 [US3] Implement Domain Analysis and Provenance evaluators using Git commit history via `gix` in `crates/core/src/methodologies/provenance.rs` (FR-018, FR-025)
- [x] T036 [US3] Implement Information Architecture (Dijkstra shortest path, scent proxy, dead-ends) and BM25F Information Retrieval evaluators using Tantivy in `crates/core/src/methodologies/information_retrieval.rs` (FR-020, FR-021)
- [x] T037 [US3] Implement Terminology Management engine using Aho-Corasick DFA in `crates/core/src/methodologies/terminology.rs` (FR-022, SC-008)
- [x] T038 [US3] Implement Network Analysis computing Brandes betweenness centrality and power-iteration PageRank in `crates/core/src/methodologies/network_analysis.rs` (FR-023, SC-008)
- [x] T039 [US3] Implement Knowledge Drift and Context Trace routing engines in `crates/core/src/methodologies/context_routing.rs` (FR-026, FR-027, SC-004)

**Checkpoint**: User Stories 1, 2, AND 3 are functional and fully integrated

---

## Phase 6: User Story 4 - Evidence-Based Strategy Selection with Explicit Trade-Offs (Priority: P4)

**Goal**: Evaluate the 6 candidate transformation strategies, compute empirical Fit Scores via Analytic Hierarchy Process (AHP), categorize into pairwise roles, and itemize both expected benefits and countervailing trade-offs.

**Independent Test**: Evaluate an overloaded multi-topical document, asserting that `SplitDecompose` is generated with an explicit trade-off matrix (benefits vs costs), categorized as `MUTUALLY_EXCLUSIVE` with `PreserveDocument`.

### Tests for User Story 4 ⚠️

- [x] T040 [P] [US4] Unit test for AHP matrix priority solver and Fit Score calculation in `crates/core/tests/ahp_scoring_test.rs` (FR-032, FR-033)
- [x] T041 [P] [US4] Unit test for pairwise strategy roles (`PRIMARY`, `SUPPORTING`, `ALTERNATIVE`, `MUTUALLY_EXCLUSIVE`) in `crates/core/tests/strategy_roles_test.rs` (FR-034)
- [x] T042 [P] [US4] Unit test verifying that all recommendations itemize both expected benefits and countervailing trade-offs (Invariant XV) in `crates/core/tests/recommendation_tradeoffs_test.rs` (FR-031, FR-035)

### Implementation for User Story 4

- [x] T043 [US4] Implement Saaty Analytic Hierarchy Process (AHP) matrix solver and implementation cost calculation in `crates/core/src/strategies/ahp.rs` (FR-033)
- [x] T044 [US4] Implement candidate evaluators for the 6 strategies (`PreserveDocument`, `SplitDecompose`, `ExtractFacetMetadata`, `ConsolidateIndex`, `MergeCandidates`, `StandardizeTerminology`) in `crates/core/src/strategies/catalog.rs` (FR-032)
- [x] T045 [US4] Implement pairwise relationship classifier assigning Primary, Supporting, Alternative, or Mutually Exclusive roles in `crates/core/src/strategies/classifier.rs` (FR-034)
- [x] T046 [US4] Implement trade-off generator deriving concrete mathematical benefits and countervailing costs in `crates/core/src/strategies/tradeoffs.rs` (FR-031, FR-035)

**Checkpoint**: User Stories 1 through 4 are functional and fully integrated

---

## Phase 7: User Story 5 - User-Controlled Downstream Automation & Before/After Re-Measurement (Priority: P5)

**Goal**: Generate transformation plans with unified diffs, enforce strict user approval tokens (`Auto-Apply: OFF`), execute discrete AST and filesystem mutations, pre-commit validation, post-commit sync, and post-apply re-measurement.

**Independent Test**: Generate a transformation plan, attempt application without token (rejected), apply with token, verify on-disk unified diff, and inspect Before/After delta report.

### Tests for User Story 5 ⚠️

- [x] T047 [P] [US5] Unit test for cryptographic `approval_token` verification and hard constraint gate in `crates/core/tests/approval_gate_test.rs` (FR-036, FR-037, SC-005)
- [x] T048 [P] [US5] Unit test for deterministic AST and filesystem mutators generating unified diffs in `crates/core/tests/transformation_mutator_test.rs` (FR-038)
- [x] T049 [P] [US5] Integration test for post-apply Before/After delta re-measurement in `crates/core/tests/re_measurement_test.rs` (FR-040)
- [x] T050 [P] [US5] Contract test for `engine/generateTransformationPlan` and `engine/applyTransformationPlan` in `crates/engine/tests/rpc_plan_test.rs` (FR-041)

### Implementation for User Story 5

- [x] T051 [US5] Implement `TransformationPlanGenerator` creating atomic mutation plans, unified diffs, and cryptographic approval tokens in `crates/core/src/automation/plan_generator.rs` and CLI commands `llm-wiki-core plan generate` / `apply` in `crates/cli/src/commands/plan.rs` (FR-036, FR-037)
- [x] T052 [US5] Implement discrete AST and filesystem mutators (file moves, frontmatter updates, link retargeting, section extraction) and automatic frontmatter re-staging in `crates/core/src/automation/mutators.rs` (FR-038, FR-039)
- [x] T053 [US5] Implement Before/After comparative re-measurement engine calculating score and token deltas in `crates/core/src/automation/re_measurement.rs` (FR-040)
- [x] T054 [US5] Implement pre-commit validation command `llm-wiki-core validate` (alias `pre-commit`) with $\le 500$ ms fail-open timeout in `crates/cli/src/commands/validate.rs` (FR-039, NFR-005, SC-003)
- [x] T054b [US5] Implement post-commit graph sync command `llm-wiki-core sync` (alias `post-commit`, `Hook 2: llm-wiki-sync`) applying committed Git deltas to persistent graph in $\le 50$ ms in `crates/cli/src/commands/sync.rs` (FR-039b, NFR-003)
- [x] T055 [US5] Implement JSON-RPC handlers for `engine/generateTransformationPlan` and `engine/applyTransformationPlan` in `crates/engine/src/rpc/handlers.rs` (FR-041)

**Checkpoint**: User Stories 1 through 5 are functional and fully integrated

---

## Phase 8: User Story 6 - High-Performance VS Code Extension Host & Management Experience UI (Priority: P6)

**Goal**: VS Code Webview with 4 drawers (Health, Structure, Evolution, Settings), bidirectional deep-linking to editor line/column, $\le 5$ chars compact presentation formatting, CSP nonces, and secret management.

**Independent Test**: Open the webview, verify 4 drawers render within latency budgets, click a broken reference to jump to editor line/column, and verify compact numeric formatting.

### Tests for User Story 6 ⚠️

- [x] T056 [P] [US6] Unit test for presentation compact numeric formatter ($\le 5$ chars) in `extension/test/formatters.test.ts` (FR-030, NFR-007)
- [x] T057 [P] [US6] Integration test for JSON-RPC stdio client and process lifecycle manager in `extension/test/rpcClient.test.ts` (FR-041)
- [x] T058 [P] [US6] Integration test for bidirectional editor deep linking in `extension/test/deepLink.test.ts` (FR-044)

### Implementation for User Story 6

- [x] T059 [US6] Implement JSON-RPC stdio client communicating with `llm-wiki-core` sidecar in `extension/src/rpc/client.ts` (FR-041)
- [x] T060 [US6] Implement secret retrieval via `vscode.SecretStorage` passing credentials in-memory in `extension/src/security/secretManager.ts` (FR-045)
- [x] T061 [US6] Implement Manager Webview panel hosting 4 views (Health, Structure, Evolution, Settings) and Structure 2D canvas with dynamic CSP nonces in `extension/src/webview/panel.ts` (FR-042, FR-046, NFR-008)
- [x] T062 [US6] Implement compact presentation numeric formatting utility ($\le 5$ chars) in `extension/src/webview/formatters.ts` (FR-030, NFR-007)
- [x] T063 [US6] Implement bidirectional editor navigation opening files at exact line and column in `extension/src/navigation/deepLink.ts` (FR-044)
- [x] T064 [US6] Implement Evolution UI controls displaying strategy cards, trade-off matrices, diff preview, and approval button in `extension/src/webview/evolutionView.ts` (FR-043)

**Checkpoint**: All 6 User Stories are fully implemented and integrated

---

## Phase 9: Polish, Performance Benchmarks & Quality Gate

**Purpose**: Latency benchmarks, memory budget verification, cross-platform packaging, and audit readiness

- [x] T065 [P] Implement Criterion benchmark `crates/core/benches/cold_scan_bench.rs` asserting 1,000 documents cold scan in $\le 1,500$ ms (NFR-004, SC-002)
- [x] T066 [P] Implement Criterion benchmark `crates/core/benches/incremental_bench.rs` asserting single-file candidate patch in $\le 50$ ms and graph calculation $< 200$ ms (NFR-003, SC-002)
- [x] T067 [P] Implement Criterion benchmark `crates/core/benches/pre_commit_bench.rs` asserting pre-commit validation in $\le 500$ ms fail-open and post-commit sync $\le 50$ ms (NFR-003, NFR-005, SC-003)
- [x] T067b [P] Implement memory profiling benchmark in `crates/core/benches/memory_bench.rs` verifying resting RSS $\le 35$ MB for 5,000 documents and peak scan RAM $\le 250$ MB (NFR-006)
- [x] T068 Verify binary packaging size $\le 25$ MB per platform in `crates/cli` release build (NFR-009)
- [x] T068b [P] Configure CI cross-compilation matrix workflow for Linux x86_64/aarch64, macOS arm64/x86_64, Windows x86_64 in `.github/workflows/engine_ci.yml` (NFR-010)
- [x] T069 End-to-end integration test verifying full pipeline execution across all 18 KOS methodologies with 0 external AI calls (FR-001..FR-046, NFR-001..NFR-010, SC-001..SC-008)
