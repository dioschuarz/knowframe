# Technical Research: Admission Control, Deterministic Drift & Debt Baseline

**Feature Branch:** `003-knowledge-admission-control`  
**Target Delivery:** `llm_wiki_ext4vsc` (`llm-wiki-core` in Rust + TypeScript Extension Host)  
**Status:** Approved Research / Architecture Synthesis  

---

## 1. Ephemeral Candidate Graph Overlay

### The Problem
During `pre-commit`, the Rust engine must evaluate the impact of staged changes on graph integrity, hop distances, and token budgets in $\le 150\text{ ms}$. If we clone the entire `petgraph::Graph` and all document ASTs for a 5,000-note vault, latency and memory usage would violate our budgets.

### The Solution: Overlay Architecture
Instead of deep-cloning:
1. The persistent `GraphTopology` is held in memory as an immutable baseline (`&GraphTopology`).
2. Staged changes are parsed into a lightweight `OverlayDelta`:
   - `modified_nodes`: Map of changed `NoteNode` ASTs.
   - `added_nodes`: New `NoteNode` ASTs.
   - `deleted_nodes`: Set of deleted paths.
   - `modified_edges`: Edge mutations derived from changed links.
3. Candidate evaluation queries look up nodes in `OverlayDelta` first, falling back to `&GraphTopology`.
4. The candidate evaluation produces `CandidateValidationReport` and is dropped from RAM at the end of the `pre-commit` command. Zero mutation touches the persistent graph.

---

## 2. Brownfield Debt Baseline Differential Engine

### The Problem
Brownfield repositories often have pre-existing broken links and unmapped orphans. If pre-commit blocks indiscriminately, developers cannot adopt the tool.

### The Solution: Stable Finding Signatures
Every finding is serialized into a stable deterministic signature:
- **Broken Link**: `hash(source_file, target_path, heading_anchor)`.
- **Orphan**: `hash(file_path)`.
- **Collision**: `hash(min(doc_a, doc_b), max(doc_a, doc_b))`.
- **Technical Drift**: `hash(doc_path, code_path, symbol)`.

On first scan, the set of signatures is serialized to `.context-vault/baseline.json`.  
During pre-commit:
$$\text{New Regressions} = \text{Candidate Findings} \setminus \text{Baseline Findings}$$
Only signatures present in $\text{New Regressions}$ trigger `BLOCK` or `WARN`.  
When a commit resolves baseline findings:
$$\text{Resolved} = \text{Baseline Findings} \setminus \text{Candidate Findings}$$
The baseline automatically ratchets down ($\text{Total Debt} \leftarrow \text{Total Debt} - |\text{Resolved}|$).

---

## 3. Deterministic Semantic Collision without External AI

### The Problem
Comparing $N$ documents against each other requires $O(N^2)$ comparisons. For 5,000 documents, that is 12,500,000 comparisons. External vector embeddings and LLM calls are strictly prohibited by Constitution Principle I.

### The Solution: Two-Stage Filter & Deterministic Fingerprints
1. **Candidate Retrieval via Inverted Index ($O(1)$ to $O(K)$)**:
   - For staged documents, extract title tokens, headings, and high-IDF term tokens.
   - Query an in-memory inverted token index to retrieve the top 10–50 candidate documents sharing the highest token intersection.
2. **Deterministic Composite Scoring**:
   For each candidate pair $(A, B)$, compute:
   - **Title Jaccard**: Token intersection over union.
   - **Heading Overlap**: Shared markdown heading tokens (`## ...`).
   - **BM25 Cosine**: Dot product of normalized BM25 term vectors.
   - **3-Gram Character Shingles**: Substring overlap catching morphologically similar phrases.
   - **Tag Overlap**: Jaccard similarity of hashtags and frontmatter tags.
3. **Thresholds**:
   - $\ge 0.85$: High-confidence collision (`Potential Duplicate` / `Conflict`) $\to$ `BLOCK` if canonical conflict exists.
   - $0.75 - 0.84$: Moderate collision (`Potential Fragmentation` / `Specialization`) $\to$ `WARN`.
   - $< 0.75$: Distinct conceptual territory $\to$ `PASS`.

---

## 4. Code-to-Doc Reverse Impact Index

### The Problem
When a developer refactors code (e.g. renames `AuthService` to `IdentityService` or deletes `src/auth/token.ts`), technical documentation becomes stale (Technical Drift).

### The Solution: Symbol & Path Citation Extraction
1. **Indexing Pass**: When indexing Markdown documents, the AST parser extracts:
   - File path citations: e.g. ``[`src/auth/service.ts`]`` or markdown links to code files.
   - Code symbol citations: e.g. `` `AuthService.login()` ``.
2. **Reverse Map**: Maintained as `HashMap<String, HashSet<String>>`:
   - `code_to_docs`: `src/auth/service.ts` $\to$ `[docs/architecture/auth.md, docs/security.md]`.
   - `symbol_to_docs`: `AuthService` $\to$ `[docs/architecture/auth.md]`.
3. **Pre-Commit Trigger**:
   - Staged files include code paths. If a code path is marked `deleted` or `renamed`, any citing documentation that is NOT also modified in staged changes triggers a `TechnicalDriftFinding`.

---

## 5. Context Trace Algorithm

### The Formulation
Given query string $Q$ and entry point $E$ (default `AGENTS.md`):
1. **Target Selection**: Score all vault notes against $Q$ using BM25, selecting target note $T$ with maximum relevance.
2. **Graph-Constrained Shortest Path**: Run Dijkstra on directed graph edges from $E$ to $T$.
3. **Path Quality Metrics**:
   - Depth (number of hops).
   - Relevant token ratio: $\frac{\text{Tokens}(T)}{\sum_{v \in \text{Path}} \text{Tokens}(v)}$.
   - Path Quality Score:
     $$\text{Quality} = 100 \times \left( \frac{1}{1 + 0.3 \cdot \text{Depth}} \right) \times \text{RelevanceRatio}$$
4. **Token Accounting**: Total cumulative BPE tokens calculated via `tiktoken-rs` (`cl100k_base`).
