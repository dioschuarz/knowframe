---
wiki:
  content_hash: f4c9ac8c744acace95fd3752bb5b6b912f7c6a9dc600f8b5f39b45ac3a29c103
  fragments:
    feature-specification-llm-wiki-management-experience-structural-evolution-ui:
      tokens: 7912
      inbound: 0
      outbound: 0
    1-executive-summary-vision:
      tokens: 671
      inbound: 0
      outbound: 0
    the-52-section-ux-redesign-thesis:
      tokens: 70
      inbound: 0
      outbound: 0
    the-four-knowledge-pillars:
      tokens: 207
      inbound: 0
      outbound: 0
    the-10-core-design-principles-p1-p10:
      tokens: 328
      inbound: 0
      outbound: 0
    2-user-scenarios-testing:
      tokens: 3111
      inbound: 0
      outbound: 0
    user-story-1-decision-oriented-manager-summary-priority-p1:
      tokens: 427
      inbound: 0
      outbound: 0
    user-story-2-consolidated-health-finding-vs-score-separation-priority-p1:
      tokens: 298
      inbound: 0
      outbound: 0
    user-story-3-unambiguous-structure-terminology-graph-relationship-priority-p1:
      tokens: 262
      inbound: 0
      outbound: 0
    user-story-4-problem-first-structural-opportunities-human-readable-evidence-priority-p1:
      tokens: 384
      inbound: 0
      outbound: 0
    user-story-5-strategy-fit-hybrid-recommendations-transparent-trade-offs-priority-p1:
      tokens: 355
      inbound: 0
      outbound: 0
    user-story-6-impact-preview-user-approval-post-change-re-measurement-priority-p2:
      tokens: 434
      inbound: 0
      outbound: 0
    user-story-7-insights-vs-settings-separation-cohesive-navigation-priority-p2:
      tokens: 320
      inbound: 0
      outbound: 0
    user-story-8-progressive-disclosure-informational-bubbles-recommendation-language-priority-p2:
      tokens: 331
      inbound: 0
      outbound: 0
    edge-cases:
      tokens: 290
      inbound: 0
      outbound: 0
    3-functional-requirements-fr-001-fr-046:
      tokens: 2513
      inbound: 0
      outbound: 0
    31-manager-card-architecture-layout-fr-001-fr-010:
      tokens: 527
      inbound: 0
      outbound: 0
    32-health-dimensions-findings-vs-score-link-consolidation-fr-011-fr-017:
      tokens: 301
      inbound: 0
      outbound: 0
    33-terminology-standardization-graph-responsibilities-fr-018-fr-021:
      tokens: 302
      inbound: 0
      outbound: 0
    34-problem-first-structural-opportunities-10-d-evidence-metrics-fr-022-fr-029:
      tokens: 449
      inbound: 0
      outbound: 0
    35-strategy-fit-hybrid-recommendations-trade-off-matrix-fr-030-fr-036:
      tokens: 340
      inbound: 0
      outbound: 0
    36-preview-before-automation-post-change-re-measurement-fr-037-fr-040:
      tokens: 272
      inbound: 0
      outbound: 0
    37-insights-vs-settings-separation-cohesive-interaction-model-fr-041-fr-046:
      tokens: 307
      inbound: 0
      outbound: 0
    4-non-functional-requirements-nfr-001-nfr-010:
      tokens: 493
      inbound: 0
      outbound: 0
    5-success-criteria-sc-001-sc-008:
      tokens: 385
      inbound: 0
      outbound: 0
    6-key-entities-ui-models:
      tokens: 453
      inbound: 0
      outbound: 0
    61-managersummarystate:
      tokens: 130
      inbound: 0
      outbound: 0
    62-optimizationprofile-invariant-xiv:
      tokens: 35
      inbound: 0
      outbound: 0
    63-structuralopportunity:
      tokens: 135
      inbound: 0
      outbound: 0
    64-strategyrecommendation:
      tokens: 143
      inbound: 0
      outbound: 0
    7-assumptions-dependencies:
      tokens: 166
      inbound: 0
      outbound: 0
---
# Feature Specification: LLM Wiki Management Experience & Structural Evolution UI

**Feature Branch:** `006-management-experience-ui`  
**Target Delivery:** `llm_wiki_ext4vsc` (VS Code Extension + Rust Core `llm-wiki-core`)  
**Status:** In Review / Specification  
**Constitution Authority:** `CONSTITUTION.md` v1.6.0 (The 16 Critical Invariants, Principles I through XII)  
**External AI / Remote LLM:** **STRICTLY PROHIBITED** (Local Deterministic Execution Only)  

---

## 1. Executive Summary & Vision

The `006-management-experience-ui` specification transitions the LLM Wiki VS Code extension from an **engine diagnostic console** into a **calm, decision-oriented management control center**. It establishes a clear, rigorous information architecture centered around user questions and decisions rather than internal engine mechanics.

