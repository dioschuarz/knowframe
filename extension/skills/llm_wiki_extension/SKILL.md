---
name: llm-wiki-extension
description: Operate the KnowFrame extension, its Rust core engine (llm-wiki-core), and JSON-RPC daemon from the terminal or AI agent workflows. Covers workspace scanning, 5-pillar quality diagnostics and remediation, context trace & section budgeting, frontmatter materialization, dual Git hooks & candidate validation, knowledge debt baselining, and multi-strategy method evolution (KOS strategy selection, combination, composite plan valuation, and atomic transformation application).
compatibility: Requires the llm-wiki-core binary and a workspace containing Markdown documentation.
wiki:
  content_hash: a654378be46c211052af1fc73cf509519a21b9f568663188fadc9348e5845018
  fragments:
    knowframe-extension-cli-automation-skill:
      tokens: 4838
      inbound: 0
      outbound: 0
    1-locating-the-binary-execution-syntax:
      tokens: 152
      inbound: 0
      outbound: 0
    specifying-the-workspace-root:
      tokens: 81
      inbound: 0
      outbound: 0
    2-workspace-inspection-graph-topology:
      tokens: 380
      inbound: 0
      outbound: 0
    scan-the-knowledge-graph:
      tokens: 121
      inbound: 0
      outbound: 0
    compute-5-pillar-quality-score:
      tokens: 164
      inbound: 0
      outbound: 0
    query-quality-issues-by-pillar:
      tokens: 84
      inbound: 0
      outbound: 0
    3-the-5-quality-pillars-defect-remediation-guide:
      tokens: 585
      inbound: 0
      outbound: 0
    4-ai-context-observatory-token-budgeting:
      tokens: 352
      inbound: 0
      outbound: 0
    multi-tier-context-decay-law:
      tokens: 143
      inbound: 0
      outbound: 0
    token-warning-thresholds:
      tokens: 46
      inbound: 0
      outbound: 0
    section-level-context-trace-routing:
      tokens: 122
      inbound: 0
      outbound: 0
    5-frontmatter-fragment-materialization:
      tokens: 186
      inbound: 0
      outbound: 0
    materialized-metadata-format:
      tokens: 100
      inbound: 0
      outbound: 0
    6-knowledge-organization-systems-kos-multi-strategy-evolution:
      tokens: 643
      inbound: 0
      outbound: 0
    the-4-standard-kos-strategies:
      tokens: 244
      inbound: 0
      outbound: 0
    strategy-fit-score-thresholds:
      tokens: 67
      inbound: 0
      outbound: 0
    strategy-roles:
      tokens: 56
      inbound: 0
      outbound: 0
    pairwise-strategy-relationships:
      tokens: 92
      inbound: 0
      outbound: 0
    composite-plan-value-calculation:
      tokens: 123
      inbound: 0
      outbound: 0
    7-evolution-configuration-suggestions-atomic-application:
      tokens: 452
      inbound: 0
      outbound: 0
    configure-documentation-orientation-profiles:
      tokens: 185
      inbound: 0
      outbound: 0
    custom-profile-configuration:
      tokens: 98
      inbound: 0
      outbound: 0
    review-suggestions:
      tokens: 50
      inbound: 0
      outbound: 0
    generate-a-dry-run-transformation-plan:
      tokens: 122
      inbound: 0
      outbound: 0
    apply-approved-transformation-atomic:
      tokens: 83
      inbound: 0
      outbound: 0
    8-git-admission-control-hooks-debt-baseline:
      tokens: 390
      inbound: 0
      outbound: 0
    dual-hook-architecture-prek:
      tokens: 76
      inbound: 0
      outbound: 0
    pre-commit-validation-command:
      tokens: 133
      inbound: 0
      outbound: 0
    post-commit-git-delta-notification:
      tokens: 31
      inbound: 0
      outbound: 0
    knowledge-debt-baseline-context-vaultbaselinejson:
      tokens: 136
      inbound: 0
      outbound: 0
    9-background-websocket-json-rpc-20-daemon:
      tokens: 831
      inbound: 0
      outbound: 0
    supported-json-rpc-20-methods-reference:
      tokens: 709
      inbound: 0
      outbound: 0
    10-constitutional-invariants-guardrails:
      tokens: 394
      inbound: 0
      outbound: 0
    11-autonomous-agent-workflow-recipes:
      tokens: 403
      inbound: 0
      outbound: 0
    recipe-a-full-workspace-health-audit-triage:
      tokens: 116
      inbound: 0
      outbound: 0
    recipe-b-multi-strategy-evolution-re-measurement:
      tokens: 173
      inbound: 0
      outbound: 0
    recipe-c-pre-commit-cicd-gate:
      tokens: 105
      inbound: 0
      outbound: 0
