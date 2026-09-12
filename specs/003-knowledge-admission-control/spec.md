# Feature Specification: Knowledge Quality Admission Control & Continuous Enforcement

**Feature Branch:** `003-knowledge-admission-control`  
**Target Delivery:** `llm_wiki_ext4vsc` (VS Code Extension + Rust Core `llm-wiki-core`)  
**Status:** In Review / Specification  
**Constitution Authority:** `CONSTITUTION.md` v1.4.0 (Principles I, II, VI, VII, VIII, IX, X)  
**External AI / Remote LLM:** **STRICTLY PROHIBITED** (Local Deterministic Execution Only)  

---

## 1. Executive Summary & Vision

The `003-knowledge-admission-control` feature evolves `llm_wiki_ext4vsc` from a reactive repository analyzer into a **continuous quality gate and knowledge admission control system operating at the Git boundary**.

Knowledge mutations occur exclusively at Git commits. File saves in the editor do not alter the persistent Knowledge Graph. Instead:
1. **Pre-Commit Admission Control (`llm-wiki-validate`)**: Staged changes are combined in-memory with the current committed graph to construct an ephemeral **Candidate Graph**. The candidate graph is validated against a brownfield **Knowledge Debt Baseline**. If the change introduces new high-confidence regressions (new broken links, new high-confidence semantic collisions, or direct technical drift), the commit is blocked (or warned). Existing legacy debt never blocks.
2. **Post-Commit Synchronization (`llm-wiki-sync`)**: When the commit succeeds, the committed Git delta is incrementally incorporated into the persistent Rust Knowledge Graph in $\le 50\text{ ms}$.
3. **Deterministic Semantic & Technical Drift Engines**: Local BM25, term frequency, heading fingerprints, and reverse code-doc indexes detect overlap, canonical ownership candidates, and documentation staleness without external AI or cloud embeddings.
4. **Context Trace & Topological Quality Scoring**: Quantitative evaluation of agent discoverability starting from entry points (`AGENTS.md`) to task targets, coupled with a network-aware 4-pillar quality score (Integrity, Consistency, Discoverability, Freshness).

---

## 2. User Stories & Priority Matrix

### User Story 1: Brownfield Debt Baseline & Non-Blocking Adoption (Priority: P1)
> *As a software engineer adopting LLM Wiki on an existing codebase with legacy documentation debt (broken links, orphans, stale files), I want the system to record a baseline of existing problems and never block my commits due to pre-existing debt, so that adopting LLM Wiki does not disrupt my immediate delivery.*

- **Acceptance Criteria**:
  1. On initial scan or `baseline.json` generation, the Rust core records `baseline_commit = <sha>` and counts of legacy broken links, orphan nodes, collisions, and drift findings.
  2. Commits that touch files unrelated to existing debt proceed without hindrance (`PASS`).
  3. Pre-commit validation compares candidate findings against baseline counts: only **New Regressions** ($\Delta > 0$) can trigger `BLOCK`.
  4. Resolving legacy issues decrements the baseline counter, permanently locking in debt reductions.

### User Story 2: In-Memory Candidate Graph & Dual Git Hooks (Priority: P1)
> *As a developer committing changes, I want the pre-commit hook (`llm-wiki-validate`) to evaluate my staged files against an ephemeral candidate graph in memory without modifying the persistent graph, and I want the post-commit hook (`llm-wiki-sync`) to update the persistent graph only after the commit succeeds.*

- **Acceptance Criteria**:
  1. `llm-wiki-validate` runs at `pre-commit` stage in $\le 150\text{ ms}$ (with hard fail-open timeout of $\le 500\text{ ms}$).
  2. Staged changes are merged in-memory with the committed topology to evaluate the candidate state.
  3. If validation fails or the user cancels the commit, the candidate graph is discarded; zero uncommitted state leaks into the persistent graph.
  4. `llm-wiki-sync` runs at `post-commit` stage, transmitting the committed SHA and delta to update the persistent graph in $\le 50\text{ ms}$.