### The 52-Section UX Redesign Thesis
> **LLM Wiki should not feel like a dashboard for a knowledge graph engine. It should feel like a system that understands the user's knowledge structure, identifies where that structure creates measurable costs, explains why, exposes alternative ways to evolve it, and makes those changes controllable and measurable.**

### The Four Knowledge Pillars
The interface organizes all user interactions around four core questions:
1. **Health** — *What is wrong or requires attention?* (Quality dimensions, actionable findings vs. score separation, consolidated broken references).
2. **Structure** — *How is my knowledge organized?* (Standardized terminology: documents as nodes, references as edges, connected vs. isolated documents, explore graph trigger).
3. **Evolution** — *How can my knowledge evolve?* (Problem-first structural opportunities, human-readable 10-D evidence metrics with `ⓘ` tooltips, strategy fit scores, multi-strategy/hybrid recommendations, explicit trade-offs, preview before automation, post-change re-measurement).
4. **Settings** — *How is the system configured?* (Strict separation between **Insights** [Health, Structure, Evolution] and **Settings** [Knowledge, Exclusions, Clusters, Git/Harness, Advanced], gear icon for Advanced Settings, unified Manager ↔ Detail panel interaction).

### The 10 Core Design Principles (P1 – P10)
- **P1 — Decision over diagnostics**: Show information because it helps the user understand or decide, not merely because the engine can calculate it.
- **P2 — Problem before method**: Identify the structural problem before presenting organizational strategies (superseding the raw "Methods" tab).
- **P3 — Evidence before recommendation**: Every recommendation must be traceable to observable, multi-dimensional metrics.
- **P4 — Trade-offs must be explicit**: Every structural strategy has both benefits and countervailing costs.
- **P5 — Orientation defines optimization**: Orientations (`AI-Oriented`, `Human-Oriented`, `Balanced`, `Custom`) define what the system optimizes for.
- **P6 — Custom removes the ceiling**: Advanced users can tune high-level objectives (Tier 1) and underlying parameters (Tier 2) with deterministic mappings.
- **P7 — Progressive disclosure**: 5-level hierarchy: What matters? (L1) → Why? (L2) → What evidence? (L3) → What can I do? (L4) → How does engine calculate it? (L5).
- **P8 — No universal organization**: LLM Wiki evaluates organizational choices against concrete trade-offs rather than prescribing methodology dogmas (Invariant XIII).
- **P9 — Manager and detail are one system**: Selecting an item in the Manager navigates directly to the corresponding detail panel context.
- **P10 — Measure the result**: Every automated structural change must be re-measured and verified against its motivating objectives.

---

## 2. User Scenarios & Testing

### User Story 1: Decision-Oriented Manager Summary (Priority: P1)
> *As a software developer or architect, I want the LLM Wiki Manager in the VS Code sidebar to present a clean, 4-card persistent summary answering what matters at a glance, so that I can immediately grasp health, structure, evolution, and sync state without being overwhelmed by diagnostic noise.*

**Why this priority**: Solves the primary UX failure of information overload and competing controls. Establishes the entry point for the entire extension.  
**Independent Test**: Can be tested by loading vaults of varying sizes and verifying that the sidebar renders exactly 4 cards (**Health**, **Structure**, **Evolution**, **Sync**) with decision-relevant metrics and deep-linking action buttons.

**Acceptance Scenarios**:
1. **Given** an open workspace with 51 Markdown documents, **When** the LLM Wiki Manager sidebar loads, **Then** it renders:
   - **Health Card**: Overall Quality Score (e.g. `38/100`), 5 dimension bars (`Integrity`, `Structure`, `Consistency`, `Discoverability`, `Freshness`), actionable finding count (`6 findings need attention`), and button `[Review health]`.
   - **Structure Card**: Total documents (`51 documents`), connectivity breakdown (`4 connected documents · 47 isolated documents`), total references (`72 references`), and button `[Explore graph]`.
   - **Evolution Card**: Active orientation badge (`🤖 AI-Oriented`), brief optimization summary ($\le 60$ chars), opportunity count (`3 structural opportunities`), and button `[Review evolution]`.
   - **Sync Card**: Synchronization state (`● Synced`), and last committed Git SHA (`Last commit · abc1234`).
2. **Given** a vault with zero broken links and full connectivity, **When** the Manager renders, **Then** the Health Card displays `0 findings need attention` and button `[Review health]`, while the Structure Card displays `X connected documents · 0 isolated documents`.

---

### User Story 2: Consolidated Health & Finding vs. Score Separation (Priority: P1)
> *As a documentation maintainer, I want the UI to strictly separate Quality Scores from Finding Counts and consolidate broken link counters into a single Integrity context, so that I am never confused between an aggregate health percentage and raw issue counts.*

