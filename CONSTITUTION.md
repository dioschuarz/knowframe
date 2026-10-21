---
wiki:
  content_hash: 82125b2efc7fc3fb1048e70f08338e28d66b06379584c05d6d3710d7cb6f56e0
  fragments:
    knowframe-extension-for-vs-code-knowframe-constitution:
      tokens: 8288
      inbound: 0
      outbound: 3
    the-17-critical-invariants-non-negotiable:
      tokens: 1266
      inbound: 0
      outbound: 0
    core-principles:
      tokens: 5698
      inbound: 0
      outbound: 0
    i-dual-process-separation-local-determinism-zero-external-ai-non-negotiable:
      tokens: 319
      inbound: 0
      outbound: 0
    ii-git-knowledge-state-boundary-ephemeral-candidate-graph-dual-hook-lifecycle-non-negotiable:
      tokens: 700
      inbound: 0
      outbound: 0
    iii-organic-discovery-curated-representation-noise-exclusion:
      tokens: 457
      inbound: 0
      outbound: 0
    iv-document-centric-graph-section-addressability-materialized-frontmatter-layer-non-negotiable:
      tokens: 654
      inbound: 0
      outbound: 0
    v-multi-tier-context-hierarchy-semantic-sizing-focus-mode-token-budgeting:
      tokens: 670
      inbound: 0
      outbound: 0
    vi-strict-link-integrity-cross-submodule-vault-traversal:
      tokens: 155
      inbound: 0
      outbound: 0
    vii-brownfield-debt-baseline-continuous-regression-based-admission-control-non-negotiable:
      tokens: 390
      inbound: 0
      outbound: 0
    viii-deterministic-knowledge-drift-reverse-index-impact-semantic-collision-engine:
      tokens: 443
      inbound: 0
      outbound: 0
    ix-context-trace-5-pillar-network-aware-knowledge-health-model:
      tokens: 589
      inbound: 0
      outbound: 0
    x-sandboxed-webview-progressive-disclosure-zero-action-duplication:
      tokens: 388
      inbound: 0
      outbound: 0
    xi-test-driven-development-tdd-multi-platform-binary-verification-performance-budgets:
      tokens: 248
      inbound: 0
      outbound: 0
    xii-documentation-orientation-strategy-trade-offs-user-controlled-structural-evolution:
      tokens: 681
      inbound: 0
      outbound: 0
    technology-stack-performance-standards:
      tokens: 526
      inbound: 0
      outbound: 0
    1-technology-matrix:
      tokens: 226
      inbound: 0
      outbound: 0
    2-non-functional-performance-budgets:
      tokens: 293
      inbound: 0
      outbound: 0
    development-workflow-review-process-quality-gates:
      tokens: 266
      inbound: 0
      outbound: 0
    1-code-quality-gates:
      tokens: 135
      inbound: 0
      outbound: 0
    2-review-protocol:
      tokens: 121
      inbound: 0
      outbound: 0
    governance:
      tokens: 157
      inbound: 0
      outbound: 0
---
# KnowFrame Extension for VS Code (`knowframe`) Constitution

> Canonical Source of Truth: [`.specify/memory/constitution.md`](.specify/memory/constitution.md)  
> AI Agent Entry Points: [`AGENTS.md`](AGENTS.md) | [`GEMINI.md`](GEMINI.md)

---

<!--
=== SYNC IMPACT REPORT ===
Version Change: 1.6.0 -> 1.7.0 (Autonomous Product Core, Elimination of Decision Fatigue & Invariant XVII / P1–P7)
Modified Principles:
  - Expanded Section: "The 17 Critical Invariants" establishing the non-negotiable architectural bedrock:
      - Added Invariant XVII (Autonomous Product Core & Non-Interfering Complementarity / P1–P7):
          * P1 (Autonomy): KnowFrame operates 100% standalone with zero mandatory external tool prerequisites.
          * P2 (Exclusive Responsibility): Bounded, non-overlapping component roles (KnowFrame = documented knowledge; AI Memory = cross-session agent working memory; Graphify = optional code AST diagnostic; Understand Anything = eliminated).
          * P3 (Non-Interference): Complementary tools are passive and MUST NEVER auto-mutate Markdown files, canonical graph, or product decisions.
          * P4 (Centralized Governance): KnowFrame is the sole active governor of documented knowledge.
          * P5 (Optional Complementarity): External tools enhance specific workflows optionally; their absence never impairs core functionality.
          * P6 (Proportional Complexity): No integrations without concrete gaps justifying maintenance and UX costs.
          * P7 (Zero Unnecessary Decisions): The user never faces configuration hurdles, tool selection, or decision fatigue to begin using KnowFrame.
Follow-up TODOs: None deferred.
==========================
-->

## The 17 Critical Invariants (NON-NEGOTIABLE)

The following seventeen invariants constitute the immutable architectural foundation of KnowFrame. Every component, data model, RPC interface, and user workflow MUST strictly obey these invariants without exception:

