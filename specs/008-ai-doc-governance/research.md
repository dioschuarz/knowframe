---
wiki:
  content_hash: 5b65d7b8a4957dc3e4b59ce2618e389e81fa827e9b17981f0a57b00c4bbf9d1f
  fragments:
    research-architectural-decisions-ai-oriented-documentation-governance:
      tokens: 2924
      inbound: 0
      outbound: 0
    1-deterministic-harness-entry-point-discovery-multi-harness-resolution:
      tokens: 463
      inbound: 0
      outbound: 0
    context:
      tokens: 106
      inbound: 0
      outbound: 0
    decisions-rationale:
      tokens: 250
      inbound: 0
      outbound: 0
    alternatives-considered:
      tokens: 91
      inbound: 0
      outbound: 0
    2-first-class-knowledge-object-modeling-ast-section-extraction-for-harnesses:
      tokens: 486
      inbound: 0
      outbound: 0
    context-1:
      tokens: 62
      inbound: 0
      outbound: 0
    decisions-rationale-1:
      tokens: 331
      inbound: 0
      outbound: 0
    alternatives-considered-1:
      tokens: 76
      inbound: 0
      outbound: 0
    3-embedded-knowledge-index-anti-secondary-routing-invariants:
      tokens: 459
      inbound: 0
      outbound: 0
    context-2:
      tokens: 81
      inbound: 0
      outbound: 0
    decisions-rationale-2:
      tokens: 251
      inbound: 0
      outbound: 0
    alternatives-considered-2:
      tokens: 112
      inbound: 0
      outbound: 0
    4-skill-governance-procedure-vs-capability-distinctions:
      tokens: 410
      inbound: 0
      outbound: 0
    context-3:
      tokens: 64
      inbound: 0
      outbound: 0
    decisions-rationale-3:
      tokens: 267
      inbound: 0
      outbound: 0
    alternatives-considered-3:
      tokens: 65
      inbound: 0
      outbound: 0
    5-non-prescriptive-physical-directory-analysis-signal-detection:
      tokens: 314
      inbound: 0
      outbound: 0
    context-4:
      tokens: 57
      inbound: 0
      outbound: 0
    decisions-rationale-4:
      tokens: 165
      inbound: 0
      outbound: 0
    alternatives-considered-4:
      tokens: 77
      inbound: 0
      outbound: 0
    6-harness-anchored-context-trace-traversal-4-capability-scoring-engine:
      tokens: 418
      inbound: 0
      outbound: 0
    context-5:
      tokens: 53
      inbound: 0
      outbound: 0
    decisions-rationale-5:
      tokens: 299
      inbound: 0
      outbound: 0
    alternatives-considered-5:
      tokens: 44
      inbound: 0
      outbound: 0
    7-user-controlled-structural-evolution-workflow-for-ai-governance:
      tokens: 326
      inbound: 0
      outbound: 0
    context-6:
      tokens: 37
      inbound: 0
      outbound: 0
    decisions-rationale-6:
      tokens: 275
      inbound: 0
      outbound: 0
---
# Research & Architectural Decisions: AI-Oriented Documentation Governance

**Feature**: `008-ai-doc-governance`
**Date**: 2026-09-29
**Status**: Completed (Phase 0)

---

## 1. Deterministic Harness Entry Point Discovery & Multi-Harness Resolution

### Context
AI development workflows leverage specialized AI agents (e.g. Claude Code, Gemini CLI, Cursor, GitHub Copilot, OpenAI Codex) which require an authoritative root instructions or context document. Standard entry points include `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.github/copilot-instructions.md`, or custom locations specified in `.context-vault.yaml`. When `AI-Oriented` documentation mode is active, LLM Wiki must locate and evaluate these entry points deterministically.

