---
wiki:
  content_hash: 555bf09df1a1733c517b4a228365d3b2548e7d2b590f9b8eab18cbe3e4854fb1
  fragments:
    implementation-plan-deterministic-traceability-review-knowledge-consistency:
      tokens: 2279
      inbound: 0
      outbound: 1
    summary:
      tokens: 157
      inbound: 0
      outbound: 0
    technical-context:
      tokens: 387
      inbound: 0
      outbound: 0
    constitution-check:
      tokens: 507
      inbound: 0
      outbound: 0
    project-structure:
      tokens: 504
      inbound: 0
      outbound: 0
    documentation-this-feature:
      tokens: 124
      inbound: 0
      outbound: 0
    source-code-layout:
      tokens: 376
      inbound: 0
      outbound: 0
    implementation-phases:
      tokens: 629
      inbound: 0
      outbound: 0
    phase-1-core-domain-entities-frontmatter-ingestion-days-12:
      tokens: 116
      inbound: 0
      outbound: 0
    phase-2-git-diff-ingestion-affected-documents-engine-days-23:
      tokens: 118
      inbound: 0
      outbound: 0
    phase-3-prci-review-lifecycle-verifiable-receipts-days-34:
      tokens: 113
      inbound: 0
      outbound: 0
    phase-4-deterministic-architectural-rules-engine-ast-integrity-days-45:
      tokens: 105
      inbound: 0
      outbound: 0
    phase-5-json-rpc-sidecar-api-vs-code-webview-integration-days-56:
      tokens: 91
      inbound: 0
      outbound: 0
    phase-6-isolated-graphify-spike-harness-quality-benchmarks-days-67:
      tokens: 81
      inbound: 0
      outbound: 0
---
# Implementation Plan: Deterministic Traceability, Review & Knowledge Consistency

