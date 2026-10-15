---
wiki:
  content_hash: b203fb2ee8c6af2270cac41eb075ed4b05d18fb5e27c74e8a5a8818aabeb3129
  fragments:
    feature-specification-deterministic-knowledge-engineering-engine:
      tokens: 7271
      inbound: 0
      outbound: 0
    executive-summary-product-positioning:
      tokens: 422
      inbound: 0
      outbound: 0
    the-deterministic-processing-pipeline:
      tokens: 239
      inbound: 0
      outbound: 0
    user-scenarios-testing-mandatory:
      tokens: 2500
      inbound: 0
      outbound: 0
    user-story-1-deterministic-repository-observation-epistemic-audit-priority-p1:
      tokens: 495
      inbound: 0
      outbound: 0
    user-story-2-profile-relative-health-assessment-across-4-evolution-modes-priority-p2:
      tokens: 382
      inbound: 0
      outbound: 0
    user-story-3-comprehensive-18-methodology-fit-evaluation-antipattern-diagnosis-priority-p3:
      tokens: 469
      inbound: 0
      outbound: 0
    user-story-4-evidence-based-strategy-selection-with-explicit-trade-offs-priority-p4:
      tokens: 401
      inbound: 0
      outbound: 0
    user-story-5-user-controlled-downstream-automation-beforeafter-re-measurement-priority-p5:
      tokens: 332
      inbound: 0
      outbound: 0
    user-story-6-high-performance-vs-code-extension-host-management-experience-ui-priority-p6:
      tokens: 411
      inbound: 0
      outbound: 0
    edge-cases:
      tokens: 290
      inbound: 0
      outbound: 0
    requirements-mandatory:
      tokens: 2940
      inbound: 0
      outbound: 0
    functional-requirements:
      tokens: 2602
      inbound: 0
      outbound: 0
    core-engine-determinism-epistemic-boundaries:
      tokens: 342
      inbound: 0
      outbound: 0
    optimization-profile-evolution-modes:
      tokens: 225
      inbound: 0
      outbound: 0
    18-knowledge-organization-methodologies-catalog:
      tokens: 921
      inbound: 0
      outbound: 0
    5-pillar-profile-relative-health-model:
      tokens: 241
      inbound: 0
      outbound: 0
    structural-diagnosis-strategy-catalog-recommendations:
      tokens: 339
      inbound: 0
      outbound: 0
    user-controlled-automation-downstream-governance:
      tokens: 272
      inbound: 0
      outbound: 0
    integration-ipc-vs-code-extension:
      tokens: 258
      inbound: 0
      outbound: 0
    key-entities:
      tokens: 332
      inbound: 0
      outbound: 0
    non-functional-requirements-nfr:
      tokens: 495
      inbound: 0
      outbound: 0
    success-criteria-mandatory:
      tokens: 328
      inbound: 0
      outbound: 0
    measurable-outcomes:
      tokens: 321
      inbound: 0
      outbound: 0
    assumptions:
      tokens: 145
      inbound: 0
      outbound: 0
---
# Feature Specification: Deterministic Knowledge Engineering Engine

**Feature Branch**: `009-deterministic-knowledge-engine`  
**Created**: 2026-10-06  
**Status**: Draft  
**Input**: User description: "Deterministic Knowledge Engineering Engine consolidating SPEC.md, RESEARCH_1.md, and RESEARCH_2.md: observe, measure, diagnose, and guide the evolution of knowledge structure in version-controlled Markdown corpora using a native Rust engine without external AI/LLM dependencies, featuring first-class Optimization Profiles across 4 evolution modes, 4 epistemic tiers, a 18-methodology catalog, profile-relative 5-pillar health, evidence-based structural recommendations with explicit trade-offs, and user-controlled downstream automation."

---

## Executive Summary & Product Positioning

The **Deterministic Knowledge Engineering Engine** is a local, high-performance analytical engine implemented in Rust, integrated into Visual Studio Code via the `llm_wiki_ext4vsc` extension. Its sole mission is to observe, measure, diagnose, and guide the evolution of knowledge organization across version-controlled Markdown corpora.

The product is explicitly **NOT**:
- A Markdown editor, wiki, or note-taking frontend;
- A chatbot, conversational assistant, or LLM wrapper;
- A vector database or probabilistic embedding store;
- A speculative automatic documentation rewriter.

Instead, the product is:
> **An instrument of deterministic observation, structural measurement, diagnostic evaluation, and decision support for evolving knowledge repositories.**

