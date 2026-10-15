---
wiki:
  content_hash: cf6f4bc8d805acfabcdc1a4adfa2cf9d61d134a6e63cb214ddcb91d575522c8b
  fragments:
    ? feature-specification-fragment-level-knowledge-architecture-materialized-frontmatter-layer-5-pillar-quality-model-evidence-based-freshness
    : tokens: 5396
      inbound: 0
      outbound: 0
    1-executive-summary-vision:
      tokens: 566
      inbound: 0
      outbound: 0
    2-user-stories-priority-matrix:
      tokens: 1363
      inbound: 0
      outbound: 0
    user-story-1-document-centric-graph-fragment-addressing-priority-p1:
      tokens: 195
      inbound: 0
      outbound: 0
    user-story-2-pre-commit-materialized-frontmatter-layer-priority-p1:
      tokens: 253
      inbound: 0
      outbound: 0
    user-story-3-raw-numeric-integrity-compact-presentation-separation-priority-p1:
      tokens: 190
      inbound: 0
      outbound: 0
    user-story-4-fragment-inspector-reference-concentration-analytics-priority-p2:
      tokens: 245
      inbound: 0
      outbound: 0
    user-story-5-context-trace-section-level-routing-context-optimization-priority-p2:
      tokens: 215
      inbound: 0
      outbound: 0
    user-story-6-5-pillar-health-model-evidence-based-freshness-priority-p2:
      tokens: 255
      inbound: 0
      outbound: 0
    3-functional-requirements-fr-001-fr-042:
      tokens: 2656
      inbound: 0
      outbound: 0
    31-section-level-fragment-knowledge-modeling:
      tokens: 418
      inbound: 0
      outbound: 0
    32-materialized-frontmatter-layer-pre-commit-synchronization:
      tokens: 362
      inbound: 0
      outbound: 0
    33-raw-numeric-integrity-presentation-separation:
      tokens: 236
      inbound: 0
      outbound: 0
    34-fragment-inspector-webview-selection-ux:
      tokens: 296
      inbound: 0
      outbound: 0
    35-reference-concentration-structure-debt-analytics:
      tokens: 270
      inbound: 0
      outbound: 0
    36-context-trace-section-level-routing-beam-search:
      tokens: 365
      inbound: 0
      outbound: 0
    37-5-pillar-health-model-evidence-based-freshness:
      tokens: 694
      inbound: 0
      outbound: 0
    4-non-functional-requirements-nfr-001-nfr-009:
      tokens: 381
      inbound: 0
      outbound: 0
    5-success-criteria-verification-matrix:
      tokens: 295
      inbound: 0
      outbound: 0
---
# Feature Specification: Fragment-Level Knowledge Architecture, Materialized Frontmatter Layer, 5-Pillar Quality Model & Evidence-Based Freshness

**Feature Branch:** `004-fragment-architecture-quality`  
**Target Delivery:** `llm_wiki_ext4vsc` (VS Code Extension + Rust Core `llm-wiki-core`)  
**Status:** In Review / Specification  
**Constitution Authority:** `CONSTITUTION.md` v1.5.0 (The 12 Critical Invariants, Principles I through XI)  
**External AI / Remote LLM:** **STRICTLY PROHIBITED** (Local Deterministic Execution Only)  

---

## 1. Executive Summary & Vision

The `004-fragment-architecture-quality` specification establishes the **Consolidated Baseline / Point of Stop** for `llm_wiki_ext4vsc`. It codifies the architectural paradigm that transforms the physical Markdown document from a monolithic black box into a **structured, section-addressable knowledge substrate**:

1. **Document-Centric Graph & Section Addressing**: Documents remain the sole graph nodes ($O(N)$ complexity, preventing graph explosion). Sections (`## Heading`) are extracted as addressable **fragment metadata** belonging to Document Nodes, enabling standard Markdown URI targets: `[Label](./doc.md#fragment)`.
2. **Materialized Frontmatter Layer**: Frontmatter serves as a materialized structural metadata cache (`wiki.fragments`) derived deterministically from the Markdown AST during pre-commit. The AST is the structural source of truth; shifting computational cost to the commit lifecycle prevents expensive on-demand AST parsing at runtime.
3. **Raw Numeric Integrity vs. Presentation Separation**: Analytical models, frontmatter YAML, and IPC contracts store complete raw numeric integers (e.g., `tokens: 1043256`, `inbound: 31`), while the presentation layer transforms values into compact representations strictly $\le 5$ characters (`1.04M`, `543`, `13.3k`).
4. **Context Trace Section-Level Routing**: Constrained beam-search traversal from entry points (`AGENTS.md`) targeting document fragments (`doc.md#fragment`), achieving $\approx 90\%$ context reduction (e.g. $50\text{k} \to 5\text{k}$ tokens) for AI agent consumption with explainable relevance justification.
5. **5-Pillar Network-Aware Quality Model**: Health is evaluated across 5 distinct dimensions:
   - `Pillar 1: Integrity` (Weight: 25%): Link, wikilink, and fragment anchor validity.
   - `Pillar 2: Structure` (Weight: 20%): Missing fragment metadata, oversized documents ($>50\text{k}$ tokens), high reference concentration ($>70\%$), unfragmented hubs, low context utilization.
   - `Pillar 3: Consistency` (Weight: 20%): Multi-signal collisions, canonical conflicts, and conceptual contradictions.
   - `Pillar 4: Discoverability` (Weight: 20%): Context Trace depth, reachability from entry points, centrality distribution, and orphan ratio.
   - `Pillar 5: Freshness` (Weight: 15%): Deterministic evidence of currentness (code entity existence, Git changes on code after doc updates, deleted/renamed symbols, canonical contradictions).

---

## 2. User Stories & Priority Matrix

### User Story 1: Document-Centric Graph & Fragment Addressing (Priority: P1)
> *As a software architect or AI developer, I want to reference specific sections of large documentation files using standard Markdown fragment targets (`doc.md#authentication`) without turning every section into a graph node, so that my graph remains clean and scalable while my references are pinpoint accurate.*

- **Acceptance Criteria**:
  1. Only Markdown documents are represented as graph nodes in the 2D Canvas force graph ($O(N)$ nodes).
  2. Sections/headings are parsed into fragment metadata attached to Document Nodes with deterministic anchor normalization (`## Heading Name` $\to$ `#heading-name`).
  3. Standard Markdown links with fragment targets (`[Label](./architecture.md#authentication)`) resolve deterministically.
  4. Fragment links pointing to non-existent or renamed heading anchors are flagged as high-confidence Integrity defects with exact file and line coordinates.

### User Story 2: Pre-Commit Materialized Frontmatter Layer (Priority: P1)
> *As a developer committing documentation changes, I want the pre-commit hook to automatically calculate and materialize fragment metadata into YAML frontmatter (`wiki.fragments`), so that runtime analytics and Context Trace do not need to re-parse large Markdown ASTs repeatedly.*

- **Acceptance Criteria**:
  1. The Markdown AST is the structural source of truth. Frontmatter metadata is derived from and validated against the AST.
  2. During pre-commit (`llm-wiki-validate`), staged Markdown documents have their fragment metadata (token count, inbound/outbound references, start/end lines) materialized into `wiki.fragments`.
  3. Non-wiki frontmatter fields (e.g. user `title`, `tags`, `author`, `date`, custom keys) are preserved non-destructively.
  4. The pre-commit hook automatically re-stages the modified file (`git add <path>`), ensuring the Git commit atomically contains the materialized frontmatter with zero working tree drift.
  5. Missing fragment metadata on oversized documents ($>50\text{k}$ tokens or $>10$ headings) is flagged as structural debt.

### User Story 3: Raw Numeric Integrity & Compact Presentation Separation (Priority: P1)
> *As an engineer inspecting knowledge metrics, I want the system to store exact integer values in storage and data contracts while rendering compact $\le 5$-character badges in the UI, so that calculations are 100% mathematically precise while the UI remains legible.*

- **Acceptance Criteria**:
  1. All persisted YAML frontmatter, JSON-RPC contracts, Rust data structs, and TypeScript models store complete raw integer values (`tokens: 1043256`, `inbound: 31`).
  2. The UI formatter transforms raw values into compact strings strictly $\le 5$ characters (`543`, `4.23k`, `13.3k`, `1.04M`).
  3. Formatted strings are never parsed or fed back into analytics, scoring, or persistence.