1. **Invariant I — Committed Truth**: The persistent Knowledge Graph MUST represent committed repository state only. Working-tree changes or file-save events MUST NOT mutate the persistent graph (`file saved ≠ knowledge graph update`).
2. **Invariant II — Candidate Isolation**: Pre-commit analysis MUST evaluate staged changes exclusively in an in-memory ephemeral candidate graph, which is completely discarded after validation and MUST NOT contaminate persistent state.
3. **Invariant III — Determinism**: Identical repository state MUST produce identical analytical results. Randomness, unseeded heuristics, and non-deterministic traversals are strictly prohibited.
4. **Invariant IV — Locality**: Core knowledge analysis, graph computation, token accounting, collision detection, and drift estimation MUST NOT depend on external AI APIs, cloud services, remote embeddings, or external networks.
5. **Invariant V — Document-Centric Graph**: Only Markdown documents are Knowledge Graph nodes ($O(N)$ graph complexity). Sections and fragments are addressable structural metadata attached to Document Nodes, NEVER independent graph nodes.
6. **Invariant VI — Section Addressability**: Document fragments MUST be deterministically addressable using standard Markdown URI fragment targets derived from normalized heading anchors (`document.md#fragment-anchor`).
7. **Invariant VII — AST Authority**: The Markdown AST is the ultimate structural source of truth. Any structural property (headings, anchors, source ranges, code citations) MUST be derived directly from the AST.
8. **Invariant VIII — Materialized Metadata**: Document frontmatter serves as a materialized structural metadata layer (`wiki.fragments`), deterministically derived from and validated against the Markdown AST during pre-commit. Computational cost is shifted to commit time rather than query time.
9. **Invariant IX — Raw Numeric Integrity**: All analytical and persisted numeric values MUST retain their complete raw integer representation (e.g., `tokens: 1043256`, `inbound: 31`). Persisted files, RPC payloads, and graph models MUST NOT truncate or pre-format numbers.
10. **Invariant X — Presentation Separation**: Compact numeric formatting (e.g., `1.04M`, `543`, `13.3k`) MUST exist exclusively at the presentation layer and MUST NOT exceed 5 characters. Formatted strings MUST NEVER be fed back into analytics, scoring, or persistent data models.
11. **Invariant XI — Brownfield Tolerance**: Pre-existing repository debt MUST be baselined and visible without blocking commits. Pre-commit validation MUST strictly distinguish legacy debt from new regressions (`New Regressions = Candidate \ Baseline`).
12. **Invariant XII — Quality Gate**: New high-confidence knowledge regressions (e.g., new broken internal links, new invalid heading anchors, new high-confidence collision duplicates) MUST be capable of deterministically blocking commits.
13. **Invariant XIII — Non-Prescriptive Knowledge Organization**: The system MUST NOT mandate or dogmatize a single documentation methodology (e.g., DITA, Wiki-links, hierarchical taxonomy, topic-based). Document size, link count, and section density are analytical signals, NEVER automatic verdicts. A 50k-token document can be structurally healthy, while a 5k-token document can be overloaded.
14. **Invariant XIV — Unified Optimization Profile**: All Documentation Orientations (`AI-Oriented` [default], `Human-Oriented`, `Balanced`, `Custom`) MUST compile into a single, inspectable internal `OptimizationProfile` struct. The engine MUST NOT run separate, divergent heuristic engines per mode.
15. **Invariant XV — Transparent Trade-Off & Evidence-Based Recommendation**: Every structural problem detection MUST be derived from observable multi-dimensional metrics (cohesion, coupling, concentration, semantic diversity, context efficiency), and every recommendation MUST explicitly articulate both expected benefits and countervailing trade-offs (e.g., increased fragmentation, metadata complexity, or index maintenance).
16. **Invariant XVI — User-Controlled Downstream Automation & Re-Measurement**: Automated structural transformations (e.g., generating index routers, section-level refactoring, frontmatter injection) MUST be strictly downstream of explicit user orientation choice and approval (`Auto-Apply: OFF`, `Require Approval: ON` by default) and MUST respect Hard Constraints. Every applied transformation MUST be verified through deterministic Before-vs-After comparative re-measurement.
17. **Invariant XVII — Autonomous Product Core & Non-Interfering Complementarity (P1–P7)**:
    - **P1 (Autonomy)**: KnowFrame MUST deliver its complete core value proposition standalone. Complementary tools are never prerequisites.
    - **P2 (Exclusive Responsibility)**: Each component has a strictly bounded, non-overlapping responsibility:
      - **KnowFrame**: Governs Markdown files, organizes knowledge, maintains the canonical graph, detects inconsistencies, and manages documentation evolution.
      - **AI Memory**: Maintains local cross-session working memory for agent and dev tooling contexts; does NOT govern documentation or replace the canonical graph.
      - **Graphify**: Analyzes observed code architecture; does NOT govern KnowFrame, alter Markdown, or auto-update the canonical graph.
      - **Understand Anything**: Overlaps with Graphify; strictly eliminated from product recommendations, catalogs, and UI.
    - **P3 (Non-Interference)**: Complementary tools are passive relative to KnowFrame and MUST NEVER mutate Markdown files, the canonical graph, or product decisions.
    - **P4 (Centralized Governance)**: KnowFrame is the sole active authority over documented knowledge.
    - **P5 (Optional Complementarity)**: External tools provide optional diagnostic analyses without hindering core functionality when absent.
    - **P6 (Proportional Complexity)**: Integrations are only introduced when solving a concrete, verified need that justifies their maintenance and UX cost.
    - **P7 (Zero Unnecessary Decisions)**: The user NEVER has to choose tools, select strategies, or perform configuration setups to begin using KnowFrame. No configuration hurdle or decision fatigue.