The fundamental evaluation question answered by the engine is:
> *"Is the organization of this knowledge suitable for the purpose for which it must be used?"*

### The Deterministic Processing Pipeline

The engine executes an immutable, bit-exact sequential pipeline:

```text
Repository Snapshot (Markdown + Git)
    ↓
Observation (CommonMark AST, Materialized Frontmatter, Git History, Filesystem)
    ↓
Measurement (Structural, Topological, Lexical, Information-Theoretic Metrics)
    ↓
Structural Model (Document Graph, Fragments Metadata, Lexical Inverted Index)
    ↓
Optimization Profile (AI-Oriented | Human-Oriented | Balanced | Custom)
    ↓
Diagnosis (Pillar Health, Rule Violations, Anti-pattern Detection)
    ↓
Method Evaluation (MCDA / AHP Scoring across 18 Knowledge Organization Methodologies)
    ↓
Recommendation (Candidate Strategies with Evidence & Countervailing Trade-offs)
    ↓
Human / Agent Decision Gate (Approval Token, Auto-Apply: OFF)
    ↓
Transformation (Mechanically Verifiable Discrete AST & Filesystem Mutators)
    ↓
Git Commit (Pre-commit Validation / Post-commit Sync)
    ↓
Re-analysis (Before-vs-After Empirical Delta Re-measurement)
```

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Deterministic Repository Observation & Epistemic Audit (Priority: P1)

As a repository architect or AI agent author, I need the engine to deterministically parse and index the entire Markdown knowledge corpus without network calls, LLM APIs, or stochastic embeddings, producing a bit-level reproducible structural model indexed with a cryptographic BLAKE3 `analysis_id`, while strictly classifying every observation and metric into its correct Epistemic Tier (Tier A: Calculated, Tier B: Candidate, Tier C: Decision Support, Tier D: Not Determinable).

**Why this priority**: Without absolute determinism and strict epistemic tiering, diagnostics cannot be trusted in automated pre-commit gates or CI/CD pipelines, and speculative inferences will pollute factual measurements.

**Independent Test**: Can be tested by running the Rust core binary against a fixture repository twice under identical configuration, verifying identical JSON output byte-for-byte, matching BLAKE3 `analysis_id`, zero network activity, and 100% adherence to epistemic tier boundaries.

**Acceptance Scenarios**:
1. **Given** a repository of $N$ Markdown documents at Git commit $H$ with configuration $C$ and methodology version $M$, **When** analyzed, **Then** the engine computes the exact same metrics across runs and outputs $\text{analysis\_id} = \text{BLAKE3}(H \,\|\, C \,\|\, M)$.
2. **Given** an observation that can be calculated from syntax (AST links, heading anchors, PageRank, BM25 scores), **When** output, **Then** it is marked as `Tier A (Fully Deterministic)`.
3. **Given** a syntactic similarity that suggests near-duplicate notes or candidate synonyms, **When** output, **Then** it is marked as `Tier B (Candidate Generation)` and never presented as definitive truth.
4. **Given** a structural decision (e.g. which note is canonical, whether to split a monolith), **When** generated, **Then** it is marked as `Tier C (Decision Support)` with itemized evidence and countervailing trade-offs.
5. **Given** a request to verify the conceptual truth or semantic intent of freeform prose, **When** evaluated, **Then** the engine explicitly flags the property as `Tier D (Not Reliably Determinable)` and refuses speculative verdict.

---

### User Story 2 - Profile-Relative Health Assessment across 4 Evolution Modes (Priority: P2)

As a technical lead managing documentation, I need to evaluate the repository health against a first-class `OptimizationProfile` representing one of four Documentation Evolution Modes (`AI-Oriented` [default], `Human-Oriented`, `Balanced`, `Custom`), evaluating 5 health pillars (Integrity, Structure, Consistency, Discoverability, Freshness) relative to that profile's objective function, rather than an arbitrary universal scalar score.

**Why this priority**: Different consumers require fundamentally different organizational properties. AI agents require high section addressability, embedded index routing, and minimal context surface tokens; human readers require narrative flow, low cognitive load, and readable hierarchy. An identical structure can be healthy for humans but dysfunctional for agents.

**Independent Test**: Can be tested by evaluating the same repository under `AI-Oriented` mode and `Human-Oriented` mode, verifying that the computed pillar scores and diagnostic findings shift according to profile weights and thresholds while preserving raw underlying metric integrity.

