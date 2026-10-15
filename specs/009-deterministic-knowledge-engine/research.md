---
wiki:
  content_hash: ffef5c25a6ae83128f099b72af31743afb8c9013148839031c036d8e5544f74e
  fragments:
    technical-research-deterministic-knowledge-engineering-engine:
      tokens: 3937
      inbound: 0
      outbound: 0
    1-executive-summary-epistemological-boundaries:
      tokens: 447
      inbound: 0
      outbound: 0
    the-epistemological-invariant:
      tokens: 157
      inbound: 0
      outbound: 0
    the-4-epistemic-tiers:
      tokens: 230
      inbound: 0
      outbound: 0
    2-core-algorithmic-selections:
      tokens: 505
      inbound: 0
      outbound: 0
    3-mathematical-formulations-of-the-18-methodologies:
      tokens: 2358
      inbound: 0
      outbound: 0
    31-classification:
      tokens: 126
      inbound: 0
      outbound: 0
    32-taxonomy:
      tokens: 130
      inbound: 0
      outbound: 0
    33-faceted-classification:
      tokens: 149
      inbound: 0
      outbound: 0
    34-controlled-vocabulary:
      tokens: 156
      inbound: 0
      outbound: 0
    35-thesaurus-iso-25964-skos:
      tokens: 134
      inbound: 0
      outbound: 0
    36-semantic-networks:
      tokens: 108
      inbound: 0
      outbound: 0
    37-conceptual-maps:
      tokens: 102
      inbound: 0
      outbound: 0
    38-ontology-engineering:
      tokens: 150
      inbound: 0
      outbound: 0
    39-domain-analysis:
      tokens: 157
      inbound: 0
      outbound: 0
    310-metadata-application-profiles-dcap:
      tokens: 101
      inbound: 0
      outbound: 0
    311-information-architecture:
      tokens: 164
      inbound: 0
      outbound: 0
    312-information-retrieval-multi-field-bm25f:
      tokens: 216
      inbound: 0
      outbound: 0
    313-terminology-management:
      tokens: 80
      inbound: 0
      outbound: 0
    314-knowledge-graph-network-analysis:
      tokens: 136
      inbound: 0
      outbound: 0
    315-knowledge-quality-5-pillar-model:
      tokens: 84
      inbound: 0
      outbound: 0
    316-provenance-w3c-prov-git:
      tokens: 84
      inbound: 0
      outbound: 0
    317-knowledge-drift-tennis-subject-ontogeny:
      tokens: 120
      inbound: 0
      outbound: 0
    318-context-knowledge-routing-context-trace:
      tokens: 147
      inbound: 0
      outbound: 0
    4-multi-criteria-decision-analysis-ahp-for-strategy-fit:
      tokens: 235
      inbound: 0
      outbound: 0
    5-implementation-cost-deterministic-tie-breaking-hierarchy:
      tokens: 317
      inbound: 0
      outbound: 0
    51-implementation-cost-formulation:
      tokens: 121
      inbound: 0
      outbound: 0
    52-deterministic-4-stage-tie-breaking:
      tokens: 180
      inbound: 0
      outbound: 0
---
# Technical Research: Deterministic Knowledge Engineering Engine

**Feature**: `009-deterministic-knowledge-engine`  
**Date**: 2026-10-06  
**Status**: Completed  
**Sources**: `RESEARCH_1.md`, `RESEARCH_2.md`, `SPEC.md`, `CONSTITUTION.md` (v1.6.0)

---

## 1. Executive Summary & Epistemological Boundaries

This research establishes the algorithmic and mathematical foundations for the **Deterministic Knowledge Engineering Engine**, a high-performance local Rust engine and VS Code extension designed to observe, measure, diagnose, and guide the evolution of version-controlled Markdown knowledge repositories.

### The Epistemological Invariant
The engine operates under a strict epistemological boundary:
$$\text{Repository Snapshot } (S_H) \times \text{Configuration } (C) \times \text{Optimization Profile } (P) \times \text{Methodology Version } (M) \to \text{Deterministic Result}$$

Every analytical result is bit-level reproducible, fully auditable, and fingerprinted by:
$$\text{analysis\_id} = \text{BLAKE3}\Big(H_{\text{git}} \,\|\, \text{CanonicalJSON}(C) \,\|\, \text{CanonicalJSON}(P) \,\|\, M_{\text{version}}\Big)$$