---

## Core Principles

### I. Dual-Process Separation, Local Determinism & Zero External AI (NON-NEGOTIABLE)
The system MUST be architected into two decoupled execution tiers: a lightweight Node.js/TypeScript Extension Host and a standalone Rust Core Engine sidecar (`llm-wiki-core`).
- **Heavy Compute in Rust**: Parsing Markdown AST (`pulldown-cmark`), computing graph topologies and centrality metrics (`petgraph`), evaluating token economics (`tiktoken-rs` with `cl100k_base`), maintaining reverse code-doc indexes, evaluating deterministic similarity, extracting section fragments, calculating freshness evidence, and parsing Git diff deltas MUST be executed exclusively by the Rust sidecar process.
- **Node.js as Lean UI Orchestrator**: The VS Code Extension Host process MUST NOT perform file scraping, heavy AST parsing, or graph math. Its responsibilities are strictly constrained to process lifecycle management, IPC/WebSocket bridging, native VS Code API dispatch, and Webview hosting.
- **Zero External AI / LLM Invariant**: The product MUST NOT use external LLM APIs, remote embeddings, external cloud semantic search, remote rerankers, or external AI services. All analysis, drift detection, candidate evaluation, Context Trace routing, and quality scoring MUST be executed locally and deterministically.
- **Resource Ceiling**: The Rust binary MUST NOT exceed 35 MB of resident RAM in resting state for vaults up to 5,000 Markdown files. Full vault graph calculation for 1,000 notes MUST execute in under 200 ms.

### II. Git Knowledge State Boundary, Ephemeral Candidate Graph & Dual Hook Lifecycle (NON-NEGOTIABLE)
The system MUST NOT continuously monitor file saves or keystrokes. The boundary of persistent knowledge is strictly the Git commit.
- **The Knowledge Graph State Boundary Invariant**:
  > *The Knowledge Graph MUST represent only committed repository state. Working-tree changes MAY be analyzed in an ephemeral candidate graph during pre-commit validation, but MUST NOT mutate the persistent Knowledge Graph until the corresponding Git commit succeeds.*
  A file saved in the editor is NOT a knowledge graph update (`file saved ≠ knowledge graph update`).
- **Dual Analytical Modes**:
  1. *Mode A (Admission Control)*: Executed during `pre-commit` against an in-memory candidate graph. Answers: *"Can this change enter the knowledge base?"* Potentially blocks or warns.
  2. *Mode B (Knowledge State Analysis)*: Executed against the persistent committed graph. Answers: *"What is the current state of my knowledge base?"* Never mutates until commit succeeds.
- **Ephemeral Candidate Graph Construction**:
  During pre-commit, Rust constructs an ephemeral candidate graph:
  $$\text{Candidate Graph} = \text{Committed Graph} + \text{Staged Git Delta}$$
  The candidate graph exists solely in memory. If validation passes, the commit proceeds, and post-commit incorporates the delta into the persistent index. If validation blocks or the user aborts, the candidate graph is discarded without contaminating persistent state.
- **Dual Hook Architecture via `prek`**:
  1. *Hook 1 (`llm-wiki-validate`, stage: `pre-commit`)*: Analyzes staged files (Markdown and code), performs reverse impact analysis, checks for regressions against the debt baseline, materializes structural frontmatter, and returns `PASS`, `WARN`, or `BLOCK`.
  2. *Hook 2 (`llm-wiki-sync`, stage: `post-commit`)*: Applies the committed Git delta to the persistent Rust Knowledge Graph via incremental patching ($\le 50\text{ ms}$).
- **Hook Discovery & Execution Fallbacks**:
  1. *Tier 1 (Rust-Native / Primary)*: `prek` executing declarative *.pre-commit-config.yaml* with parallel hook execution.
  2. *Tier 2 (Ecosystem Compatibility)*: Python `pre-commit` or `lefthook` if already present in repository.
  3. *Tier 3 (Zero-Dependency Script)*: Standalone `.git/hooks/` shell dispatchers executing `llm-wiki-core validate` and `llm-wiki-core git-delta`.
- **Strict Non-Blocking Fail-Open**: All hook invocations MUST enforce a hard non-blocking timeout of $\le 500\text{ ms}$. If the socket is unresponsive, the sidecar is offline, or the daemon encounters an internal error, the hook MUST immediately exit with code 0 (silent fail-open), never obstructing or delaying developer Git commits.
- **Loopback Authorization**: Loopback `git/validate` and `git/commitDelta` notifications on `127.0.0.1` are write-only notifications carrying Git metadata, accepted without requiring `vscode.SecretStorage` credentials so external terminal commits succeed non-blockingly.