### User Story 3: Deterministic Semantic Drift & Collision Detection (Priority: P2)
> *As a technical writer or engineer, I want the pre-commit hook to detect when my staged documentation duplicates, fragments, or conflicts with existing documentation using deterministic fingerprints, so that our team does not create fragmented or redundant conceptual authority.*

- **Acceptance Criteria**:
  1. Multi-signal deterministic fingerprinting (title similarity, heading tokens, BM25/TF-IDF term vectors, tag overlap, structural n-grams) runs locally without external LLM APIs.
  2. Staged documents select a candidate comparison set (top 10–50 plausible matches) rather than evaluating $O(N^2)$ pairs.
  3. High-confidence collisions ($\ge 85\%$ similarity with conflicting authority) produce `BLOCK` on new additions; moderate overlap ($\ge 75\%$) produces `WARN`.
  4. Proposes canonical knowledge candidates based on deterministic evidence: Git creation chronology, in-degree centrality, entrypoint references (`AGENTS.md`), and heading completeness.

### User Story 4: Code-Doc Impact Analysis & Technical Drift Detection (Priority: P2)
> *As a backend engineer refactoring code, I want the pre-commit hook to notify me when my staged code changes delete or rename functions/classes/paths that are explicitly referenced in project documentation, so that technical documentation stays synchronized with implementation.*

- **Acceptance Criteria**:
  1. Rust core maintains a reverse impact index mapping code symbols and paths to documentation references.
  2. Staging code file deletions, renames, or structural modifications (removing or renaming exported symbols) automatically identifies referencing documentation via the reverse impact index.
  3. High-confidence technical contradictions (e.g. deleted service path cited in architecture docs) produce clear pre-commit diagnostics.

### User Story 5: Context Trace & Network-Aware Quality Score (Priority: P3)
> *As an AI prompt engineer or architect, I want to query how discoverable a specific technical concept is from `AGENTS.md` and view an overall Knowledge Quality score weighted by graph centrality, so that I can optimize agent context paths and knowledge discoverability.*

- **Acceptance Criteria**:
  1. `Context Trace` executes deterministic graph-constrained traversal from designated entry points (e.g. `AGENTS.md`) to query targets, computing path depth, cumulative BPE tokens (`cl100k_base`), and path quality score ($0\text{--}100\%$).
  2. Overall Knowledge Quality Score ($0\text{--}100$) aggregates Integrity, Consistency, Discoverability, and Freshness.
  3. Quality penalties are weighted by node centrality (betweenness and degree): issues in high-centrality hub nodes incur higher penalties than leaf notes.
  4. UI explicitly displays Legacy Debt versus New Regressions (e.g. `Quality: 78/100 · 0 regressions · 34 legacy debt`).

---

## 3. Functional Requirements (FR-001 – FR-033)

### 3.1 Knowledge State Boundary & Candidate Graph Lifecycle
- **FR-001**: The persistent Knowledge Graph MUST represent exclusively committed Git state. Working-tree changes MUST NOT alter the persistent index.
- **FR-002**: Pre-commit validation MUST construct an in-memory `CandidateGraph` combining the latest committed graph with staged file modifications, additions, and deletions.
- **FR-003**: The in-memory `CandidateGraph` MUST be discarded immediately upon completion of pre-commit evaluation, regardless of whether the decision is `PASS`, `WARN`, or `BLOCK`.
- **FR-004**: Post-commit synchronization MUST receive the newly committed Git SHA and delta, incrementally applying changes to the persistent index in $\le 50\text{ ms}$.

### 3.2 Dual Hook Orchestration & Fail-Open Resilience
- **FR-005**: The system MUST provide two distinct Git hooks:
  1. `llm-wiki-validate` (stage: `pre-commit`): Evaluates candidate knowledge quality and regressions.
  2. `llm-wiki-sync` (stage: `post-commit`): Applies committed delta to persistent graph.