### Decisions & Rationale
1. **Deterministic Priority & Multi-Harness Support**:
   - The engine will scan the workspace root and configured directories for known harness files:
     1. `AGENTS.md` (Standard Open AI Agent Specification / LLM Wiki default)
     2. `CLAUDE.md` (Anthropic Claude Code harness)
     3. `GEMINI.md` (Google Gemini CLI / Antigravity harness)
     4. `.github/copilot-instructions.md` (GitHub Copilot repository instructions)
     5. Custom entry points defined in `.context-vault.yaml` under `harness.entry_points: [...]`.
   - If multiple harness files exist, LLM Wiki analyzes each as a distinct, first-class knowledge object, cross-checking that routes and terminology are consistent across them.
2. **Explicit Zero-Network / Local Discovery**:
   - Discovery is strictly filesystem-based using standard path normalization in Rust without running shell subprocesses or calling remote APIs.
3. **No Automatic Presumption of Universal Format**:
   - The parser evaluates each harness according to its native conventions while extracting standard Markdown headings, wikilinks, markdown links, code blocks, and lists.

### Alternatives Considered
- *Single Fixed Entry Point (`AGENTS.md` only)*: Rejected because modern poly-agent development teams often maintain both `AGENTS.md` and `CLAUDE.md` or `.github/copilot-instructions.md`.
- *Dynamic Entry Point Sniffing via Heuristic Regex*: Rejected because Invariant III requires deterministic discovery. Scanning fixed canonical paths and configured paths is 100% reproducible.

---

## 2. First-Class Knowledge Object Modeling & AST Section Extraction for Harnesses

### Context
In many documentation tools, the root agent instructions file is treated as an external configuration file or ignored during vault scanning. In LLM Wiki, the Harness Entry Point must be a first-class citizen of the Knowledge Graph so its links, anchors, token footprint, and health metrics are fully auditable.

### Decisions & Rationale
1. **Document Node with Special Archetype**:
   - In `petgraph`, the harness file is modeled as a standard `DocumentNode`, but flagged with `is_harness: true` and archetype `NodeArchetype::Harness`.
   - It possesses full fragment metadata (`wiki.fragments`), line ranges, BPE token counts, in-degree, out-degree, and centrality scores.
2. **5-Pillar Quality & Context Efficiency Auditing**:
   - **Integrity**: Evaluates internal links (`[Title](./path.md)` or `[[wikilink]]`) and fragment anchors (`#anchor`). References to excluded paths (`scan.exclude`) or non-existent files are flagged as Integrity defects.
   - **Structure**: Evaluates section density, reference concentration, and semantic diversity.
   - **Discoverability**: Measures how many core repository domains and documents are reachable via the entry point.
   - **Consistency**: Detects duplicate routes pointing to different targets, or conflicting instructions.
   - **Freshness**: Cross-references citations against git history and filesystem presence.
   - **Context Efficiency**: Computes the ratio of useful routing tokens vs. unstructured boilerplate or duplicated code samples.
3. **Invariant XIII Compliance (Size is Signal, Not Verdict)**:
   - An `AGENTS.md` of 30,000 to 50,000 tokens is NOT penalized purely for size if its cohesion is high, its routes are well-indexed, and its context traces are short. A 5,000-token file with chaotic circular links will receive a lower health score.

### Alternatives Considered
- *Separate Harness Graph*: Rejected because Invariant V mandates a single unified document-centric graph. Isolating the harness into a parallel graph prevents holistic Context Trace traversals.
- *Strict Token Cap (e.g. max 5,000 tokens)*: Rejected because comprehensive systems with hundreds of modules legitimately require extensive architectural guidelines.

---

## 3. Embedded Knowledge Index & Anti-Secondary Routing Invariants

### Context
A recurring anti-pattern in developer documentation is the creation of intermediate "hops" or secondary routing documents, such as:
```text
AGENTS.md -> docs/index.md -> docs/architecture/system.md
or
AGENTS.md -> memory.md -> hot-index.md -> docs/...
```
These intermediate files introduce unnecessary token consumption, increase navigation hop count, and dilute agent context.

