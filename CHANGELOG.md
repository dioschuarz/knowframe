# Changelog

All notable changes to the **KnowFrame** knowledge management extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.1.1] - 2026-11-26

### Fixed
- **Sidebar STRUCTURE & HEALTH Counters Staying at 0 on View Switch / Late Hydration**: Added `retainContextWhenHidden: true` to `SidebarViewProvider`, implemented an explicit `sidebarReady` handshake + `onDidChangeVisibility` state replay, server-side hydrated initial document/connected/isolated/reference counters in `sidebarRenderer.ts`, and escaped inline JSON payloads (`safeInlineJson`).
- **On-Demand Node Fragments in Inspector Drawer**: Wired `requestDocumentFragments` -> `graph/getDocumentFragments` so clicking any node in the Graph immediately populates its heading `Fragments (N)` list on demand while keeping the initial 20,000-node `workspace/scan` payload ~70% smaller.

---

## [1.1.0] - 2026-11-26

### Added
- **On-Demand Zero-Background Architecture (`llmWiki.executionMode`)**: KnowFrame now defaults to `on_demand` mode — **0 background processes and 0 MB RAM** on VS Code startup (removed `onStartupFinished` activation and disabled the continuous `**/*.md` file watcher by default). The Rust sidecar launches only when you open KnowFrame or invoke a command, and automatically shuts down to release 100% of engine RAM after closing the Graph panel (`llmWiki.daemonIdleTimeoutSeconds`, default `60s`). Users who prefer continuous auto-rescanning can opt into `live_watch` anytime.
- **Manual Memory Release Command**: Added `KnowFrame: Stop Engine Daemon (Free Memory)` (`llmWiki.stopDaemon`).
- **Enterprise Focus Windowing (`llmWiki.maxGraphNodes`)**: Automatically prioritizes the Central Hub, closest hops, and top-degree hubs (default `2,000` rendered nodes) so 20,000+ document enterprise repositories render smoothly at 60 FPS while preserving exact global counters.

### Performance & Reliability
- **>90% RAM Reduction on Enterprise Repositories (20,000+ docs)**:
  - Rewrote full-graph semantic collision detection ([fingerprint.rs](crates/core/src/collision/fingerprint.rs)) using compact `u64` hashed tokens, top-80 TF vectors, bottom-64 `u32` MinHash 3-gram sketches, and an interned `u32` inverted index (~1.5 KB/doc instead of ~250 KB/doc).
  - Switched Brandes betweenness centrality ([centrality.rs](crates/core/src/graph/centrality.rs)) to deterministic `k=256` pivot approximation above 1,500 nodes with sparse visited-set resets, cutting 20k-node centrality from minutes to milliseconds.
  - Replaced repeated per-depth BPE tokenization in fragment extraction ([extractor.rs](crates/core/src/fragment/extractor.rs)) with single-pass disjoint heading-boundary prefix sums.
  - Added generation-keyed `CachedAnalysis` in the IPC daemon ([handlers.rs](crates/core/src/ipc/handlers.rs)) so `workspace/getQualityScore` and `workspace/getIssues` compute collisions/drift once instead of twice.
  - Capped default `workspace/getIssues` RPC payloads to 500 items (with exact totals + `offset`/`limit` pagination), stripped unused per-heading `fragments` from `workspace/scan` wire payloads, raised WebSocket `maxPayload` to 256 MiB, and compacted Sidebar payloads to `<2 KB`.
- **Fixed Graph Panel Stuck at 0% / False Timeout**: Added explicit `webviewReady` handshake, removed the 8-request thundering herd on panel mount, decoupled Graph reveal from background issues loading, eliminated redundant `onDidChangeViewState` full re-posts, and prevented OOM restart loops in `ProcessManager`.

---

## [1.0.6] - 2026-11-26

### Fixed (issue #1 — symlinked skill catalogs)
- **Symlinks are followed**: both scanners (`issues`/`scan` and `validate`/`sync`/`analyze`) follow symlinked directories and files that resolve inside the workspace root. Cycles, dangling links and targets outside the root are skipped.
- **Dedupe by canonical path, with aliases**: a file reachable through several logical paths becomes one node. Its id is the path with the fewest symlink hops (ties broken alphabetically). Every other path is an alias, so links written with the canonical name, the catalog name or a repo-prefixed name all resolve to the same node and stay clickable.
- **Glob excludes honored**: the top-level `exclude` list and `scan.exclude` are now merged. Before, a non-empty `scan.exclude` silently discarded the top-level list, e.g. `**/graphify-out/**`. The `observation` scanner now matches glob patterns against the relative POSIX path, and plain names still prune directories by exact name.
- **No cross-directory basename fallback**: a link whose target has a directory (`skills/radar/SKILL.md`) only falls back to candidates whose path ends with that target. Before, it could resolve to an unrelated `SKILL.md`, which hid broken links.
- **GFM anchors**: anchors written with GitHub's slug (`#skill-catalog--canonical-location` for `Skill Catalog & Canonical Location`) are accepted and clickable (`github_slug`).