---
# KnowFrame Extension CLI & Automation Skill

Operate the KnowFrame extension directly from a terminal or within an autonomous AI agent workflow as the full headless equivalent of its VS Code user interface. All knowledge graph operations, quality audits, admission validations, and structural transformations are executed by the standalone Rust core engine (`llm-wiki-core`).

---

## 1. Locating the Binary & Execution Syntax

When developing inside the extension repository, use the bundled binary:

```bash
./extension/bin/llm-wiki-core <command>
```

When installed globally on `PATH`, invoke `llm-wiki-core` directly:

```bash
llm-wiki-core <command>
```

### Specifying the Workspace Root
Pass `--root <path>`, `--path <path>`, or provide a positional argument:

```bash
llm-wiki-core scan --root /path/to/workspace --pretty
llm-wiki-core quality /path/to/workspace --json
```

For scripts and AI agents, always pass `--json` to receive structured machine-readable responses.

---

## 2. Workspace Inspection & Graph Topology

### Scan the Knowledge Graph
Parses all Markdown documents (excluding technical ignores like `.git`, `node_modules`, `target`, `dist`), derives heading anchors, calculates Petgraph centrality metrics (in-degree, out-degree, betweenness), and detects central hubs:

```bash
llm-wiki-core scan --root . --pretty
llm-wiki-core scan --root . --hub "notes/00_meta/wiki_index.md" --pretty
```

Output includes `nodes`, `edges`, `metrics`, `unresolved_links`, `orphans`, and `recommended_hub`.

### Compute 5-Pillar Quality Score
Calculates the balanced 0–100 knowledge health score across the 5 quality pillars, topologically weighted by node betweenness centrality $(1.0 + 3.0 \cdot B(v))$:

```bash
llm-wiki-core quality --root .
llm-wiki-core quality --root . --json
```

Output JSON format:
```json
{
  "overall_score": 87,
  "pillars": {
    "integrity": 95,
    "structure": 82,
    "consistency": 90,
    "discoverability": 84,
    "freshness": 88
  },
  "legacy_debt_count": 14,
  "new_regression_count": 0
}
```

### Query Quality Issues by Pillar
Inspect itemized diagnostics across specific quality pillars:

```bash
# List all active issues
llm-wiki-core issues --root .

# Filter by a specific pillar with a limit
llm-wiki-core issues --root . --pillar structure --limit 20
llm-wiki-core issues --root . --pillar integrity --json > integrity_issues.json
```

---

## 3. The 5 Quality Pillars & Defect Remediation Guide

Every issue reported by `llm-wiki-core issues` belongs to one of five architectural pillars. AI agents should remediate issues using the following protocols:

| Pillar | Defect Types Audited | Root Cause | Agent Remediation Protocol |
| :--- | :--- | :--- | :--- |
| **`integrity`** | Broken wikilinks `[[target]]`, broken Markdown links `[text](./missing.md)`, dangling section anchors `doc.md#non-existent-heading` | Target file moved/renamed/deleted, or heading text altered without updating anchor | 1. Resolve correct relative path or updated note name.<br>2. Run `materialize` on target to verify valid `#slug` anchors.<br>3. Update referencing link target or restore missing section. |
| **`structure`** | Oversized documents (>50k tokens), missing fragment metadata on large files, reference concentration ($\ge 70\%$), unfragmented central hubs | Monolithic notes accumulating disparate responsibilities; agent prompts forced to load full docs | 1. Run `llm-wiki-core materialize <path> --stage` to cache frontmatter anchors.<br>2. Adopt `section_addressing` to direct agent queries to `#specific-anchor`.<br>3. Use `method-evolution` to generate topic decomposition or lightweight index routers. |
| **`consistency`** | Duplicate conceptual authority, collision duplicates (high lexical/term overlap $\ge 75\%$), title divergence, canonical conflict | Multiple notes authored on the same topic without cross-referencing | 1. Identify canonical candidate via Git chronology and inbound hub centrality.<br>2. Consolidate secondary content into canonical doc or reframe as specialized extension.<br>3. Replace redundant duplicate with a link or router pointer. |
| **`discoverability`** | Orphan notes (0 in-degree, 0 out-degree), isolated silos, excessive Context Trace depth (>4 hops), unlinked hubs | Notes created in leaf folders without integration into index or harness files | 1. Add bidirectional wikilinks from Level 0 hubs (`AGENTS.md`, `wiki_index.md`) or domain indexes.<br>2. Introduce a `lightweight_index` router note in the silo directory. |
| **`freshness`** | Code citations pointing to deleted/renamed symbols or files, code modified in Git after documentation commit | Documentation drifting away from underlying codebase implementation | 1. Inspect Git delta and reverse code-doc impact map.<br>2. Re-verify cited code symbols and line numbers against current disk state.<br>3. Update documentation text, API signatures, and file references to match code. |