### Decisions & Rationale
1. **Directly Embedded Knowledge Index**:
   - For `AI-Oriented` documentation, the Knowledge Index MUST be an explicit section (`## Knowledge Index` or `## Architecture & Knowledge Index`) embedded directly within the Harness Entry Point.
   - Routes in the index link directly to target documents or section fragments:
     ```markdown
     ## Knowledge Index
     ### Architecture
     - System Overview → [[docs/architecture/system.md]]
     - Service Boundaries → [[docs/architecture/services.md#boundaries]]
     ```
2. **Prohibition of Secondary Routing / Memory Layers**:
   - The engine detects when a document linked from the harness acts merely as a bridge or router (high out-degree, low body content, zero domain explanations) before reaching leaves.
   - When detected, the engine flags a structural defect: `SecondaryRoutingLayerDetected`.
   - The recommendation engine proposes consolidating those routes directly into the primary Harness Entry Point and eliminating the intermediary note.
3. **Deterministic Routing Candidate Derivation**:
   - Routing candidates are derived locally from AST headings, document titles, tags, and centrality hubs using deterministic BM25 / token similarity. Zero external LLM calls.

### Alternatives Considered
- *Allowing `docs/index.md` as standard practice*: While common for human websites (like MkDocs), it creates an extra hop for AI agents that always start at `AGENTS.md`. In AI-Oriented mode, direct embedding is strictly superior for context efficiency.
- *Auto-generating a shadow `hot-index.md`*: Strictly prohibited by the prompt specification ("LLM Wiki MUST NOT create secondary routing, memory, or index artifacts"). Structural evolution must fix the entry point, not spawn shadow files.

---

## 4. Skill Governance & Procedure vs. Capability Distinctions

### Context
AI agents increasingly use "Skills" (specialized workflows or tool suites defined in `.agents/skills/<skill-name>/` or similar directories). A common source of confusion is treating a one-off task instruction as a Skill, or having a Skill with broken script references, duplicate capabilities, or missing manifests.

### Decisions & Rationale
1. **Standard Skill Entity Definition**:
   - A Skill is recognized when a directory contains a primary `SKILL.md` manifest, optionally accompanied by:
     - `scripts/`: executable helper code
     - `templates/`: boilerplates and documents
     - `references/`: domain background notes
2. **Procedure vs. Skill Distinction**:
   - **Procedure / Task**: A sequence of operational steps in a single document (e.g., "Step 1: Check logs, Step 2: Restart pod"). If executed once or tied to a single issue, it remains a task procedure.
   - **Skill Candidate**: When identical or highly overlapping procedural step sequences occur across multiple documents, or when associated automation scripts exist with recurring execution patterns, the engine detects a `ProcedureCandidate` and suggests promoting it into a reusable Skill.
3. **Skill Governance Auditing**:
   - Validates that `SKILL.md` exists and contains valid YAML frontmatter (name, description).
   - Validates that relative links to scripts (e.g. in `scripts/`), templates, and references resolve on disk.
   - Checks for duplicate skill names or high pairwise overlap in capabilities.
   - Identifies orphaned skills that have zero references from the harness or other documentation.

### Alternatives Considered
- *Treating any directory with scripts as a Skill*: Causes false positives for standard project code. Requiring `SKILL.md` ensures clear developer intent.
- *Treating Skills as regular Markdown notes*: Misses the multi-file asset bundle nature of modern skills (scripts, templates).

---

## 5. Non-Prescriptive Physical Directory Analysis & Signal Detection

### Context
Developers organize files in wildly different directory structures: flat `docs/`, nested `notes/01_projects/`, domain-driven `src/modules/**/README.md`, or date-based `logs/`. Enforcing a single directory structure violates developer sovereignty and breaks external links.

### Decisions & Rationale
1. **Non-Prescriptive Philosophy (Invariant XIII)**:
   - The system NEVER enforces a specific folder layout.
   - Instead, it measures the mathematical efficiency of the physical structure:
     - **Navigation Depth**: Maximum and average directory depth from root.
     - **Concentration Entropy**: Ratio of files clustered in a single directory vs. evenly distributed.
     - **Domain Cohesion**: Whether files in the same directory share semantic tags and link references, or are completely disparate.
     - **Cross-Folder Coupling**: Degree to which documents in directory A link exclusively to directory B.