**Acceptance Scenarios**:
1. **Given** an 8,000-token modular note with comprehensive section fragments and high link density, **When** evaluated in `AI-Oriented` mode, **Then** its Context Efficiency and Discoverability score high (>85/100).
2. **Given** the same 8,000-token note, **When** evaluated in `Human-Oriented` mode with low human chunk tolerance, **Then** the engine flags high reading length while respecting Invariant XIII (size is a signal, not a verdict).
3. **Given** a repository with pre-existing documentation debt, **When** baselined via *.context-vault/baseline.json*, **Then** the health model strictly isolates pre-existing debt from new commit regressions.

---

### User Story 3 - Comprehensive 18-Methodology Fit Evaluation & Antipattern Diagnosis (Priority: P3)

As a knowledge engineer, I need the engine to evaluate the repository against the catalog of 18 established Knowledge Organization System (KOS) methodologies (Classification, Taxonomy, Facets, Controlled Vocabulary, Thesaurus, Semantic Networks, Concept Maps, Ontologies, Domain Analysis, Metadata Profiles, Information Architecture, Information Retrieval, Terminology Management, Network Analysis, Knowledge Quality, Provenance, Knowledge Drift, Context Routing), surfacing concrete structural findings based on observable signals.

**Why this priority**: Real knowledge bases suffer from specific structural breakdowns: taxonomic cycles, vocabulary drift, missing DCAP frontmatter, citation bottlenecks, and stale code references. The engine must systematically evaluate which methodologies provide the mathematical remedy.

**Independent Test**: Can be tested by introducing 18 targeted structural defects (e.g., circular broader terms in thesaurus, unindexed deep files in IA, out-of-vocabulary tags in controlled vocabulary, stale git commit code citations in freshness) and verifying that the engine detects each corresponding diagnostic rule with exact AST file and line pointers.

**Acceptance Scenarios**:
1. **Given** frontmatter tags with uncontrolled variants (`javascript`, `Javascript`, `JS`), **When** Terminology & Controlled Vocabulary diagnostics run, **Then** an `UncontrolledVocabularyVariant` finding is raised referencing the canonical glossary.
2. **Given** a thesaurus relationship graph with asymmetric `broader` terms missing reciprocal `narrower` terms, **When** Thesaurus diagnostics run (ISO 25964), **Then** a `ThesaurusReciprocityViolation` is raised.
3. **Given** a document with code citations whose underlying source files were modified in Git 5 commits ago without doc updates, **When** Freshness and Provenance diagnostics execute (W3C PROV / Git), **Then** a `StaleCodeCitation` finding is raised with empirical Git commit delta evidence.
4. **Given** an isolated cluster of notes with no path to the root harness entry point, **When** Network & Routing diagnostics run, **Then** an `UnroutedKnowledgeCluster` finding is raised.

---

### User Story 4 - Evidence-Based Strategy Selection with Explicit Trade-Offs (Priority: P4)

As an architect reviewing restructuring recommendations, I need the engine to evaluate 6 candidate transformation strategies (`PreserveDocument`, `SplitDecompose`, `ExtractFacetMetadata`, `ConsolidateIndex`, `MergeCandidates`, `StandardizeTerminology`), assign an empirical Fit Score based on profile objective weights, and present recommendations categorized by pairwise role (Primary, Supporting, Alternative, Mutually Exclusive) with explicit expected benefits and countervailing trade-offs.

**Why this priority**: Structural changes always involve compromises. Decomposing a document improves context efficiency but increases fragmentation and metadata maintenance. The engine must never prescribe dogma; it must make trade-offs transparent.

**Independent Test**: Can be tested on an overloaded 45,000-token note with three disparate topics, verifying that the engine computes Fit Scores for `SplitDecompose` vs `PreserveDocument`, classifies them as `MUTUALLY_EXCLUSIVE`, and itemizes both benefits (e.g., $+45\%$ Context Efficiency) and countervailing trade-offs (e.g., $+3$ files, $+12$ cross-links to maintain).

**Acceptance Scenarios**:
1. **Given** an overloaded document, **When** strategy recommendations are generated, **Then** the engine outputs a Primary recommendation, zero or more Supporting recommendations, and marks conflicting strategies as `MUTUALLY_EXCLUSIVE`.
2. **Given** any recommended strategy, **When** inspected in the UI or CLI, **Then** it contains both an `expected_benefits` list and a `countervailing_trade_offs` list derived from mathematical metrics.
3. **Given** two strategies with identical Fit Scores, **When** ranked, **Then** the engine applies deterministic tie-breaking (Fit Score $\to$ Implementation Cost $\to$ Profile Dimension Weight $\to$ Canonical Enum Order).

