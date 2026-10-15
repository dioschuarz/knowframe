---
wiki:
  content_hash: c152150b476effb3a1b8658ce6dbb24b74b21efc36f7831c632fa884a7ab3cf9
  fragments:
    feature-specification-ai-oriented-documentation-governance:
      tokens: 5236
      inbound: 0
      outbound: 0
    user-scenarios-testing-mandatory:
      tokens: 2665
      inbound: 0
      outbound: 0
    user-story-1-deterministic-harness-entry-point-knowledge-index-discovery-priority-p1:
      tokens: 429
      inbound: 0
      outbound: 0
    user-story-2-harness-entry-point-quality-context-efficiency-audit-priority-p2:
      tokens: 404
      inbound: 0
      outbound: 0
    user-story-3-skill-governance-procedure-vs-capability-distinction-priority-p3:
      tokens: 367
      inbound: 0
      outbound: 0
    user-story-4-harness-anchored-context-trace-4-capability-quality-model-priority-p4:
      tokens: 350
      inbound: 0
      outbound: 0
    user-story-5-evidence-based-structural-evolution-for-ai-governance-priority-p5:
      tokens: 363
      inbound: 0
      outbound: 0
    user-story-6-physical-directory-knowledge-organization-analysis-priority-p6:
      tokens: 313
      inbound: 0
      outbound: 0
    edge-cases:
      tokens: 429
      inbound: 0
      outbound: 0
    requirements-mandatory:
      tokens: 1918
      inbound: 0
      outbound: 0
    functional-requirements:
      tokens: 1481
      inbound: 0
      outbound: 0
    1-harness-entry-point-detection-representation:
      tokens: 412
      inbound: 0
      outbound: 0
    2-knowledge-index-anti-secondary-routing-invariants:
      tokens: 326
      inbound: 0
      outbound: 0
    3-skill-governance-procedure-distinction:
      tokens: 211
      inbound: 0
      outbound: 0
    4-physical-directory-organization-analysis:
      tokens: 136
      inbound: 0
      outbound: 0
    5-deterministic-local-engine-context-trace:
      tokens: 160
      inbound: 0
      outbound: 0
    6-user-controlled-structural-evolution-re-measurement:
      tokens: 232
      inbound: 0
      outbound: 0
    key-entities:
      tokens: 431
      inbound: 0
      outbound: 0
    success-criteria-mandatory:
      tokens: 316
      inbound: 0
      outbound: 0
    measurable-outcomes:
      tokens: 309
      inbound: 0
      outbound: 0
    assumptions:
      tokens: 236
      inbound: 0
      outbound: 0
---
# Feature Specification: AI-Oriented Documentation Governance

**Feature Branch**: `008-ai-doc-governance`

**Created**: 2026-09-29

**Status**: Draft

**Input**: User description: "AI-Oriented Documentation Governance: deterministic knowledge architecture evaluating Harness Entry Point, Knowledge Index, Skills, procedures, Markdown documents, references, and directory structure as part of the same knowledge architecture, measuring retrieval, navigation, maintenance, and reuse costs without imposing dogmatic structures."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Deterministic Harness Entry Point & Knowledge Index Discovery (Priority: P1)

As an AI coding agent or developer working in an AI-oriented repository, I need LLM Wiki to deterministically detect configured AI Harness Entry Points (`AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.github/copilot-instructions.md`, or custom configured entry points) and verify that the Knowledge Index is incorporated directly into the entry point without intermediate routing layers, so agents can reliably enter and navigate the repository knowledge base in a single jump.

**Why this priority**: Without a deterministic and authoritative entry point containing an embedded index, AI agents suffer from navigation blind spots, circular routing, or hallucinations trying to discover where architectural guidance and operational procedures reside.

**Independent Test**: Can be fully tested by creating a workspace with `AGENTS.md` containing an embedded `## Knowledge Index` and verifying that the engine detects the harness, parses the index routes, resolves document and fragment targets, and surfaces the harness status with 0 intermediary routing hops.

**Acceptance Scenarios**:

1. **Given** a repository with `AGENTS.md` containing an embedded `## Knowledge Index`, **When** the vault is scanned in `AI-Oriented` mode, **Then** `AGENTS.md` is identified as the authoritative Harness Entry Point, its embedded index routes are indexed as first-class routes, and no secondary routing warnings are issued.
2. **Given** a repository where `AGENTS.md` links to a separate `index.md` or `docs/memory.md` before reaching actual documentation, **When** the vault is scanned, **Then** the engine detects an intermediate routing layer, flags a structural degradation defect, and recommends consolidating the index directly into `AGENTS.md`.
3. **Given** a project configuring multiple harnesses (e.g. `AGENTS.md` and `CLAUDE.md`), **When** scanned, **Then** both entry points are evaluated independently as first-class knowledge objects with UI controls allowing the user to select the primary harness via `setActiveHarness`.

---

### User Story 2 - Harness Entry Point Quality & Context Efficiency Audit (Priority: P2)

As a repository maintainer, I need the system to comprehensively audit the Harness Entry Point across Integrity (broken links, dead anchors, references to excluded files), Structure (section organization, reference concentration, semantic diversity), Consistency (conflicting instructions, duplicate routes), Freshness (stale references to changed/deleted code), and Context Efficiency (token load, ratio of useful routes to noise), so that entry point degradation is flagged based on measurable evidence rather than raw token size alone.

**Why this priority**: The Harness Entry Point is loaded into every AI prompt or session. Undetected broken references waste tokens, outdated routes cause wrong tool calls, and conflicting instructions degrade reasoning.

**Independent Test**: Can be tested by introducing broken wikilinks, stale code paths, and conflicting routes into `AGENTS.md` and verifying that the core quality engine produces specific, itemized quality issues under Integrity, Consistency, Freshness, and Context Efficiency with exact line numbers and remediation hints.

**Acceptance Scenarios**:

1. **Given** an `AGENTS.md` containing a broken link `[[non_existent_note]]` and a broken section anchor `[[architecture#deleted-subsystem]]`, **When** quality evaluation runs, **Then** both defects are reported with penalty points and line-level navigation links.
2. **Given** an `AGENTS.md` of 40,000 tokens with high cohesion, high index coverage, and 0 broken routes, **When** evaluated, **Then** it is classified as structurally sound (Invariant XIII: size is a signal, not a verdict).
3. **Given** an `AGENTS.md` of 6,000 tokens with low context efficiency (< 40%), circular references, and low routing density, **When** evaluated, **Then** the engine detects structural degradation and recommends decomposing or structuring it into clear domain routes.

---

### User Story 3 - Skill Governance & Procedure vs. Capability Distinction (Priority: P3)

As a developer maintaining automated agent capabilities, I need LLM Wiki to distinguish between one-time procedures and reusable Skills (`SKILL.md` with optional `scripts/`, `templates/`, `references/`), audit Skill definitions for structural validity and broken references, and detect candidates for new Skills when repeated multi-step procedures appear across the documentation corpus.

**Why this priority**: Reusable agent capabilities require structured governance. One-off prompt instructions clutter context, while uncurated skills accumulate broken script paths, duplicate capabilities, and orphaned templates.

**Independent Test**: Can be tested by adding a valid skill folder (`.agents/skills/deploy-service/`), an invalid skill missing `SKILL.md`, and a repeated 5-step deployment procedure in three separate markdown notes, verifying that the system validates the healthy skill, flags the broken skill, and surfaces a Skill Candidate recommendation for the repeated procedure.

**Acceptance Scenarios**:

1. **Given** a directory matching the Skill structure (`SKILL.md` + supporting assets), **When** scanned, **Then** it is recognized as a first-class `SkillDefinition`, its internal references to scripts/templates are validated, and it is checked for overlapping capabilities against existing skills.
2. **Given** a directory claiming skill semantics but lacking `SKILL.md` or possessing broken script references, **When** scanned, **Then** a structural/integrity finding is raised detailing the missing entry point or invalid asset path.
3. **Given** identical or highly similar procedural steps appearing across 3 or more documents, **When** structural analytics execute, **Then** the engine identifies a `ProcedureCandidate` and suggests promoting it to a dedicated Skill (`CreateSkillFromProcedure`).

---

### User Story 4 - Harness-Anchored Context Trace & 4-Capability Quality Model (Priority: P4)

As an engineer querying or optimizing project documentation, I need Context Trace to initiate directly from the configured Harness Entry Point down through the embedded Knowledge Index and section fragments, evaluating the four AI-Oriented capabilities (Entry, Discovery, Retrieval, Reuse) across the 5 quality pillars without calling external LLM APIs.

