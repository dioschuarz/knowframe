---
wiki:
  content_hash: 184618a68f4778ed6479d15d82c399c42db046ab593cf93dd0b9a9e0d243a2a4
  fragments:
    quickstart-deterministic-knowledge-engineering-engine:
      tokens: 965
      inbound: 0
      outbound: 0
    1-overview:
      tokens: 40
      inbound: 0
      outbound: 0
    2-cli-verification-scenarios:
      tokens: 665
      inbound: 0
      outbound: 0
    scenario-1-bit-level-deterministic-repository-analysis:
      tokens: 138
      inbound: 0
      outbound: 0
    scenario-2-switching-optimization-profiles-observing-relative-health:
      tokens: 156
      inbound: 0
      outbound: 0
    scenario-3-pre-commit-validation-brownfield-debt-isolation:
      tokens: 186
      inbound: 0
      outbound: 0
    scenario-4-user-controlled-structural-transformation-plan:
      tokens: 176
      inbound: 0
      outbound: 0
    3-vs-code-extension-verification:
      tokens: 220
      inbound: 0
      outbound: 0
---
# Quickstart: Deterministic Knowledge Engineering Engine

**Feature**: `009-deterministic-knowledge-engine`  
**Date**: 2026-10-06  
**Status**: Completed  

---

## 1. Overview

This guide walks through verifying the **Deterministic Knowledge Engineering Engine** using both the native CLI binary (`llm-wiki-core`) and the VS Code extension integration.

---

## 2. CLI Verification Scenarios

### Scenario 1: Bit-Level Deterministic Repository Analysis
Execute a full analysis over a Markdown repository and verify identical BLAKE3 `analysis_id` across repeated runs.

```bash
# First analytical run
llm-wiki-core analyze --workspace /path/to/repo --profile ai_oriented --format json > run1.json

# Second analytical run
llm-wiki-core analyze --workspace /path/to/repo --profile ai_oriented --format json > run2.json

# Verify bit-for-bit equivalence
diff run1.json run2.json
echo "Analysis IDs match:"
jq .analysis_id run1.json
jq .analysis_id run2.json
```

### Scenario 2: Switching Optimization Profiles & Observing Relative Health
Compare the health score of the same repository under `AI-Oriented` vs `Human-Oriented` modes.

```bash
# Evaluate in AI-Oriented mode
llm-wiki-core analyze --workspace /path/to/repo --profile ai_oriented

# Evaluate in Human-Oriented mode
llm-wiki-core analyze --workspace /path/to/repo --profile human_oriented
```
Notice how:
- `AI-Oriented` mode penalizes long unindexed paths and rewards dense section-level fragment anchors.
- `Human-Oriented` mode penalizes deep hierarchies and rewards linear reading flow.
- Raw underlying token counts and link counts remain invariant.

### Scenario 3: Pre-Commit Validation & Brownfield Debt Isolation
Simulate a Git pre-commit hook run with existing legacy debt:

```bash
# Capture brownfield baseline
llm-wiki-core baseline init --workspace /path/to/repo

# Stage a note with a broken link
echo "See [[deleted_architecture]]" >> notes/overview.md
git add notes/overview.md

# Run pre-commit validation (must block on new regression)
llm-wiki-core validate --workspace /path/to/repo # (alias: llm-wiki-core pre-commit)
# Expected exit code: 1 (Blocked: 1 new regression detected)

# After resolving regression and committing, sync persistent graph
llm-wiki-core sync --workspace /path/to/repo # (alias: llm-wiki-core post-commit)
# Graph updated incrementally in <= 50 ms
```

### Scenario 4: User-Controlled Structural Transformation Plan
Generate and inspect an evidence-based recommendation, then apply it using the explicit `approval_token`:

```bash
# Generate transformation plan for candidate strategy
llm-wiki-core plan generate --strategy split_decompose --target notes/monolith.md > plan.json

# Inspect diff preview and approval token
jq .unified_diff plan.json
TOKEN=$(jq -r .approval_token plan.json)

# Attempt execution without token (MUST FAIL)
llm-wiki-core plan apply --plan-id $(jq -r .plan_id plan.json) # Error: Missing approval_token

# Apply with explicit token
llm-wiki-core plan apply --plan-id $(jq -r .plan_id plan.json) --token "$TOKEN"

# View empirical Before/After delta report
git status
```

---

## 3. VS Code Extension Verification

1. Open the repository in Visual Studio Code.
2. Open the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`) and execute:
   `LLM Wiki: Open Knowledge Observatory & Manager`
3. Verify the 4-card UI:
   - **Health**: 5 pillars displayed with profile-relative scores and compact formatting ($\le 5$ chars, e.g. `14.2k`, `92/100`).
   - **Structure**: Force-directed 2D canvas showing document nodes ($O(N)$ nodes) with fragment metadata inspector.
   - **Evolution**: Candidate strategy cards showing Primary, Supporting, and Mutually Exclusive options with itemized benefits and countervailing trade-offs.
   - **Settings**: Interactive mode switcher (`AI-Oriented`, `Human-Oriented`, `Balanced`, `Custom`) and hard constraint toggles.
4. Click on any finding or broken link in the Health drawer to verify that the editor jumps directly to the exact file line and column.
