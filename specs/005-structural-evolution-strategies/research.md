# Research & Technical Decisions: Knowledge Organization Systems, Structural Evidence & Trade-Offs

**Feature Branch:** `005-structural-evolution-strategies`  
**Target Delivery:** `llm_wiki_ext4vsc` (`llm-wiki-core` + VS Code Extension)  
**Status:** Approved Baseline  
**Constitution Authority:** `CONSTITUTION.md` v1.6.0  

---

## 1. Research Questions & Resolution Matrix

### RQ-1: Mathematical Formulation of the 10 Structural Dimensions
- **Problem**: How to measure document structure across 10 dimensions locally in Rust without external LLMs or vector embeddings?
- **Decision**:
  1. **Cohesion ($C \in [0.0, 1.0]$)**: Jaccard/TF-IDF similarity across sections within the document:
     $$C(D) = \frac{1}{\binom{K}{2}} \sum_{i < j} \frac{|T(s_i) \cap T(s_j)|}{|T(s_i) \cup T(s_j)|}$$
  2. **Coupling ($K \in [0.0, 1.0]$)**: Normalized in-degree and out-degree relative to network size:
     $$K(D) = \min\left(1.0, \frac{\text{in\_degree}(D) + \text{out\_degree}(D)}{20}\right)$$
  3. **Reference Concentration ($R \in [0.0, 1.0]$)**: Ratio of references to top fragment vs total document:
     $$R(D) = \max_{f \in F(D)} \frac{\text{inbound}(f)}{\max(1, \text{inbound}(D))}$$
  4. **Granularity ($G \in [0.0, 1.0]$)**: Closeness of average fragment size to target (e.g. 5,000 tokens):
     $$G(D) = 1.0 - \min\left(1.0, \frac{|\text{avg\_tokens}(D) - T_{\text{target}}|}{T_{\text{target}}}\right)$$
  5. **Context Efficiency ($E \in [0.0, 1.0]$)**: Ratio of target fragment tokens to total document tokens:
     $$E(D) = \frac{\text{median\_fragment\_tokens}(D)}{\text{total\_document\_tokens}(D)}$$
  6. **Discoverability ($D \in [0.0, 1.0]$)**: Shortest hop distance from central hub and betweenness centrality:
     $$D(D) = 0.5 \cdot \left(1.0 - \min(1.0, \frac{\text{hop\_distance}(D)}{4})\right) + 0.5 \cdot \text{betweenness}(D)$$
  7. **Reusability ($U \in [0.0, 1.0]$)**: Proportion of inbound links targeting fragments:
     $$U(D) = \frac{\sum_{f \in F(D)} \text{inbound}(f)}{\max(1, \text{inbound}(D))}$$
  8. **Semantic Diversity ($S \in [0.0, 1.0]$)**: 1.0 minus internal section cohesion:
     $$S(D) = 1.0 - C(D)$$
  9. **Navigational Complexity ($N \in [0.0, 1.0]$)**: Link density and cycle count around the node:
     $$N(D) = \min\left(1.0, \frac{\text{clustering\_coefficient}(D) \cdot \text{out\_degree}(D)}{10}\right)$$
  10. **Maintenance Coupling ($M \in [0.0, 1.0]$)**: Reverse index count of citing documents and code paths:
      $$M(D) = \min\left(1.0, \frac{\text{citing\_docs}(D) + \text{citing\_code}(D)}{15}\right)$$

