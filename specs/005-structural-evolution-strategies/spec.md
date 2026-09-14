# Feature Specification: Knowledge Organization Systems, Structural Evidence & Trade-Offs, Documentation Orientations & User-Controlled Evolution

**Feature Branch:** `005-structural-evolution-strategies`  
**Target Delivery:** `llm_wiki_ext4vsc` (VS Code Extension + Rust Core `llm-wiki-core`)  
**Status:** In Review / Specification  
**Constitution Authority:** `CONSTITUTION.md` v1.6.0 (The 16 Critical Invariants, Principles I through XII)  
**External AI / Remote LLM:** **STRICTLY PROHIBITED** (Local Deterministic Execution Only)  

---

## 1. Executive Summary & Vision

The `005-structural-evolution-strategies` specification represents the **New Point of Consolidation** for `llm_wiki_ext4vsc`. It elevates LLM Wiki from an observational quality gate into a **deterministic knowledge management system** that analyzes, measures, and evolves the structure of a project's Markdown knowledge through Git-integrated, evidence-based workflows.

### Foundational Principles
1. **Non-Prescriptive Architecture (No Structural Dogma)**:
   LLM Wiki does **not** prescribe a universal documentation structure. It rejects dogmas such as *"documents must be small"*, *"topic-based is always superior"*, or *"DITA is superior to Wiki-links"*. Instead, it measures the structural consequences of how knowledge is organized and helps users evolve that organization according to the problems they want to solve and the trade-offs they accept.
2. **Documentation Orientations**:
   The system introduces four high-level operational modes:
   - **🤖 AI-Oriented (Default)**: Optimizes knowledge discovery, canonical routing, section addressability, and Context Trace traversal for AI agents (Claude Code, Codex, Antigravity, Roo/Cline).
   - **👤 Human-Oriented**: Prioritizes narrative continuity, readability, conceptual explanation, human navigation, and maintainability.
   - **⚖️ Balanced**: Multi-objective optimization balancing AI retrieval efficiency and human comprehension.
   - **⚙️ Custom**: Unrestricted escape hatch offering two levels of customization (Tier 1 high-level objectives with tooltips + Tier 2 underlying engine parameters with deterministic mapping).
3. **Constraint Hierarchy & Conflict Detection**:
   Distinguishes *Preferences* (soft bias), *Constraints* (penalized trade-off), and *Hard Constraints* (strictly forbidden automated operations). The system deterministically detects and surfaces mathematically contradictory constraint configurations.
4. **10-Dimensional Structural Analytics & The Structural "God Document"**:
   Analyzes 10 structural dimensions: Cohesion, Coupling, Reference Concentration, Granularity, Context Efficiency, Discoverability, Reusability, Semantic Diversity, Navigational Complexity, Maintenance Coupling. An overloaded "God Document" is defined structurally (high diversity + high concentration + low context efficiency + high coupling), NOT by token count alone (a 50k-token cohesive reference can be healthy; a 5k-token overloaded note can be defective).
5. **Knowledge Organization Systems (KOS) Strategy Evaluation & Trade-Offs**:
   Treats classical methodologies (Section Addressing, Topic Decomposition, Lightweight Index Routers, Faceted Classification, Preserve Document, Hybrid Strategies) as candidate responses to detected problem signatures. Evaluates each against the active `OptimizationProfile`, assigning deterministic confidence scores $[0.0, 1.0]$ and explaining countervailing trade-offs.
6. **User-Controlled Downstream Automation & Re-Measurement**:
   Automated transformations are strictly downstream of user choice (`Auto-Apply: OFF`, `Require Approval: ON`). Generates an inspectable `TransformationPlan`, requires explicit user consent, and verifies execution via Before-vs-After comparative re-measurement ($\Delta \text{Context Efficiency}$, $\Delta \text{Trace Depth}$, $\Delta \text{Maintenance Coupling}$).

---

## 2. User Stories & Priority Matrix

### User Story 1: Documentation Orientations & Unified Optimization Profiles (Priority: P1)
> *As an engineer or technical lead, I want to select a Documentation Orientation (`AI-Oriented`, `Human-Oriented`, `Balanced`, or `Custom`) so that LLM Wiki evaluates and optimizes our knowledge base according to our explicit organizational goals.*