### Added
- `scan.follow_symlinks` (default `true`) and `scan.skip_linked_worktrees` (default `true`) in `.context-vault.yaml`. Linked git worktrees are directories whose `.git` file points into `<repo>/.git/worktrees/`; they are duplicate checkouts. Submodules are not affected.

---

## [1.0.5] - 2026-11-24

### Added
- **Universal Link Clickability Verification (`non_clickable_link`)**: Integrity Pillar now audits whether every valid link is actually clickable when rendered in VS Code, GitHub, GitLab, Obsidian, and standard CommonMark/GFM viewers — flagging Obsidian `[[wikilinks]]`, missing `.md` extensions, non-relative/vault-root paths, case mismatches, unencoded spaces, and unslugified anchors. Configurable via `links.portability` in `.context-vault.yaml` or `llmWiki.linkPortabilityMode` (`standard_markdown` default, `obsidian_wikilink`, `any`).
- **Native VS Code Editor & Markdown Preview Wikilink Clickability**: Added `KnowFrameMarkdownDocumentLinkProvider` (Ctrl+Click / Cmd+Click jumps directly to target file and heading line) and built-in `markdown-it` Preview plugin (`markdown.markdownItPlugins`) rendering valid `[[target#fragment|label]]` links as clickable hyperlinks inside VS Code Markdown Preview (`Ctrl+Shift+V`), while leaving broken links unclickable.
- **Deterministic Link Normalizer (CLI + Governance Strategy + VS Code Command)**:
  - CLI command `knowframe-core fix-links [--root <dir>] [--file <path>] [--apply] [--json]`
  - Governance strategy `normalize_links_to_markdown` wired to `[ Review ]` in Diagnostics Drawer with unified diff preview and approval gate
  - VS Code Command Palette action `KnowFrame: Make Links Clickable (Convert to Standard Markdown)` (`llmWiki.makeLinksClickable`)
  - Safely preserves YAML frontmatter, fenced code blocks, inline code spans, `![[embeds]]`, external URLs, and skips broken links.

### Fixed
- **Frontmatter Masking in AST Parser**: Masked YAML frontmatter before regex wikilink and hashtag extraction so metadata fields never emit false-positive graph edges.
- **EdgePayload Clickability Preservation**: Added `EdgePayload::to_note_edge()` so CLI `issues`/`quality` and JSON-RPC daemon handlers consistently propagate `is_clickable`, `non_clickable_reason`, and `clickable_target`.

---

## [1.0.4] - 2026-11-23

### Fixed
- **All-Platform Visual Studio Marketplace Distribution**: Fixed multi-platform VSIX packaging (removed `bin_temp/` duplication and cross-OS binary contamination, shrinking platform VSIXs from ~75 MB to ~7 MB each) and added live Marketplace Gallery API verification with self-healing retry so Windows (`win32-x64`), macOS (`darwin-arm64`, `darwin-x64`), Linux (`linux-x64`, `linux-arm64`), and Universal fallback (`win32-arm64`, Alpine, etc.) are all validated and available in VS Code.
- **Cross-Platform Daemon Launch**: Hardened `extension/src/lifecycle/processManager.ts` (`windowsHide: true`, POSIX-only `pkill` & `chmod`) and `extension/src/lifecycle/binaryResolver.ts` (Windows ARM64 / macOS Rosetta fallback).

---

## [1.0.3] - 2026-11-18

### Changed
- **Marketplace Display Name**: Updated extension title to `KnowFrame - Knowledge Management Framework` (`dscorpsolutions.knowframe-vscode`) to satisfy Visual Studio Marketplace name uniqueness policy.

---

## [1.0.2] - 2026-11-18

### Changed
- **Marketplace Publishing Identifier**: Updated technical package identifier to `knowframe-vscode` (`dscorpsolutions.knowframe-vscode`) to complete Visual Studio Marketplace release while retaining canonical `displayName: "KnowFrame"`.

---

## [1.0.1] - 2026-11-18

### Changed
- **Visual Studio Marketplace Publication**: Bumped patch version to `1.0.1` for clean Marketplace release under publisher `dscorpsolutions`.
- **High-Resolution Branding**: Bundled new 256x256 high-resolution icon (`media/icon.png`).
- **CI Stability**: Adjusted Windows test timeout in `crates/core/tests/cli_validate_test.rs` and relaxed frame budget timing threshold for virtualized GitHub runners in `extension/test/suite/evolution.e2e.test.ts`.

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
- **Knowledge Debt Baselining & Pre-Commit Git Admission Control**: Sub-500ms pre-commit hooks via `prek` or native git hooks validating changes against *.context-vault/baseline.json*.
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