### III. Organic Discovery, Curated Representation & Noise Exclusion
The knowledge graph MUST build organically from the user's existing filesystem chaos without demanding YAML frontmatter, rigid directory structures, or canonical entry filenames (`README.md`, `index.md`).
- **Core Knowledge Axiom**: *The filesystem is the source; the knowledge graph is a curated representation of what matters.* The system is not a raw file tree visualizer; its mandate is to distill structural knowledge relevant to developer tasks and AI agent context.
- **Tri-State Node Classification**: Every Markdown document known to the system MUST belong to exactly one of three lifecycle states:
  1. **Connected**: Part of the knowledge graph and reachable via link traversals.
  2. **Orphan**: Part of the knowledge graph but possessing zero inbound and outbound connections.
  3. **Excluded**: Intentionally outside the knowledge graph by user directive or default workspace hygiene rules.
- **Two-Tier Exclusion Architecture**:
  1. *Technical Scan Exclusion (`scan.exclude`)*: Files and directories matching technical ignore globs (e.g. `.git/**`, `node_modules/**`, `target/**`, `dist/**`, `build/**`) are completely bypassed by the AST parser and indexer.
  2. *Curated Knowledge Exclusion (`graph.exclude`)*: Files and directory trees (e.g. package documentation, library `**/README.md`, vendor licenses) that are parsed and indexed to maintain link integrity, but are intentionally excluded from the rendered graph and orphan calculations.
- **Target Resolvability Invariant**: Valid Markdown links or wikilinks targeting a `graph.exclude` document MUST resolve cleanly and MUST NOT be classified as broken links. Excluded targets are valid knowledge destinations that the user has chosen not to visualize.
- **Algorithmic Hub Discovery**: The Rust engine MUST calculate degree centrality (in-degree, out-degree) and betweenness centrality across all unexcluded Markdown files to mathematically deduce central knowledge hubs.
- **User Agency & Consent**: The system MUST propose top centrality nodes as default hubs and propose noisy file clusters for exclusion, but MUST NEVER silently exclude documentation or alter central hubs without explicit user consent.

### IV. Document-Centric Graph, Section Addressability & Materialized Frontmatter Layer (NON-NEGOTIABLE)
The physical unit of storage and graph modeling remains the Markdown document. Documents are graph nodes; fragments are addressable metadata attached to Document Nodes.
- **Document Node Representation**: The graph node represents the complete Markdown document (path, title, tags, total tokens, degree, centrality, status, fragments). Graph layout remains $O(N)$, preventing topological explosion while enabling section-level precision.
- **Fragment / Section Derivation**: The Rust AST parser extracts all Markdown headings (`## Heading`), deterministically normalizes them into anchors (`#heading`), and associates each fragment with:
  - Anchor string (unique identity within the document)
  - Heading title text
  - Source byte/line range (`start_line`, `end_line`)
  - Fragment BPE token count (`tokens`)
  - Inbound reference count (`inbound`)
  - Outbound reference count (`outbound`)
- **Standard Section Addressability**: The system fully supports standard Markdown section targets: `[Label](./doc.md#fragment)`. Incoming fragment references are resolved against the target document's derived fragment anchors. Missing or dangling anchors MUST be flagged as Integrity defects.
- **Materialized Frontmatter Layer**: Frontmatter (`wiki.fragments`) serves as a materialized structural metadata cache.
  - *AST Authority*: The Markdown AST is the structural source of truth. Frontmatter is strictly derived from and validated against the AST.
  - *Commit-Time Materialization*: During the pre-commit stage, when a document is modified, Rust recalculates fragment metadata and writes or updates the materialized frontmatter block.
  - *Pay at Commit, Fast at Query*: Shifting computational cost to the Git commit prevents repeated on-demand AST re-parsing, heading lookups, and token counting during interactive queries.
- **Raw Numeric Integrity vs. Presentation Separation**:
  - *Analytical Layer*: Data models, frontmatter YAML, JSON-RPC contracts, and Rust structs MUST store complete raw numeric values as integers (e.g. `tokens: 50231`, `inbound: 31`).
  - *Presentation Layer*: The UI transforms raw numbers into compact representations strictly $\le 5$ characters (`50.2k`, `543`, `1.04M`). Formatted strings MUST NEVER be fed back into analytics.
- **Fragment Inspector**: Selecting a Document Node in the Webview displays its fragments in a dedicated 3-column table (`Fragment | Tok | I/O`) using compact display values while backed by raw metrics.
- **Fragment Reference Analytics & Context Reduction**:
  - The engine calculates Reference Concentration ($C = \text{inbound}_{\text{fragment}} / \text{inbound}_{\text{doc}}$).
  - High reference concentration on large documents ($C \ge 70\%$, tokens $\ge 50\text{k}$) produces structural guidance: routing directly to `doc.md#fragment` achieves $\approx 90\%$ context reduction (e.g., $50\text{k} \to 5\text{k}$ tokens) for AI agent consumption.