**Branch**: `010-deterministic-traceability-review` | **Date**: 2026-10-13 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/010-deterministic-traceability-review/spec.md` consolidating `/home/ds/Downloads/LLM_Wiki_Specificacao_Camada_Deterministica.md`.

---

## Summary

The **Deterministic Traceability, Review & Knowledge Consistency Layer** (`010-deterministic-traceability-review`) establishes a verifiable, bidirectionally traceable bridge between Markdown documentation, knowledge graph topology, source code files, and Git change history in `llm-wiki-core`. It introduces explicit frontmatter `code_dependencies` alongside inline citations, implements a high-performance reverse impact index supporting Git branch range diffs, formally distinguishes `change_detected`, `review_required`, and `rule_violation`, provides a PR/CI review lifecycle with verifiable receipts (`reviewed_unchanged`), validates cross-file heading and anchor mutations, and hosts an isolated research spike for third-party observed architecture tools (Graphify) without polluting canonical graph truth.

---

## Technical Context

**Language/Version**: Rust 1.78+ (Edition 2021) & TypeScript 5.4+ / Node.js 20+  
**Primary Dependencies**:
- Rust: `pulldown-cmark` (CommonMark AST & headings), `petgraph` (Graph topology), `serde`/`serde_json`/`serde_yaml`, `tiktoken-rs` (`cl100k_base`), `regex`, `pathdiff`, `walkdir`.
- TypeScript: `vscode` API, stdio JSON-RPC 2.0 client, Webview UI toolkit.  
**Storage**: Git repository filesystem, `.context-vault/reviews.json`, *.context-vault/baseline.json*. Zero external database engines.  
**Testing**: `cargo test` (unit, integration, and CLI regression tests), `npm test` (extension & webview suites), `criterion` microbenchmarks.  
**Target Platform**: Cross-platform (Linux x86_64/aarch64, macOS arm64/x86_64, Windows x86_64).  
**Project Type**: Standalone Native Binary Sidecar (`llm-wiki-core`) + VS Code Extension (`llm_wiki_ext4vsc`).  
**Performance Goals**:
- Reverse impact analysis $\le 50\text{ ms}$ for 50 changed files across 2,000 documents.
- PR check review verification $\le 100\text{ ms}$.
- Pre-commit hook timeout ceiling: strict $500\text{ ms}$ non-blocking fail-open.
- Sidecar resting memory RSS $\le 35\text{ MB}$ for 5,000 files.  
**Constraints**: 100% deterministic local computation (Zero External AI per Invariants III & IV), raw numeric integrity (Invariant IX), presentation formatting $\le 5$ chars (Invariant X).

---

## Constitution Check

*GATE: Must pass all 16 Critical Invariants of Constitution v1.6.0.*

1. **Invariant I (Committed Truth)**: Verified. Persistent knowledge graph reflects committed repository state. Staged code changes and PR diffs evaluated in ephemeral candidate overlays.
2. **Invariant II (Candidate Isolation)**: Verified. Pre-commit evaluations and review status checks operate in-memory without mutating persistent index files until commit succeeds.
3. **Invariant III (Determinism)**: Verified. Lexicographical tie-breaking, normalized POSIX path comparisons, reproducible BLAKE3/SHA-256 finding IDs.
4. **Invariant IV (Locality)**: Verified. Zero external AI APIs, zero cloud semantic services, 100% native Rust logic.
5. **Invariant V (Document-Centric Graph)**: Verified. Documents are graph nodes ($O(N)$ graph); code dependencies and section fragments are addressable node metadata.
6. **Invariant VI (Section Addressability)**: Verified. Headings and anchors normalized deterministically; cross-file heading changes validate target anchors across referencing notes.
7. **Invariant VII (AST Authority)**: Verified. AST is the sole structural authority for headings, anchors, and citation links.
8. **Invariant VIII (Materialized Metadata)**: Verified. Frontmatter `code_dependencies` validated and indexed at commit time.
9. **Invariant IX (Raw Numeric Integrity)**: Verified. Complete integer token counts and line numbers preserved in RPC models.
10. **Invariant X (Presentation Separation)**: Verified. Compact formatting applied strictly at the UI presentation boundary.
11. **Invariant XI (Brownfield Tolerance)**: Verified. Pre-existing documentation debt isolated via *.context-vault/baseline.json*; legacy debt never blocks PRs unless explicitly configured.
12. **Invariant XII (Quality Gate)**: Verified. High-confidence broken anchors and unreviewed PR regressions trigger configurable block/warn gates.
13. **Invariant XIII (Non-Prescriptive KOS)**: Verified. Document size is an analytical signal; review obligations arise from explicit dependencies, not arbitrary size thresholds.
14. **Invariant XIV (Unified Optimization Profile)**: Verified. Optimization profile compiles to a single internal structure.
15. **Invariant XV (Transparent Trade-Offs)**: Verified. Review obligations clearly articulate triggering code paths and required actions.
16. **Invariant XVI (User-Controlled Downstream Automation)**: Verified. No automatic document rewriting; review resolutions require human/agent receipt.

---

## Project Structure

### Documentation (this feature)

```text
specs/010-deterministic-traceability-review/
├── spec.md              # Consolidated feature specification
├── plan.md              # Implementation plan (this file)
├── research.md          # Technical research & formulations
├── data-model.md        # Rust structs & TypeScript interfaces
├── quickstart.md        # Runnable verification scenarios
├── contracts/
│   └── traceability_rpc_protocol.json  # JSON-RPC 2.0 interface definition
└── tasks.md             # Phased implementation tasks
```

### Source Code Layout

```text
crates/core/src/
├── model/
│   └── mod.rs             # NoteNode metadata with code_dependencies
├── ast/
│   ├── parser.rs          # CommonMark AST heading & link extraction
│   └── visitor.rs         # Inline code citations visitor
├── fragment/
│   └── frontmatter.rs     # Frontmatter parser supporting code_dependencies
├── drift/
│   ├── extractor.rs       # Code citation & dependency extractor
│   ├── index.rs           # Bidirectional ReverseImpactIndex (code <-> docs)
│   ├── detector.rs        # Technical drift detector (staged deltas)
│   ├── freshness.rs       # Git commit history & freshness evidence
│   └── review.rs          # PR/CI Review Lifecycle & receipt validator (NEW)
├── architecture/
│   └── rules.rs           # Deterministic architectural rule checker (NEW)
├── ipc/
│   └── handlers.rs        # RPC methods (trace/getAffectedDocuments, review/resolve)
└── cli/
    ├── validate.rs        # Pre-commit candidate validator
    ├── trace_impact.rs    # `llm-wiki-core trace-impact` CLI command (NEW)
    └── review_cmd.rs      # `llm-wiki-core review` CLI command (NEW)