### User Story 4: Fragment Inspector & Reference Concentration Analytics (Priority: P2)
> *As a developer inspecting a document in the graph, I want to see a 3-column table of its fragments with token counts and reference traffic, so that I can see which sections carry the primary cognitive weight.*

- **Acceptance Criteria**:
  1. Selecting a Document Node in the Webview displays a "Fragments" section in the contextual Inspector Drawer.
  2. The table renders three columns: `Fragment | Tok | I/O` (`Tok` = token count, `I` = inbound refs, `O` = outbound refs).
  3. Clicking a fragment row opens the file in VS Code scrolled directly to the fragment's `start_line`.
  4. The engine calculates reference concentration ($C = \text{inbound}_{\text{fragment}} / \text{inbound}_{\text{doc}}$) and flags documents where a single section absorbs $\ge 70\%$ of references.
  5. If a document has zero fragments (no headings), the Inspector Drawer renders an empty state: `"No heading sections found"`.

### User Story 5: Context Trace Section-Level Routing & Context Optimization (Priority: P2)
> *As an AI agent or engineer formulating a prompt, I want Context Trace to discover specific section fragments from `AGENTS.md` rather than loading entire multi-thousand-token documents, so that AI context consumption is reduced by up to 90%.*

- **Acceptance Criteria**:
  1. Context Trace executes deterministic constrained beam search from entry points (`AGENTS.md`) evaluating relevance against document and fragment headings.
  2. Output paths route directly to fragment targets: e.g. `AGENTS.md` $\to$ `architecture.md#authentication`.
  3. Output reports document token load ($50.2\text{k}$), selected fragment token load ($5.1\text{k}$), and potential context reduction ($\approx 90\%$). If no specific fragment matches, the path targets the document with 0.0% reduction.
  4. The trace explains why the fragment was selected with deterministic relevance reasons.

### User Story 6: 5-Pillar Health Model & Evidence-Based Freshness (Priority: P2)
> *As a technical lead, I want an overall Knowledge Quality Score evaluated across 5 dimensions (Integrity, Structure, Consistency, Discoverability, Freshness) with explainable evidence, so that our knowledge base improves continuously.*

- **Acceptance Criteria**:
  1. Overall score ($0\text{--}100$) aggregates 5 weighted dimensions, with topology weighting by betweenness centrality $(1.0 + 3.0 \cdot B(v))$.
  2. `Structure` pillar (Pillar 2) measures missing fragment metadata, oversized documents ($>50\text{k}$ tokens), high reference concentration ($>70\%$), and unfragmented hubs.
  3. `Freshness` pillar (Pillar 5) is calculated by deterministic evidence of currentness (code entity existence, Git changes on cited code occurring after doc updates, deleted symbols, canonical contradictions), NOT raw file age.
  4. The UI displays the score alongside itemized, deterministic reasons and clear debt counters (`Legacy Debt`, `New Regressions`, `Resolved`).

---

## 3. Functional Requirements (FR-001 – FR-042)

### 3.1 Section-Level Fragment Knowledge Modeling
- **FR-001**: The Knowledge Graph MUST model Markdown documents as the exclusive graph nodes ($O(N)$ graph topology). Fragments MUST be modeled as addressable metadata attached to Document Nodes and MUST NOT be created as independent graph nodes.
- **FR-002**: The Rust AST parser MUST parse Markdown headings (levels 1 through 6), extract heading text, and deterministically normalize heading text into URL-safe anchors (lowercase, alphanumeric characters and hyphens, collapsing consecutive spaces/hyphens).
- **FR-003**: Each fragment metadata entry MUST contain:
  - `anchor`: Deterministic string anchor (unique identifier within document, e.g. `"authentication"`).
  - `heading`: Original plain-text heading title.
  - `start_line`: 1-indexed starting line in source Markdown.
  - `end_line`: 1-indexed ending line (up to next heading of equal/higher rank or EOF).
  - `tokens`: Complete integer BPE token count (`cl100k_base`).
  - `inbound`: Complete integer count of inbound links targeting this specific fragment.
  - `outbound`: Complete integer count of outbound links originating within this fragment's line range.
