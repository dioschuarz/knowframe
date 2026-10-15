---
wiki:
  content_hash: e15ce8754a9b9b5437c22f07cab7274012e24ae7199a17f1aa3a228ff1c73d61
  fragments:
    quickstart-validation-guide-ai-oriented-documentation-governance:
      tokens: 1170
      inbound: 0
      outbound: 0
    1-overview-prerequisites:
      tokens: 118
      inbound: 0
      outbound: 0
    prerequisites:
      tokens: 58
      inbound: 0
      outbound: 0
    2-validation-scenarios:
      tokens: 1005
      inbound: 0
      outbound: 0
    scenario-1-deterministic-harness-entry-point-knowledge-index-discovery:
      tokens: 233
      inbound: 0
      outbound: 0
    scenario-2-intermediate-routing-layer-detection-consolidation-warning:
      tokens: 207
      inbound: 0
      outbound: 0
    scenario-3-skill-governance-procedure-detection:
      tokens: 223
      inbound: 0
      outbound: 0
    scenario-4-harness-anchored-context-trace-4-capability-audit:
      tokens: 198
      inbound: 0
      outbound: 0
    scenario-5-user-approved-structural-transformation-re-measurement:
      tokens: 136
      inbound: 0
      outbound: 0
---
# Quickstart & Validation Guide: AI-Oriented Documentation Governance

**Feature**: `008-ai-doc-governance`
**Date**: 2026-09-29
**Status**: Completed (Phase 1)

---

## 1. Overview & Prerequisites

This guide provides runnable scenarios to validate the AI-Oriented Documentation Governance engine end-to-end, testing Harness Entry Point discovery, embedded Knowledge Index validation, intermediate routing layer detection, Skill governance, harness-anchored Context Trace, and user-approved structural evolution.

### Prerequisites
- Rust 1.75+ toolchain (`cargo`, `rustc`).
- Node.js 20+ and npm (`npm test` in `extension/`).
- Workspace root `/home/ds/projects/llm_wiki_ext4vsc`.

---

## 2. Validation Scenarios

### Scenario 1: Deterministic Harness Entry Point & Knowledge Index Discovery

**Objective**: Verify that `AGENTS.md` is detected as the primary harness entry point, its embedded index routes are indexed without secondary hops, and routes resolve to valid documents/sections.

**Setup**:
```bash
cd /home/ds/projects/llm_wiki_ext4vsc
# Ensure .context-vault.yaml sets orientation: ai_oriented
cat << EOF > test_vault/AGENTS.md
# AI Agent Instructions

## Knowledge Index
### Architecture
- System Overview → [[docs/architecture.md]]
- Security Boundary → [[docs/security.md#auth-tokens]]
EOF
```

**Run & Verification**:
```bash
cargo test --test harness_governance_test test_detect_agents_md_with_embedded_index
```
**Expected Outcome**:
- `HarnessEntryPoint.file_path` == `"AGENTS.md"`
- `HarnessEntryPoint.harness_type` == `HarnessType::AgentsMd`
- `EmbeddedIndexMetadata.routes.len()` == 2
- Both routes resolve with status `Resolved`
- Zero secondary routing layer warnings.

---

### Scenario 2: Intermediate Routing Layer Detection & Consolidation Warning

**Objective**: Verify that an intermediate routing layer (`AGENTS.md` -> `docs/index.md` -> docs) is flagged as a structural defect with a consolidation recommendation.

**Setup**:
```bash
cat << EOF > test_vault/AGENTS.md
# AI Agent Instructions
For knowledge navigation, consult [[docs/index.md]].
EOF

cat << EOF > test_vault/docs/index.md
# Master Index
- Architecture → [[docs/architecture.md]]
- Operations → [[docs/ops.md]]
EOF
```

**Run & Verification**:
```bash
cargo test --test harness_governance_test test_detect_secondary_routing_layer
```
**Expected Outcome**:
- Engine detects `SecondaryRoutingLayer` on `docs/index.md`.
- Issue category: `secondary_routing_layer_detected`.
- Recommendation emitted: `EliminateSecondaryRoutingLayer`, proposing to inline the routes directly into `AGENTS.md` under `## Knowledge Index`.

---

### Scenario 3: Skill Governance & Procedure Detection

**Objective**: Verify that valid skills (`SKILL.md` + scripts) are cataloged, broken skill assets are flagged, and recurring procedural steps across notes are detected as `ProcedureCandidate`.

**Setup**:
```bash
mkdir -p test_vault/.agents/skills/deploy-helper/scripts
cat << EOF > test_vault/.agents/skills/deploy-helper/SKILL.md
---
name: deploy-helper
description: Automated deployment helper
---
See deploy helper script
EOF
touch test_vault/.agents/skills/deploy-helper/scripts/deploy.sh
```

**Run & Verification**:
```bash
cargo test --test skill_governance_test test_validate_healthy_skill_and_detect_procedure
```
**Expected Outcome**:
- `deploy-helper` recognized as a valid `SkillDefinition` with status `Valid`.
- Internal link to deploy script resolves with `exists_on_disk: true`.
- When identical 4-step procedures appear in 3 disparate notes, a `ProcedureCandidate` is produced with high recurrence score.

---

### Scenario 4: Harness-Anchored Context Trace & 4-Capability Audit

**Objective**: Verify that Context Trace originates from the Harness Entry Point and evaluates Entry, Discovery, Retrieval, and Reuse capabilities locally against a deterministic benchmark query suite (SC-002).

**Run & Verification**:
```bash
cargo test --test context_trace_harness_test test_harness_anchored_trace_execution
```
**Expected Outcome**:
- Context Trace starts at `AGENTS.md` (Hop 0).
- Navigates through embedded index (Hop 1) to target document section (Hop 2).
- Resolves $\ge 90\%$ of benchmark queries (synthesized from document titles, section headings, and route labels) within $\le 3$ hops and $< 10,000$ BPE tokens.
- `AIOrientedCapabilityMatrix` reports calibrated scores for Entry, Discovery, Retrieval, and Reuse.

---

### Scenario 5: User-Approved Structural Transformation & Re-Measurement

**Objective**: Verify that structural transformations adhere to Invariant XVI (`Auto-Apply: OFF`, `Require Approval: ON`), preview clean unified diffs, and generate Before-vs-After delta metrics.

**Run & Verification**:
```bash
cargo test --test transformation_governance_test test_governance_transformation_approval_gate
```
**Expected Outcome**:
- Generated plan produces unified diff inserting `## Knowledge Index` into `AGENTS.md`.
- Requires explicit approval token before disk write.
- Re-measurement reports positive $\Delta	ext{Context Efficiency}$ and reduced average trace depth.