---

### User Story 5 - User-Controlled Downstream Automation & Before/After Re-Measurement (Priority: P5)

As a developer or AI assistant, I need structural transformations (such as creating index routers, normalizing frontmatter metadata, standardizing terms, or refactoring sections) to execute only after explicit user approval through a cryptographic `approval_token`, verifying hard constraints, applying discrete file edits, and immediately performing Before-vs-After re-measurement.

**Why this priority**: Automated restructuring must never destroy user knowledge or run unattended without consent (`Auto-Apply: OFF`, `Require Approval: ON` by default). Users must see exact diffs and verify that the re-measurement confirms the predicted improvement.

**Independent Test**: Can be tested by generating a transformation plan for frontmatter standardization, attempting execution without a token (must be rejected), approving with valid token, verifying on-disk unified diff application, and checking the generated empirical delta report.

**Acceptance Scenarios**:
1. **Given** a generated transformation plan with `plan_id` and `approval_token`, **When** execution is attempted with `Auto-Apply: OFF` without providing the token, **Then** the engine refuses execution and leaves files untouched.
2. **Given** valid approval, **When** the transformation executes, **Then** mechanical changes are written to the working tree, and pre-commit validation re-stages modified metadata.
3. **Given** completed transformation, **When** re-measurement runs, **Then** the engine outputs an empirical Before/After delta report comparing all 5 pillar scores, raw token counts, and graph diameter.

---

### User Story 6 - High-Performance VS Code Extension Host & Management Experience UI (Priority: P6)

As an engineer working in VS Code, I need a rich webview interface featuring 4 views (Health, Structure, Evolution, Settings) connected to the Rust engine via JSON-RPC 2.0 over standard I/O, respecting strict platform budgets ($\le 50$ ms incremental updates, $\le 150$ ms re-measurement, $\le 5$ chars compact presentation formatting in UI, zero plaintext secrets), with bidirectional deep-linking to code and notes.

**Why this priority**: The developer experience must remain fluid and non-blocking. Heavy computation must stay in Rust, while the UI provides transparent, interactive controls and quick navigation to offending source lines.

**Independent Test**: Can be tested by loading a 500-document repository into VS Code, opening the LLM Wiki Manager webview, verifying initial render in $< 200$ ms, clicking through Health, Structure, Evolution, and Settings drawers, and clicking a broken reference link to verify that the editor jumps to the exact line.

**Acceptance Scenarios**:
1. **Given** the webview is opened, **Then** it renders the 4-card management summary (Health, Structure, Evolution, Settings) using CSP nonces and sandboxed iframe isolation.
2. **Given** any numeric value rendered in the UI, **Then** it adheres to compact presentation formatting ($\le 5$ characters, e.g., `12.5k`, `420`, `1.2M`) while preserving raw integer values in internal state.
3. **Given** a file modification or save event in the editor, **When** validation is requested, **Then** the Rust engine computes an ephemeral candidate AST and candidate diagnostics in $\le 50$ ms without mutating the persistent knowledge graph (Invariants I & II), while persistent graph updates occur strictly during post-commit sync (`Hook 2: llm-wiki-sync`).

---

## Edge Cases

- **Empty Repository / Zero Markdown Files**: Engine MUST return a valid, empty `StructuralModel` with `analysis_id`, 0 nodes, 0 edges, and baseline score 100/100 without panicking.
- **Single Monolithic Document (e.g. 100k tokens)**: Engine MUST NOT automatically recommend splitting if internal section cohesion is high, heading hierarchy is clean, and references are resolved (Invariant XIII).
- **Cyclic Taxonomy or Graph Structures**: Tarjan SCC algorithm MUST detect cycles, isolate cyclic components, and report them as Integrity/Taxonomy findings without infinite recursion.
- **Malformed Frontmatter / Invalid YAML Syntax**: Engine MUST report a Tier A syntax error with exact line/column offset, gracefully treating the document body as raw text without crashing.
- **Git Detached HEAD or Uncommitted Submodules**: Engine MUST resolve commit hash via Git plumbing (`git rev-parse HEAD`), falling back to a deterministic tree hash over tracked files if HEAD is unresolvable.
- **Unicode Normalization & Case Insensitivity**: File paths, wikilink targets, and controlled vocabulary terms MUST be normalized via Unicode NFC and canonical case folding before graph lookups.
- **Large Repository Scaling ($\ge 10,000$ documents)**: Engine MUST employ memory-mapped file access, arena allocators, parallel tokenization via Rayon, and bitsets to maintain response times within budgets.

