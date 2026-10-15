---
wiki:
  content_hash: 8718626d685b4f158460d0d3a79b40926e6f01c02535c55d5ebb41d71021f785
  fragments:
    phase-0-research-strategy-selection-combination-architecture:
      tokens: 2988
      inbound: 0
      outbound: 0
    1-candidate-strategy-fit-score-engine-eligibility-gate:
      tokens: 566
      inbound: 0
      outbound: 0
    decision:
      tokens: 116
      inbound: 0
      outbound: 0
    rationale:
      tokens: 308
      inbound: 0
      outbound: 0
    alternatives-considered:
      tokens: 118
      inbound: 0
      outbound: 0
    2-pairwise-strategy-compatibility-relationship-matrix:
      tokens: 510
      inbound: 0
      outbound: 0
    decision-1:
      tokens: 195
      inbound: 0
      outbound: 0
    rationale-1:
      tokens: 251
      inbound: 0
      outbound: 0
    alternatives-considered-1:
      tokens: 52
      inbound: 0
      outbound: 0
    3-9-step-selection-algorithm-structured-output-strategyset:
      tokens: 654
      inbound: 0
      outbound: 0
    decision-2:
      tokens: 481
      inbound: 0
      outbound: 0
    rationale-2:
      tokens: 105
      inbound: 0
      outbound: 0
    alternatives-considered-2:
      tokens: 49
      inbound: 0
      outbound: 0
    4-composite-plan-value-trade-off-valuation-model:
      tokens: 572
      inbound: 0
      outbound: 0
    decision-3:
      tokens: 462
      inbound: 0
      outbound: 0
    rationale-3:
      tokens: 95
      inbound: 0
      outbound: 0
    5-interactive-ui-selection-real-time-incompatibility-guards:
      tokens: 418
      inbound: 0
      outbound: 0
    decision-4:
      tokens: 151
      inbound: 0
      outbound: 0
    incompatibility-guard-rules-fr-011:
      tokens: 174
      inbound: 0
      outbound: 0
    rationale-4:
      tokens: 78
      inbound: 0
      outbound: 0
    6-multi-strategy-transformation-plan-synthesis-atomic-preview:
      tokens: 206
      inbound: 0
      outbound: 0
    decision-5:
      tokens: 77
      inbound: 0
      outbound: 0
    conflict-prevention-in-actions:
      tokens: 114
      inbound: 0
      outbound: 0
---
# Phase 0 Research: Strategy Selection & Combination Architecture

**Feature**: Strategy Selection & Combination (`007-strategy-selection-combination`)  
**Branch**: `007-strategy-selection-combination`  
**Date**: 2026-09-23  
**Status**: Completed & Resolved  

---

## 1. Candidate Strategy Fit Score Engine ($0\text{--}100$) & Eligibility Gate

### Decision
We implement a deterministic integer Fit Score engine evaluating each of the 4 candidate strategies (`section_addressing`, `faceted_classification`, `lightweight_index`, `preserve_document`) against observable 10-dimensional structural metrics and the active `OptimizationProfile` (Invariants III, XIV). The score is scaled strictly to an integer in the range $0\text{--}100$. Any strategy scoring $\text{fit} < 60$ is strictly disqualified as an eligible candidate (FR-003) and omitted from primary, supporting, and alternative recommendation lists.

### Rationale
- **Objective Mathematical Calibration**: Rather than treating strategy selection as an arbitrary heuristic, each strategy calculates alignment with concrete evidence dimensions:
  - `section_addressing`: Driven by high reference concentration ($C \ge 0.70$), high document token load ($>50\text{k}$), low context efficiency, modulated by `context_efficiency_weight` and `maintenance_simplicity_weight`.
  - `lightweight_index`: Driven by high betweenness centrality (implicit router), multiple outgoing links, and sparse prose, modulated by `discoverability_weight` and `narrative_coherence_weight`.
  - `faceted_classification`: Driven by conceptual fragmentation across loosely coupled documents, tag inconsistency, modulated by `discoverability_weight` and `coupling_weight`.
  - `preserve_document`: Driven by high narrative coherence, low structural debt, or explicit `never_split_documents` constraints.
