---
wiki:
  content_hash: f9e2aa7cd416df7de715a760be94f6ed2988bd9694393310e5af23bacd202abf
  fragments:
    quickstart-knowledge-quality-admission-control-git-lifecycles:
      tokens: 927
      inbound: 0
      outbound: 0
    1-establishing-the-initial-debt-baseline:
      tokens: 140
      inbound: 0
      outbound: 0
    2-pre-commit-validation-in-action-llm-wiki-validate:
      tokens: 310
      inbound: 0
      outbound: 0
    scenario-a-clean-commit-or-legacy-fix:
      tokens: 91
      inbound: 0
      outbound: 0
    scenario-b-regression-introduced-commit-blocked:
      tokens: 189
      inbound: 0
      outbound: 0
    3-running-context-trace-via-cli:
      tokens: 216
      inbound: 0
      outbound: 0
    4-viewing-knowledge-quality-score-debt-breakdown:
      tokens: 161
      inbound: 0
      outbound: 0
    5-testing-verification:
      tokens: 57
      inbound: 0
      outbound: 0
---
# Quickstart: Knowledge Quality Admission Control & Git Lifecycles

**Feature Branch:** `003-knowledge-admission-control`  
**Target Delivery:** `llm_wiki_ext4vsc`  

---

## 1. Establishing the Initial Debt Baseline

When opening or initializing LLM Wiki on a repository for the first time, establish the baseline:

```bash
# In your repository root:
llm-wiki-core baseline init

# Output:
# Knowledge Debt Baseline established for commit abc123
#   Legacy broken references: 42
#   Legacy orphan nodes:      19
#   Legacy collisions:         8
#   Legacy technical drifts:   3
#   Total Legacy Debt:        72
# Pre-commit will not block on these 72 items.
```

The baseline is saved to *.context-vault/baseline.json*.

---

## 2. Pre-Commit Validation in Action (`llm-wiki-validate`)

When a developer stages changes and runs `git commit`:

### Scenario A: Clean Commit or Legacy Fix
```bash
git add docs/architecture/auth.md
git commit -m "fix(docs): resolve broken oauth link"

# Pre-commit hook runs:
# [llm-wiki-validate] Evaluating staged candidate graph...
# PASS: 0 new regressions. (1 legacy debt item resolved!)
# [post-commit] Synced commit def456. Updated debt baseline: 71 items.
```

### Scenario B: Regression Introduced (Commit Blocked)
```bash
git add docs/notes/new_draft.md
git commit -m "add draft with broken reference"

# Pre-commit hook runs:
# ══════════════════════════════════════════════════════════════
# LLM Wiki: Commit Blocked (1 knowledge regression detected)
# ══════════════════════════════════════════════════════════════
#
# ✕ Broken Reference: docs/notes/new_draft.md:14
#   Target: [[non_existent_note]] does not exist.
#   Remediation: Fix the wikilink target or create the note before committing.
#
# Commit aborted. Fix issues or use --no-verify if urgent.
```

---

## 3. Running Context Trace via CLI

To inspect discoverability and cumulative BPE token depth from `AGENTS.md`:

```bash
llm-wiki-core trace "OAuth token refresh flow"

# Output:
# ──────────────────────────────────────────────────────────────
# Context Trace Result
# ──────────────────────────────────────────────────────────────
# Query:       "OAuth token refresh flow"
# Entry Point: AGENTS.md
# Target Note: notes/05_secrets/oauth_refresh.md
#
# Best Discovery Path:
#   AGENTS.md (Level 0)
#     └─► notes/03_architecture/security.md (Hop 1)
#           └─► notes/05_secrets/oauth_refresh.md (Hop 2)
#
# Traversal Depth:      2 hops
# Cumulative BPE Load:  4,820 tokens (cl100k_base)
# Target Note Tokens:   1,240 tokens
# Path Quality Score:   91.4%
```

---

## 4. Viewing Knowledge Quality Score & Debt Breakdown

```bash
llm-wiki-core quality

# Output:
# ──────────────────────────────────────────────────────────────
# Knowledge Quality Score: 78 / 100
# ──────────────────────────────────────────────────────────────
# Integrity:       84 / 100 (weighted by graph centrality)
# Consistency:     76 / 100
# Discoverability: 80 / 100
# Freshness:       72 / 100
#
# Debt Accounting:
#   Legacy Debt:   71 items
#   New Debt:       0 items
#   Regressions:    0 items
#   Resolved Debt:  1 item
```

---

## 5. Testing & Verification

Run both test suites:
```bash
# Rust core tests (AST, baseline diffing, collision detection, context trace)
cargo test

# VS Code extension & webview integration tests
cd extension && npm test && npm run lint
```
