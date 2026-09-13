# Implementation Plan: Fragment-Level Architecture & 5-Pillar Quality Model

**Feature Branch:** `004-fragment-architecture-quality`  
**Target Delivery:** `llm_wiki_ext4vsc` (`llm-wiki-core` in Rust + TypeScript Extension Host)  
**Status:** In Review / Implementation Plan  
**Constitution Authority:** `CONSTITUTION.md` v1.5.0 (Invariants I–XII, Principles I–XI)  

---

## 1. Architectural Overview & Component Topology

```mermaid
graph TD
    subgraph MarkdownSource["Markdown AST & Frontmatter Layer"]
        MDFile["Markdown Document (Source File)"] --> ASTParser["pulldown-cmark AST Parser"]
        ASTParser --> HeadingExtraction["Heading Extraction & Anchor Normalization"]
        HeadingExtraction --> FragmentTokens["BPE cl100k_base Token Accounting per Fragment"]
        FragmentTokens --> FrontmatterMaterializer["Pre-Commit Frontmatter Materializer (wiki.fragments)"]
        FrontmatterMaterializer --> AutoStaging["Automatic Git Index Re-staging (git add)"]
        AutoStaging --> MDFile
    end

    subgraph RustCore["Rust Core Engine (llm-wiki-core)"]
        ASTParser --> DocumentNode["Document Node (Graph Node O(N))"]
        HeadingExtraction --> FragmentMeta["Fragment Metadata (Attached to Document Node)"]
        DocumentNode --> FragmentMeta
        
        FragmentMeta --> ReferenceConcentration["Reference Concentration Engine"]
        FragmentMeta --> FragmentLinkResolver["Section Link & Anchor Resolver"]
        FragmentLinkResolver --> CandidateValidator["Candidate Admission Validator (BLOCK on BrokenFragmentAnchor)"]
        
        DocumentNode --> StructurePillar["Pillar 2: Structure Evaluator (20%)"]
        ReferenceConcentration --> StructurePillar
        
        DocumentNode --> FreshnessEngine["Pillar 5: Evidence-Based Freshness Engine (15%)"]
        
        DocumentNode --> BeamTrace["Context Trace Constrained Beam Search"]
        FragmentMeta --> BeamTrace
        
        StructurePillar --> FivePillarScore["5-Pillar Network-Aware Quality Score"]
        FreshnessEngine --> FivePillarScore
    end

    subgraph ExtensionHost["Extension Host & Webview Presentation Layer"]
        DocumentNode --> NodeInspector["Contextual Selection Inspector Drawer"]
        FragmentMeta --> FragmentTable["3-Column Fragment Inspector (Fragment | Tok | I/O)"]
        FivePillarScore --> DiagnosticsDrawer["5-Pillar Diagnostics Drawer"]
        BeamTrace --> TokenEconomicsPopover["Token Economics (~90% Context Reduction)"]
        NumberFormatter["UI Formatter (formatCompactNumber <= 5 chars)"] --> FragmentTable
        NumberFormatter --> DiagnosticsDrawer
    end
```

---

## 2. Directory & Module Layout

### Rust Core (`crates/core/src/`)
- `fragment/`: Section fragment extraction, normalization, and frontmatter materialization.
  - `fragment/mod.rs`: `FragmentMetadata`, `MaterializedWikiFrontmatter`, serde types.
  - `fragment/extractor.rs`: CommonMark AST heading extraction, URL-safe anchor normalization, line range detection, BPE token counting.
  - `fragment/frontmatter.rs`: Non-destructive YAML frontmatter reading, injection, and updating (`wiki.fragments`) preserving user keys, with automatic git staging (`git add`).
  - `fragment/concentration.rs`: Reference concentration calculation ($C(f) = \text{inbound}(f) / \sum \text{inbound}$).
- `graph/`:
  - `graph/node.rs`: Integrate `fragments: Vec<FragmentMetadata>` and `freshness: Option<FreshnessEvidence>` into `NoteNode`.
  - `graph/edge.rs`: Support `target_fragment: Option<String>` and `LinkResolutionStatus::BrokenFragmentAnchor`.
  - `graph/resolver.rs`: Resolve section fragment links (`doc.md#anchor`) and detect dangling anchors.
- `candidate/`:
  - `candidate/validator.rs`: Pre-commit admission control evaluating candidate graph; enforces `BLOCK` on new `BrokenFragmentAnchor` regressions and emits `WARN` on oversized documents lacking metadata.
- `trace/`:
  - `trace/beam.rs`: Constrained beam search ($W=5, D=4$) with section-level routing and context reduction calculation.
- `drift/`:
  - `drift/freshness.rs`: Deterministic evidence-based freshness scoring (code citation existence, Git commit chronology, deleted symbols, canonical contradictions).