- **Acceptance Criteria**:
  1. The system provides 4 selectable orientations: `AI-Oriented` (default), `Human-Oriented`, `Balanced`, and `Custom`.
  2. All orientations compile deterministically into a single internal `OptimizationProfile` struct.
  3. Switching orientations dynamically re-weights structural evaluations and recommendations in $\le 20\text{ ms}$ without restarting the daemon.
  4. The active orientation is persisted in `.context-vault.yaml` under `orientation: ...`.

### User Story 2: Two-Tier Custom Configuration & Informational Tooltips (Priority: P1)
> *As a knowledge architect, I want an understandable two-tier Custom configuration where high-level objectives have clear tooltips explaining trade-offs while advanced parameters remain accessible, so that I have full control without configuration guesswork.*

- **Acceptance Criteria**:
  1. **Tier 1 (High-Level Objectives)**: Exposes 6 high-level knobs: `AI Discoverability`, `Context Efficiency`, `Human Readability`, `Knowledge Reuse`, `Narrative Coherence`, `Maintenance Simplicity` with `Low`, `Medium`, `High` options.
  2. Every Tier 1 knob features an informational bubble (`ⓘ`) explaining: (a) what it controls, (b) higher values meaning, (c) lower values meaning, (d) affected problems, and (e) countervailing trade-offs.
  3. **Tier 2 (Advanced Engine Parameters)**: Directly exposes the underlying weights ($w_i$), thresholds (cohesion $\ge 0.70$, diversity $\ge 0.75$, concentration $\ge 0.65$), and limits (trace depth $\le 3$, context surface $\le 12\text{k}$).
  4. Every Tier 1 setting maps explicitly and deterministically to underlying Tier 2 engine variables with inspectable parameter relationship views.

### User Story 3: Constraint Hierarchy & Conflict Detection Engine (Priority: P1)
> *As a documentation maintainer, I want to define preferences and hard constraints (e.g. "Never automatically split canonical documents"), and be warned if my settings are contradictory, so that the engine never produces impossible or unwanted recommendations.*

- **Acceptance Criteria**:
  1. Allows users to categorize rules into `Preference` (soft bias), `Constraint` (penalized trade-off), and `Hard Constraint` (strict invariant).
  2. Hard constraints are strictly enforced: automated transformation plans MUST NEVER propose violating a hard constraint.
  3. The conflict detector analyzes the constraint set in $\le 10\text{ ms}$ and surfaces actionable warnings when constraints are mathematically irreconcilable.

### User Story 4: 10-Dimensional Structural Analytics & Evidence-Based Signatures (Priority: P1)
> *As an architect, I want LLM Wiki to evaluate 10 structural dimensions across our Markdown files and identify structural problem signatures based on evidence rather than arbitrary file size rules, so that I can fix real architectural debt.*

- **Acceptance Criteria**:
  1. The Rust engine extracts 10 structural dimensions: Cohesion, Coupling, Reference Concentration, Granularity, Context Efficiency, Discoverability, Reusability, Semantic Diversity, Navigational Complexity, Maintenance Coupling.
  2. Documents are evaluated against signature definitions: `GodDocument`, `ImplicitRouter`, `FragmentedConcept`, `IsolatedSilo`.
  3. Document size is treated strictly as a signal, not a verdict: 50k-token cohesive documents are not flagged as God Documents; overloaded 5k-token documents with high diversity and high concentration are accurately detected.
  4. Every problem detection separates raw **Evidence** (metrics) from **Interpretation** (problem signature) and assigns a confidence score $[0.0, 1.0]$.

### User Story 5: Candidate KOS Strategy Evaluation & Trade-Off Matrix (Priority: P2)
> *As a technical author, I want LLM Wiki to evaluate candidate Knowledge Organization strategies for each detected problem and explain their trade-offs, so that I can make an informed decision on how to restructure.*

- **Acceptance Criteria**:
  1. Evaluates candidate strategies: `SectionAddressing`, `TopicDecomposition`, `LightweightIndex`, `FacetedClassification`, `PreserveDocument`, and `Hybrid`.
  2. Compares strategies against the active `OptimizationProfile` and outputs expected benefits, trade-offs, and a confidence score.
  3. Displays a transparent trade-off matrix in the UI.