- **Strict Empirical Threshold ($\text{fit} \ge 60$)**: A threshold of 60 prevents weak, noise-inducing recommendations from cluttering the developer's interface (SC-001).
- **Default Fallback to `preserve_document` (Edge Case 1 & Invariant XIII)**: If no transformation strategy reaches $\text{fit} \ge 60$, the system deterministically defaults to `preserve_document` as the sole `PRIMARY` strategy with an advisory notice: *"No active structural transformation meets the 60% confidence threshold under current profile constraints."*

### Alternatives Considered
- **Binary Recommender (One Winner Takes All)**: The existing implementation selected a single strategy. This was discarded because real-world knowledge architectures require combinations (e.g. section addressing inside a large chapter paired with a lightweight index table of contents).
- **Continuous Float Scores ($0.0\text{--}1.0$)**: Exposing floats in UI contradicts presentation clarity (Invariant X) and creates floating-point rounding ambiguities in user thresholding. Integer $0\text{--}100$ guarantees unambiguous equality and ranking.

---

## 2. Pairwise Strategy Compatibility & Relationship Matrix

### Decision
We model strategy compatibility as a symmetric, statically verified pairwise relationship matrix classified into three canonical variants: `COMPLEMENTARY`, `ALTERNATIVE`, and `MUTUALLY_EXCLUSIVE` (FR-004, FR-005).

```text
                               section_addressing   faceted_classification   lightweight_index   preserve_document
section_addressing             MUTUALLY_EXCLUSIVE   ALTERNATIVE              COMPLEMENTARY       MUTUALLY_EXCLUSIVE
faceted_classification         ALTERNATIVE          MUTUALLY_EXCLUSIVE       COMPLEMENTARY       MUTUALLY_EXCLUSIVE
lightweight_index              COMPLEMENTARY        COMPLEMENTARY            MUTUALLY_EXCLUSIVE  MUTUALLY_EXCLUSIVE
preserve_document              MUTUALLY_EXCLUSIVE   MUTUALLY_EXCLUSIVE       MUTUALLY_EXCLUSIVE  MUTUALLY_EXCLUSIVE
```

*(Note: Identity pairs $(A, A)$ are marked `MUTUALLY_EXCLUSIVE` to prevent duplicate inclusion).*

### Rationale
- **`section_addressing` + `lightweight_index` $\to$ `COMPLEMENTARY`**: These two strategies operate on orthogonal structural layers. `section_addressing` provides deep fragment anchors inside the Markdown document, while `lightweight_index` provides a high-level navigational router outside or at the head of the document. Together, they create an optimal two-tier retrieval pathway (high-level index $\to$ targeted section anchor).
- **`faceted_classification` + `lightweight_index` $\to$ `COMPLEMENTARY`**: Faceted taxonomy tags group documents by cross-cutting attributes; the lightweight index materializes those groups into browsable indexes.
- **`section_addressing` + `faceted_classification` $\to$ `ALTERNATIVE`**: Both attempt to resolve multi-topic overload—one through hierarchical heading anchor segmentation, the other through orthogonal category tagging. Combining them on a single note creates redundant categorization mechanisms.
- **`preserve_document` Invariant (FR-006)**: `preserve_document` represents an explicit decision **not** to mutate the target document's structure. Therefore, it cannot be combined with any strategy that introduces structural mutations (anchor injection, file splitting, or tag rewriting).

### Alternatives Considered
- **Directed Graph of Compatibility**: Asymmetric compatibility (where $A$ can accompany $B$, but $B$ cannot accompany $A$) was rejected because coexistence on the same target document is inherently bidirectional.

---

## 3. 9-Step Selection Algorithm & Structured Output (`StrategySet`)