- **FR-004**: The system MUST resolve standard Markdown fragment links formatted as `[text](path/to/doc.md#anchor)` and wikilinks formatted as `[[doc#anchor|text]]`.
- **FR-005**: If a target document exists but does not contain the specified `#anchor`, the link resolver MUST flag the reference as a broken anchor Integrity defect with exact source file and line coordinates.
- **FR-006**: The AST parser MUST detect duplicate headings within the same document and append deterministic numerical suffixes (e.g. `#configuration`, `#configuration-1`) matching CommonMark anchor conventions.
- **FR-007**: Fragment references MUST be exposed in the graph RPC protocol as edge metadata (`target_fragment: Option<String>`), preserving document-level source and target node identifiers.

### 3.2 Materialized Frontmatter Layer & Pre-Commit Synchronization
- **FR-008**: The Markdown AST is the structural source of truth. Document frontmatter MUST serve as a materialized structural metadata cache derived from and validated against the AST.
- **FR-009**: The materialized frontmatter schema MUST be stored under a dedicated top-level `wiki` key in YAML frontmatter:
  ```yaml
  ---
  wiki:
    fragments:
      <anchor>:
        tokens: <integer>
        inbound: <integer>
        outbound: <integer>
  ---
  ```
- **FR-010**: The pre-commit hook (`llm-wiki-validate`) MUST detect staged Markdown documents whose AST structure or headings have changed, recalculate fragment metadata, update or inject the materialized `wiki.fragments` frontmatter into the document on disk, and automatically re-stage the modified file into Git index (`git add <path>`) so that the Git commit atomically contains the materialized frontmatter without leaving a dirty working tree.
- **FR-011**: Materialization MUST be non-destructive: all user frontmatter keys (e.g. `title`, `tags`, `author`, `date`, custom keys) MUST be preserved exactly as authored without reordering or data loss.
- **FR-012**: If a document exceeds the structural threshold ($>50\text{k}$ tokens or $>10$ headings) and lacks materialized fragment metadata, the pre-commit validator MUST emit a `WARN` structural debt notice guiding the developer to materialize metadata.
- **FR-013**: The Rust Core MUST prioritize materialized frontmatter for token economics, reference analytics, and search queries, completely bypassing runtime AST re-parsing when frontmatter metadata matches the document content hash.

### 3.3 Raw Numeric Integrity & Presentation Separation
- **FR-014**: All persisted frontmatter YAML, JSON-RPC protocol payloads, Rust memory structures, and TypeScript extension models MUST store numeric values exclusively as complete raw integers (e.g., `tokens: 1043256`, `inbound: 31`).
- **FR-015**: Numeric formatting (such as `1.04M`, `543`, `13.3k`) MUST exist exclusively in presentation formatting functions (`formatCompactNumber`) within the Webview UI layer.
- **FR-016**: The UI formatter MUST enforce a strict maximum length of 5 characters for all compact numeric representations (e.g., `543` [3 chars], `13.3k` [5 chars], `1.04M` [5 chars]).
- **FR-017**: Formatted presentation strings MUST NEVER be parsed back into numbers or fed into analytical calculations, scoring engines, or persistent data models.
- **FR-018**: Unit test suites in both Rust and TypeScript MUST assert raw numeric fidelity across serialization roundtrips and assert that formatting functions adhere to the 5-character ceiling.

### 3.4 Fragment Inspector & Webview Selection UX
- **FR-019**: When a user selects a Document Node in the Webview knowledge graph, the contextual Inspector Drawer MUST render an expandable "Fragments" section.
- **FR-020**: The Fragments section MUST render a 3-column table with headers: `Fragment | Tok | I/O`.
  - Column 1 (`Fragment`): Heading text with anchor badge.
  - Column 2 (`Tok`): Formatted compact token count ($\le 5$ chars).
  - Column 3 (`I/O`): Formatted compact inbound/outbound counts (e.g., `31/7`).
- **FR-021**: Clicking on a fragment row MUST dispatch a `webview/openDocument` RPC command with the document path, `start_line`, and `end_line`, opening the file in VS Code and highlighting the section.
- **FR-022**: Hovering over the `I/O` cell of a fragment MUST display a tooltip enumerating the top inbound referring documents and outbound target documents.
- **FR-023**: If a document has zero fragments (e.g. contains no headings), the inspector MUST display a muted notification: `"No heading sections found"`.
- **FR-024**: The Inspector Drawer MUST maintain progressive disclosure: Primary document metrics (tokens, degree, centrality) $\to$ Secondary Fragments table $\to$ Advanced Quality breakdown.