### User Story 6: User-Controlled Downstream Automation & Re-Measurement (Priority: P2)
> *As a developer, I want to approve automated structural improvements before they are applied, and verify the resulting improvements with Before-vs-After measurements, so that my repository stays clean and under my control.*

- **Acceptance Criteria**:
  1. Automation is strictly downstream of user approval: default settings enforce `Auto-Apply: OFF` and `Require Approval: ON`.
  2. Proposed refactorings generate a previewable `TransformationPlan` detailing proposed actions (create index, update links, inject frontmatter).
  3. Applying an approved plan re-measures the corpus and displays an empirical delta report: $\Delta \text{Context Efficiency}$, $\Delta \text{Trace Depth}$, $\Delta \text{Maintenance Coupling}$.

---

## 3. Functional Requirements (FR-001 – FR-040)

### 3.1 Documentation Orientations & Profiles (FR-001 – FR-008)
- **FR-001**: The system MUST support four mutually exclusive Documentation Orientations: `ai_oriented`, `human_oriented`, `balanced`, and `custom`.
- **FR-002**: The default orientation for all unconfigured repositories MUST be `ai_oriented`.
- **FR-003**: The system MUST compile any selected orientation into a single unified internal `OptimizationProfile` struct in the Rust sidecar.
- **FR-004**: The `ai_oriented` profile MUST set high weights for context efficiency ($\ge 0.85$), section addressability ($\ge 0.80$), canonical routing ($\ge 0.80$), and retrieval depth penalty ($\ge 0.75$).
- **FR-005**: The `human_oriented` profile MUST set high weights for narrative continuity ($\ge 0.85$), document coherence ($\ge 0.80$), and navigational simplicity ($\ge 0.75$), tolerating lower context efficiency.
- **FR-006**: The `balanced` profile MUST enforce a multi-objective compromise where neither AI efficiency nor human readability exceeds a 60/40 ratio.
- **FR-007**: The active orientation MUST be persisted in `.context-vault.yaml` and discoverable at startup.
- **FR-008**: Changing the active orientation in the UI MUST dispatch JSON-RPC `profile/setOrientation`, re-evaluating recommendations in $\le 20\text{ ms}$.

### 3.2 Two-Tier Custom Configuration & Parameter Mapping (FR-009 – FR-016)
- **FR-009**: When orientation is `custom`, the system MUST present a two-tier configuration drawer: Tier 1 (High-Level Objectives) and Tier 2 (Advanced Engine Parameters).
- **FR-010**: Tier 1 MUST provide discrete controls (`low`, `medium`, `high`) for:
  1. `ai_discoverability`
  2. `context_efficiency`
  3. `human_readability`
  4. `knowledge_reuse`
  5. `narrative_coherence`
  6. `maintenance_simplicity`
- **FR-011**: Every Tier 1 knob MUST display an informational tooltip (`ⓘ`) detailing: description, higher value impact, lower value impact, affected problems, and introduced trade-offs.
- **FR-012**: Tier 2 MUST expose underlying continuous parameters: `cohesion_threshold`, `semantic_diversity_threshold`, `reference_concentration_threshold`, `fragment_size_target`, `max_trace_depth`, `candidate_expansion_factor`, `max_context_surface_tokens`, `split_sensitivity`, `canonical_ownership_sensitivity`, `duplicate_tolerance`, `fragmentation_tolerance`.
- **FR-013**: The system MUST provide an inspectable parameter mapping view demonstrating exactly which Tier 2 parameters are influenced by each Tier 1 objective.
- **FR-014**: Custom settings MUST be stored in `.context-vault.yaml` under the `custom_profile:` section.
- **FR-015**: Modifying a Tier 1 objective MUST deterministically update the underlying Tier 2 values according to the documented mapping matrix.
- **FR-016**: Manual adjustments in Tier 2 MUST be preserved and flagged as overridden in the UI.