---

## Requirements *(mandatory)*

### Functional Requirements

#### Core Engine, Determinism & Epistemic Boundaries
- **FR-001**: The engine MUST be implemented in Rust as a standalone core (`llm-wiki-core`) with zero runtime dependencies on external AI APIs, remote embeddings, LLMs, or cloud services (Invariants III & IV).
- **FR-002**: Every analytical run MUST produce an identical bit-level output for the same repository commit, configuration, profile, and methodology version, uniquely fingerprinted by `analysis_id = BLAKE3(GitCommitHash || CanonicalConfigBytes || CanonicalProfileBytes || MethodologyVersionString)` (Invariants III & IV).
- **FR-003**: The engine MUST classify every output object into exactly one of the four Epistemic Tiers:
  - `TierA_FullyDeterministic`: Direct algorithmic derivation from AST, filesystem, or graph math;
  - `TierB_CandidateGeneration`: Syntactic heuristics suggesting potential candidates, synonyms, or collisions;
  - `TierC_DecisionSupport`: Multi-criteria recommendations, trade-offs, and fit evaluations;
  - `TierD_NotReliablyDeterminable`: Conceptual claims requiring human domain authority that cannot be mathematically proven.
- **FR-004**: Document nodes in the Knowledge Graph MUST represent Markdown documents exclusively ($O(N)$ graph nodes), while section headings and fragments MUST be stored as addressable structural metadata attached to document nodes (Invariants V & VI).
- **FR-005**: All structural properties, code citations, wikilinks, and fragment anchors MUST be derived directly from the CommonMark Abstract Syntax Tree parsed via `pulldown-cmark` (Invariant VII).

#### Optimization Profile & Evolution Modes
- **FR-006**: The engine MUST support four first-class Documentation Evolution Modes: `AI-Oriented` (default), `Human-Oriented`, `Balanced`, and `Custom` (Invariant XIV).
- **FR-007**: All evolution modes MUST compile into a single, unified, inspectable internal `OptimizationProfile` struct containing concrete objective weights, parameter thresholds, and constraint sets.
- **FR-008**: The `OptimizationProfile` MUST support two distinct configuration tiers:
  - **Preferences (Soft Constraints / Weights)**: Objective function weights (Cohesion, Coupling, Context Efficiency, Discoverability, Navigational Depth, Vocabulary Consistency) used in ranking and scoring;
  - **Constraints (Hard vs Soft Boundaries)**: Inviolable boundaries (e.g., maximum fragment token size, forbidden directory cross-linking, mandatory frontmatter schemas, `never_auto_split_monoliths`).
- **FR-009**: Hard constraints MUST veto any candidate transformation strategy that would violate them, regardless of the strategy's calculated Fit Score.