**Why this priority**: Eliminates action duplication and cognitive confusion caused by parallel counters ("6 broken links", "Integrity 6", "6 broken references").  
**Independent Test**: Introduce broken wikilinks in a test vault; verify that the Manager displays a single consolidated finding count, and clicking `[Review health]` or the Integrity row navigates directly to the Health Detail tab with the broken references list.

**Acceptance Scenarios**:
1. **Given** a vault with 4 broken references and 2 invalid heading anchors, **When** the user inspects the Health Card, **Then** the UI displays `38/100` as the Quality Score and `6 findings need attention` as findings, without conflating the score number with the finding count.
2. **Given** the Health Card, **When** the user clicks `Integrity` or `[Review health]`, **Then** the detail drawer opens to the Health tab with the Integrity section expanded, detailing `4 broken references` and `2 invalid anchors`, with action `[Fix reference]` contextualized per finding rather than floating globally.

---

### User Story 3: Unambiguous Structure Terminology & Graph Relationship (Priority: P1)
> *As a user exploring my knowledge base, I want the Structure card to use clear, standardized terminology (`documents`, `references`, `connected documents`, `isolated documents`) and provide an immediate trigger to explore the graph, so that I understand the relationship between corpus metrics and the visual topology.*

**Why this priority**: Replaces ambiguous labels (`4 CONNECTED`, `47 ORPHANS`, `72 LINKS`) with unambiguous graph terminology where documents are nodes and references are edges.  
**Independent Test**: Verify that the Structure card renders standardized labels and clicking `[Explore graph]` opens or focuses the interactive 2D Canvas Webview.

**Acceptance Scenarios**:
1. **Given** a vault with 51 Markdown files, 4 within the link graph and 47 without links, **When** viewing the Structure Card, **Then** the labels strictly read `51 documents`, `4 connected documents`, `47 isolated documents`, and `72 references`.
2. **Given** the Structure Card, **When** the user clicks `[Explore graph]`, **Then** the extension executes command `llmWiki.showKnowledgeGraph`, opening or revealing the Knowledge Graph editor tab.

---

### User Story 4: Problem-First Structural Opportunities & Human-Readable Evidence (Priority: P1)
> *As an architect reviewing knowledge structure, I want structural opportunities to be presented problem-first with human-readable evidence metrics and `ⓘ` informational tooltips rather than cryptic metric abbreviations or methodology dogma, so that I understand exactly what structural inefficiency exists and why.*

**Why this priority**: Replaces the abstract "Methods" tab with actionable, evidence-based opportunities. Adheres to Invariants XIII and XV.  
**Independent Test**: Load a vault with an overloaded 50k-token document; verify that the Evolution panel surfaces `Structurally Overloaded Document` (or `High Context Waste`), lists full human-readable metrics with `ⓘ` bubbles, and avoids cryptic abbreviations like `CONC` or `DIV`.

**Acceptance Scenarios**:
1. **Given** a detected structural opportunity on `Architecture.md`, **When** the user views the opportunity card, **Then** it presents the human-readable problem title (`High Context Waste` or `Structurally Overloaded Document`), confidence score (`87% confidence` mapped to `High`), a narrative `WHY` section, and an `EVIDENCE` section displaying:
   - `Document size: 50k tokens`
   - `Reference concentration: 74%`
   - `Navigational complexity: 4.8 hops`
   - `Context efficiency: 19%`
2. **Given** any metric in the Evidence or Custom settings panel, **When** the user hovers over or clicks its `ⓘ` icon, **Then** a tooltip renders answering:
   - What does this mean?
   - What does a higher/lower value imply?
   - Why does it matter?
   - What does it affect?
   - What trade-off does it introduce?

---

### User Story 5: Strategy Fit, Hybrid Recommendations & Transparent Trade-Offs (Priority: P1)
> *As an engineer evaluating evolution paths, I want each candidate strategy to display an explainable Fit Score, expected benefits, countervailing trade-offs, and support hybrid combinations, so that I make an informed decision aligned with our documentation orientation.*

**Why this priority**: Enforces Invariant XV (Transparent Trade-Offs) and provides multi-strategy flexibility without dogmatic "best method" assertions.  
**Independent Test**: Trigger recommendations for a high context waste problem; verify that the UI displays strategies with Fit %, benefits ($\downarrow$ Context surface, $\uparrow$ Knowledge reuse), trade-offs ($\uparrow$ Granularity, link complexity), and supports hybrid pairings.

**Acceptance Scenarios**:
1. **Given** a structural opportunity, **When** recommendations are displayed, **Then** candidate strategies list:
   - Strategy Name (e.g. `Section Addressing`, `Lightweight Index Router`, `Topic Decomposition`)
   - Fit Score with `ⓘ` tooltip explaining alignment with current orientation and constraints
   - Expected Benefits (e.g. `↓ Context surface`, `↓ Retrieval depth`, `↑ Knowledge reuse`)
   - Countervailing Trade-Offs (e.g. `May increase structural granularity and link complexity`)
   - Estimated Effort (`Low`, `Medium`, `High`)
   - Action `[Preview change]`