### V. Multi-Tier Context Hierarchy, Semantic Sizing, Focus Mode & Token Budgeting
The extension MUST function as an **AI Context Observatory**, providing instant visual clarity on agent context window consumption to prevent context saturation in AI agents (Claude Code, Codex, Antigravity, Roo/Cline).
- **Context Tiers & Opacity Decay Law**:
  - **Level 0 (Active Context / Harness Base)**: Fixed agent prompts and harness files (`AGENTS.md`, `CLAUDE.md`, or selected Hub). Rendered with 100% solid opacity and expanded node radius.
  - **Level 1 (First Hop)**: Files directly referenced by Level 0. Rendered at 60% opacity.
  - **Level 2+ (Deep Hops)**: Transitive reference chains. Rendered at decaying opacity (30% at Level 2, 15% at Level 3+).
  - **Orphan Nodes**: Disconnected notes with zero edges. Rendered in muted gray (`#6B7280`) on the graph perimeter unless custom cluster rules override.
- **Semantic Visual Hierarchy & Sizing**: Node geometry MUST visually communicate context role rather than raw connection volume alone. Level 0 hubs are rendered with prominent radius and bold labels; Level 1 with medium radius; Level 2 with compact radius; Level 3+ with subtle indicators.
- **Focus Modes**: To eliminate visual overwhelm in large vaults (500+ notes), the graph renderer MUST support hop-constrained focus view states (`Hub`, `1 hop`, `2 hops`, `All`). In focused modes, secondary or peripheral nodes are subdued or collapsed to keep cognitive load minimal.
- **User-Defined Cluster Coloring (Hashtags & Paths)**:
  - The system MUST allow users to define custom chromatic cluster groups based on:
    1. *Cluster Hashtags*: Extracted from inline Markdown hashtags (`#tag`) and YAML frontmatter `tags: [...]`.
    2. *Filesystem Paths*: Directory globs matching note physical paths (e.g. `docs/architecture/**`, `notes/05_secrets/**`).
  - *Hue + Opacity Composition Invariant*: User-defined cluster colors determine the **base hue/saturation** of the node. The **alpha/opacity channel is strictly governed by the hop distance** (Level 0 = 1.0, Level 1 = 0.6, Level 2 = 0.3, Level 3+ = 0.15). This guarantees that custom styling enriches semantic clustering without compromising the visual differentiation between active and secondary LLM context.
  - *Neutral Grey Baseline*: Unstyled nodes default to neutral grey (`#64748B` for connected, `#6B7280` for orphans). Grey strictly signifies that no custom cluster color rule has been assigned.
- **BPE Token Accounting**: The Rust engine MUST compute exact BPE tokens (`cl100k_base`) per file, per fragment, and cumulative branch cost. The UI MUST display **Active Prompt Load (L0)** versus **Reachable On-Demand Context (L1+)**, raising warnings at 60,000 tokens and critical alerts at 100,000 tokens.

### VI. Strict Link Integrity & Cross-Submodule Vault Traversal
The engine MUST enforce 100% internal link validation across both standard Markdown links `[text](path)` and wikilinks `[[filename]]`.
- **Dead Link Detection**: Links pointing to missing files, broken anchors, or moved targets MUST be immediately flagged with red dashed edges, diagnostic issue cards, and line-precise coordinates.
- **Section Anchor & Fragment Integrity**: Links specifying `#fragment` anchors MUST validate against the target document's derived headings. Dangling fragment links are classified as Integrity defects.
- **Multi-Repo & Submodule Boundary Resolution**: In multi-repo and unified vault configurations, relative links crossing Git submodule boundaries MUST be resolved transparently without fragmenting the graph topology.

### VII. Brownfield Debt Baseline & Continuous Regression-Based Admission Control (NON-NEGOTIABLE)
A repository adopted by KnowFrame may start with legacy broken links, orphans, collisions, and stale documents. The product MUST NOT make adoption impossible by immediately blocking existing problems.
- **Regression-Based Enforcement Invariant**:
  > *Pre-commit validation MUST prevent the introduction of new high-confidence knowledge-quality regressions while preserving and exposing pre-existing brownfield debt.*
- **Knowledge Debt Baseline (*baseline.json*)**:
  - The Rust Core establishes a baseline of known findings upon initialization or initial scan (`baseline_commit = <sha>`), recording counts of legacy broken links, orphans, collisions, structural flaws, and drift findings.
  - Existing debt is reported, tracked, and prioritized for remediation, but NEVER blocks developer commits.
- **Continuous Enforcement Lifecycle**:
  - New issues introduced by a commit (new broken links, new broken fragment anchors, new high-confidence collisions) are classified as **New Regressions** and trigger pre-commit `BLOCK`.
  - When developers fix legacy problems, the baseline progressively ratchets down toward zero ($\Delta < 0$).
  - The system MUST track five distinct accounting dimensions: $\text{Total Debt}$, $\text{Legacy Debt}$, $\text{New Debt}$, $\text{Resolved Debt}$, and $\text{Regressions}$.
- **Decision Engine Thresholds**:
  - `BLOCK`: High-confidence regressions that must not enter the repo (new broken internal references, new invalid heading anchors, high-confidence duplicate with canonical collision).
  - `WARN`: Ambiguous or potential problems requiring human judgment (moderate semantic similarity $\ge 75\%$, missing fragment metadata on oversized documents, potential fragmentation, potential drift with incomplete evidence).
  - `PASS`: No regressions introduced, or only improvements/remediation committed.