#### 18 Knowledge Organization Methodologies Catalog
- **FR-010**: The engine MUST implement the **Classification** methodology: path-to-class mapping, centroid term vectors via BM25, category coverage calculation ($C_{cov}$), and directory nesting depth verification.
- **FR-011**: The engine MUST implement the **Taxonomy** methodology: hierarchical DAG validation, Tarjan SCC cycle detection, tree balance ratio ($I_{tax}$), and pathological branching factor detection ($b(u) > 15$).
- **FR-012**: The engine MUST implement the **Faceted Classification** methodology: frontmatter attribute orthogonality evaluation via Pairwise Mutual Information (PMI), facet sparsity calculation, and multidimensional queryability scoring.
- **FR-013**: The engine MUST implement the **Controlled Vocabulary** methodology: Finite State Transducer (FST) dictionary matching, case normalization, and Levenshtein string distance ($\le 2$) near-duplicate tag detection.
- **FR-014**: The engine MUST implement the **Thesaurus** methodology (ISO 25964 / SKOS): verification of reciprocal relationships (BT $\iff$ NT, USE $\iff$ UF, RT $\iff$ RT), hierarchical transitivity consistency, and cycle detection in broader-term trees.
- **FR-015**: The engine MUST implement the **Semantic Networks** methodology: directed typed relational edge validation (`[[relation::target]]`), relation type diversity ($H_{rel}$), and missing inverse relation detection.
- **FR-016**: The engine MUST implement the **Conceptual Maps** methodology: extraction and validation of propositional triples from structured syntax (Mermaid diagrams, triple tables), proposition density calculation ($P_{dens}$), and unlabeled link detection.
- **FR-017**: The engine MUST implement the **Ontology Engineering** methodology: deterministic structural shape validation via a W3C SHACL-like schema evaluator over YAML frontmatter, RDFS sub-property closure, and required property completeness checking.
- **FR-018**: The engine MUST implement the **Domain Analysis** methodology: TF-IDF domain lexical profiling, author-document bipartite graph modularity, and Git commit co-change coupling analysis without speculating on author intent.
- **FR-019**: The engine MUST implement the **Metadata / Application Profiles** methodology (Dublin Core Application Profiles - DCAP): schema conformance verification, mandatory property presence ($M_{comp}$), and date/type validation.
- **FR-020**: The engine MUST implement the **Information Architecture** methodology: all-pairs Dijkstra shortest path depth, heading lexical match Information Scent proxy, navigation dead-end detection, and click-depth budgeting ($d \le 5$).
- **FR-021**: The engine MUST implement the **Information Retrieval** methodology: embedded Tantivy inverted index, Okapi BM25 scoring, multi-field BM25F ranking with field weighting (title, H1, H2, body, code blocks), and deterministic tie-breaking.
- **FR-022**: The engine MUST implement the **Terminology Management** methodology: Aho-Corasick linear-time multi-pattern term searching, acronym expansion ambiguity detection, and deprecated terminology identification.
- **FR-023**: The engine MUST implement the **Knowledge Graph / Network Analysis** methodology: Petgraph directed graph model, Brandes betweenness centrality ($C_B$), PageRank with fixed power-iteration tolerance ($10^{-6}$), and Hopcroft-Tarjan bridge/articulation point identification.
- **FR-024**: The engine MUST implement the **Knowledge Quality** methodology: mathematical aggregation across the 5 canonical pillars (Integrity, Structure, Consistency, Discoverability, Freshness) evaluated relative to the active `OptimizationProfile`.
- **FR-025**: The engine MUST implement the **Provenance** methodology: W3C PROV-DM mapping to Git commit history, author attribution, commit co-change frequency, and commit drift tracking.
- **FR-026**: The engine MUST implement the **Knowledge Drift** methodology (Tennis Subject Ontogeny): tracking semantic and structural drift over successive Git revisions, measuring citation delta turnover and heading churn.
- **FR-027**: The engine MUST implement the **Context / Knowledge Routing** methodology: Context Trace section-level routing starting from configured AI Harness Entry Points (`AGENTS.md`, `CLAUDE.md`, `GEMINI.md`) via constrained beam search, evaluating BPE tokens, hop count, and context reduction percentage.

#### 5-Pillar Profile-Relative Health Model
- **FR-028**: Health scores MUST be computed separately for each of the 5 Pillars:
  - **Integrity**: Broken links, broken heading anchors, schema violations, invalid file paths;
  - **Structure**: Overloaded documents, taxonomic balance, fragment depth, cyclic inheritance, bridge bottlenecks;
  - **Consistency**: Uncontrolled tag variants, thesaurus reciprocity failures, naming collisions;
  - **Discoverability**: Orphan documents, unrouted clusters, high navigational depth ($d > 5$), low BM25 searchability;
  - **Freshness**: Stale code citations (Git commits modifying cited code after note commit), deleted symbol references.
- **FR-029**: The aggregate health score MUST be computed as a profile-weighted composite of the 5 pillar scores, never as an unweighted universal scalar.
- **FR-030**: All score models MUST maintain complete raw integer metrics (tokens, link counts, violation counts) internally and in JSON payloads, deferring compact formatting ($\le 5$ chars) strictly to the presentation layer (Invariants IX & X).

#### Structural Diagnosis, Strategy Catalog & Recommendations
- **FR-031**: Every structural diagnosis MUST clearly distinguish between **Observable Evidence** (raw metric values, AST spans, Git commits), **Interpretation** (anti-pattern classification), and **Recommendation** (suggested intervention).
- **FR-032**: The engine MUST evaluate candidate strategies from the 6-strategy catalog:
  - `PreserveDocument`: Leave document intact when cohesion is high despite large token size;
  - `SplitDecompose`: Partition overloaded, multi-topical documents into cohesive focused notes;
  - `ExtractFacetMetadata`: Factor out orthogonal dimensions into structured frontmatter facets;
  - `ConsolidateIndex`: Embed direct routing tables into harness entry points to eliminate intermediate hopping;
  - `MergeCandidates`: Consolidate fragmented near-duplicate documents into a canonical authority;
  - `StandardizeTerminology`: Reconcile variant terms and tags with an authorized controlled glossary.