2. **Evidence-Based Reorganization Recommendations**:
   - Directory restructuring is only recommended when both high concentration/depth and low navigational cohesion create measurable retrieval penalties during Context Traces.

### Alternatives Considered
- *Enforcing Diátaxis Framework (Tutorials, How-To, Reference, Explanation)*: While Diátaxis is a great framework, dogmatically requiring it on existing repositories creates unnecessary friction.
- *Ignoring directory structure completely*: Ignores physical context locality, which AI agents and tools rely upon when listing directories or globbing files.

---

## 6. Harness-Anchored Context Trace Traversal & 4-Capability Scoring Engine

### Context
Context Trace traverses the Knowledge Graph to find optimal paths to relevant knowledge. In `AI-Oriented` mode, traversing from arbitrary nodes does not reflect real-world agent behavior. Agents always start from the entry point (`AGENTS.md`).

### Decisions & Rationale
1. **Harness-Anchored Root**:
   - Context Trace locks the root of the traversal graph to the configured Harness Entry Point.
   - Traversal flows: $	ext{Harness Entry Point} 	o 	ext{Embedded Knowledge Index} 	o 	ext{Domain Document} 	o 	ext{Target Section Fragment}$.
2. **The 4 AI-Oriented Capabilities**:
   - **Entry ($E$)**: Accessibility, validity, and completeness of the Harness Entry Point.
   - **Discovery ($D$)**: Percentage of repository domains and key nodes mapped in the embedded Knowledge Index.
   - **Retrieval ($R$)**: Hop distance, token surface, and relevance progression from the entry point to target concepts.
   - **Reuse ($U$)**: Reusability score derived from Skill integrity, procedure consolidation, and operational asset availability.
3. **Synthesis with 5 Quality Pillars**:
   - These 4 capabilities map directly into the existing 5 Pillars:
     - Integrity $\gets$ Valid links, anchors, script paths.
     - Structure $\gets$ Direct index embedding, no secondary layers, skill directory organization.
     - Discoverability $\gets$ Unrouted knowledge penalty, trace reachability.
     - Consistency $\gets$ Non-conflicting harness instructions, terminology parity.
     - Freshness $\gets$ Up-to-date routes, git-verified code citations.

### Alternatives Considered
- *Multi-start Context Tracing*: Exploring from all hub nodes simultaneously. While useful in Human-Oriented mode, AI-Oriented mode must test entry-point navigation specifically.

---

## 7. User-Controlled Structural Evolution Workflow for AI Governance

### Context
When structural problems are identified in the harness, skills, or directory layout, LLM Wiki must not silently rewrite documents. Invariant XVI mandates explicit user selection and approval.

### Decisions & Rationale
1. **Deterministic Strategy Suite**:
   - `EmbedKnowledgeIndex`: Injects or updates `## Knowledge Index` directly in `AGENTS.md`.
   - `EliminateSecondaryLayer`: Replaces intermediate `index.md`/`memory.md` hops with direct routes in the harness.
   - `CreateSkillFromProcedure`: Consolidates repeated procedural steps into `.agents/skills/<name>/SKILL.md`.
   - `DecomposeHarness`: Breaks down an overloaded, low-cohesion harness into structured domain modules linked from the root index.
   - `IntroduceSectionAddressing`: Updates coarse document routes to pinpoint normalized fragment anchors (`#section-anchor`).
2. **Approval Gate & Re-Measurement Lifecycle**:
   - Standard 9-phase flow: Evidence $	o$ Problem $	o$ Candidate Strategies $	o$ Fit Score $	o$ Compatibility $	o$ User Selection $	o$ Diff Preview $	o$ Approval Gate $	o$ Apply $	o$ Re-Measure.
   - Requires explicit click on `Apply Changes` in the UI (`Auto-Apply: OFF`, `Require Approval: ON`).
   - Generates empirical Before-vs-After delta report post-apply ($\Delta	ext{Tokens}$, $\Delta	ext{Hops}$, $\Delta	ext{Health}$).

---
