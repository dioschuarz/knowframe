---
wiki:
  content_hash: 280da1086e1306cda5c8f7a4588996026857b8588308a23e850253c7a16bfb25
  fragments:
    implementation-plan-deterministic-knowledge-engineering-engine:
      tokens: 2425
      inbound: 0
      outbound: 1
    summary:
      tokens: 132
      inbound: 0
      outbound: 0
    technical-context:
      tokens: 473
      inbound: 0
      outbound: 0
    constitution-check:
      tokens: 460
      inbound: 0
      outbound: 0
    project-structure:
      tokens: 575
      inbound: 0
      outbound: 0
    documentation-this-feature:
      tokens: 124
      inbound: 0
      outbound: 0
    source-code-layout:
      tokens: 447
      inbound: 0
      outbound: 0
    implementation-phases:
      tokens: 700
      inbound: 0
      outbound: 0
    phase-1-core-domain-entities-determinism-bedrock:
      tokens: 68
      inbound: 0
      outbound: 0
    phase-2-observation-structural-ingestion-subsystem:
      tokens: 75
      inbound: 0
      outbound: 0
    phase-3-graph-analytics-information-retrieval-subsystem:
      tokens: 68
      inbound: 0
      outbound: 0
    phase-4-18-knowledge-organization-methodologies:
      tokens: 108
      inbound: 0
      outbound: 0
    phase-5-profile-relative-5-pillar-health-scoring:
      tokens: 69
      inbound: 0
      outbound: 0
    phase-6-strategy-selection-ahp-fit-scoring-recommendation-engine:
      tokens: 64
      inbound: 0
      outbound: 0
    phase-7-user-controlled-downstream-automation-re-measurement:
      tokens: 118
      inbound: 0
      outbound: 0
    phase-8-vs-code-extension-integration-management-ux:
      tokens: 66
      inbound: 0
      outbound: 0
    phase-9-end-to-end-validation-performance-benchmarks:
      tokens: 59
      inbound: 0
      outbound: 0
---
# Implementation Plan: Deterministic Knowledge Engineering Engine