**Why this priority**: AI agents always start context exploration from the entry point. Evaluating retrieval paths from arbitrary isolated nodes fails to measure how easily an agent can actually navigate from entry point to solution.

**Independent Test**: Can be tested by executing a simulated query trace from `AGENTS.md` to a specific section `docs/security/authentication.md#token-refresh`, verifying that the trace calculates path hops, accumulated BPE tokens, relevance progression, and evaluates Entry/Discovery/Retrieval/Reuse metrics locally.

**Acceptance Scenarios**:

1. **Given** a user query, **When** Context Trace runs in `AI-Oriented` mode, **Then** the root traversal node is locked to the active Harness Entry Point, navigating through the embedded index routes to the target document fragment.
2. **Given** a document with no incoming route from the Harness Entry Point or Knowledge Index, **When** Context Trace evaluates reachability, **Then** the document is flagged with an "Unrouted Knowledge" Discoverability penalty.
3. **Given** a trace resolving in $\le 2$ hops and $< 3,000$ tokens, **When** scored, **Then** Retrieval and Context Efficiency capabilities receive optimal scores ($> 90\%$).

---

### User Story 5 - Evidence-Based Structural Evolution for AI Governance (Priority: P5)

As a repository owner whose documentation structure has become fragmented or unindexed, I need LLM Wiki to recommend user-controlled structural transformations (embedding index routes, removing secondary routing layers, establishing skills, section addressing) with Before-vs-After verification, strictly enforcing `Auto-Apply: OFF` and `Require Approval: ON` with explicit approval tokens without creating secondary hidden routing files (`hot-index.md`, `memory.md`).

**Why this priority**: When documentation degrades, automated agents must not arbitrarily rewrite files or introduce shadow memory files that hide structural rot. Users must review and approve deterministic transformations.

**Independent Test**: Can be tested by generating a transformation plan that embeds missing index routes into `AGENTS.md` and eliminates an intermediate `index.md`, inspecting the unified diff, approval token, and projected deltas, applying the change via user approval token, and verifying post-apply re-measurement.

**Acceptance Scenarios**:

1. **Given** an entry point missing routes to existing core architectural documents, **When** structural diagnosis completes, **Then** a candidate strategy proposing index route insertion (`EmbedKnowledgeIndex`) is generated with an explicit trade-off matrix and unique `approval_token`.
2. **Given** a generated transformation plan, **When** inspected in the UI, **Then** no file modifications occur until the user approves the plan, dispatching `governance/applyGovernancePlan` with the valid `approval_token` (`Auto-Apply: OFF`, Invariant XVI).
3. **Given** an approved transformation applied to disk, **When** post-apply re-measurement executes, **Then** an empirical delta report shows before vs. after Context Efficiency and Trace Depth.

---

### User Story 6 - Physical Directory & Knowledge Organization Analysis (Priority: P6)

As an architect organizing a large codebase, I need the system to analyze the physical directory hierarchy for excessive depth, document overconcentration, scattered related knowledge, or mixed domains, recommending directory consolidation only when evidence indicates measurable retrieval or maintenance costs.

**Why this priority**: Arbitrary folder reorganizations break bookmarks and git history. Restructuring must only be recommended when multi-dimensional metrics prove that the physical layout hinders navigation and discovery.

**Independent Test**: Can be tested by placing 150 unrelated notes in a single root folder or nesting notes 8 levels deep, verifying that structural analytics flag high concentration/depth signals and propose domain grouping without prescribing a dogmatic folder hierarchy.

**Acceptance Scenarios**:

1. **Given** a folder containing 100+ documents with disparate semantic tags and zero internal clustering (concentration entropy $< 0.35$), **When** analyzed, **Then** a high concentration and domain mixing signal is surfaced.
2. **Given** related documents consistently cross-referenced but scattered across distant directories (nesting depth $> 5$ or cross-folder coupling $> 0.75$), **When** analyzed, **Then** high navigational complexity is detected with a recommended domain consolidation strategy (`RestructureDirectoryDomain`).
3. **Given** a flat `docs/` folder with 15 cohesive documents, **When** analyzed, **Then** no directory restructuring is recommended (non-prescriptive organization).

---