### VIII. Deterministic Knowledge Drift, Reverse-Index Impact & Semantic Collision Engine
The engine MUST detect divergence between code and documentation and eliminate duplicate conceptual authority without utilizing external LLMs or vector embeddings.
- **Code-Doc Reverse Impact Index**:
  - The Rust engine maintains reverse indexes mapping:
    1. Code file paths $\to$ documentation references citing those symbols/paths.
    2. Document paths $\to$ related code paths and APIs.
    3. Document $\to$ inbound and outbound knowledge links.
  - Staged code changes (e.g. `src/auth/*`) immediately identify the affected documentation surface (`authentication.md`, `oauth.md`) without repository-wide recomputation.
- **Technical Drift Detection (at Pre-Commit)**:
  - Technical drift occurs when documented entities (classes, methods, file references) are deleted, renamed, or modified in code while corresponding documentation remains unchanged.
  - Technical drift MUST be detected at pre-commit so that out-of-sync documentation can be flagged before entering the repository.
- **Deterministic Semantic Collision Engine (at Pre-Commit)**:
  - Avoids $O(N^2)$ exhaustive comparison by generating multi-signal search fingerprints: title fingerprint, heading tokens, BM25/TF-IDF term vectors, tag sets, and structural n-grams.
  - Staged documents retrieve candidate match sets (top 10–50 plausible matches) for detailed deterministic scoring.
  - Similarity combines title matching, heading overlap, term overlap, n-gram overlap, and reference intersection into a normalized similarity score $[0.0, 1.0]$.
  - Distinguishes 5 actionable classifications: `PotentialDuplicate`, `PotentialFragmentation`, `PotentialSpecialization`, `PotentialSummary`, `PotentialConflict`.
- **Canonical Knowledge Candidate Identification**:
  - For overlapping knowledge, the engine proposes canonical candidates based on deterministic signals: Git creation chronology, in-degree centrality, reference by harness entrypoints (`AGENTS.md`), and heading completeness.
  - *Chronology Invariant*: Git creation date is evidence, NOT infallible authority. The system suggests canonical ownership; the developer confirms.

### IX. Context Trace & 5-Pillar Network-Aware Knowledge Health Model
The engine MUST evaluate structural graph health, organizational architecture, and agent discoverability to guide developers toward optimal knowledge engineering.
- **Context Trace Formulation**:
  - Evaluates: *"Given a concrete task query and a knowledge entry point (e.g. `AGENTS.md`), what path through the project's knowledge leads to the relevant information, and can it be targeted at the section level?"*
  - Executes deterministically via constrained beam search:
    $$\text{Query} \to \text{Relevance Scoring} \to \text{Beam Expansion} \to \text{Candidate Paths} \to \text{Section-Level Refinement}$$
  - Signals: query terms, document titles, fragment headings, aliases, tags, graph proximity, centrality, token cost.
  - Outputs: Traversal path hops, path depth, selected fragment, document token load, fragment token load, potential context reduction ($\approx 90\%$), and explainable relevance justification. Zero external LLM inference.
- **The 5-Pillar Health Model**:
  Overall Knowledge Quality Score ($0\text{--}100$) aggregates five balanced dimensions:
  1. **Integrity**: Structural validity of knowledge references (broken links, dangling anchors, unmapped targets, invalid cross-repo links).
  2. **Structure**: Organizational architecture and addressability (missing fragment metadata on large files, oversized documents $>50\text{k}$ tokens, reference concentration $>70\%$, unfragmented hubs, low context utilization).
  3. **Consistency**: Agreement and semantic coherence (collisions, canonical conflict, contradictory claims, conceptual fragmentation).
  4. **Discoverability**: Reachability through meaningful paths (Context Trace depth, hub accessibility, centrality distribution, orphan ratio).
  5. **Freshness**: Evidence of currentness (referenced code entities exist on disk, code modified in Git after doc updates, deleted/renamed symbols, canonical contradictions). Freshness is calculated by deterministic evidence, NOT raw file age.
- **Topological Weighting**: Quality deductions are weighted by node centrality (betweenness and degree): Multiplier $(1.0 + 3.0 \cdot B(v))$. A broken reference or collision in `AGENTS.md` or a central hub causes significantly higher quality penalties than one in an isolated leaf note.
- **Score + Reasons**: The system MUST NEVER expose an unexplained number. Every pillar score MUST be accompanied by deterministic, itemized, and explainable reasons.
- **Debt vs Regression Clarity**: The Quality Score UI MUST explicitly distinguish between Legacy Debt and New Regressions (e.g. `Quality: 74/100 · 0 new regressions · 42 legacy issues`).