- **FR-033**: The engine MUST calculate an empirical **Fit Score** ($0.0 \dots 1.0$) for each candidate strategy using the Analytic Hierarchy Process (AHP) weighted against the active `OptimizationProfile`, and MUST compute an `implementation_cost` score based on predicted file mutation count and line churn for deterministic tie-breaking.
- **FR-034**: Strategies MUST be classified into pairwise relational roles: `PRIMARY`, `SUPPORTING`, `ALTERNATIVE`, and `MUTUALLY_EXCLUSIVE`.
- **FR-035**: Every recommendation MUST explicitly output both `expected_benefits` and `countervailing_trade_offs` (Invariant XV).

#### User-Controlled Automation & Downstream Governance
- **FR-036**: All structural transformations MUST have `Auto-Apply: OFF` and `Require Approval: ON` by default (Invariant XVI).
- **FR-037**: Execution of any transformation plan MUST require an explicit user-provided `approval_token` generated during the recommendation inspection phase.
- **FR-038**: Applied transformations MUST execute purely deterministic AST and filesystem mutators (file moves, frontmatter updates, link retargeting, section extraction) without AI code generation.
- **FR-039**: Pre-commit hook validation (`llm-wiki-validate`, stage: pre-commit) MUST evaluate staged changes against an in-memory ephemeral candidate graph, re-staging modified frontmatter and blocking commits only on new high-confidence regressions within $\le 500$ ms fail-open (Invariants I, II, XI, XII).
- **FR-039b**: Post-commit hook synchronization (`llm-wiki-sync`, stage: post-commit) MUST ingest the committed Git delta and incrementally update the persistent knowledge graph in $\le 50$ ms without performing a cold full-vault rescan (Constitution Principle II).
- **FR-040**: Every applied transformation MUST trigger an immediate Before-vs-After comparative re-measurement displaying empirical deltas across all 5 pillars and raw token economics.

#### Integration, IPC & VS Code Extension
- **FR-041**: Communication between the VS Code Extension Host and the Rust core sidecar MUST use JSON-RPC 2.0 over standard input/output (stdio), adhering to typed contracts.
- **FR-042**: The extension MUST present a unified 4-card management UI (Health, Structure, Evolution, Settings) in a sandboxed Webview using Content Security Policy (CSP) nonces.
- **FR-043**: The UI MUST provide interactive controls to switch Documentation Evolution Modes, inspect candidate strategies, toggle constraints, and review Before/After transformation diffs.
- **FR-044**: The extension MUST support bidirectional navigation: clicking any finding or recommendation in the webview MUST open the corresponding Markdown file in the editor jumped to the exact line and column.
- **FR-045**: The extension MUST retrieve any sensitive tokens or configuration from `vscode.SecretStorage` and pass them in-memory, never writing plaintext credentials to disk or command-line arguments.
- **FR-046**: The engine MUST expose graph topology data ($O(N)$ document nodes and cross-document reference edges) over JSON-RPC 2.0 to support the Webview 2D force-directed canvas and Structure drawer inspection (Invariant V).

---

### Key Entities

- **`OptimizationProfile`**: The first-class objective function entity specifying active evolution mode, pillar weights, threshold cutoffs, and constraint lists.
- **`EpistemicTier`**: Classification tag (`TierA_FullyDeterministic`, `TierB_CandidateGeneration`, `TierC_DecisionSupport`, `TierD_NotReliablyDeterminable`) attached to every diagnostic and recommendation.
- **`Methodology`**: Enum representing the 18 KOS disciplines evaluated by the engine.
- **`StructuralFinding`**: Concrete diagnostic instance containing finding type, severity, pillar, epistemic tier, exact file/AST span, observed evidence metrics, and human-readable explanation.
- **`StrategyRecommendation`**: Actionable transformation candidate containing strategy type, target document(s), Fit Score, pairwise role (`PRIMARY`, `SUPPORTING`, `ALTERNATIVE`, `MUTUALLY_EXCLUSIVE`), expected benefits, and countervailing trade-offs.
- **`TransformationPlan`**: Concrete execution bundle containing `plan_id`, `approval_token`, ordered atomic mutation steps, unified diff previews, and rollback actions.
- **`HealthScore`**: Profile-relative 5-pillar composite score ($0 \dots 100$) with individual pillar breakdowns, legacy debt baselines, and new regression deltas.
- **`ContextTrace`**: Simulated retrieval path from a harness entry point to a target fragment, capturing hop distance, accumulated BPE tokens, relevance score, and context reduction percentage.
- **`BLAKE3AnalysisId`**: 256-bit cryptographic digest uniquely identifying the analytical execution context.