### Edge Cases

- **Missing Harness Entry Point**: When `AI-Oriented` mode is active but no supported entry point (`AGENTS.md`, `CLAUDE.md`, etc.) exists, the system flags a P1 configuration issue and offers a one-click action to initialize a template `AGENTS.md` with an embedded Knowledge Index.
- **Multiple Configured Harnesses**: When a repository contains both `AGENTS.md` and `.github/copilot-instructions.md`, the system audits both entry points, allows the user to select the primary trace entry via `setActiveHarness`, and verifies that shared routing information remains consistent.
- **Circular or Dangling Routes**: An embedded index route points to a document that links back to the index as its only outbound link, or points to an anchor that does not exist. Handled by graph cycle detection and fragment anchor validation, reporting dead ends.
- **Shadow Routing / Memory Files**: A repository contains `memory.md`, `hot-index.md`, or `context_map.md` attempting to act as secondary dispatchers. The engine flags these as architectural anti-patterns that fragment agent attention, recommending consolidation into the primary entry point via `EliminateSecondaryRoutingLayer`.
- **One-off Script vs. Reusable Skill**: A directory contains a single python script and a readme describing a one-time migration. Evaluated as a task/procedure rather than a Skill, avoiding false-positive "missing SKILL.md" errors.
- **Excluded Path Referencing**: A harness entry point references documents located in `scan.exclude` or `graph.exclude`. Handled by classifying `scan.exclude` references as Integrity defects (unresolvable) while allowing `graph.exclude` references to resolve cleanly per the Target Resolvability Invariant.
- **Large Monolithic Entry Point**: An `AGENTS.md` file reaches 50k tokens. If reference concentration is balanced, sections are well-defined, and context traces are short, it is NOT penalized purely for size. If token efficiency is low (< 40%) and routes are ambiguous, it triggers a `DecomposeHarness` recommendation.

---

## Requirements *(mandatory)*

### Functional Requirements

#### 1. Harness Entry Point Detection & Representation
- **FR-001**: System MUST deterministically detect supported AI Harness Entry Points (`AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.github/copilot-instructions.md`, and user-defined custom entry paths) at workspace root or configured harness locations.
- **FR-002**: System MUST require at least one valid Harness Entry Point to be configured when `AI-Oriented` documentation mode is active, providing an automated one-click action to initialize a standard `AGENTS.md` template with embedded Knowledge Index if absent.
- **FR-003**: System MUST support configuration and concurrent analysis of multiple independent harnesses without assuming a single universal entry point format.
- **FR-004**: System MUST represent the Harness Entry Point as a first-class knowledge object within the Knowledge Graph and analytical model.
- **FR-005**: System MUST evaluate the Harness Entry Point across Integrity (broken links, broken fragment anchors, references to non-existent or excluded documents).
- **FR-006**: System MUST evaluate the Harness Entry Point across Structure (size, section organization, structural complexity, reference concentration, semantic diversity, fragmentation).
- **FR-007**: System MUST evaluate the Harness Entry Point across Discoverability (Knowledge Index coverage, routing quality, accessibility, context trace depth).
- **FR-008**: System MUST evaluate the Harness Entry Point across Consistency (conflicting instructions, duplicated routes, inconsistent terminology or aliases).
- **FR-009**: System MUST evaluate the Harness Entry Point across Freshness (stale references, outdated routes, drift against repository structure).
- **FR-010**: System MUST evaluate the Harness Entry Point across Context Efficiency (token count, context surface, ratio of useful routing information to irrelevant content).
- **FR-011**: System MUST NOT treat document size as an isolated defect; large entry points MUST only trigger structural findings when accompanied by measurable costs (context efficiency ratio $< 40\%$, high diversity $> 0.60$, or dead routes).