### X. Sandboxed Webview, Progressive Disclosure & Zero Action Duplication
All embedded interfaces and runtime interactions MUST adhere to strict information architecture and security principles:
- **Calm Control Center & The Five Pillars**: The user experience MUST be organized around calm, distinct pillars: Knowledge topology, AI Context horizons, Health & Issues diagnostics, Quality & Admission gate, and Fragment analytics.
- **Zero Action Duplication**: Administrative actions MUST NOT be duplicated across competing UI surfaces. Primary controls are reserved exclusively for active graph manipulation.
- **State-Driven Action Emergence**: High-friction administrative operations MUST emerge naturally from displayed system states rather than existing as permanent clutter.
- **Selection-Driven Contextual Inspector**: When an element is selected (node, edge, collision, or fragment), the Webview MUST present a contextual inspector drawer containing relevant metadata and targeted actions (`[Open file]`, `[Set as hub]`, `[Exclude file/folder]`, `[Fix reference]`, `[Review collision]`, `[Inspect fragments]`).
- **Webview Sandboxing & CSP**: Webview panels MUST enforce a strict Content Security Policy (CSP) with unique cryptographic nonces (`script-src 'nonce-${nonce}'`). Local assets MUST strictly be loaded via `webview.asWebviewUri` under allowed `localResourceRoots`.
- **Typed Asynchronous RPC**: Communication between the Extension Host and Webview iframe MUST use strongly typed JSON-RPC 2.0 messages with request-response correlation tokens (`requestId`).
- **Memory-Safe State Lifecycle**: Webview panel state MUST be serialized via `vscode.setState()` / `vscode.getState()` and `WebviewPanelSerializer`.
- **Zero Plaintext Secrets**: Any credentials, tokens, or private settings MUST be stored exclusively via `vscode.SecretStorage` (backed by OS Keychains/safeStorage) and NEVER written to plaintext configuration files or disk logs.

### XI. Test-Driven Development (TDD), Multi-Platform Binary Verification & Performance Budgets
Quality and cross-platform stability are guaranteed through strict automated test gates before code merge:
- **Rust Core Test Suite**: 100% test coverage for Markdown AST parsing, heading anchor derivation, fragment metadata extraction, frontmatter materialization, wikilink normalization, petgraph centrality computation, BPE tokenization, baseline debt diffing, reverse code-doc indexing, deterministic similarity fingerprints, and evidence-based freshness scoring.
- **Pre-Commit Hook Simulation Suite**: Automated end-to-end tests validating candidate graph evaluation, frontmatter materialization during pre-commit, `BLOCK` on new broken links/anchors/collisions, and silent fail-open $\le 500\text{ ms}$.
- **Extension Host Integration Tests**: Headless VS Code extension tests (`@vscode/test-electron`) verifying sidecar process spawning, IPC socket resilience, Fragment Inspector rendering, and Webview message roundtrips.
- **Multi-Platform Support**: Binaries MUST be built and tested for Linux (x86_64, aarch64), macOS (x86_64, Apple Silicon aarch64), and Windows (x86_64).

### XII. Documentation Orientation, Strategy Trade-Offs & User-Controlled Structural Evolution
The engine bridges structural observation with intentional evolution:
- **Four Documentation Orientations**:
  1. *AI-Oriented (Default)*: Optimizes for machine-mediated discovery and retrieval, context efficiency, canonical routing, section addressability, and short, high-relevance Context Traces.
  2. *Human-Oriented*: Prioritizes narrative continuity, readability, conceptual explanation, human navigation, and maintainability.
  3. *Balanced*: Multi-objective optimization balancing AI retrieval efficiency and human comprehension without letting either dominate unnecessarily.
  4. *Custom*: Unrestricted configuration for specific organizational or philosophical standards.
- **Two-Tier Custom Configuration Architecture**:
  - *Tier 1 (High-Level Objectives)*: Intuitive knobs (`AI Discoverability`, `Context Efficiency`, `Human Readability`, `Knowledge Reuse`, `Narrative Coherence`, `Maintenance Simplicity`) with informational tooltips (`ⓘ`) explaining trade-offs.
  - *Tier 2 (Advanced Engine Parameters)*: Inspectable underlying parameters (`cohesion_threshold`, `semantic_diversity_threshold`, `reference_concentration_threshold`, `max_trace_depth`, `max_context_surface`, `split_sensitivity`, `canonical_ownership_sensitivity`).
  - *Deterministic Parameter Mapping*: Every Tier 1 setting maps explicitly and deterministically to underlying Tier 2 engine variables.
- **Constraint Hierarchy & Conflict Detection**:
  - Classifies configuration directives into *Preferences* (soft bias), *Constraints* (penalized trade-off), and *Hard Constraints* (strictly forbidden automated actions, e.g. never split canonical docs).
  - Deterministically detects and surfaces mathematically contradictory constraint sets before producing recommendations.
- **10-Dimensional Structural Analytics & Evidence-Based Signatures**:
  - Measures: Cohesion, Coupling, Reference Concentration, Granularity, Context Efficiency, Discoverability, Reusability, Semantic Diversity, Navigational Complexity, Maintenance Coupling.
  - Combines metrics into problem signatures (e.g., structural "God Document" defined by high diversity, high concentration, and low context efficiency rather than raw token count alone).