- `quality/`:
  - `quality/structure.rs`: Structure debt finding engine (missing fragment metadata, oversized documents, reference concentration $\ge 70\%$, unfragmented hubs).
  - `quality/score.rs`: Upgrade composite score to 5 pillars (Integrity 25%, Structure 20%, Consistency 20%, Discoverability 20%, Freshness 15%) with betweenness weighting and explainable reasons.
- `cli/`:
  - `cli/materialize_cmd.rs`: CLI subcommand `llm-wiki-core materialize <path>`.
  - `cli/trace_cmd.rs`: Update `llm-wiki-core trace` with fragment targeting and context reduction reporting.
  - `cli/issues_cmd.rs`: Update issues filtering for 5 pillars.

### VS Code Extension Host & Webview (`src/`)
- `src/types/fragment.ts`: TypeScript definitions for fragments, 5-pillar scores, debt summaries, and Context Trace responses.
- `src/utils/numberFormat.ts`: Universal `formatCompactNumber` enforcing $\le 5$ character limit across all numbers.
- `src/views/inspectorDrawer.ts`: Add collapsible "Fragments" 3-column table (`Fragment | Tok | I/O`), empty state banner (`"No heading sections found"`), progressive disclosure ordering, click-to-line navigation, and reference tooltips.
- `src/views/settingsDrawer.ts`: Add Pillar 2 (`Structure`) and Pillar 5 (`Freshness`) to Diagnostics drawer tabs and itemized issue rendering.
- `src/views/statusDashboard.ts`: Update Quality gauge and pillar breakdown to 5 dimensions.
- `src/rpc/rpcClient.ts`: Strongly typed client calls for fragment retrieval, frontmatter materialization, and 5-pillar scoring.

---

## 3. Execution Phases & Milestones

### Phase 1: Markdown AST Fragment Extraction & Anchor Normalization
- Unit tests for heading levels H1–H6, CommonMark URL-safe anchor normalization, line range mapping, duplicate anchor disambiguation (`#heading-1`).
- Implement `extractor.rs` and integrate with `tiktoken-rs` for fragment token counting.

### Phase 2: Non-Destructive Frontmatter Materialization Layer & Pre-Commit Staging
- Unit tests verifying YAML frontmatter injection and update under `wiki.fragments` while preserving existing user keys (`title`, `tags`, custom YAML).
- Implement `frontmatter.rs` and pre-commit hook integration in `candidate/validator.rs` with automatic Git index re-staging (`git add <path>`).

### Phase 3: Section Fragment Link Resolution & Pre-Commit Admission Control
- Extend `NoteEdge` with `target_fragment` and `BrokenFragmentAnchor`.
- Update link resolver to check `#anchor` against target document fragments and report line-precise Integrity regressions.
- Wire `BrokenFragmentAnchor` into `candidate/validator.rs` to enforce `BLOCK` exit code 1 during pre-commit on new dangling fragment links.

### Phase 4: Reference Concentration & Pillar 2 Structure Engine
- Implement `concentration.rs` and `structure.rs`.
- Detect oversized documents ($>50\text{k}$ tokens or $>10$ headings), high reference concentration ($C \ge 70\%$), and unfragmented central hubs.
- Integrate `Structure` as Pillar 2 (20% weight) with explainable guidance reasons.

### Phase 5: Deterministic Evidence-Based Freshness Engine (Pillar 5)
- Implement `freshness.rs` evaluating local code citations on disk, Git code changes occurring after doc commits (capped at -30 pts), deleted/renamed symbols, and canonical contradictions.
- Integrate `Freshness` as Pillar 5 (15% weight) with `Fresh`, `PotentiallyStale`, `Stale` classifications.

### Phase 6: Context Trace Section-Level Routing & Beam Search
- Implement `beam.rs` executing constrained beam search ($W=5, D=4$) from entry point (`AGENTS.md`) targeting document fragments.
- Compute and report quantitative context reduction ($\approx 90\%$) and explainable relevance justifications (with 0.0% fallback when no fragment is selected).

### Phase 7: Webview Fragment Inspector & Compact Presentation Formatting
- Implement `formatCompactNumber` with unit tests verifying $\le 5$ characters across all scales up to billions.
- Implement Fragment Inspector in `inspectorDrawer.ts` (3-column table `Fragment | Tok | I/O`, empty state, and progressive disclosure).
- Update Diagnostics drawer in `settingsDrawer.ts` with 5 pillar tabs.

### Phase 8: Benchmarking, End-to-End Verification & Documentation
- Criterion microbenchmarks for fragment parsing ($\le 50\text{ ms}$, NFR-001), incremental graph patch ($\le 50\text{ ms}$, NFR-002), normal git delta ($< 150\text{ ms}$, NFR-003), Context Trace beam search ($\le 100\text{ ms}$, NFR-005), and warm search ($\le 50\text{ ms}$, NFR-006).
- Complete test execution (`cargo test`, `npm test`), linter checks, packaging size check ($\le 25\text{ MB}$, NFR-009), and VSIX installation.