### The 4 Epistemic Tiers
The engine strictly delineates four cognitive and algorithmic tiers:
1. **Tier A — Fully Deterministic**: Direct calculations from CommonMark AST, filesystem topology, graph theory, and lexical indexes. (e.g., links, anchors, AST spans, PageRank, BM25 scores, DCAP schema conformance).
2. **Tier B — Deterministic Candidate Generation**: Algorithmic generation of hypotheses, near-duplicates, or clustering candidates without asserting semantic truth. (e.g., candidate synonyms via string distance, candidate collisions, facet candidates).
3. **Tier C — Decision Support**: Multi-criteria utility models, trade-off evaluations, and fit scoring where ultimate decision authority belongs to human developers or explicit agent prompts. (e.g., strategy recommendations, canonical note selection, monolith decomposition plans).
4. **Tier D — Not Reliably Determinable**: Properties that cannot be mathematically proven without subjective interpretation or non-deterministic inference. (e.g., natural language semantic truth, latent conceptual equivalence, authorial intent). The engine refuses to emit definitive verdicts in this tier.

---

## 2. Core Algorithmic Selections

| Domain / Function | Selected Algorithm / Technique | Crate / Implementation | Complexity | Determinism Rationale |
| :--- | :--- | :--- | :--- | :--- |
| **Cryptographic Digest** | BLAKE3 Tree Hashing | `blake3` | $O(N)$ SIMD | Sub-microsecond cryptographic hashing, reproducible tree hashing across threads. |
| **Markdown Parsing & AST** | Event-driven CommonMark AST parser | `pulldown-cmark` | $O(L)$ linear | Compliant CommonMark 0.30+, AST spans, zero allocations for inline text slices. |
| **Graph Topology & Metrics** | Directed Graph, Brandes Centrality, PageRank | `petgraph` + custom Brandes | $O(V \cdot E)$ betweenness, $O(k \cdot E)$ PageRank | Fixed power-iteration tolerance ($10^{-6}$), deterministic node iteration sorting. |
| **Cycle & Articulation Points** | Tarjan SCC & Hopcroft-Tarjan Bridges | Custom / `petgraph::algo` | $O(V + E)$ | Linear time, canonical topological ordering. |
| **Lexical Search & Ranking** | Inverted Index, Okapi BM25 & Multi-field BM25F | `tantivy` | $O(|q| \cdot \text{postings})$ | Embedded, zero-cloud, deterministic field weights and BM25 tie-breaking. |
| **Multi-Pattern Term Matching** | Deterministic Finite Automaton (DFA) | `aho-corasick` | $O(N + M)$ | Linear-time multi-string matching for controlled vocabularies and TBX terms. |
| **String Distance & Variants** | Levenshtein & Jaro-Winkler | `strsim` | $O(m \cdot n)$ | Deterministic edit distance with threshold $\tau \le 2$ for variant detection. |
| **Schema & Shape Validation** | W3C SHACL-like Structural Shape Evaluator | Custom Rust validator | $O(\text{props})$ | Fast, zero-overhead validation of YAML frontmatter application profiles. |
| **Decision Support Scoring** | Saaty Analytic Hierarchy Process (AHP) / MCDA | Custom fixed-point matrix solver | $O(K^2)$ | Eigenvector priority ranking with consistent pairwise comparison matrices. |

---

## 3. Mathematical Formulations of the 18 Methodologies

### 3.1 Classification
- **Category Coverage ($C_{\text{cov}}$)**:
  $$C_{\text{cov}} = \frac{|D_{\text{classified}}|}{|D_{\text{total}}|}$$
- **Classification Consistency ($C_{\text{con}}$)**:
  $$C_{\text{con}} = 1 - \frac{\sum_{c \in C} \text{Var}\Big(\text{BM25}(d \in c, \bar{c})\Big)}{|C|}$$

### 3.2 Taxonomy
- **Taxonomic Imbalance Ratio ($I_{\text{tax}}$)**:
  $$I_{\text{tax}} = \frac{\sigma\big(\text{depth}(\text{Leaves})\big)}{\mu\big(\text{depth}(\text{Leaves})\big)}$$
- **Branching Factor ($b(u)$)**:
  $$b(u) = \text{deg}^{+}(u) = |\{v \in V \mid (u, v) \in E_{\text{tax}}\}|$$

### 3.3 Faceted Classification
- **Pairwise Mutual Information (PMI)**:
  $$I(F_i; F_j) = \sum_{x \in F_i} \sum_{y \in F_j} p(x,y) \log_2 \frac{p(x,y)}{p(x)p(y)}$$
- **Facet Orthogonality ($O_{\text{fac}}$)**:
  $$O_{\text{fac}} = 1 - \frac{2}{|F|(|F|-1)} \sum_{i < j} \frac{I(F_i; F_j)}{\min\big(H(F_i), H(F_j)\big)}$$

### 3.4 Controlled Vocabulary
- **Uncontrolled Term Ratio ($R_{\text{unctrl}}$)**:
  $$R_{\text{unctrl}} = \frac{|T_{\text{used}} \setminus V_{\text{authorized}}|}{|T_{\text{used}}|}$$