2. **Given** a problem where both routing and sectioning are beneficial, **When** the engine evaluates the candidate set, **Then** it can present a `Recommended Approach: Section Addressing + Lightweight Index` with explicit justification for the combination.

---

### User Story 6: Impact Preview, User Approval & Post-Change Re-Measurement (Priority: P2)
> *As a project lead, I want to inspect a clear before-and-after impact preview showing both benefits and costs before applying any structural transformation, and see empirical post-change re-measurements evaluated without violating Invariant I committed truth, so that automated changes remain completely safe and verifiable.*

**Why this priority**: Enforces Invariant XVI (User-Controlled Downstream Automation & Re-Measurement) and Invariant I (Committed Truth). Guarantees zero unintended repository changes.  
**Independent Test**: Click `[Preview change]` on a strategy; verify modal/panel displays Before $\to$ After metrics; click `[Apply changes]`; verify automated transformation writes files and calculates re-measurement on an ephemeral candidate graph.

**Acceptance Scenarios**:
1. **Given** a selected transformation strategy, **When** the user clicks `[Preview change]`, **Then** the UI opens a preview displaying:
   - Target document(s)
   - Expected Impact: Before $\to$ After comparisons (e.g. Context efficiency `32% → 81%`, Navigational complexity `4.8 → 2.1`, Documents `51 → 54`, References `72 → 79`)
   - Explicit Trade-offs (e.g. `Document count increases by 3`)
   - Action buttons: `[Cancel]` and `[Apply changes]`
2. **Given** the preview modal, **When** the user clicks `[Apply changes]`, **Then** the extension applies the transformation to the workspace, evaluates the result on an in-memory ephemeral candidate graph (preserving Invariant I committed truth until a Git commit occurs), and renders the Post-Change Measurement card displaying empirical deltas ($\Delta \text{Context Efficiency}: +49\%$, $\Delta \text{Navigational Complexity}: -56\%$, $\Delta \text{Maintenance Coupling}: -23\%$, $\Delta \text{Documents}: +3$, $\Delta \text{References}: +7$).

---

### User Story 7: Insights vs. Settings Separation & Cohesive Navigation (Priority: P2)
> *As a user configuring the extension, I want a clean separation between Insights (Health, Structure, Evolution) and Settings (Knowledge, Exclusions, Clusters, Git/Harness, Advanced), with bidirectional Manager ↔ Detail panel deep-linking, so that navigation feels like a unified, coherent control system.*

**Why this priority**: Eliminates confusing drawer terminology (no longer calling analysis "Advanced Settings") and establishes frictionless deep linking between summary cards and detail panels.  
**Independent Test**: Click individual dimension rows on the Health or Evolution cards in the Manager sidebar; verify the right-side drawer opens directly to the clicked sub-tab under Insights; verify settings categories live under a distinct Settings section with a gear icon.

**Acceptance Scenarios**:
1. **Given** the Manager sidebar, **When** the user clicks the `Evolution` card orientation badge, **Then** the drawer opens directly to `Insights → Evolution → Orientation`.
2. **Given** the Manager sidebar, **When** the user clicks the `Evolution` card opportunities count, **Then** the drawer opens directly to `Insights → Evolution → Opportunities`.
3. **Given** the drawer header, **When** inspecting navigation, **Then** it clearly separates `INSIGHTS` (`Health`, `Structure`, `Evolution`) from `SETTINGS` (`Knowledge`, `Exclusions`, `Clusters`, `Git / Harness`, `Advanced` with a gear icon).

---

### User Story 8: Progressive Disclosure, Informational Bubbles `ⓘ` & Recommendation Language (Priority: P2)
> *As a team member reading LLM Wiki recommendations, I want the language to be neutral, non-dogmatic, and respectful of diverse documentation styles, with progressive disclosure from high-level summaries down to technical calculations, so that our team embraces recommendations constructively.*

**Why this priority**: Human-centered product design. Replaces accusatory phrasing ("bad structure", "God document", "wrong organization") with objective architectural language.  
**Independent Test**: Review all UI copy, tooltips, and recommendation templates to verify 100% compliance with Recommendation Language Standards and 5-level progressive disclosure.

**Acceptance Scenarios**:
1. **Given** any system recommendation, **When** rendered in the UI, **Then** the copy adheres strictly to:
   - *"Structural opportunity detected"* instead of *"Bad structure"*
   - *"Current organization introduces measurable retrieval cost"* instead of *"Wrong organization"*
   - *"This strategy has the strongest fit under current objectives and constraints"* instead of *"Correct method"*
   - *"Structurally overloaded document"* instead of *"God Document"*