---

## Non-Functional Requirements (NFR)

- **NFR-001 (Zero External AI)**: The Rust core engine MUST NOT execute HTTP network requests, link against neural network runtimes, or require external LLM inference keys.
- **NFR-002 (Determinism Verification)**: Two identical runs on the same Git commit and configuration MUST produce identical BLAKE3 hashes for all serialized analysis reports.
- **NFR-003 (Incremental Calculation & Sync Latency)**: Recalculating candidate diagnostics for an edited document or performing post-commit graph sync (`Hook 2: llm-wiki-sync`) for a committed delta in a 1,000-document repository MUST complete in $\le 50\text{ ms}$.
- **NFR-004 (Full Repository Scan Latency)**: Cold scanning and full 18-methodology analysis of a 1,000-document repository MUST complete in $\le 1,500\text{ ms}$ on standard multi-core hardware.
- **NFR-005 (Pre-Commit Hook Budget)**: The pre-commit validation hook (`llm-wiki-validate`) MUST execute within $\le 500\text{ ms}$, failing open (exit code 0) if an unrecoverable internal error or timeout occurs.
- **NFR-006 (Memory Footprint)**: Resident memory (RSS) in resting state MUST NOT exceed 35 MB for 5,000 documents, and peak memory consumption during full repository analysis of 2,000 documents MUST NOT exceed 250 MB.
- **NFR-007 (Numeric Integrity & Presentation Budget)**: All persisted and IPC data models MUST use 64-bit integer numeric types; all UI formatted numbers MUST NOT exceed 5 characters.
- **NFR-008 (Sandboxed Webview Security)**: VS Code Webview panels MUST enforce strict CSP without `unsafe-inline` or `unsafe-eval`, using dynamic cryptographically random nonces.
- **NFR-009 (Binary Packaging Size)**: The compiled standalone `llm-wiki-core` native binary bundled inside the VS Code extension MUST NOT exceed 25 MB per target platform.
- **NFR-010 (Platform Portability)**: The Rust engine MUST compile and pass 100% of integration test suites on Linux x86_64, Linux aarch64, macOS x86_64, macOS Apple Silicon (arm64), and Windows x86_64.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of diagnostic findings and recommendations are strictly tagged with their appropriate Epistemic Tier (Tier A, B, C, or D), with 0 false claims of semantic truth in Tier A.
- **SC-002**: Cold analysis of a 1,000-note repository executes in $\le 1.5$ seconds, and incremental candidate evaluation or post-commit sync completes in $\le 50$ ms.
- **SC-003**: 100% of pre-commit hook runs complete within $\le 500$ ms fail-open budget, successfully catching new broken links and dead anchors while ignoring legacy brownfield debt.
- **SC-004**: Context Trace section-level routing demonstrates $\ge 75\%$ context token reduction compared to full-document loading across benchmark repositories.
- **SC-005**: 100% of generated structural transformation plans require explicit user approval (`Auto-Apply: OFF`) and successfully verify Before/After deltas upon application.
- **SC-006**: Health scores and recommendations dynamically update when switching between `AI-Oriented`, `Human-Oriented`, `Balanced`, and `Custom` modes without restarting the core process.
- **SC-007**: 0 external AI API calls or cloud network requests executed across the entire test suite and runtime operation.
- **SC-008**: 100% test coverage for Tarjan SCC cycle detection, Brandes betweenness centrality, PageRank convergence, Aho-Corasick term matching, and ISO 25964 thesaurus reciprocity.

---

## Assumptions

- The primary corpus consists of CommonMark-compliant Markdown documents with optional YAML or TOML frontmatter headers.
- Repositories are managed with Git version control, providing an immutable commit log and staging area.
- The user's system provides a 64-bit operating system (Linux, macOS, or Windows) with Visual Studio Code 1.85+ installed.
- Complex semantic interpretation (e.g. evaluating whether two conceptually different technical explanations are equivalent in meaning) is recognized as fundamentally non-deterministic and delegated to human or external agent review.
- Transformations generated by the engine are discrete, syntactically verifiable AST and filesystem mutations that can be cleanly reviewed in standard Git diff viewers.