**Branch**: `009-deterministic-knowledge-engine` | **Date**: 2026-10-06 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/009-deterministic-knowledge-engine/spec.md` consolidating `SPEC.md`, `RESEARCH_1.md`, and `RESEARCH_2.md`.

---

## Summary

The Deterministic Knowledge Engineering Engine implements a high-performance, purely deterministic knowledge organization, diagnostic, and decision-support engine in Rust with VS Code extension integration. It evaluates version-controlled Markdown repositories against 18 Knowledge Organization System (KOS) disciplines, computes profile-relative 5-pillar health across 4 Documentation Evolution Modes (`AI-Oriented`, `Human-Oriented`, `Balanced`, `Custom`), enforces strict epistemic tier boundaries (Tiers A–D), guarantees bit-level determinism with BLAKE3 `analysis_id`, and delivers user-controlled transformation plans with explicit trade-offs and Before/After re-measurement.

---

## Technical Context

**Language/Version**: Rust 1.78+ (Edition 2021) & TypeScript 5.4+ / Node.js 20+  
**Primary Dependencies**:
- Rust: `pulldown-cmark` (CommonMark AST), `petgraph` (Graph theory), `tantivy` (BM25/BM25F inverted index), `tiktoken-rs` (exact BPE tokenizer `cl100k_base`), `aho-corasick` (pattern matching), `blake3` (cryptographic hashing), `strsim` (string metrics), `serde`/`serde_json`/`serde_yaml`, `rayon` (parallel processing).
- TypeScript: `vscode` API, `rpc-websockets` / stdio JSON-RPC 2.0 client, Webview UI toolkit with CSP nonces.  
**Storage**: Git repository filesystem, *.context-vault/baseline.json*, in-memory ephemeral candidate graph. Zero external databases.  
**Testing**: `cargo test` (unit and integration tests), `criterion` (performance benchmarks), `vitest` / VS Code extension integration tests.  
**Target Platform**: Cross-platform (Linux x86_64/aarch64, macOS arm64/x86_64, Windows x86_64).  
**Project Type**: Standalone Native Binary Sidecar (`llm-wiki-core`) + VS Code Extension (`llm_wiki_ext4vsc`).  
**Performance Goals**:
- Full cold repository scan (1,000 documents) $\le 1,500$ ms.
- Incremental candidate document re-analysis $\le 50$ ms.
- Post-commit graph synchronization (`Hook 2: llm-wiki-sync`) $\le 50$ ms.
- Pre-commit hook evaluation (`Hook 1: llm-wiki-validate`) $\le 500$ ms fail-open.
- Resident memory (RSS) in resting state $\le 35$ MB for 5,000 documents.
- Webview UI responsiveness $\le 16$ ms (60 FPS).  
**Constraints**: Zero external AI / LLM APIs (Invariants III & IV), raw numeric integrity (Invariant IX), presentation formatting $\le 5$ chars (Invariant X), Sandboxed CSP Webview.

---

## Constitution Check

*GATE: Must pass all 16 Critical Invariants of Constitution v1.6.0.*

1. **Invariant I (Committed Truth)**: Verified. Knowledge Graph reflects committed Git state; working edits analyzed in-memory.
2. **Invariant II (Candidate Isolation)**: Verified. Pre-commit evaluations isolated in `CandidateGraph`, discarded after validation.
3. **Invariant III (Determinism)**: Verified. Fixed algorithms, BLAKE3 `analysis_id`, canonical ordering, reproducible math.
4. **Invariant IV (Locality)**: Verified. Zero external AI APIs, local Tantivy and Petgraph processing.
5. **Invariant V (Document-Centric Graph)**: Verified. Documents are nodes ($O(N)$ graph), fragments are metadata.
6. **Invariant VI (Section Addressability)**: Verified. Headings addressable via normalized URI anchors (`doc.md#anchor`).
7. **Invariant VII (AST Authority)**: Verified. CommonMark AST from `pulldown-cmark` is the sole structural authority.
8. **Invariant VIII (Materialized Metadata)**: Verified. `wiki.fragments` frontmatter validated at pre-commit.
9. **Invariant IX (Raw Numeric Integrity)**: Verified. 64-bit integers stored and passed across RPC.
10. **Invariant X (Presentation Separation)**: Verified. $\le 5$ characters compact formatting applied exclusively in UI.
11. **Invariant XI (Brownfield Tolerance)**: Verified. Pre-existing debt tracked in *.context-vault/baseline.json*.
12. **Invariant XII (Quality Gate)**: Verified. Pre-commit blocks only on new regressions.
13. **Invariant XIII (Non-Prescriptive KOS)**: Verified. Size is a signal, not a verdict; healthy monoliths protected.
14. **Invariant XIV (Unified Optimization Profile)**: Verified. All 4 modes compile to single internal `OptimizationProfile`.
15. **Invariant XV (Transparent Trade-Offs)**: Verified. Expected benefits and countervailing trade-offs itemized for every recommendation.
16. **Invariant XVI (User-Controlled Downstream Automation)**: Verified. `Auto-Apply: OFF`, `approval_token` mandatory, Before/After validation.

---

## Project Structure

### Documentation (this feature)

```text
specs/009-deterministic-knowledge-engine/
├── spec.md              # Consolidated feature specification
├── plan.md              # Implementation plan (this file)
├── research.md          # Technical research & mathematical formulations
├── data-model.md        # Rust structs & TypeScript interfaces
├── quickstart.md        # Verification scenarios & validation guide
├── contracts/
│   └── knowledge_engine_protocol.json  # JSON-RPC 2.0 interface
└── tasks.md             # Phased implementation tasks
```

### Source Code Layout