### RQ-2: Deterministic Mapping from Tier 1 Objectives to Tier 2 Parameters
- **Problem**: How to translate high-level user knobs (`low`, `medium`, `high`) into concrete engine parameters without hidden side-effects?
- **Decision**:
  - Define a deterministic linear mapping matrix:
    - `AI Discoverability`: drives `max_trace_depth` ($5 \to 3 \to 2$), `candidate_expansion_factor` ($2 \to 3 \to 5$), and `discoverability_weight` ($0.5 \to 0.8 \to 1.0$).
    - `Context Efficiency`: drives `context_efficiency_weight` ($0.5 \to 0.8 \to 1.0$), `max_context_surface_tokens` ($25\text{k} \to 12\text{k} \to 5\text{k}$ tokens), and `fragment_size_target` ($15\text{k} \to 8\text{k} \to 3\text{k}$ tokens).
    - `Human Readability`: drives `narrative_coherence_weight` ($0.4 \to 0.7 \to 1.0$), `cohesion_threshold` ($0.4 \to 0.7 \to 1.0$), and `split_sensitivity` ($0.8 \to 0.5 \to 0.2$).
    - `Knowledge Reuse`: drives `canonical_ownership_sensitivity` ($0.5 \to 0.75 \to 0.95$) and `duplicate_tolerance` ($0.3 \to 0.1 \to 0.02$).
    - `Narrative Coherence`: drives `fragmentation_tolerance` ($0.5 \to 0.3 \to 0.1$), penalizing topic decomposition.
    - `Maintenance Simplicity`: drives `maintenance_simplicity_weight` ($0.3 \to 0.6 \to 0.9$) and `reference_concentration_threshold` ($0.5 \to 0.7 \to 0.85$).
  - Expose this mapping as inspectable metadata so the UI can draw direct dependency links when hovering over tooltips (`ⓘ`).

### RQ-3: Constraint Conflict Detection Algorithm
- **Problem**: How to detect mathematically contradictory configurations (e.g. demanding $\le 5\text{k}$ context surface, but setting a hard constraint never to split documents)?
- **Decision**:
  - Implement a fast rule-based SAT evaluator in Rust executing in $\le 5\text{ ms}$:
    - Conflict Rule 1: `max_context_surface < max(document_tokens)` $\land$ `never_split_documents` $\implies$ Conflict: Context budget cannot be satisfied without splitting oversized documents.
    - Conflict Rule 2: `context_efficiency == high` $\land$ `narrative_coherence == high` $\land$ `never_split_documents` $\implies$ Warning: Tension between preserving long-form continuity and eliminating unreferenced context.
    - Conflict Rule 3: `duplicate_tolerance == 0.0` $\land$ `never_delete_content` $\land$ high-confidence duplicate exists $\implies$ Conflict: Cannot eliminate duplicate without modification or consolidation.
  - Return structured findings: `ConstraintConflict { conflicting_rules: Vec<String>, reason: String, suggestion: String }`.

### RQ-4: Evidence-Based Structural Signatures vs. Size Signals
- **Problem**: In linters, large files are blindly marked as errors. How to strictly adhere to Invariant XIII (*"size is a signal, not a verdict"*)?
- **Decision**:
  - A document is classified as a `GodDocument` **only when all four criteria hold simultaneously**:
    1. $\text{semantic\_diversity} \ge 0.75$ (multiple unrelated topics coexisting).
    2. $\text{reference\_concentration} \ge 0.70$ (one section absorbs $>70\%$ of external citations).
    3. $\text{context\_efficiency} \le 0.35$ (traversing the doc loads $>65\%$ useless tokens).
    4. $\text{coupling} \ge 0.70$ (many incoming/outgoing edges).
  - If a 50k-token document has $\text{cohesion} \ge 0.80$ (single unified subject, e.g. complete API reference or historical chronicle) and uniform reference distribution ($R \le 0.30$), its signature is `HealthyMonolith` (no splitting recommended; only section addressing for fast routing).

### RQ-5: Deterministic Transformation Plan Generation & Re-Measurement
- **Problem**: How to automate refactoring safely without AI hallucinations or corrupted Git trees?
- **Decision**:
  - All transformations are deterministic AST and link operations:
    1. `SectionAddressingAction`: Inject heading anchor frontmatter (`wiki.fragments`), rewrite calling links `[Doc](doc.md)` to `[Doc#Sec](doc.md#sec)` when calling context targets that section.
    2. `LightweightIndexAction`: Create router document `index.md` listing child documents with 1-sentence summaries derived from first paragraphs.
    3. `TopicDecompositionAction`: Extract subsections into `doc_subtopic.md`, replace source section with router link, update inbound links targeting that fragment to the new note.
  - Every transformation generates a unified diff preview.
  - Post-apply re-measurement runs an incremental pass recalculating the 10 structural metrics and returning:
    $$\Delta \text{Metric} = \text{Metric}_{\text{after}} - \text{Metric}_{\text{before}}$$
  - The UI presents: `Context Efficiency: 32% → 81% (+49%) | Trace Depth: 4.8 → 2.1 (-2.7 hops) | Document Count: 1 → 4 (+3 docs)`.
