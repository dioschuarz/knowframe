# Changelog

All notable changes to the **KnowFrame** knowledge management extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.1] - 2026-11-18

### Changed
- **Visual Studio Marketplace Publication**: Bumped patch version to `1.0.1` for clean Marketplace release under publisher `dscorpsolutions`.
- **High-Resolution Branding**: Bundled new 256x256 high-resolution icon (`media/icon.png`).
- **CI Stability**: Adjusted Windows test timeout in `cli_validate_test.rs` and relaxed frame budget timing threshold for virtualized GitHub runners in `evolution.e2e.test.ts`.

---

## [1.0.0] - 2026-11-17

### Added
- **Production 1.0.0 Milestone Release** published under publisher `dscorpsolutions`.
- **Dynamic Interactive Knowledge Graph**: High-performance D3 Force-Directed physics simulation with radial hub pinning, multi-hop neighborhood filters, and customizable cluster opacity styling.
- **Deterministic 5-Pillar Quality Scoring Engine**: Sub-second Rust mathematical audit without LLM hallucinations:
  - *Integrity* (broken wikilinks, circular paths, unanchored section references)
  - *Structure* (God documents >3k tokens, taxonomy cycles, reference concentration, harness drift)
  - *Consistency* (homograph term collisions, synonym ring drift, schema validation)
  - *Discoverability* (isolated silos, unindexed leaves, bridge cut vertices)
  - *Traceability & Freshness* (Git commit decay, review obligations, AST code dependencies)
- **Active Prompt Load & BPE Token Observatory**: Exact byte-pair encoding (cl100k_base) token calculations and live warning thresholds (60k/100k tokens) to prevent AI context window saturation.
- **Knowledge Debt Baselining & Pre-Commit Git Admission Control**: Sub-500ms pre-commit hooks via `prek` or native git hooks validating changes against `.context-vault/baseline.json`.
- **18 KOS Structural Evolution Strategies**: Automated transformation recommendations (Topic Decomposition, Section Addressing, Faceted Classification, Thesaurus Relational Reciprocity, Lightweight Routing, Metadata Materialization) with composite plan valuation and unified diff preview.
- **Unified Split-Button Action Architecture**:
  - Deterministic fast-path `[ Review | ▾ ]` applying algorithmic repairs with zero LLM tokens.
  - Dropdown `✨ Review with AI` delegating complex semantic reasoning to configured AI coding agents (Claude Code, Antigravity, Codex, Cursor).
  - Direct file navigation clicking `Source` and `Target` paths with concise 3-word tooltips.
- **Headless CLI Tools**: Standalone `knowframe-core` daemon/auditor and `knowframe-cli` query driver for CI/CD gates.

### Changed
- Rebranded extension identifier to `dscorpsolutions.knowframe`.
- Upgraded workspace crate versions to `1.0.0`.
- Streamlined Health and Evolution drawers into unified single-action components.
- Auto-flipping dropdown popouts with dynamic z-index layering over cards and dividers.