```text
crates/
├── core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── model/             # DocumentNode, FragmentMetadata, OptimizationProfile
│       ├── epistemic/         # EpistemicTier tagging & boundary enforcement
│       ├── ast/               # CommonMark AST visitor & citation extractor
│       ├── graph/             # Petgraph topology, Brandes, PageRank, Tarjan SCC
│       ├── ir/                # Tantivy inverted index, Okapi BM25, multi-field BM25F
│       ├── terminology/       # Aho-Corasick matching, controlled vocabulary FST
│       ├── methodologies/     # 18 KOS methodology diagnostic modules
│       ├── health/            # 5-Pillar profile-relative health scoring
│       ├── diagnosis/         # Antipattern rules, evidence vs interpretation
│       ├── strategies/        # 6 candidate strategies, AHP Fit Scoring, trade-offs
│       ├── automation/        # AST mutators, unified diff generator, approval tokens
│       └── trace/             # Context Trace beam search & token accounting
├── cli/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs            # `llm-wiki-core` CLI binary & hook runner
│       └── commands/          # validate, sync, analyze, trace, plan, apply, baseline
└── engine/
    ├── Cargo.toml
    └── src/
        └── rpc/               # JSON-RPC 2.0 server over stdio

extension/src/                 # VS Code Extension Host (TypeScript)
├── extension.ts               # Extension activation & sidecar lifecycle
├── rpc/                       # JSON-RPC 2.0 stdio client
├── webview/                   # Manager Webview (Health, Structure, Evolution, Settings)
├── navigation/                # Bidirectional editor deep-linking
└── types/                     # TypeScript contracts & data models
```

---

## Implementation Phases

### Phase 1: Core Domain Entities & Determinism Bedrock
- Implement `EpistemicTier`, `EvolutionMode`, `Methodology`, `OptimizationProfile` in `crates/core`.
- Implement BLAKE3 `analysis_id` derivation and canonical serialization.
- Unit tests verifying bit-exact determinism across runs.

### Phase 2: Observation & Structural Ingestion Subsystem
- CommonMark AST visitor via `pulldown-cmark` extracting headings, anchors, wikilinks, and code citations.
- Materialized frontmatter parser and Dublin Core Application Profile (DCAP) validator.
- In-memory `CandidateGraph` and Git commit history ingestion via `gix`.

### Phase 3: Graph Analytics & Information Retrieval Subsystem
- Directed graph model with Petgraph.
- Brandes betweenness centrality, power-iteration PageRank ($10^{-6}$), Tarjan SCC cycle detection.
- Embedded Tantivy index with multi-field BM25F and Aho-Corasick terminology DFA.

### Phase 4: 18 Knowledge Organization Methodologies
- Implement diagnostic modules for all 18 KOS disciplines.
- Verify exact mathematical formulation for each metric ($C_{\text{cov}}$, $I_{\text{tax}}$, $O_{\text{fac}}$, $R_{\text{recip}}$, $H_{\text{rel}}$, $P_{\text{dens}}$, etc.).
- Tag every finding with its strict Epistemic Tier (Tier A or Tier B).

### Phase 5: Profile-Relative 5-Pillar Health Scoring
- Health scoring engine evaluating Integrity, Structure, Consistency, Discoverability, Freshness.
- Integration of *.context-vault/baseline.json* for brownfield debt isolation.
- Verification of Invariant XIII: size as a signal, monolith protection guard.

### Phase 6: Strategy Selection, AHP Fit Scoring & Recommendation Engine
- Candidate strategy evaluators for all 6 strategies.
- Saaty AHP decision model computing empirical Fit Scores and implementation cost.
- Categorization into Primary, Supporting, Alternative, and Mutually Exclusive roles with explicit trade-offs.

### Phase 7: User-Controlled Downstream Automation & Re-Measurement
- Cryptographic `approval_token` generator and validation gate (`Auto-Apply: OFF`).
- Mechanically verifiable AST and filesystem mutators generating unified diffs.
- Pre-commit validation hook (`llm-wiki-validate`, stage: pre-commit) $\le 500$ ms fail-open.
- Post-commit synchronization hook (`llm-wiki-sync`, stage: post-commit) applying committed Git deltas in $\le 50$ ms.
- Post-apply Before/After delta re-measurement engine.

### Phase 8: VS Code Extension Integration & Management UX
- JSON-RPC 2.0 stdio communication bridge.
- Sandboxed 4-card Webview with CSP nonces (Health, Structure, Evolution, Settings).
- Bidirectional deep linking to editor line/column and presentation formatting $\le 5$ chars.

### Phase 9: End-to-End Validation & Performance Benchmarks
- Full Criterion benchmarking suite verifying $\le 1,500$ ms cold scan, $\le 50$ ms incremental updates, and $\le 500$ ms pre-commit hook.
- Cross-platform packaging verification.
