---
wiki:
  content_hash: 4a3357e44d5d718ae679bb8beaf80c0d019f7ab00fd12ed652bcfd6b4f413db3
  fragments:
    quickstart-validation-guide-deterministic-traceability-review-knowledge-consistency:
      tokens: 1147
      inbound: 0
      outbound: 2
    1-overview-prerequisites:
      tokens: 123
      inbound: 0
      outbound: 0
    prerequisites:
      tokens: 32
      inbound: 0
      outbound: 0
    2-scenario-1-reverse-code-impact-analysis-via-cli:
      tokens: 239
      inbound: 0
      outbound: 0
    3-scenario-2-prci-review-resolution-reviewed-unchanged:
      tokens: 227
      inbound: 0
      outbound: 0
    4-scenario-3-path-traversal-protection:
      tokens: 170
      inbound: 0
      outbound: 0
    5-scenario-4-heading-mutation-cross-file-broken-anchors:
      tokens: 213
      inbound: 0
      outbound: 0
    6-scenario-5-running-the-isolated-graphify-spike:
      tokens: 109
      inbound: 0
      outbound: 0
---
# Quickstart Validation Guide: Deterministic Traceability, Review & Knowledge Consistency

**Feature Branch**: `010-deterministic-traceability-review`  
**Date**: 2026-10-13  
**Spec**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)  

---

## 1. Overview & Prerequisites

This guide provides end-to-end reproducible scenarios demonstrating:
1. Reverse code impact analysis from frontmatter `code_dependencies` and inline citations;
2. PR/CI review obligation lifecycle (`pending` vs `reviewed_unchanged`);
3. Path containment validation (`PATH_TRAVERSAL_PROHIBITED`);
4. Heading rename integrity and broken anchor detection;
5. Isolated Graphify spike benchmark execution.

### Prerequisites
- Rust 1.78+ (`cargo`, `rustc`)
- Node.js 20+ and `npm`
- Git

---

## 2. Scenario 1: Reverse Code Impact Analysis via CLI

Verify that modifying a code file identifies dependent documentation notes:

```bash
# 1. Inspect fixture note with code_dependencies
cat << 'EOF' > notes/sample_arch.md
---
title: Sample Architecture
code_dependencies:
  - crates/core/src/model/mod.rs
---
# Sample Architecture Note
Documents core model entities.
EOF

# 2. Run impact analysis against modified file
./target/release/llm-wiki-core trace-impact --paths "crates/core/src/model/mod.rs" --json

# Expected Output:
# {
#   "changed_code_files": ["crates/core/src/model/mod.rs"],
#   "obligations": [
#     {
#       "doc_id": "notes/sample_arch.md",
#       "status": "pending",
#       "triggering_code_files": ["crates/core/src/model/mod.rs"],
#       "dependency_sources": ["frontmatter"]
#     }
#   ],
#   "pending_count": 1,
#   "reviewed_count": 0,
#   "blocked_count": 0
# }
```

---

## 3. Scenario 2: PR/CI Review Resolution (`reviewed_unchanged`)

Verify that a review receipt transitions status from `pending` to `reviewed_unchanged` without editing document content:

```bash
# 1. Register review receipt via CLI
./target/release/llm-wiki-core review resolve "notes/sample_arch.md"

# 2. Re-run impact analysis
./target/release/llm-wiki-core trace-impact --paths "crates/core/src/model/mod.rs" --json

# Expected Output:
# {
#   "obligations": [
#     {
#       "doc_id": "notes/sample_arch.md",
#       "status": "reviewed_unchanged",
#       "triggering_code_files": ["crates/core/src/model/mod.rs"],
#       "dependency_sources": ["frontmatter"],
#       "resolution_receipt": "receipt_..."
#     }
#   ],
#   "pending_count": 0,
#   "reviewed_count": 1,
#   "blocked_count": 0
# }
```

---

## 4. Scenario 3: Path Traversal Protection

Verify that relative path escapes outside the repository root are strictly rejected:

```bash
cat << 'EOF' > notes/malicious.md
---
title: Malicious
code_dependencies:
  - ../../etc/passwd
---
EOF

./target/release/llm-wiki-core validate --paths "notes/malicious.md" --json

# Expected Output:
# Findings contains:
# {
#   "rule_id": "PATH_TRAVERSAL_PROHIBITED",
#   "severity": "error",
#   "subject": "notes/malicious.md",
#   "evidence": { "changed_paths": ["../../etc/passwd"] },
#   "required_action": "Ensure code dependencies are relative paths within the workspace root"
# }
```

---

## 5. Scenario 4: Heading Mutation & Cross-File Broken Anchors

Verify that altering a heading emits `BROKEN_FRAGMENT_ANCHOR` on referencing documents:

```bash
# Note A references Section in Note B
cat << 'EOF' > notes/note_a.md
See [Topology Details](notes/note_b.md#system-topology).
EOF

# Note B renamed heading
cat << 'EOF' > notes/note_b.md
# Note B
## Architecture Overview
(Heading changed from System Topology)
EOF

./target/release/llm-wiki-core validate --paths "notes/note_b.md" --json

# Expected Output:
# Findings contains:
# {
#   "rule_id": "BROKEN_FRAGMENT_ANCHOR",
#   "severity": "warning",
#   "subject": "notes/note_a.md",
#   "evidence": { "anchor": "system-topology", "target": "notes/note_b.md" }
# }
```

---

## 6. Scenario 5: Running the Isolated Graphify Spike

Execute the isolated benchmark harness to compare Graphify against canonical graph truth:

```bash
cargo run --example graphify_spike -- --fixture tests/fixtures/sample_repo

# Expected Output:
# Graphify evaluation completed.
# Comparison report written to target/graphify_evaluation_report.md:
# - Precision: 87.4%
# - Recall: 92.1%
# - Memory RSS: 14.2 MB
# - Zero canonical graph mutations confirmed.
```