- **Vocabulary Drift ($V_{\text{drift}}$)**:
  $$V_{\text{drift}}(t_1, t_2) = \frac{|V_{\text{auth}}(t_2) \setminus V_{\text{auth}}(t_1)|}{|V_{\text{auth}}(t_1)|}$$

### 3.5 Thesaurus (ISO 25964 / SKOS)
- **Thesaurus Reciprocity Violation Ratio ($R_{\text{recip}}$)**:
  $$R_{\text{recip}} = \frac{|\{(u, v) \in E_{\text{BT}} \mid (v, u) \notin E_{\text{NT}}\}| + |\{(u, v) \in E_{\text{RT}} \mid (v, u) \notin E_{\text{RT}}\}|}{|E_{\text{total}}|}$$

### 3.6 Semantic Networks
- **Relational Type Shannon Diversity ($H_{\text{rel}}$)**:
  $$H_{\text{rel}} = -\sum_{r \in R} p(r) \log_2 p(r)$$
- **Relational Edge Density ($\rho_{\text{rel}}$)**:
  $$\rho_{\text{rel}} = \frac{|E_{\text{typed}}|}{|V|(|V|-1)}$$

### 3.7 Conceptual Maps
- **Proposition Density ($P_{\text{dens}}$)**:
  $$P_{\text{dens}} = \frac{|Propositions|}{|Words|}$$
- **Cross-Link Ratio ($R_{\text{xlink}}$)**:
  $$R_{\text{xlink}} = \frac{|E_{\text{cross\_domain}}|}{|E_{\text{total}}|}$$

### 3.8 Ontology Engineering
- **Ontological Conformance ($O_{\text{conf}}$)**:
  $$O_{\text{conf}} = 1 - \frac{|D_{\text{violating\_shapes}}|}{|D_{\text{total}}|}$$
- **Property Sparsity ($S_{\text{prop}}$)**:
  $$S_{\text{prop}}(C) = \frac{|\{(i, p) \mid i \in C, p \in \text{Props}(C), \text{value}(i, p) = \emptyset\}|}{|Instances(C)| \cdot |Props(C)|}$$

### 3.9 Domain Analysis
- **Domain Lexical Concentration ($L_{\text{dom}}$)**:
  $$L_{\text{dom}}(t) = \text{TF}(t, D) \cdot \ln\left(\frac{N}{\text{DF}(t)}\right) \cdot \frac{1}{\text{Dispersion}(t)}$$
- **Git Commit Author-Doc Bipartite Modularity ($Q_{\text{author}}$)**:
  $$Q = \frac{1}{2m} \sum_{vw} \left[ A_{vw} - \frac{k_v k_w}{2m} \right] \delta(c_v, c_w)$$

### 3.10 Metadata / Application Profiles (DCAP)
- **Metadata Completeness ($M_{\text{comp}}$)**:
  $$M_{\text{comp}}(d) = \frac{|\text{PresentMandatoryProps}(d)|}{|\text{RequiredMandatoryProps}|}, \quad M_{\text{conf}} = \frac{\sum_{d \in D} M_{\text{comp}}(d)}{|D|}$$

### 3.11 Information Architecture
- **Information Scent Proxy ($\text{Scent}(u, v)$)**:
  $$\text{Scent}(u, v) = \text{Jaccard}\Big(\text{Tokens}(\text{Label}(u, v)), \text{Tokens}(\text{Title}(v))\Big)$$
- **Findability Index ($F_{\text{idx}}$)**:
  $$F_{\text{idx}}(d) = \sum_{p \in \text{Paths}(\text{Root}, d)} \prod_{(u, v) \in p} \text{Scent}(u, v) \cdot \frac{1}{\text{length}(p)}$$

### 3.12 Information Retrieval (Multi-Field BM25F)
- **Field-Weighted Term Frequency ($\tilde{c}(t, d)$)**:
  $$\tilde{c}(t, d) = \sum_{f \in \text{Fields}} w_f \cdot \frac{c(t, d_f)}{1 - b_f + b_f \cdot \frac{l_{d_f}}{\bar{l}_f}}$$
- **BM25F Score**:
  $$S_{\text{BM25F}}(q, d) = \sum_{t \in q} \text{IDF}(t) \cdot \frac{\tilde{c}(t, d)}{k_1 + \tilde{c}(t, d)}, \quad \text{IDF}(t) = \ln\left( \frac{N - n(t) + 0.5}{n(t) + 0.5} + 1 \right)$$

### 3.13 Terminology Management
- **Terminology Consistency ($T_{\text{con}}$)**:
  $$T_{\text{con}} = 1 - \frac{\sum_{t \in T_{\text{canon}}} |Variants(t)|}{|T_{\text{canon}}| \cdot |D|}$$