2. **Given** the user experience, **When** navigating from sidebar to engine internals, **Then** information is disclosed strictly in 5 levels: Level 1 (What matters?) $\to$ Level 2 (Why?) $\to$ Level 3 (What evidence?) $\to$ Level 4 (What can I do?) $\to$ Level 5 (How does engine calculate it?).

---

### Edge Cases
1. **Zero Unexcluded Documents**: If the repository has zero Markdown documents or all are excluded via `scan.exclude`, the Manager displays an empty state: `0 documents`, Health score `100/100 (No active documents)`, and an onboarding prompt `[Configure scan exclusions]`.
2. **Massive Repository (5,000+ notes)**: In large vaults, the Manager summary calculations MUST be cached and computed in $\le 50\text{ ms}$ in Rust, and the UI MUST NOT freeze during detail drawer transitions.
3. **Contradictory Custom Constraints**: If a user configures Custom settings with mathematically irreconcilable rules (e.g. `Never split documents` + `Context surface <= 3k` on a 50k-token note), the Evolution card flags a yellow badge `⚠ Conflicting constraints` and links to the Conflict Resolver in the Custom settings panel.
4. **Git Detached HEAD / Uncommitted Initial Repo**: The Sync card displays `● Detached HEAD` or `● Uninitialized Git` with a tooltip explaining that admission control and Git commit sync require an initialized Git repository.
5. **Simultaneous External Git Commit**: If an external terminal commit completes while the Manager is open, the loopback `git/commitDelta` event refreshes the Manager cards without resetting user scroll positions or closing active modals.

---

## 3. Functional Requirements (FR-001 – FR-046)

### 3.1 Manager Card Architecture & Layout (FR-001 – FR-010)
- **FR-001**: The VS Code sidebar webview (`extension/src/webview/sidebarViewProvider.ts`) MUST render the LLM Wiki Manager as a persistent 4-card layout: **Health**, **Structure**, **Evolution**, and **Sync**.
- **FR-002**: The Manager MUST function as a decision-oriented summary answering: (1) Is my knowledge healthy? (2) What requires attention? (3) How is the corpus structurally behaving? (4) What am I optimizing for? (5) Is the system synchronized?
- **FR-003**: The Manager MUST NOT expose diagnostic engine variables (e.g. IPC socket paths, raw petgraph edge counts, internal protocol hashes) in primary view cards.
- **FR-004**: The **Health Card** MUST render the overall Quality Score ($0\text{--}100$), five dimension progress rows (`Integrity`, `Structure`, `Consistency`, `Discoverability`, `Freshness`), actionable finding count (`N findings need attention`), and button `[Review health]`.
- **FR-005**: The **Structure Card** MUST render: total documents (`N documents`), connectivity breakdown (`X connected documents · Y isolated documents`), total reference count (`Z references`), and button `[Explore graph]`.
- **FR-006**: The **Evolution Card** MUST render: active orientation icon and title (`🤖 AI-Oriented` / `👤 Human-Oriented` / `⚖️ Balanced` / `⚙️ Custom`), brief optimization objective description ($\le 60$ characters), structural opportunity count (`N structural opportunities`), and button `[Review evolution]`.
- **FR-007**: The **Sync Card** MUST render: synchronization status dot (`● Synced` / `● Syncing` / `● Error`), and the short SHA of the last committed Git commit (`Last commit · <sha>`).
- **FR-008**: The Sync Card MUST be visually and conceptually distinct from the Health Card, explicitly embodying the product rule that `Synced ≠ Healthy`.
- **FR-009**: Clicking `[Explore graph]` on the Structure Card MUST invoke VS Code command `llmWiki.showKnowledgeGraph` to open or reveal the Knowledge Graph tab.
- **FR-010**: All Manager cards MUST use partial DOM reconciliation or Virtual DOM diffing during updates to guarantee that user scroll positions in the sidebar are never reset.

### 3.2 Health Dimensions, Findings vs. Score & Link Consolidation (FR-011 – FR-017)
- **FR-011**: The UI MUST strictly separate Quality Scores ($0\text{--}100$) from Finding Counts ($0\text{--}\infty$) across all typography and visual framing, preventing users from conflating finding numbers with percentages.
- **FR-012**: Broken link reporting MUST be consolidated strictly under the **Integrity** quality dimension. The UI MUST NOT render competing parallel broken-link counters.
- **FR-013**: The Integrity detail section MUST break down findings into: `Broken references` (target file missing or unmapped) and `Invalid anchors` (target file exists but `#heading-anchor` is missing).
- **FR-014**: Actions to remediate broken links (`[Fix reference]`, `[Exclude path]`) MUST be positioned contextually within the specific finding row, NOT as a duplicate global button.
- **FR-015**: The Consistency dimension MUST report: `Collisions`, `Canonical conflicts`, and `Conceptual fragmentation`.
- **FR-016**: The Discoverability dimension MUST report: `Isolated documents`, `Context Trace traversal bottlenecks`, and `Centrality bottlenecks`.
- **FR-017**: The Freshness dimension MUST report deterministic code-doc drift evidence: `Cited code deleted`, `Cited code modified post-doc-update`, and `Renamed symbols`.