### 3.5 Reference Concentration & Structure Debt Analytics
- **FR-025**: The Rust engine MUST compute the Reference Concentration metric for every document fragment:
  $$C(f) = \frac{\text{inbound}(f)}{\sum_{f' \in D} \text{inbound}(f')}$$
  Where inbound references target specific sections.
- **FR-026**: If a single fragment absorbs $C(f) \ge 70\%$ of all inbound references on a document with total tokens $\ge 50\text{k}$, the engine MUST flag a **High Reference Concentration** structural warning.
- **FR-027**: The engine MUST detect **Unfragmented Central Hubs**: any document with in-degree $\ge 10$ and tokens $\ge 20\text{k}$ that contains $\le 1$ heading fragment.
- **FR-028**: Structural debt findings MUST provide actionable guidance text (e.g., *"Consider splitting document or targeting sections directly to reduce downstream AI prompt context"*), without performing automated file restructuring.
- **FR-029**: The `Structure` pillar score ($0\text{--}100$) MUST be calculated deterministically from: missing fragment metadata penalty, oversized document penalty, high reference concentration penalty, and unfragmented hub penalty.

### 3.6 Context Trace Section-Level Routing & Beam Search
- **FR-030**: Context Trace MUST execute a deterministic constrained beam search (beam width $W \le 5$, max depth $D \le 4$) starting from designated entry points (`AGENTS.md` or configured Hub) toward task queries.
- **FR-031**: Candidate scoring during beam search MUST combine: query term lexical match, document title match, fragment heading match, tag overlap, and graph shortest-path proximity.
- **FR-032**: When the highest-scoring candidate is a specific document fragment (`doc.md#fragment`), Context Trace MUST route directly to the fragment anchor rather than the entire document.
- **FR-033**: Context Trace output MUST report:
  - Traversal path hops: e.g. `AGENTS.md` $\to$ `architecture.md#authentication`.
  - Path traversal depth integer.
  - Document total raw tokens ($T_{\text{doc}}$).
  - Selected fragment raw tokens ($T_{\text{fragment}}$). If no section fragment is selected, `fragment_raw_tokens` is `null` and `context_reduction_percent` is `0.0%`.
  - Potential Context Reduction percentage:
    $$\text{Reduction} = \frac{T_{\text{doc}} - T_{\text{fragment}}}{T_{\text{doc}}} \times 100\%$$
  - Explainable relevance justification enumerating matching query terms and heading signals.
- **FR-034**: Context Trace execution MUST complete within $\le 100\text{ ms}$ for vaults up to 1,000 documents.
- **FR-035**: Context Trace MUST NOT use external LLM APIs, vector databases, or remote embeddings.

### 3.7 5-Pillar Health Model & Evidence-Based Freshness
- **FR-036**: The overall Knowledge Quality Score ($0\text{--}100$) MUST aggregate five distinct, deterministic dimensions:
  1. `Integrity` (Pillar 1, Weight: 25%): Dead links, broken anchors, missing files, dangling wikilinks.
  2. `Structure` (Pillar 2, Weight: 20%): Missing fragment metadata, oversized documents, reference concentration, unfragmented hubs.
  3. `Consistency` (Pillar 3, Weight: 20%): Semantic collisions, canonical conflicts, conflicting documentation.
  4. `Discoverability` (Pillar 4, Weight: 20%): Context Trace depth, hub connectivity, centrality distribution, orphan ratio.
  5. `Freshness` (Pillar 5, Weight: 15%): Evidence-based technical drift and currentness signals.
- **FR-037**: Quality deductions MUST be weighted by node topological betweenness centrality:
  $$\text{Penalty}(v) = \text{BasePenalty} \times (1.0 + 3.0 \cdot B(v))$$
- **FR-038**: The Freshness pillar MUST calculate document currentness using deterministic evidence signals:
  - **Signal 1 (Code Citation Existence)**: All code symbols, file paths, and configuration keys cited in documentation exist on disk. Missing citations penalize freshness (-15 points per broken citation).
  - **Signal 2 (Code Git Modification After Doc)**: Staged or committed code changes affecting cited source paths that occurred after the document's last Git commit. Warns of potential staleness (-10 points per unreviewed Git commit affecting cited code paths, capped at -30 points max).
  - **Signal 3 (Deleted/Renamed Symbols)**: Symbols cited in documentation that have been removed or renamed in Git history (-20 points per deleted symbol).
  - **Signal 4 (Canonical Contradictions)**: Document contradicts an identified authoritative canonical document (-25 points).
  - Document age MAY be used only as a tertiary tie-breaker and MUST NOT serve as primary staleness criteria.