---

## 4. AI Context Observatory & Token Budgeting

The extension enforces strict token economics to prevent context saturation in AI agents (Claude Code, Codex, Antigravity, Cline).

### Multi-Tier Context Decay Law
- **Level 0 (Active Context / Harness)**: Base prompt files (`AGENTS.md`, `CLAUDE.md`, or designated Hub). Rendered at 100% solid opacity with prominent visual radius.
- **Level 1 (First Hop)**: Directly referenced notes. Rendered at 60% opacity.
- **Level 2 (Deep Hops)**: Transitive references. Rendered at 30% opacity.
- **Level 3+ (Transitive Perimeter)**: Distant references. Rendered at 15% opacity.
- **Orphans**: Disconnected notes rendered in muted neutral grey (`#6B7280`).

### Token Warning Thresholds
- **Prompt Load Warning**: Triggered when Level 0 active context exceeds 60,000 tokens.
- **Critical Context Saturation**: Triggered when active context exceeds 100,000 tokens.

### Section-Level Context Trace Routing
Instead of loading entire 50k-token documents into an agent's prompt, use `trace` to identify the exact section-level path:

```bash
llm-wiki-core trace "authentication flow" --root .
llm-wiki-core trace "rate limiting implementation" --root . --entrypoint AGENTS.md --json
```

**Context Reduction Metric**: Targeting specific fragment anchors (e.g. `auth.md#jwt-verification`) delivers $\approx 90\%$ prompt load reduction compared to full-document ingestion, saving tokens and improving reasoning accuracy.

---

## 5. Frontmatter Fragment Materialization

Document frontmatter serves as a materialized structural metadata cache (`wiki.fragments`), derived deterministically from the Markdown AST.

```bash
# Materialize fragments for a single file
llm-wiki-core materialize docs/architecture/auth.md

# Materialize an entire directory and stage modified files in Git
llm-wiki-core materialize specs/ --stage
```

### Materialized Metadata Format
```yaml
---
wiki:
  fragments:
    - anchor: "#jwt-verification"
      heading: "JWT Verification"
      start_line: 42
      end_line: 88
      tokens: 412
      inbound: 6
      outbound: 2
---
```

**Pay at Commit, Fast at Query**: Shifting AST heading extraction and token accounting to commit time eliminates on-demand parsing overhead during interactive agent queries and context assembly.

---

## 6. Knowledge Organization Systems (KOS) & Multi-Strategy Evolution

Under Feature 005 and Feature 007, `method-evolution` analyzes structural problems (God Documents, Silos, Concentrated References) and evaluates candidates across 4 standard Knowledge Organization System (KOS) strategies.

### The 4 Standard KOS Strategies
1. **`section_addressing`**:
   - Preserves document integrity while injecting addressable fragment anchors (`#section-name`) into frontmatter and internal links.
   - Ideal for large, highly cohesive documents ($5\text{k}\text{--}50\text{k}$ tokens) that agents need to access selectively.
2. **`faceted_classification`**:
   - Classifies document sections across orthogonal facets (e.g., audience, architectural tier, lifecycle phase) using structured tags and frontmatter metadata.
   - Ideal for multifaceted documents containing mixed domain knowledge.