### 3.3 Terminology Standardization & Graph Responsibilities (FR-018 – FR-021)
- **FR-018**: The entire UI across all views MUST enforce standard, unambiguous graph terminology:
  - **Document**: Represents a Markdown file (Knowledge Graph Node, $O(N)$ graph complexity). The term "Node" MUST NOT be used in user-facing Manager cards.
  - **Reference**: Represents a directed knowledge link or wikilink (Knowledge Graph Edge). The terms "Edge" and "Link" MUST NOT be used where "Reference" applies.
  - **Fragment**: Represents addressable structural section metadata within a document.
  - **Connected Document**: A document node belonging to a connected graph component.
  - **Isolated Document**: A document node possessing zero active graph references. The term "Orphan" MUST be replaced with "Isolated Document" across all user-facing views.
- **FR-019**: The Knowledge Graph view (`d3-force` Canvas) MUST remain focused on visual and spatial exploration: rendering documents, references, cluster hues, hop-distance alpha decay, and active context paths.
- **FR-020**: Selecting a Document node in the Graph MUST open the document-centric Inspector Drawer displaying tokens, degree, and the 3-column fragment table (`Fragment | Tok | I/O`).
- **FR-021**: Fragments MUST remain metadata attached to Document nodes and MUST NOT be rendered as independent nodes in the force-directed graph (Invariant V).

### 3.4 Problem-First Structural Opportunities & 10-D Evidence Metrics (FR-022 – FR-029)
- **FR-022**: The Evolution tab MUST present structural optimization **problem-first**, replacing the legacy "Methods" tab with a list of **Structural Opportunities**.
- **FR-023**: The system MUST evaluate 10 canonical structural dimensions: `Cohesion`, `Coupling`, `Reference Concentration`, `Granularity`, `Context Efficiency`, `Discoverability`, `Knowledge Reuse`, `Semantic Diversity`, `Navigational Complexity`, and `Maintenance Coupling`.
- **FR-024**: All 10 structural dimensions MUST be rendered using their full human-readable titles in primary views, NEVER cryptic abbreviations (`COH`, `EFF`, `CONC`).
- **FR-025**: Every structural opportunity MUST clearly separate **Evidence** (raw observable metrics) from **Interpretation** (problem signature title and narrative `WHY`).
- **FR-026**: Every structural metric and parameter MUST feature an informational bubble (`ⓘ`) answering all five mandatory questions: (1) What does this mean? (2) What does a higher/lower value imply? (3) Why does it matter? (4) What does it affect? (5) What trade-off does it introduce?
- **FR-027**: The UI MUST recognize four primary problem signatures: `Structurally Overloaded Document` (high diversity + high concentration + high context waste), `Knowledge Fragmentation` (semantic duplicate concepts split across files), `Weak Discovery Path` (excessive Context Trace hops), and `Isolated Knowledge Silo` (unreferenced cluster).
- **FR-028**: Document size MUST be evaluated strictly as an analytical signal, NOT an automatic verdict. Monolithic documents with high cohesion and low reference concentration MUST NOT be flagged for splitting (Invariant XIII).
- **FR-029**: Each opportunity card MUST display a confidence rating ($[0.0, 1.0]$) categorized as: High ($\ge 0.80$), Medium ($0.50\text{--}0.79$), or Low ($< 0.50$).

### 3.5 Strategy Fit, Hybrid Recommendations & Trade-Off Matrix (FR-030 – FR-036)
- **FR-030**: For each structural opportunity, the system MUST evaluate candidate Knowledge Organization strategies: `Section Addressing`, `Topic Decomposition`, `Lightweight Index Router`, `Faceted Classification`, `Preserve Document`, and `Hybrid Strategies`.
- **FR-031**: Every candidate strategy MUST display an explainable **Fit Score** ($0\text{--}100\%$) indicating mathematical alignment with the active orientation and configured constraints.
- **FR-032**: The Fit Score MUST feature an informational bubble (`ⓘ`) explaining its derivation from problem alignment, objective weights, and constraint compatibility.
- **FR-033**: Every strategy recommendation MUST itemize both **Expected Benefits** (with $\uparrow$/$\downarrow$ direction indicators) and **Countervailing Trade-Offs** (e.g. increased granularity, index maintenance, or metadata overhead).
- **FR-034**: The UI MUST support **Hybrid Recommendations** combining complementary strategies (e.g. `Section Addressing + Lightweight Index Router`) with clear synergistic justification.
- **FR-035**: Strategy recommendations MUST use objective, non-dogmatic language: "Best fit under current objectives and constraints", NEVER "Best method" or "Correct method".
- **FR-036**: Each candidate strategy card MUST provide an estimated effort indicator based on objective heuristics: `Low` (single-file anchors/frontmatter), `Medium` (1 new index router + link updates), `High` (multi-file document decomposition), along with button `[Preview change]`.