### Decision
We implement the canonical 9-step selection pipeline in the Rust core (`crates/core/src/evolution/strategies.rs`), outputting a structured `StrategySet` data model (FR-007, FR-008):

1. **Step 1 — Detect**: Receive structural problem signature on target document.
2. **Step 2 — Evaluate Fit**: Calculate integer Fit Score ($0\text{--}100$) for all 4 candidate strategies.
3. **Step 3 — Filter Ineligible**: Discard strategies with $\text{Fit} < 60$. If empty, apply `preserve_document` fallback (Edge Case 1).
4. **Step 4 — Classify Relationships**: Query pairwise relationship matrix for all eligible candidates.
5. **Step 5 — Select Primary**: Designate the eligible candidate with the highest Fit Score as `PRIMARY`.
   - *Deterministic Tie-Breaking (Edge Case 2, Invariants III, XIV)*: If two eligible strategies share the exact same top Fit Score, the tie is broken using an exhaustive 4-stage hierarchy:
     1. Highest Fit Score ($0\text{--}100$).
     2. Lowest transformation cost (`transformation_cost` integer).
     3. Highest corresponding objective weight in active `OptimizationProfile` (e.g. `context_efficiency_weight` vs `narrative_coherence_weight`).
     4. Canonical declaration order (`section_addressing` > `lightweight_index` > `faceted_classification` > `preserve_document`) as absolute mathematical fallback.
6. **Step 6 — Select Supporting**: Add any eligible candidate that is `COMPLEMENTARY` to the `PRIMARY` strategy into the `SUPPORTING` list.
7. **Step 7 — Collect Alternatives**: Any eligible candidate that is `ALTERNATIVE` or `MUTUALLY_EXCLUSIVE` to the `PRIMARY` strategy is placed in `ALTERNATIVES` with role `ALTERNATIVE`.
8. **Step 8 — Calculate Total Plan Value**: Compute the composite utility score for the active recommendation set (`primary` + `supporting`).
9. **Step 9 — Synthesize StrategySet**: Package results into a strongly typed `StrategySet` payload for IPC dispatch to the VS Code Extension Host and Webview.

### Rationale
- **Deterministic & Fast ($\le 10\text{ ms}$, SC-006)**: The 4-candidate space has a fixed $4 \times 4$ search bound. Execution completes in microseconds, well below the 10 ms budget.
- **Clear Conceptual Separation**: Eliminates user confusion by cleanly distinguishing what the engine recommends as the main solution (`primary`), what amplifies it (`supporting`), and what other paths exist (`alternatives`).

### Alternatives Considered
- **Permutational Optimization (Knapsack)**: Searching all $2^N$ combinations with arbitrary weights was rejected as over-engineered for a 4-strategy space with well-defined orthogonal relationships.

---

## 4. Composite Plan Value & Trade-Off Valuation Model

### Decision
We formulate the composite plan value as an additive valuation model balancing problem-solving fit, architectural synergy, conceptual overlap, and transformation effort (FR-008, FR-014, Invariant XV):

$$\text{TotalPlanValue}(\mathcal{S}) = \sum_{s \in \mathcal{S}} \text{Fit}(s) + \sum_{(s_i, s_j) \in \mathcal{S}^2, i < j} \text{Synergy}(s_i, s_j) - \sum_{(s_i, s_j) \in \mathcal{S}^2, i < j} \text{Overlap}(s_i, s_j) - \sum_{s \in \mathcal{S}} \text{Effort}(s)$$

Where:
- $\text{Fit}(s) \in [60, 100]$: Raw integer fit of strategy $s$.
- $\text{Synergy}(s_i, s_j)$:
  - $(\text{section\_addressing}, \text{lightweight\_index}) = +15$ (dual-pathway retrieval: deep anchor addressing + high-level catalog routing).
  - $(\text{faceted\_classification}, \text{lightweight\_index}) = +12$ (taxonomic index grouping).
  - All other strategy pairs have synergy $0$.