3. **`lightweight_index`**:
   - Creates or enriches a directory router (`index.md`) that links to underlying notes with fragment targets and summaries.
   - Ideal for isolated clusters, folder silos, or directories with high internal coupling but poor external discoverability.
4. **`preserve_document`**:
   - Explicitly preserves a monolithic document without splitting or restructuring.
   - Enforces **Invariant XIII (Non-Prescriptive Knowledge Organization)**: A 50k-token document with high cohesion is a healthy monolith, not an automatic defect.

### Strategy Fit Score & Thresholds
- **Fit Score Scale**: Integer $0\text{--}100$, measuring how effectively a strategy resolves the observed structural problem.
- **Eligibility Threshold**: $\text{Fit} \ge 60$. Strategies scoring below 60 are considered unsuitable and excluded from primary recommendations.

### Strategy Roles
- **`PRIMARY`**: The dominant organizational pattern solving the primary structural defect.
- **`SUPPORTING`**: Complementary strategies combined to address secondary aspects.
- **`ALTERNATIVE`**: Viable competing strategy presented as a mutually exclusive option.

### Pairwise Strategy Relationships
- **`COMPLEMENTARY`**: Strategies that synergize when applied together (e.g. `section_addressing` + `lightweight_index` + `faceted_classification`).
- **`ALTERNATIVE`**: Competing approaches that address the same problem from different angles.
- **`MUTUALLY_EXCLUSIVE`**: Strictly incompatible strategies (e.g. `preserve_document` combined with any splitting strategy).

### Composite Plan Value Calculation
When combining multiple strategies, the engine calculates the net value:

$$\text{Net Value} = \text{Base Fit Sum} + \text{Synergy Bonus} - \text{Overlap Penalty} - \text{Transformation Cost}$$

- **Base Fit Sum**: Sum of individual strategy fit scores.
- **Synergy Bonus**: $+15$ for each compatible complementary pair.
- **Overlap Penalty**: $-25$ if strategies have redundant action scopes.
- **Transformation Cost**: Operational cost based on file edits, link rewrites, and file creations.

---

## 7. Evolution Configuration, Suggestions & Atomic Application

### Configure Documentation Orientation Profiles
The engine compiles all configurations into a single inspectable `OptimizationProfile` (Invariant XIV):

```bash
# Inspect current active profile
llm-wiki-core method-evolution profile --root . --json

# Switch to a preset: ai-oriented (default), human-oriented, balanced, custom
llm-wiki-core method-evolution profile --root . --preset balanced
```

#### Custom Profile Configuration
Set Tier 1 objectives (`low`, `medium`, `high`) or Tier 2 engine parameters:

```bash
llm-wiki-core method-evolution profile --root . \
  --preset custom \
  --set context_efficiency=high \
  --set ai_discoverability=high \
  --set fragment_size_target=5000 \
  --set max_trace_depth=4 \
  --set split_sensitivity=0.65
```

### Review Suggestions
List ranked method evolution opportunities detected in the workspace:

```bash
llm-wiki-core method-evolution suggestions --root .
llm-wiki-core method-evolution suggestions --root . --limit 10 --json
```

### Generate a Dry-Run Transformation Plan
Generate a dry-run plan using the recommendation ID:

```bash
llm-wiki-core method-evolution plan <recommendation-id> --root . --json > plan.json
```

The output plan contains:
- `plan_id` & `opportunity_id`
- `actions`: Exact file modifications, frontmatter injections, or index creations.
- `diff_preview`: Unified diff showing all proposed changes.
- `projected_delta`: Predicted changes in context efficiency, trace depth, and quality score.
- `requires_approval`: Boolean enforcing Invariant XVI.

### Apply Approved Transformation (Atomic)
Applying requires explicit user approval. Never apply transformations without prior plan inspection:

```bash
llm-wiki-core method-evolution apply <recommendation-id> --root . --approve
```

Successful application:
1. Atomically executes all file writes and link updates.
2. Performs immediate Before-vs-After re-measurement.
3. Reports `Applied and validated`.

---

## 8. Git Admission Control, Hooks & Debt Baseline

### Dual Hook Architecture (`prek`)
- **Hook 1 (`llm-wiki-validate`, stage: `pre-commit`)**: Evaluates staged files against an in-memory ephemeral candidate graph.
- **Hook 2 (`llm-wiki-sync`, stage: `post-commit`)**: Incremental graph synchronization ($\le 50\text{ ms}$).