- **FR-006**: Both hooks MUST support the 3-tier discovery hierarchy: Tier 1 (`prek` / `.pre-commit-config.yaml`), Tier 2 (Python `pre-commit` / `lefthook`), Tier 3 (standalone `.git/hooks/` scripts).
- **FR-007**: Both hooks MUST enforce a hard non-blocking timeout of $\le 500\text{ ms}$. If the daemon is offline, socket hangs, or execution times out, the hook MUST immediately exit with code 0 (`fail-open`).
- **FR-008**: Pre-commit hook CLI output MUST present human-readable diagnostics with clear file paths, line numbers, and actionable remediation steps without requiring the user to open VS Code.

### 3.3 Brownfield Debt Baseline & Regression Decision Engine
- **FR-009**: The Rust core MUST generate and maintain a `KnowledgeDebtBaseline` structure stored at `.context-vault/baseline.json`, recording:
  - `baseline_commit`: Git commit hash where baseline was established.
  - `timestamp`: RFC 3339 timestamp.
  - `broken_links`: Array of existing broken link signatures (`source`, `target`, `raw_text`).
  - `orphan_nodes`: Set of existing orphan file paths.
  - `collisions`: Array of existing semantic collision pairs.
  - `technical_drifts`: Array of existing technical drift findings.
- **FR-010**: Initial scan on an unbaselined repository MUST automatically establish the baseline without blocking or reporting regressions.
- **FR-011**: Pre-commit validation MUST evaluate the delta between Candidate findings and Baseline findings:
  $$\text{New Regressions} = \text{Candidate Findings} \setminus \text{Baseline Findings}$$
- **FR-012**: If $\text{New Regressions} = \emptyset$, the decision engine MUST return `PASS`.
- **FR-013**: If $\text{New Regressions}$ contains high-confidence broken links, invalid anchors, or high-confidence collisions with canonical conflict, the decision engine MUST return `BLOCK` (exit code 1).
- **FR-014**: If $\text{New Regressions}$ contains moderate-confidence collisions ($75\%\le \text{similarity} < 85\%$) or unconfirmed technical drift, the engine MUST return `WARN` (exit code 0 with warning diagnostics).
- **FR-015**: When a commit resolves legacy debt items, the post-commit handler MUST automatically update the baseline to reflect the reduced debt count ($\Delta < 0$).
- **FR-016**: The system MUST compute and expose the 5 debt dimensions: Total Debt, Legacy Debt, New Debt, Resolved Debt, and Regressions.

### 3.4 Deterministic Semantic Drift & Collision Engine (Zero External AI)
- **FR-017**: The Rust core MUST implement a multi-signal deterministic document fingerprinting engine computing:
  - Title & Heading token sets.
  - BM25 term frequency vectors (normalized).
  - Hashtags and YAML frontmatter tags.
  - 3-gram and 4-gram character shingles.
- **FR-018**: Candidate comparison MUST filter the vault using inverted token indexes to select top-K ($K \le 50$) plausible candidate documents, strictly avoiding $O(N^2)$ all-pairs comparison.
- **FR-019**: Pairwise semantic similarity MUST be calculated as a composite weighted score:
  $$\text{Sim}(A, B) = w_t \cdot \text{TitleSim} + w_h \cdot \text{HeadingSim} + w_b \cdot \text{BM25Sim} + w_n \cdot \text{NgramSim} + w_g \cdot \text{TagSim}$$
  where $\sum w_i = 1.0$ (defaults: $w_t=0.25, w_h=0.20, w_b=0.30, w_n=0.15, w_g=0.10$).