- **FR-039**: Freshness score MUST classify documents into three discrete tiers:
  - `Fresh`: Score $\ge 80$.
  - `Potentially Stale`: Score $60\text{--}79$.
  - `Stale`: Score $< 60$.
- **FR-040**: Every pillar score and the composite score MUST include structured, human-readable, and machine-readable `reasons` explaining the exact evidence behind every penalty.
- **FR-041**: The Quality Score UI and CLI MUST explicitly partition issues into `Legacy Debt`, `New Regressions`, and `Resolved Debt`.
- **FR-042**: Pre-commit admission control MUST execute the decision engine:
  - `BLOCK`: New broken internal links, new dangling fragment anchors (`BrokenFragmentAnchor`), or high-confidence collisions with canonical conflict.
  - `WARN`: Moderate collisions ($75\%\le \text{similarity} < 85\%$), missing fragment metadata on oversized documents, or potential freshness drift.
  - `PASS`: No regressions introduced.

---

## 4. Non-Functional Requirements (NFR-001 – NFR-009)

| Metric | Threshold | Verification Method |
| :--- | :--- | :--- |
| **NFR-001: AST Fragment Parsing** | $\le 50\text{ ms}$ for 100 Markdown files | Criterion microbenchmarks (`crates/core/benches/fragment_benchmark.rs`) |
| **NFR-002: Incremental Graph & Fragment Patch** | $\le 50\text{ ms}$ for single-document delta | Criterion microbenchmarks (`crates/core/benches/sync_benchmark.rs`) |
| **NFR-003: Normal Git Delta Processing** | $< 150\text{ ms}$ for typical staged commit | Integration tests with Git fixture |
| **NFR-004: Hook Fail-Open Ceiling** | Strict $\le 500\text{ ms}$ timeout exit code 0 | Unit test with hanging daemon socket |
| **NFR-005: Context Trace Traversal** | $\le 100\text{ ms}$ for query beam search | Criterion benchmark on 1,000-note graph |
| **NFR-006: Warm Knowledge Search** | $\le 50\text{ ms}$ over indexed vault | Search query microbenchmarks |
| **NFR-007: Sidecar Idle RSS Memory** | $\le 35\text{ MB}$ for 5,000 Markdown files | Valgrind / Heap profiling in CI |
| **NFR-008: Numeric Presentation Formatting** | Strictly $\le 5$ characters across 100% of UI | Automated DOM assertions in extension tests |
| **NFR-009: Extension Packaging Size** | $\le 25\text{ MB}$ per platform-specific VSIX | `@vscode/vsce package` artifact audit |

---

## 5. Success Criteria & Verification Matrix

- **SC-001 (Fragment Precision)**: 100% accurate heading anchor extraction and normalization across Markdown heading levels H1 through H6, without creating additional graph nodes.
- **SC-002 (Frontmatter Materialization)**: Pre-commit hook cleanly updates `wiki.fragments` in YAML frontmatter and automatically re-stages files, while preserving 100% of existing user frontmatter keys and YAML comments.
- **SC-003 (Fragment Link Integrity)**: Standard Markdown fragment links (`doc.md#anchor`) resolve cleanly. Dangling anchors produce line-precise Integrity diagnostics.
- **SC-004 (Context Reduction Efficiency)**: Context Trace achieves $\ge 80\%$ context reduction on documents $>20\text{k}$ tokens with specific section targets, completing in $\le 100\text{ ms}$.
- **SC-005 (5-Pillar Health Diagnostic)**: Health score accurately reflects all 5 dimensions (Integrity, Structure, Consistency, Discoverability, Freshness) with itemized deterministic reasons.
- **SC-006 (Numeric Presentation Separation)**: 100% of persisted data structures and contracts retain full integer values, while all UI surfaces render numbers $\le 5$ characters.
- **SC-007 (Quality Gate Verification)**: 100% automated test pass rate across Rust and TypeScript test suites with zero compiler warnings (`cargo clippy`, `eslint`).