#### 2. Knowledge Index & Anti-Secondary Routing Invariants
- **FR-012**: In `AI-Oriented` mode, the Knowledge Index MUST be directly incorporated into the configured Harness Entry Point.
- **FR-013**: System MUST identify and flag intermediate or secondary routing/memory layers (e.g., `AGENTS.md` -> `index.md` -> docs, or `AGENTS.md` -> `memory.md` -> `index.md`, `hot-index.md`) as structural defects.
- **FR-014**: The Harness Entry Point MUST serve as the authoritative single entry point for AI knowledge discovery.
- **FR-015**: The embedded Knowledge Index MUST provide deterministic routes to major knowledge domains, documents, and addressable section fragments (`doc.md#fragment`).
- **FR-016**: When Section Addressing is available, routes SHOULD target relevant section anchors rather than entire documents via `IntroduceSectionAddressing` when this reduces context surface.
- **FR-017**: System MAY contain and evaluate semantic routing information within the Harness Entry Point, formatted as Markdown list items pairing a concept label and target link with optional inline scope descriptions.
- **FR-018**: System MUST derive routing candidates deterministically from document titles, section headings, aliases, tags, file paths, existing references, topology, Context Trace results, and repeated navigation patterns.
- **FR-019**: System MUST NOT fabricate knowledge or routes that cannot be resolved to existing repository content.
- **FR-020**: System MUST recommend structural evolution rather than introducing hidden or secondary routing artifacts when the Harness Entry Point becomes degraded or overloaded.

#### 3. Skill Governance & Procedure Distinction
- **FR-021**: System MUST treat a Skill as a reusable capability comprising a primary `SKILL.md` entry point and optional supporting resources (`scripts/`, `templates/`, `references/`), distinct from a generic prompt or one-off task.
- **FR-022**: System MUST distinguish between one-time task instructions/procedures and reusable multi-step capabilities based on evidence of reuse, recurrence, operational complexity, and associated artifacts.
- **FR-023**: System MUST detect candidates for Skills when deterministic evidence reveals repeated procedures (recurrence score $\ge 0.70$), identical ordered steps across documents, associated automation scripts, or recurring Context Trace patterns.
- **FR-024**: System MUST include Skills in the Knowledge Model and audit them for presence of `SKILL.md`, structural validity, reference validity, script/template validity, orphaned status, duplication, overlapping capabilities, and discoverability (proposing `ConsolidateDuplicateSkills` when overlap exceeds $80\%$).

#### 4. Physical Directory & Organization Analysis
- **FR-025**: System MUST evaluate physical directory and file organization in AI-Oriented mode without imposing a universal folder taxonomy.
- **FR-026**: System MUST detect physical organizational signals based on calibrated thresholds: excessive navigation depth ($> 5$), document overconcentration (Shannon entropy $< 0.35$), domain mixing ($> 0.60$), and cross-folder coupling ($> 0.75$).
- **FR-027**: Directory restructuring (`RestructureDirectoryDomain`) MUST be recommended only when empirical evidence indicates measurable retrieval, navigation, or maintenance costs exceeding defined thresholds.

#### 5. Deterministic Local Engine & Context Trace
- **FR-028**: All structural analysis, AST parsing, harness detection, route resolution, and metric computation MUST be deterministic, local, and executed in the Rust core sidecar with zero external LLM/AI API calls.
- **FR-029**: Context Trace MUST initiate from the actual configured Harness Entry Point when evaluating AI-Oriented documentation, measuring trace depth, token accumulation, context surface, relevance progression, and redundant hops.
- **FR-030**: System MUST evaluate the four primary AI-Oriented capabilities (Entry, Discovery, Retrieval, Reuse) synthesized across the 5 quality pillars (Integrity, Structure, Consistency, Discoverability, Freshness) and compiled into the active `OptimizationProfile`.

#### 6. User-Controlled Structural Evolution & Re-Measurement
- **FR-031**: System MUST execute structural transformations through a strict, user-controlled approval workflow ($\text{Evidence} \to \text{Structural Problem} \to \text{Candidate Strategies} \to \text{Fit Score} \to \text{Compatibility} \to \text{User Selection} \to \text{Preview} \to \text{Approval Gate} \to \text{Apply} \to \text{Re-Analyze}$) with `Auto-Apply: OFF` and `Require Approval: ON`, validating a generated `approval_token` before applying disk changes.
- **FR-032**: System MUST perform empirical Before-vs-After comparative re-measurement following any applied structural transformation to quantify deltas in Context Efficiency, Trace Depth, and Maintenance Coupling.
- **FR-033**: System MUST provide interactive visual diagnostics and configuration controls within the VS Code Extension (Manager Sidebar and Settings Drawer) allowing users to inspect harness health, toggle the primary harness definition (`setActiveHarness`), view skill integrity, and preview evolution plans.

---