crates/core/examples/
└── graphify_spike.rs      # Isolated observed architecture spike harness (NEW)

extension/
├── src/
│   ├── types/engine.ts    # TypeScript interfaces for ReviewObligation & findings
│   └── webview/           # Evolution & Health drawer review UI integration
└── test/suite/
    └── traceability.test.ts  # End-to-end integration tests
```

---

## Implementation Phases

### Phase 1: Core Domain Entities & Frontmatter Ingestion (Days 1–2)
- Extend frontmatter parsing in `crates/core/src/fragment/frontmatter.rs` to ingest `code_dependencies: Vec<String>`.
- Implement POSIX path normalization and strict workspace root containment guard (`PATH_TRAVERSAL_PROHIBITED`).
- Update `ReverseImpactIndex` in `crates/core/src/drift/index.rs` to index both frontmatter declarations and inline Markdown links with origin tagging.
- Unit tests in `crates/core/tests/drift_test.rs`.

### Phase 2: Git Diff Ingestion & Affected Documents Engine (Days 2–3)
- Extend `crates/core/src/delta/` to support Git commit ranges (`A..B`) and branch diffs (`base...head`).
- Implement affected documentation calculation in `crates/core/src/drift/review.rs`, mapping modified code files to dependent notes with deduplication.
- Implement rename and deletion detection with `OBSOLETE_CODE_REFERENCE` and `DELETED_CODE_FILE` findings.
- Unit and integration tests for all 4 Git diff strategies.

### Phase 3: PR/CI Review Lifecycle & Verifiable Receipts (Days 3–4)
- Implement `ReviewObligation` and resolution status transitions (`updated`, `reviewed_unchanged`, `pending`, `blocked`).
- Implement multi-tier receipt ingestion: commit trailer parser (`Reviewed-Docs: ...`), `.context-vault/reviews.json`, and RPC payload.
- Integrate PR review policy check into `llm-wiki-core validate` CLI command, distinguishing pre-existing legacy debt from new unreviewed regressions.

### Phase 4: Deterministic Architectural Rules Engine & AST Integrity (Days 4–5)
- Implement explicit rule verification in `crates/core/src/architecture/rules.rs` (`prohibited_dependencies`, `mandatory_documentation`, `path_exists`).
- Implement cross-file heading mutation tracking emitting `BROKEN_FRAGMENT_ANCHOR` when heading changes invalidate incoming links.
- Enforce strict separation: `change_detected` (info), `review_required` (warn), `rule_violation` (error).

### Phase 5: JSON-RPC Sidecar API & VS Code Webview Integration (Days 5–6)
- Expose `trace/getAffectedDocuments` and `review/resolveDocumentReview` in `crates/core/src/ipc/handlers.rs`.
- Update TypeScript types in `extension/src/types/` and RPC client in `extension/src/rpc/`.
- Add review obligations and pending review badges to Health/Evolution drawer UI.

### Phase 6: Isolated Graphify Spike Harness & Quality Benchmarks (Days 6–7)
- Author isolated spike `crates/core/examples/graphify_spike.rs` benchmarking Graphify vs Understand Anything on reference repo.
- Produce empirical benchmark report in `target/graphify_evaluation_report.md`.
- Run full automated test suite (`cargo test`, `npm test`) and Criterion latency benchmarks.