- $\text{Overlap}(s_i, s_j)$:
  - $(\text{section\_addressing}, \text{lightweight\_index}) = 5$ (slight duplication between index items and heading anchors).
  - $(\text{faceted\_classification}, \text{lightweight\_index}) = 3$ (slight duplication between category metadata listings and index catalog).
  - All other strategy pairs have overlap $0$.
- $\text{Effort}(s)$:
  - `preserve_document`: $0$
  - `section_addressing`: $5$ (AST anchor parsing + frontmatter injection)
  - `lightweight_index`: $10$ (new index file generation + link routing)
  - `faceted_classification`: $15$ (multi-document tag restructuring)

### Rationale
- **Transparent Trade-Off Accounting (Invariant XV)**: Combining a second strategy is not "free". The formula mathematically proves whether adding a supporting strategy yields positive marginal utility ($\Delta \text{Value} > 0$) or adds unnecessary maintenance drag.
- **Dynamic Recalculation (FR-014)**: As users toggle checkboxes in the UI, the client or daemon instantly updates the Total Plan Value in real time.

---

## 5. Interactive UI Selection & Real-Time Incompatibility Guards

### Decision
In the Management & Settings drawer (`extension/webview-ui/src/components/evolutionTab.ts`), recommendation cards render the synthesized `StrategySet` with three distinct visual sections:
1. **Primary Strategy (`PRIMARY`)**: Rendered with an active checkbox `☑`, distinct blue/cyan pill badge, integer fit score, and narrative justification.
2. **Supporting Strategies (`SUPPORTING`)**: Rendered with active checkboxes `☑` (pre-checked by default), green pill badges, individual fit scores, and the synergy rationale.
3. **Alternative Strategies (`ALTERNATIVES`)**: Rendered with unchecked checkboxes `☐`, amber/neutral pill badges, individual fit scores, and trade-off comparison.

### Incompatibility Guard Rules (FR-011)
- If the user checks an `ALTERNATIVE` strategy (e.g. `faceted_classification` while `section_addressing` is checked), the conflicting strategy is automatically unchecked and disabled, and an inline notification badge informs the user: *"Faceted Classification is an alternative to Section Addressing. Swapped active strategy to prevent conflicting restructuring."*
- If the user selects `preserve_document`, all transformation strategies are immediately unchecked, disabled, and grayed out with the verbatim notice: *"Preserve Document represents zero structural transformation and cannot be combined with transformation strategies."*
- If the user unchecks all strategies, the `[Preview Change]` / `[Generate Plan]` button is disabled with an inline tooltip: *"Select at least one strategy to generate a transformation plan."* (Edge Case 3).

### Rationale
- **Zero Configuration Accidents (Invariant XVI)**: Prevents users from generating an invalid or self-contradictory transformation plan.
- **Fast Interactive Response ($<16.6\text{ ms}$)**: Incompatibility checks are client-side set lookups against the relationship table, executing in $<1\text{ ms}$ without UI lag.

---

## 6. Multi-Strategy Transformation Plan Synthesis & Atomic Preview

### Decision
When the user clicks `[Generate Plan]`, the extension passes the array of `selected_strategies` to the Rust daemon via JSON-RPC `evolution/generatePlan`. The `TransformationEngine` iterates over all selected strategies, aggregates their `TransformationAction` operations, and produces a single unified diff preview and composite `MetricDeltaReport` (SC-007).

### Conflict Prevention in Actions
- Actions on the same file are composed in strict dependency order:
  1. AST parsing and anchor normalization (`section_addressing`).
  2. Materialized frontmatter injection (`inject_fragment_frontmatter`).
  3. New document / index creation (`create_document` for `lightweight_index`).
  4. Link rewriting across referencing documents (`rewrite_link_references`).
- Operations on identical target files are coalesced into a single updated file buffer before computing the unified diff, guaranteeing zero conflicting disk writes.