### Key Entities

- **`HarnessEntryPoint`**: Represents a detected or configured AI entry point document (`AGENTS.md`, `CLAUDE.md`, etc.). Attributes: file path, harness type, status (valid, degraded, missing), embedded index boundary ranges, declared routes, total BPE tokens, and composite quality ratings.
- **`KnowledgeIndexRoute`**: A deterministic navigational mapping embedded in the harness entry point. Attributes: source heading/category, label/alias, target document path, target section anchor, resolution status (resolved, dead link, dead anchor, excluded), hop distance, and projected context load.
- **`SecondaryRoutingLayer`**: Represents an anti-pattern routing layer detected in the repository (`index.md`, `memory.md`, `hot-index.md`). Attributes: file path, parent entry point, intermediate hops introduced, downstream document count, and structural penalty points.
- **`SkillDefinition`**: Represents a reusable agent capability. Attributes: directory path, primary manifest (`SKILL.md`), name, description, supporting assets (`scripts/`, `templates/`, `references/`), validation state (valid, missing manifest, dangling asset, orphan), and overlap index with other skills.
- **`ProcedureCandidate`**: A sequence of operational steps detected across documents. Attributes: ordered step signatures, occurrences, file locations, associated script paths, recurrence score, and recommendation status (candidacy for promotion to Skill).
- **`AIOrientedCapabilityMatrix`**: Scores measuring the 4 key AI-Oriented capabilities:
  - `EntryScore`: Deterministic accessibility and completeness of the Harness Entry Point.
  - `DiscoveryScore`: Knowledge Index coverage and route discoverability across repository domains.
  - `RetrievalScore`: Context Trace hop efficiency and token surface minimization.
  - `ReuseScore`: Skill validity, procedure consolidation, and operational asset availability.
- **`HarnessEvolutionRecommendation`**: A proposed structural transformation targeting AI governance. Attributes: target problem signature, strategy type (`GovernanceStrategyType`), approval token, diff preview, trade-off matrix, and user approval token.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% deterministic detection of configured Harness Entry Points across workspace without external network calls or manual file path entry.
- **SC-002**: Context Trace initiating from the Harness Entry Point resolves 90%+ of a benchmark query suite (synthesized from document titles, section headings, and route labels) in $\le 3$ hops and $\le 10,000$ tokens traversed.
- **SC-003**: 100% of intermediate routing layers (`AGENTS.md` -> `index.md` -> docs) flagged as structural defects with actionable consolidation recommendations (`EliminateSecondaryRoutingLayer`).
- **SC-004**: All Skill structures validated for `SKILL.md` presence, script/template existence, and broken references with 0 false-positive broken links for relative script/template targets.
- **SC-005**: 100% of structural evolution transformations enforce user approval gates (`Auto-Apply: OFF`, `Require Approval: ON`) requiring a validated `approval_token` before modifying files on disk.
- **SC-006**: Core AST parsing, harness detection, route resolution, and Context Trace analysis execute locally in $\le 100\text{ ms}$ for 1,000 Markdown documents adhering to Constitution performance budgets.
- **SC-007**: Every applied transformation produces an empirical Before-vs-After delta report measuring $\Delta\text{Context Efficiency}$, $\Delta\text{Trace Depth}$, and $\Delta\text{Maintenance Coupling}$.

---

## Assumptions

- **Markdown Standards**: Harness Entry Points, Knowledge Indexes, and Skills follow CommonMark and GitHub Flavored Markdown standards. Section anchors follow standard slugification rules.
- **Configured Harness Types**: When unconfigured, the system checks for `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, and `.github/copilot-instructions.md` at root in order of appearance, defaulting to `AGENTS.md` if multiple or none are found.
- **Skill Structure Convention**: Skills follow the standard directory layout (`SKILL.md` root with subdirectories for scripts, templates, and references). Non-conforming script directories lacking a `SKILL.md` are evaluated as procedural assets rather than corrupted skills unless explicitly registered.
- **Non-Prescriptive Storage**: Repositories are free to store documentation in `docs/`, `notes/`, `wiki/`, or flat at root; the governance engine measures retrieval efficiency and coupling rather than enforcing directory naming.
- **User Agency**: The system never automatically rewrites `AGENTS.md` or moves files without explicit user review and approval in the transformation modal with an approval token.
