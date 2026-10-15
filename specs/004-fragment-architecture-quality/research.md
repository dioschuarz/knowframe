---
wiki:
  content_hash: c23c3c10aaf31505a2d6f6b1cee512357b1e9ad05f5f358155ef1b7f97afdef1
  fragments:
    research-technical-decisions-fragment-level-architecture-5-pillar-quality:
      tokens: 1144
      inbound: 0
      outbound: 0
    1-research-questions-resolution-matrix:
      tokens: 1060
      inbound: 0
      outbound: 0
    rq-1-deterministic-heading-anchor-normalization-collision-handling:
      tokens: 152
      inbound: 0
      outbound: 0
    rq-2-safe-non-destructive-frontmatter-materialization-git-index-staging:
      tokens: 218
      inbound: 0
      outbound: 0
    rq-3-constrained-beam-search-vs-bfs-for-context-trace:
      tokens: 263
      inbound: 0
      outbound: 0
    rq-4-evidence-based-freshness-vs-file-age-pillar-5:
      tokens: 228
      inbound: 0
      outbound: 0
    rq-5-pillar-2-structure-mathematical-formulation:
      tokens: 189
      inbound: 0
      outbound: 0
---
# Research & Technical Decisions: Fragment-Level Architecture & 5-Pillar Quality

**Feature Branch:** `004-fragment-architecture-quality`  
**Target Delivery:** `llm_wiki_ext4vsc` (`llm-wiki-core` + VS Code Extension)  
**Status:** Approved Baseline  
**Constitution Authority:** `CONSTITUTION.md` v1.5.0  

---

## 1. Research Questions & Resolution Matrix

### RQ-1: Deterministic Heading Anchor Normalization & Collision Handling
- **Problem**: How to derive URL-safe fragment anchors from arbitrary Markdown headings (e.g. `## 2.1 Authentication & Tokens (OAuth2)`) in Rust that 100% match CommonMark and GitHub anchor conventions?
- **Decision**:
  - Convert heading text to lowercase ASCII.
  - Strip punctuation and Markdown syntax (backticks, bold, links).
  - Convert whitespace and consecutive hyphens to single hyphen (`-`).
  - Strip leading and trailing hyphens.
  - Suffix duplicate anchors within the same document deterministically with `-1`, `-2`, etc.
  - Store anchor string directly in `FragmentMetadata.anchor`.

### RQ-2: Safe, Non-Destructive Frontmatter Materialization & Git Index Staging
- **Problem**: How to update `wiki.fragments` in YAML frontmatter during pre-commit without modifying user-defined frontmatter keys, removing comments, or mutating Markdown content, while ensuring the commit snapshot includes the change?
- **Decision**:
  - Use `serde_yaml` or line-based frontmatter splitting (`---` delimiters).
  - If frontmatter exists: parse into generic YAML mapping, update or insert `wiki` key with `fragments` mapping, serialize back to YAML, and preserve remaining document lines intact.
  - If no frontmatter exists: prepend `---` frontmatter block to file.
  - Compute sha256 of AST structure before and after to verify zero text corruption.
  - Re-staging guarantee: The pre-commit hook (`llm-wiki-validate`) automatically runs `git add <file>` on modified Markdown files, ensuring the materialized frontmatter is included in the commit tree without leaving working tree drift (Invariants I & II).

### RQ-3: Constrained Beam Search vs. BFS for Context Trace
- **Problem**: BFS can suffer combinatorial explosion on large vaults; simple Dijkstra only finds shortest paths regardless of semantic query relevance.
- **Decision**:
  - Use **Constrained Beam Search** with beam width $W=5$ and max depth $D=4$.
  - At each step from current path tip, evaluate candidate neighbor documents and fragments using composite relevance:
    $$\text{Score}(u, f) = 0.4 \cdot \text{BM25}(Q, f) + 0.3 \cdot \text{TitleMatch}(Q, u) + 0.2 \cdot \text{GraphProximity}(u) + 0.1 \cdot \text{Centrality}(u)$$
  - Select top $W$ paths to expand.
  - Section-level refinement: If candidate document contains a fragment matching query terms with score $\ge 0.75$, route directly to `doc.md#fragment`. If no specific fragment matches, target the document node directly with 0.0% context reduction.
  - Guarantees completion in $\le 100\text{ ms}$ on 1,000 notes.

### RQ-4: Evidence-Based Freshness vs. File Age (Pillar 5)
- **Problem**: File mtime or git commit age penalizes evergreen documents (e.g. fundamental architecture that hasn't needed changes in 2 years).
- **Decision**:
  - Calculate Freshness entirely by **Evidence of Currentness**:
    1. Local code citations existence: Cited code files and symbols checked on disk ($100\% \to$ fresh; missing $\to$ penalty -15 pts per broken citation).
    2. Git commit chronology: Check if cited code paths were modified in Git *after* the document's last commit ($0 \text{ commits} \to$ fresh; unreviewed Git commits on cited code $\to$ penalty -10 pts per commit, capped at -30 pts max).
    3. Deleted/renamed symbols: Code modifications removing cited identifiers (-20 pts per deleted symbol).
    4. Canonical contradictions: Opposing authoritative docs (-25 pts).
  - Raw age is purely a secondary display label, never a primary scoring penalty.

### RQ-5: Pillar 2 "Structure" Mathematical Formulation
- **Problem**: How to quantify structural organization without arbitrary heuristics?
- **Decision**:
  - Base structure score = 100.
  - Deductions:
    - Missing fragment metadata on large document ($>50\text{k}$ tokens or $>10$ headings): -25 pts.
    - Document bloat ($>50\text{k}$ tokens): -15 pts.
    - Extreme reference concentration ($C(f) \ge 70\%$ on docs with $\ge 50\text{k}$ tokens): -20 pts.
    - Unfragmented central hub (in-degree $\ge 10$, tokens $\ge 20\text{k}$, fragments $\le 1$): -20 pts.
  - Clamped to $[0, 100]$. Weighted at 20% in overall Quality Score.