- **Knowledge Organization System (KOS) Candidate Strategies & Trade-Off Matrix**:
  - Evaluates classical strategies (Section Addressing, Topic Decomposition, Lightweight Index Routers, Faceted Classification, Preserve Document, Hybrid Strategies) against the active `OptimizationProfile`.
  - Assigns deterministic confidence scores $[0.0, 1.0]$ and explains countervailing trade-offs.
- **User-Controlled Downstream Automation**:
  - Workflow: $\text{Measure} \to \text{Diagnose} \to \text{Recommend} \to \text{User Selects Strategy} \to \text{Generate Transformation Plan} \to \text{User Approval} \to \text{Automate} \to \text{Re-Measure}$.
  - Default safety: `Auto-Apply: OFF`, `Require Approval: ON`.
  - Quantifies empirical Before-vs-After delta metrics ($\Delta \text{Context Efficiency}$, $\Delta \text{Trace Depth}$, $\Delta \text{Maintenance Coupling}$).

---

## Technology Stack & Performance Standards

### 1. Technology Matrix
- **Core Engine (Sidecar)**: Rust 2021/2024 edition, `pulldown-cmark` (AST Markdown parser), `petgraph` (graph data structure and centrality algorithms), `tiktoken-rs` (cl100k BPE token counter), `tokio` (async runtime), `tokio-tungstenite` / Unix Domain Sockets / Windows Named Pipes (IPC/WebSocket transport), `serde` / `serde_json` / `serde_yaml` (data serialization), deterministic n-gram and BM25 tokenizers.
- **Extension Host**: Node.js 20+, TypeScript 5.4+, VS Code Extension API (`^1.90.0`), `ws` / IPC client.
- **Webview UI**: High-performance 2D HTML5 Canvas force-directed graph renderer (`d3-force`), `@vscode/webview-ui-toolkit` Web Components, CSS custom properties adhering to active VS Code theme tokens.
- **Configuration Contract**: `.context-vault.yaml` placed at workspace root.

### 2. Non-Functional Performance Budgets
| Metric | Threshold | Verification Method |
| :--- | :--- | :--- |
| **Rust Sidecar Memory (Rest)** | $\le$ 35 MB resident RSS for 5,000 files | Valgrind / Heap profiling in CI |
| **Graph Analysis Latency** | $\le$ 200 ms for 1,000 Markdown files | Criterion microbenchmarks |
| **AST Fragment Parsing & Normalization** | $\le$ 50 ms for 100 Markdown files | Criterion microbenchmarks |
| **Incremental Graph Patching** | $\le$ 50 ms for single-note delta | Criterion microbenchmarks |
| **Normal Git Delta Processing** | $<$ 150 ms for typical staged commit | Criterion microbenchmarks |
| **Git Hook Timeout Ceiling** | Strict 500 ms non-blocking fail-open | Unit test with hanging socket |
| **Context Trace Traversal** | $\le$ 100 ms for query beam search | Criterion microbenchmarks |
| **Warm Knowledge Search** | $\le$ 50 ms over indexed vault | Criterion microbenchmarks |
| **Webview Rendering Rate** | Consistent 60 FPS during pan/zoom/drag | Chrome DevTools Frame Meter |
| **Packaging Size** | $\le$ 25 MB per platform-specific VSIX | `@vscode/vsce package` artifact audit |

---

## Development Workflow, Review Process & Quality Gates

### 1. Code Quality Gates
1. **Red-Green-Refactor Cycle**: All new parsers, fragment extractors, graph algorithms, drift estimators, or IPC handlers must start with failing unit tests prior to implementation.
2. **Formatting & Linter Compliance**:
   - Rust: `cargo fmt --check` and `cargo clippy -- -D warnings`.
   - TypeScript: `eslint` with strict type checking and `prettier --check`.
3. **Packaging Discipline**: Production `.vsix` packaging MUST use platform-specific targets (`vsce package --target <platform>`) and exclude unnecessary build artifacts via `.vscodeignore`.

### 2. Review Protocol
- Any PR introducing external LLM API calls, cloud embeddings, or remote AI dependencies MUST be immediately rejected.
- Any PR introducing filesystem watchers or continuous background polling MUST be rejected.
- Any PR modeling fragments or sections as independent graph nodes MUST be rejected.
- Any PR violating raw numeric integrity (persisting formatted strings instead of complete integers) MUST be rejected.
- Any PR modifying IPC contracts MUST include synchronized updates and tests in both the Rust engine and TypeScript extension.
- Every architectural change MUST reference this Constitution and update the decisions log (`decisions_log.md`).

---

## Governance

This Constitution represents the supreme architectural and technical policy for the `knowframe` project.
- **Supremacy**: In any conflict between feature requests, quick fixes, and these principles, this Constitution governs.
- **Amendments**: Amendments require documented rationale, impact assessment on existing performance budgets, and consensus approval.
- **Versioning Policy**:
  - **MAJOR**: Incompatible principle changes or fundamental architecture modifications.
  - **MINOR**: Addition of new principles, sections, or materially expanded guidelines.
  - **PATCH**: Clarifications, wording adjustments, or typo fixes.

**Version**: 1.7.0 | **Ratified**: 2026-09-11 | **Last Amended**: 2026-10-14