### Pre-Commit Validation Command
```bash
llm-wiki-core validate --root . --staged-only --json
```

Output decisions:
- `PASS`: No regressions detected.
- `WARN`: Ambiguous or moderate potential issues (e.g. moderate similarity $\ge 75\%$).
- `BLOCK`: High-confidence regressions (new broken links, invalid anchors, canonical duplicate collisions).

**Non-Blocking Fail-Open**: All hook invocations enforce a hard $\le 500\text{ ms}$ timeout. If the daemon is unreachable or errors occur, the command exits with code 0 to never impede developer workflow.

### Post-Commit Git Delta Notification
```bash
llm-wiki-core git-delta --root . --quiet <changed-file>...
```

### Knowledge Debt Baseline (*.context-vault/baseline.json*)
Allows legacy codebases to adopt KnowFrame without being blocked by pre-existing defects:

```bash
# Capture current workspace issues into baseline
llm-wiki-core baseline init --root .

# Force overwrite existing baseline
llm-wiki-core baseline init --root . --force

# Reset baseline to empty
llm-wiki-core baseline reset --root .
```

Commit blocking evaluates only **New Regressions** ($\text{Regressions} = \text{Candidate} \setminus \text{Baseline}$). Pre-existing brownfield debt is reported but never blocks commits (Invariant XI).

---

## 9. Background WebSocket JSON-RPC 2.0 Daemon

The engine can run as a persistent background daemon communicating over WebSocket JSON-RPC 2.0:

```bash
llm-wiki-core daemon --root . --port 0
```

- When passed `--port 0`, the OS assigns an ephemeral port.
- The daemon records its active port to `.git/llm-wiki.port` (with fallback to 8989).
- Loopback write notifications on `127.0.0.1` (`git/commitDelta`) are accepted without authentication.

### Supported JSON-RPC 2.0 Methods Reference

| Method Name | Category | Description |
| :--- | :--- | :--- |
| `workspace/scan` | Topology | Headless scan returning full nodes, edges, and centrality metrics. |
| `workspace/getQualityScore` | Quality | Evaluates overall and pillar health scores. |
| `workspace/getIssues` | Quality | Returns categorized issue cards filterable by pillar. |
| `workspace/getBaseline` | Debt | Retrieves the current debt baseline. |
| `workspace/updateBaseline` | Debt | Updates or resets baseline findings. |
| `workspace/traceContext` | Tracing | Computes beam-search context traversal path for a query. |
| `workspace/validateCandidate`| Admission | Evaluates staged Git deltas in the candidate graph. |
| `graph/setHub` | Graph | Sets the active Level 0 Hub document. |
| `graph/setCentralHub` | Graph | Persists user-confirmed central hub. |
| `graph/previewExclusion` | Graph | Previews nodes/edges removed by an exclusion glob. |
| `graph/excludePath` | Graph | Applies a curated knowledge exclusion (`graph.exclude`). |
| `graph/suggestExclusions` | Graph | Recommends noisy file clusters for exclusion. |
| `graph/getDocumentFragments`| Fragments | Returns AST-derived fragment metadata for a file. |
| `graph/materializeFrontmatter`| Fragments | Writes `wiki.fragments` into YAML frontmatter. |
| `trace/contextTraceSection` | Tracing | Returns section-level context trace with token reduction stats. |
| `quality/getFivePillarScore` | Quality | Retrieves full 5-pillar breakdown with topological weights. |
| `manager/getSummary` | Management | Returns consolidated workspace health, debt, and recommendations. |
| `profile/getOrientation` | Profiles | Retrieves active orientation preset and Tier 1 objectives. |
| `profile/setOrientation` | Profiles | Updates orientation preset (`ai-oriented`, `human-oriented`, etc.). |
| `profile/getCustomConfig` | Profiles | Retrieves granular Tier 2 configuration parameters. |
| `profile/updateCustomConfig` | Profiles | Updates Tier 2 parameters. |
| `profile/validateConstraints`| Profiles | Detects contradictory constraints. |
| `analytics/getStructuralProblems`| Analytics| Detects God Docs, silos, and reference concentrations. |
| `recommendation/getRecommendations`| Evolution | Returns ranked structural evolution recommendations. |
| `evolution/getOpportunities`| Evolution | Returns active structural improvement opportunities. |
| `evolution/getOpportunityDetail`| Evolution | Returns detailed problem metrics and signatures for an opportunity. |
| `evolution/getStrategySet` | Multi-Strategy | Returns candidate strategies, fit scores ($\ge 60$), roles, and plan value. |
| `evolution/validateStrategySelection`| Multi-Strategy | Validates pairwise compatibility across selected strategies. |
| `evolution/calculatePlanValue`| Multi-Strategy | Computes composite plan value and synergy/penalty metrics. |
| `evolution/generatePlan` | Multi-Strategy | Generates multi-strategy transformation plan and diff preview. |
| `evolution/applyTransformation`| Evolution | Atomically executes transformation with approval gate. |
| `git/commitDelta` | Git Lifecycle | Loopback notification recording committed Git delta. |