- **FR-020**: Semantic collisions MUST be classified into 5 descriptive categories:
  - `Potential Duplicate`: $\text{Sim} \ge 0.85$ with overlapping conceptual scope.
  - `Potential Fragmentation`: Distributed sub-topics without clear parent hierarchy.
  - `Potential Specialization`: High term overlap with deeper heading structure.
  - `Potential Summary`: High conceptual term coverage of a longer document in a condensed structure.
  - `Potential Conflict`: Overlapping scope claiming opposing conceptual definitions or instructions.
- **FR-021**: Canonical knowledge candidate identification MUST evaluate deterministic evidence:
  - Git creation chronology (earlier commit timestamp).
  - Degree & betweenness centrality in the knowledge graph.
  - Explicit citations from project harness files (`AGENTS.md`, `CLAUDE.md`, `README.md`).
  - Heading and content completeness.
- **FR-022**: The system MUST propose canonical candidates with explainable provenance badges and MUST NOT automatically delete, overwrite, or mutate documentation.

### 3.5 Reverse Impact Index & Technical Drift Engine
- **FR-023**: The Rust core AST parser MUST extract referenced code file paths and backticked symbol identifiers (e.g. `[`src/auth/service.ts`]`, \`AuthService.login()\`) from Markdown documents, cross-referencing them against workspace repository paths and staged Git diffs without requiring full multi-language compiler AST frontends.
- **FR-024**: The Rust core MUST maintain a bidirectional reverse impact index:
  $$\text{Code Entity} \longleftrightarrow \text{Markdown Note}$$
- **FR-025**: Pre-commit validation MUST check staged code changes against the reverse index. If a referenced code entity is deleted or renamed while the citing documentation note is unmodified in staged changes, a `TechnicalDriftFinding` MUST be registered.
- **FR-026**: High-confidence technical drift (deleted file or deleted exported symbol with zero surviving aliases) MUST trigger pre-commit warning or block based on configuration (`drift.block_on_deletion: true/false`, default `false` $\to$ `WARN`).

### 3.6 Context Trace & Network-Aware Quality Score
- **FR-027**: The Rust core MUST implement `ContextTrace`: given an entrypoint note ID (default `AGENTS.md`) and a task query string, it executes:
  1. Lexical BM25 retrieval for target notes.
  2. Constrained shortest-path Dijkstra/BFS traversal over valid graph edges from entrypoint to target notes.
  3. Path quality evaluation: hop distance, edge validity, intermediate node relevance, and cumulative BPE token count (`cl100k_base`).
- **FR-028**: Context Trace output MUST report: Best Discovery Path, Traversal Depth, Cumulative BPE Tokens, and Path Quality Percentage ($0\text{--}100\%$).
- **FR-029**: The Rust core MUST compute an overall `KnowledgeQualityScore` ($0\text{--}100$) combining:
  $$\text{Score} = 0.30 \cdot \text{Integrity} + 0.25 \cdot \text{Consistency} + 0.25 \cdot \text{Discoverability} + 0.20 \cdot \text{Freshness}$$
- **FR-030**: Deductions in Quality Score MUST be topologically weighted by node betweenness centrality ($B(v)$):
  $$\text{Penalty}(issue, v) = \text{BasePenalty}(issue) \times (1.0 + 3.0 \cdot B(v))$$
  ensuring defects in critical hub notes penalize quality more severely than leaf notes.

### 3.7 Webview Quality & Admission Center (Fourth Pillar)
- **FR-031**: The webview sidebar and status dashboard MUST expand to include the **Fourth Pillar: Quality & Admission**:
  - Overall Knowledge Quality Score ($0\text{--}100$).
  - Breakdown meters: Integrity, Consistency, Discoverability, Freshness.
  - Debt summary: Legacy Debt count, New Regressions count, Resolved Debt count.
  - Active baseline status badge (`Baseline: commit abc123 · 87 legacy items`).
- **FR-032**: Contextual inspector drawer MUST support inspection of:
  - `Collision` edges/cards: showing similarity score, shared terms, and `[Propose as Canonical]` action.
  - `Drift` cards: showing out-of-sync code path, citation line, and `[Open doc]` / `[Open code]` actions.
- **FR-033**: The Settings Drawer MUST include a dedicated **Quality & Git Hooks** tab allowing users to:
  - View hook installation status (`pre-commit` validate, `post-commit` sync).
  - Configure regression blocking thresholds (`block_on_broken_links`, `block_on_high_collisions`, `similarity_threshold`).
  - Re-baseline the repository manually (`[Capture New Baseline]`).

---

## 4. Non-Functional Requirements & Performance Budgets

| Requirement ID | Metric / Invariant | Threshold Ceiling | Verification Method |
| :--- | :--- | :--- | :--- |
| **NFR-001** | Pre-Commit Candidate Evaluation | $\le 150\text{ ms}$ for staged changes up to 20 files | Criterion benchmark `benches/candidate_benchmark.rs` |
| **NFR-002** | Hook Execution Timeout (Fail-Open) | Strict $\le 500\text{ ms}$ | Hanging socket mock test |
| **NFR-003** | Post-Commit Incremental Patch | $\le 50\text{ ms}$ for 1–5 changed notes | Criterion benchmark |
| **NFR-004** | Candidate Fingerprint Search | $\le 80\text{ ms}$ across 5,000 indexed notes | Inverted index candidate retrieval test |
| **NFR-005** | Rust Sidecar Memory (Rest) | $\le 35\text{ MB}$ resident RSS for 5,000 files | Valgrind / Heap profiling |
| **NFR-006** | Zero External AI | 0 HTTP network calls to AI APIs or cloud endpoints | Network sandbox audit / zero outbound requests |
| **NFR-007** | False Positive Regressions on Brownfield | Exactly 0 commits blocked on existing legacy debt | Automated brownfield simulation test suite |

---

## 5. Success Criteria (SC-001 – SC-007)

- **SC-001 (Seamless Brownfield Adoption)**: Initializing LLM Wiki on a repository with $\ge 50$ existing broken links and $\ge 20$ orphans results in 0 blocked commits when committing modifications to unrelated files.
- **SC-002 (Deterministic Regression Gate)**: Introducing 1 new broken internal link or 1 duplicate document with $\ge 85\%$ similarity in staged changes reliably triggers pre-commit `BLOCK` (exit code 1) with precise CLI diagnostics.
- **SC-003 (State Boundary Invariant)**: In 10,000 simulated pre-commit cancellations, zero uncommitted candidate nodes or edges ever appear in the persistent graph index or webview state.
- **SC-004 (Dual Hook Latency)**: Pre-commit evaluation completes in $\le 150\text{ ms}$ in 95% of commits; post-commit sync completes in $\le 50\text{ ms}$.
- **SC-005 (Debt Convergence Tracking)**: Fixing 5 legacy broken links correctly ratchets the baseline debt count down by 5 upon post-commit execution.
- **SC-006 (Context Trace Responsiveness)**: Executing a Context Trace query across 1,000 notes returns the optimal traversal path and BPE token accounting in $\le 100\text{ ms}$.
- **SC-007 (Quality Score Topologically Weighted)**: Introducing an issue on `AGENTS.md` (high betweenness) drops the Quality Score at least $2.5\times$ more than introducing the exact same issue on an isolated leaf note.

---

## 6. Out of Scope (Explicit Boundaries)

1. **Automatic Document Restructuring or Merging**: The system identifies and explains duplicate or fragmented sections, but MUST NOT automatically rewrite, merge, or delete files on disk.
2. **External AI or Embedding Models**: No OpenAI, Anthropic, Gemini, Ollama, or local ONNX vector embedding models. All text similarity is deterministic (BM25, n-grams, Levenshtein, set Jaccard).
3. **Continuous Filesystem Watchers**: No `chokidar`, `notify`, or editor keystroke polling. Mutations strictly trigger at the Git boundary.