### 3.14 Knowledge Graph / Network Analysis
- **Brandes Betweenness Centrality ($C_B(v)$)**:
  $$C_B(v) = \sum_{s \neq v \neq t \in V} \frac{\sigma_{st}(v)}{\sigma_{st}}$$
- **Power-Iteration PageRank ($PR^{(k+1)}$)**:
  $$PR^{(k+1)}(u) = \frac{1 - d}{|V|} + d \sum_{v \in M(u)} \frac{PR^{(k)}(v)}{L(v)}$$

### 3.15 Knowledge Quality (5-Pillar Model)
- **Profile-Relative Quality Score**:
  $$\text{Health}(D) = \sum_{p \in \text{Pillars}} W_p \cdot \text{Score}_p(D)$$
  Where $\sum W_p = 1.0$ and weights are determined by the active `OptimizationProfile`.

### 3.16 Provenance (W3C PROV / Git)
- **Commit Co-Change Coupling ($C_{\text{prov}}(A, B)$)**:
  $$C_{\text{prov}}(A, B) = \frac{|\text{Commits}(A \cap B)|}{|\text{Commits}(A \cup B)|}$$

### 3.17 Knowledge Drift (Tennis Subject Ontogeny)
- **Temporal Citation Drift ($\Delta_{\text{drift}}$)**:
  $$\Delta_{\text{drift}}(d, t_1, t_2) = 1 - \frac{|\text{Citations}(d, t_1) \cap \text{Citations}(d, t_2)|}{|\text{Citations}(d, t_1) \cup \text{Citations}(d, t_2)|}$$

### 3.18 Context / Knowledge Routing (Context Trace)
- **Context Trace Token Load ($\text{Cost}_{\text{trace}}$)**:
  $$\text{Cost}_{\text{trace}} = \sum_{n \in \text{TracePath}} \text{Tokens}(n_{\text{fragment}})$$
- **Context Reduction Percentage ($\text{CR}_{\%}$)**:
  $$\text{CR}_{\%} = 1.0 - \frac{\text{Cost}_{\text{trace}}}{\sum_{d \in \text{DocsInvolved}} \text{Tokens}(d_{\text{full}})}$$

---

## 4. Multi-Criteria Decision Analysis (AHP) for Strategy Fit

To determine the empirical **Fit Score** for candidate strategies without arbitrary heuristics, the engine utilizes Saaty's Analytic Hierarchy Process (AHP):

1. **Pairwise Comparison Matrix ($A$)**: Built based on profile objective weights:
   $$A = [a_{ij}], \quad a_{ji} = \frac{1}{a_{ij}}$$
2. **Priority Vector ($w$)**: Calculated as the normalized principal eigenvector of $A$:
   $$A w = \lambda_{\max} w$$
3. **Consistency Ratio ($CR$)**: Verified to ensure $CR = \frac{CI}{RI} < 0.10$.
4. **Strategy Utility ($U(S)$)**:
   $$U(S) = \sum_{k=1}^K w_k \cdot \mu_k(S) - \text{Penalty}_{\text{constraints}}(S)$$

This guarantees that strategy selection is mathematically explainable, transparent, and profile-relative.

---

## 5. Implementation Cost & Deterministic Tie-Breaking Hierarchy

### 5.1 Implementation Cost Formulation
To evaluate the pragmatic engineering cost of applying a transformation without human guesswork, the engine computes a normalized `implementation_cost` $\in [0.0, 1.0]$:
$$\text{Cost}_{\text{impl}}(S) = \min\left(1.0, \; \frac{0.6 \cdot |\text{TargetFiles}(S)| + 0.4 \cdot \frac{|\text{EstimatedLineChurn}(S)|}{50}}{10.0}\right)$$

### 5.2 Deterministic 4-Stage Tie-Breaking
When two or more candidate strategies are evaluated for the same target scope, ranking ties are deterministically broken using the following strict priority ladder:
1. **Fit Score**: Primary ranking criterion (higher Fit Score wins);
2. **Implementation Cost**: Secondary criterion if Fit Scores are within $\epsilon < 10^{-5}$ (lower Implementation Cost wins);
3. **Profile Dimension Weight**: Tertiary criterion comparing the active profile weight for each strategy's primary targeted quality pillar (higher weight wins);
4. **Canonical Enum Order**: Final tie-breaker following the immutable Rust enum definition order (`PreserveDocument` $\to$ `SplitDecompose` $\to$ `ExtractFacetMetadata` $\to$ `ConsolidateIndex` $\to$ `MergeCandidates` $\to$ `StandardizeTerminology`).