---

## 10. Constitutional Invariants & Guardrails

All automated operations and agent actions MUST strictly adhere to the 16 Critical Invariants (`CONSTITUTION.md`):

1. **Invariant I (Committed Truth)**: Working-tree edits or file saves do NOT update the persistent graph (`file saved ≠ graph update`).
2. **Invariant II (Candidate Isolation)**: Pre-commit analysis runs in an ephemeral in-memory candidate graph that is discarded after validation.
3. **Invariant IV (Local Determinism / Zero External AI)**: All AST parsing, token counting, centrality scoring, and collision detection run locally without external cloud APIs.
4. **Invariant V (Document-Centric Graph)**: Only Markdown files are nodes ($O(N)$ layout). Sections/fragments are metadata, never graph nodes.
5. **Invariant VII & VIII (AST Authority & Materialized Metadata)**: The AST is the source of truth. Frontmatter caches AST structure at commit time.
6. **Invariant IX & X (Numeric Integrity & Presentation Separation)**: Internal models and contracts use raw integers. Shortened strings (e.g. `50.2k`) exist only in presentation.
7. **Invariant XI & XII (Brownfield Tolerance & Quality Gate)**: Legacy debt never blocks commits. Only new regressions block commits.
8. **Invariant XIII (Non-Prescriptive Knowledge Organization)**: Size is a signal, NOT a verdict. Healthy monoliths must not be broken arbitrarily.
9. **Invariant XIV (Unified Optimization Profile)**: All orientation modes compile to a single internal `OptimizationProfile`.
10. **Invariant XV (Transparent Trade-Offs)**: Every recommendation must present expected benefits AND countervailing trade-offs.
11. **Invariant XVI (User-Controlled Automation & Re-Measurement)**: Transformations require explicit user approval (`--approve`). Applied transformations must be validated through Before-vs-After re-measurement.

---

## 11. Autonomous Agent Workflow Recipes

### Recipe A: Full Workspace Health Audit & Triage
```bash
# 1. Scan the repository and record topology
llm-wiki-core scan --root . --pretty > /tmp/topology.json

# 2. Audit 5-pillar health score
llm-wiki-core quality --root . --json > /tmp/quality.json

# 3. Retrieve itemized issues for the lowest scoring pillar
llm-wiki-core issues --root . --pillar integrity --limit 50 --json > /tmp/integrity_issues.json
```

### Recipe B: Multi-Strategy Evolution & Re-Measurement
```bash
# 1. Fetch available improvement opportunities
llm-wiki-core method-evolution suggestions --root . --limit 5 --json > /tmp/suggestions.json

# 2. Generate dry-run transformation plan for target recommendation
llm-wiki-core method-evolution plan <rec-id> --root . --json > /tmp/plan.json

# 3. Verify the diff preview and ensure no hard constraints are violated
cat /tmp/plan.json | jq '.diff_preview'

# 4. Apply transformation atomically with explicit approval
llm-wiki-core method-evolution apply <rec-id> --root . --approve

# 5. Verify post-transformation quality score
llm-wiki-core quality --root . --json
```

### Recipe C: Pre-Commit CI/CD Gate
```bash
# Run candidate validation on staged files
llm-wiki-core validate --root . --staged-only --json > /tmp/validation.json

# Exit non-zero only if validation blocked
if jq -e '.decision == "block"' /tmp/validation.json > /dev/null; then
  echo "Commit blocked by KnowFrame quality gate:"
  jq '.regressions' /tmp/validation.json
  exit 1
fi
```