### 3.3 Constraint Hierarchy & Conflict Detection (FR-017 – FR-022)
- **FR-017**: The system MUST categorize all rules and bounds into three strict levels: `preference`, `constraint`, and `hard_constraint`.
- **FR-018**: A `preference` MUST influence recommendation ranking without filtering candidate strategies.
- **FR-019**: A `constraint` MUST penalize candidate strategies in the objective function ($-\text{penalty}$).
- **FR-020**: A `hard_constraint` MUST strictly disqualify any strategy or automated transformation that would violate it.
- **FR-021**: The system MUST include built-in constraint templates: `never_split_canonical_docs` (strictly prevents splitting canonical architecture or ADR notes), `never_auto_split_documents` (prohibits unattended background/automatic document splitting while permitting explicit user-approved transformation plans to propose decomposition), and `never_delete_content` (strictly prevents automated deletions).
- **FR-022**: The system MUST evaluate the constraint set for mutual satisfiability; if contradictory constraints are detected (e.g. `max_context_surface_tokens <= 5k` + `never_split` + `never_duplicate`), it MUST surface a high-visibility warning card detailing the irreconcilable directives.

### 3.4 10-Dimensional Structural Analytics & Signatures (FR-023 – FR-029)
- **FR-023**: The Rust sidecar MUST compute 10 structural dimensions for each document node:
  1. `cohesion` $[0.0, 1.0]$: semantic term/concept overlap across document sections.
  2. `coupling` $[0.0, 1.0]$: proportion of external document dependencies.
  3. `reference_concentration` $[0.0, 1.0]$: ratio of references targeting the top section versus the document.
  4. `granularity` $[0.0, 1.0]$: ratio of actual fragment token count to optimal target.
  5. `context_efficiency` $[0.0, 1.0]$: relevant tokens retrieved relative to total tokens traversed.
  6. `discoverability` $[0.0, 1.0]$: reachability score from entry hubs.
  7. `reusability` $[0.0, 1.0]$: frequency of targeted section citations without full-document loading.
  8. `semantic_diversity` $[0.0, 1.0]$: distinct topic clusters within the document.
  9. `navigational_complexity` $[0.0, 1.0]$: average hop distance and branch factor to reach content.
  10. `maintenance_coupling` $[0.0, 1.0]$: number of files requiring updates when concepts shift.
- **FR-024**: The system MUST define and detect the structural `GodDocument` signature when:
  $$\text{semantic\_diversity} \ge 0.75 \land \text{reference\_concentration} \ge 0.70 \land \text{context\_efficiency} \le 0.35 \land \text{coupling} \ge 0.70$$
- **FR-025**: The system MUST NOT flag a document as a `GodDocument` based solely on token count or line count.
- **FR-026**: The system MUST detect the `ImplicitRouter` signature when a document has high outbound links ($\ge 20$), low internal token density ($< 500$ words of prose), and high betweenness centrality ($\text{betweenness\_centrality} \ge 0.40$).
- **FR-027**: The system MUST detect the `FragmentedConcept` signature when identical conceptual entities or citations are split across $\ge 3$ weakly linked documents (shortest $\text{hop\_distance} \ge 3 \lor \text{edge\_weight} < 0.20$).
- **FR-028**: Every detected problem signature MUST expose: `signature_id`, `document_path`, `problem_type`, `confidence` $[0.0, 1.0]$, `evidence` (raw metrics), and `rationale`.
- **FR-029**: Raw metrics MUST be stored as exact floats/integers, formatted compactly strictly at the presentation layer ($\le 5$ chars).

### 3.5 Candidate KOS Strategy Evaluator & Trade-Offs (FR-030 – FR-035)
- **FR-030**: For every detected problem signature, the engine MUST evaluate candidate KOS strategies:
  1. `SectionAddressing`: introduce section anchor targets and frontmatter metadata.
  2. `TopicDecomposition`: split distinct topics into focused standalone notes with router linking.
  3. `LightweightIndex`: generate an index router note preserving underlying documents.
  4. `FacetedClassification`: add orthogonal frontmatter categories/tags for multi-dimensional routing.
  5. `PreserveDocument`: maintain existing structure while documenting baseline cost.
  6. `Hybrid`: combine two or more candidate strategies (e.g. index router + section addressing).