### 3.6 Preview Before Automation & Post-Change Re-Measurement (FR-037 – FR-040)
- **FR-037**: Clicking `[Preview change]` MUST generate and render a `TransformationPlan` preview displaying: target documents, proposed file/link actions, expected Before $\to$ After metric impacts, and countervailing trade-offs.
- **FR-038**: Automated transformations MUST be strictly downstream of explicit user approval: default settings MUST enforce `Auto-Apply: OFF` and `Require Approval: ON` (Invariant XVI).
- **FR-039**: Clicking `[Apply changes]` in the preview modal MUST execute the transformation, write updated files to the workspace, evaluate post-change re-measurement on an in-memory ephemeral candidate graph without mutating the persistent committed graph (preserving Invariant I until a Git commit occurs), and display the **Post-Change Re-Measurement** card.
- **FR-040**: The Post-Change Re-Measurement card MUST display empirical deltas ($\Delta \text{Context Efficiency}$, $\Delta \text{Navigational Complexity}$, $\Delta \text{Maintenance Coupling}$, $\Delta \text{Documents}$, $\Delta \text{References}$) accompanied by the narrative: *"The selected objectives improved, with the following structural trade-offs."*

### 3.7 Insights vs. Settings Separation & Cohesive Interaction Model (FR-041 – FR-046)
- **FR-041**: The slide-out detail panel (formerly called "Advanced Settings Drawer") MUST be restructured into two distinct conceptual tiers: **INSIGHTS** and **SETTINGS**.
- **FR-042**: The **INSIGHTS** section MUST contain: `Health`, `Structure`, and `Evolution`.
- **FR-043**: The **SETTINGS** section MUST contain: `Knowledge`, `Exclusions` (Scan vs. Graph), `Clusters`, `Git / Harness`, and `Advanced` (with a gear icon).
- **FR-044**: Clicking any item or row in the Manager sidebar MUST dispatch a navigation intent (`manager/navigateToDetail`) that opens the detail panel directly to the corresponding tab and subsection.
- **FR-045**: The **Custom** evolution tab MUST support two tiers: Tier 1 high-level objective sliders (`Low`, `Medium`, `High` with `ⓘ` tooltips) and Tier 2 advanced parameters grouped into `Structural Analysis`, `Retrieval`, `Organization`, and `Automation`, compiling into a unified `OptimizationProfile` via `evolution/updateCustomProfile` (Invariant XIV).
- **FR-046**: The Custom configuration engine MUST evaluate constraint hierarchies (`Preference`, `Constraint`, `Hard Constraint`) and surface yellow banner warnings whenever conflicting or irreconcilable constraints are detected.

---

## 4. Non-Functional Requirements (NFR-001 – NFR-010)

- **NFR-001: Zero External AI / LLM Invariant (Constitutional Principle I)**: All health scores, 10-D structural metrics, problem signatures, fit scores, and transformation plans MUST be computed 100% locally by the deterministic Rust engine. External LLM APIs, cloud services, and remote embeddings are strictly prohibited.
- **NFR-002: Manager Sidebar Response Budget**: The Manager 4-card summary MUST compute and render in $\le 50\text{ ms}$ for vaults up to 5,000 documents.
- **NFR-003: Manager ↔ Detail Navigation Latency**: Deep-linking navigation from a Manager card click to the corresponding detail panel sub-tab MUST execute in $\le 16\text{ ms}$ (1 frame at 60 FPS).
- **NFR-004: Transformation Plan Generation Ceiling**: Generating a transformation plan preview for a complex document MUST execute in $\le 500\text{ ms}$, eliminating long timeouts.
- **NFR-005: Post-Change Re-Measurement Latency**: Re-measuring the corpus and computing empirical deltas after an automated transformation MUST complete in $\le 150\text{ ms}$.
- **NFR-006: Webview Rendering Performance**: The Manager sidebar and detail drawer MUST render at a consistent 60 FPS during scrolling, tab switching, and accordion toggling without layout thrashing.
- **NFR-007: Strict CSP Sandboxing (Constitutional Principle X)**: All webviews MUST enforce strict Content Security Policy with cryptographic nonces for scripts and styles. Inline script execution without nonces is forbidden.
- **NFR-008: Zero Plaintext Secret Leakage**: Any telemetry, authentication, or token credentials MUST be managed exclusively via `vscode.SecretStorage`.
- **NFR-009: Raw Numeric Integrity vs. Presentation Separation (Invariants IX & X)**: All internal data structures and RPC payloads MUST store full raw integers/floats. Presentation formatting ($\le 5$ chars in compact badges) exists exclusively at the view layer.
- **NFR-010: Non-Prescriptive Knowledge Organization (Invariant XIII)**: The UI and recommendation engine MUST never mandate a single documentation methodology. Document size MUST be evaluated strictly as an analytical signal, protecting healthy monoliths from automatic splitting.