- **FR-031**: The engine MUST calculate an expected fitness score for each strategy evaluated against the active `OptimizationProfile`.
- **FR-032**: The engine MUST assign a recommendation confidence score $[0.0, 1.0]$ reflecting evidence strength and profile alignment.
- **FR-033**: Recommendations with confidence $< 0.70$ MUST be labeled as low-confidence advisory suggestions.
- **FR-034**: Every recommendation MUST output an explicit trade-off table itemizing expected benefits and countervailing costs.
- **FR-035**: The Webview UI MUST render recommendation cards in the Settings Drawer under a dedicated `Structural Evolution` tab.

### 3.6 User-Controlled Automation & Re-Measurement (FR-036 – FR-040)
- **FR-036**: The system MUST enforce `Auto-Apply: OFF` and `Require Approval: ON` by default.
- **FR-037**: Selecting a recommendation MUST generate a dry-run `TransformationPlan` detailing:
  - Exact file paths to create, modify, or link.
  - Diff preview of Markdown content and frontmatter changes.
  - Verification that no Hard Constraints are violated.
  - Projected delta metrics ($\Delta \text{Efficiency}$, $\Delta \text{Trace Depth}$, $\Delta \text{Coupling}$).
- **FR-038**: Execution of a `TransformationPlan` MUST require explicit user confirmation (`[Apply Transformation]`).
- **FR-039**: Applying a transformation plan MUST execute atomically: if any file write fails, all changes MUST be rolled back cleanly.
- **FR-040**: Immediately following transformation execution, the engine MUST perform an automated re-measurement, displaying actual Before-vs-After delta metrics.

---

## 4. Non-Functional Requirements (NFR-001 – NFR-009)

- **NFR-001 (Zero External AI)**: 100% of structural metric extraction, problem signature detection, strategy evaluation, and plan generation MUST execute deterministically on the local CPU in Rust without external LLM APIs, remote embeddings, or network access.
- **NFR-002 (Profile Switching Latency)**: Re-weighting and recompiling the `OptimizationProfile` MUST execute in $\le 20\text{ ms}$.
- **NFR-003 (Structural Analytics Latency)**: Computing 10 structural dimensions over 1,000 Markdown documents MUST complete in $\le 150\text{ ms}$.
- **NFR-004 (Strategy Evaluation Latency)**: Evaluating candidate KOS strategies for up to 50 problem signatures MUST complete in $\le 50\text{ ms}$.
- **NFR-005 (Transformation Plan Dry-Run Latency)**: Generating a complete transformation plan with diff previews MUST execute in $\le 100\text{ ms}$.
- **NFR-006 (Re-Measurement Latency)**: Post-transformation incremental graph and metric re-measurement MUST execute in $\le 100\text{ ms}$.
- **NFR-007 (Extension Packaging Size)**: The packaged VSIX file MUST NOT exceed 25 MB.
- **NFR-008 (Memory Budget)**: The Rust daemon resident set size (RSS) MUST NOT exceed 35 MB in idle state for vaults up to 5,000 Markdown files.
- **NFR-009 (Webview Responsiveness)**: The Webview UI MUST maintain 60 FPS during user interactions, routing all analytics queries through asynchronous non-blocking JSON-RPC messages.

---

## 5. Success Criteria (SC-001 – SC-007)

- **SC-001**: 100% of the 4 Documentation Orientations compile deterministically to the unified internal `OptimizationProfile` with zero heuristic drift.
- **SC-002**: Two-tier custom configuration displays informational tooltips for 100% of Tier 1 knobs and detects conflicting constraints in $\le 10\text{ ms}$.
- **SC-003**: Structural "God Document" detection achieves 0 false positives on large cohesive reference files ($>50\text{k}$ tokens with high cohesion) and 100% true positive detection on overloaded documents.
- **SC-004**: 100% of generated recommendations articulate both expected benefits and countervailing trade-offs with deterministic confidence scores.
- **SC-005**: Transformation plans provide complete diff previews and projected deltas prior to file modification.
- **SC-006**: Zero automated violations of user-defined Hard Constraints under any circumstance.
- **SC-007**: Empirical Before-vs-After re-measurement accurately quantifies structural deltas post-transformation.