---

## 5. Success Criteria (SC-001 – SC-008)

- **SC-001: 100% Elimination of Action Duplication**: Zero duplicate buttons across Activity Bar, Sidebar view title, top toolbar, and context menus. Administrative actions emerge exclusively from relevant context cards.
- **SC-002: Sub-Second Manager Initial Load**: On VS Code workspace open, the Manager 4-card summary renders completely within 250 ms.
- **SC-003: Sub-50ms Navigation Fluidity**: Clicking any metric, badge, or button in the Manager navigates to the exact detail panel context in under 50 ms without scroll jump or UI stutter.
- **SC-004: Zero Confusing Terminology**: 100% elimination of ambiguous terms ("4 CONNECTED", "47 ORPHANS", "72 LINKS") in favor of standardized terms ("51 documents", "4 connected documents", "47 isolated documents", "72 references").
- **SC-005: 100% Informational Tooltip Coverage**: Every non-obvious metric, parameter, and strategy indicator features an active `ⓘ` tooltip answering the 5 mandatory questions.
- **SC-006: 100% Impact Preview Transparency**: Every automated structural transformation displays both expected benefits AND countervailing trade-offs before requiring explicit user approval.
- **SC-007: Flawless Before/After Re-Measurement**: Applied transformations produce verified empirical delta reports ($\Delta \text{Context Efficiency}$, $\Delta \text{Navigational Complexity}$, $\Delta \text{Maintenance Coupling}$) within 150 ms of disk write.
- **SC-008: Full Test Suite & Constitutional Compliance**: All existing 118+ TypeScript tests and 88+ Rust tests pass, with 0 clippy warnings, 0 ESLint errors, and 100% compliance with Constitution v1.6.0.

---

## 6. Key Entities & UI Models

### 6.1 `ManagerSummaryState`
Persistent state model for the left-side Manager:
- `health`: Overall score ($0\text{--}100$), dimension scores, finding count, legacy debt, new regressions.
- `structure`: Total documents, connected count, isolated count, reference count.
- `evolution`: Active orientation (`ai_oriented` | `human_oriented` | `balanced` | `custom`), opportunity count, brief summary ($\le 60$ chars).
- `sync`: Git status (`synced` | `syncing` | `error`), last commit SHA, timestamp.

### 6.2 `OptimizationProfile` (Invariant XIV)
Single unified internal profile compiling orientation, Tier 1 preferences, Tier 2 parameters, and hard constraints.

### 6.3 `StructuralOpportunity`
Model representing a detected organizational problem:
- `id`: Unique identifier (e.g. `OPP-001`).
- `target_document`: File path.
- `signature`: Problem title (e.g. `Structurally Overloaded Document`).
- `confidence`: Mathematical confidence score ($0.0\text{--}1.0$) mapped to High/Medium/Low.
- `narrative_why`: Plain-text explanation of the problem.
- `evidence`: Dictionary of 10-D human-readable metrics.
- `candidate_strategies`: Array of evaluated `StrategyRecommendation` items.

### 6.4 `StrategyRecommendation`
Model representing a candidate resolution:
- `strategy`: Strategy identifier (`section_addressing`, `topic_decomposition`, `lightweight_index`, `faceted_classification`, `preserve_document`, `hybrid`).
- `fit_score`: Percentage fit ($0\text{--}100\%$).
- `confidence`: Strategy confidence rating.
- `expected_benefits`: Array of positive impact strings with direction arrows.
- `countervailing_trade_offs`: Array of cost/trade-off strings.
- `effort`: Estimated implementation effort (`low` | `medium` | `high`).
- `plan_preview`: Detailed file and reference actions.

---

## 7. Assumptions & Dependencies

- **A-001**: The Rust core daemon (`llm-wiki-core`) is compiled and managed as a sidecar process communicating via local JSON-RPC.
- **A-002**: The underlying graph model uses `petgraph` where documents are graph nodes and references are directed edges.
- **A-003**: The workspace is a Git repository, allowing post-commit sync hooks and last-commit SHA retrieval. For non-Git workspaces, the Sync card displays an uninitialized state without blocking other cards.
- **A-004**: The VS Code Webview environment supports standard Web Components and CSS custom properties matching the active editor theme tokens.
- **A-005**: All structural transformations operate on Markdown files (`.md`) and standard Markdown link / wikilink syntax.
