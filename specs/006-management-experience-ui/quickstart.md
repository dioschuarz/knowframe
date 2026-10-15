---
wiki:
  content_hash: 3d0037535a71d1105a8b768878b0253def53420ae069495a3ab7a6e9fe889b37
  fragments:
    quickstart-verification-scenarios-llm-wiki-management-experience-structural-evolution-ui:
      tokens: 2010
      inbound: 0
      outbound: 0
    scenario-1-decision-oriented-manager-summary-contextual-health-remediation:
      tokens: 587
      inbound: 0
      outbound: 0
    goal:
      tokens: 22
      inbound: 0
      outbound: 0
    execution-steps:
      tokens: 547
      inbound: 0
      outbound: 0
    scenario-2-investigating-a-problem-first-structural-opportunity:
      tokens: 308
      inbound: 0
      outbound: 0
    goal-1:
      tokens: 28
      inbound: 0
      outbound: 0
    execution-steps-1:
      tokens: 266
      inbound: 0
      outbound: 0
    scenario-3-evaluating-candidate-strategies-hybrid-recommendations:
      tokens: 271
      inbound: 0
      outbound: 0
    goal-2:
      tokens: 16
      inbound: 0
      outbound: 0
    execution-steps-2:
      tokens: 242
      inbound: 0
      outbound: 0
    scenario-4-impact-preview-user-approval-gate-post-change-re-measurement:
      tokens: 399
      inbound: 0
      outbound: 0
    goal-3:
      tokens: 19
      inbound: 0
      outbound: 0
    execution-steps-3:
      tokens: 360
      inbound: 0
      outbound: 0
    scenario-5-insights-vs-settings-separation-custom-orientation-tuning:
      tokens: 276
      inbound: 0
      outbound: 0
    goal-4:
      tokens: 19
      inbound: 0
      outbound: 0
    execution-steps-4:
      tokens: 241
      inbound: 0
      outbound: 0
---
# Quickstart & Verification Scenarios: LLM Wiki Management Experience & Structural Evolution UI

**Feature:** `006-management-experience-ui`  
**Constitution Authority:** `CONSTITUTION.md` v1.6.0  

This guide provides end-to-end user scenarios validating the 11-step Core UX Loop:
$$\text{Open Manager} \to \text{See Health} \to \text{See Opportunities} \to \text{Understand Problem} \to \text{See Evidence} \to \text{See Strategies} \to \text{Understand Trade-Offs} \to \text{Check Orientation} \to \text{Choose/Approve} \to \text{Preview} \to \text{Apply} \to \text{Re-Measure}$$

---

## Scenario 1: Decision-Oriented Manager Summary & Contextual Health Remediation

### Goal
A developer opens the workspace and investigates a drop in documentation health without facing competing global action buttons.

### Execution Steps
1. Open VS Code in a project with Markdown notes (e.g. `ctaas` or `notes/`).
2. Click the **LLM Wiki** icon in the Activity Bar to open the Manager in the primary sidebar.
3. Observe the persistent 4-card summary:
   ```text
   ┌──────────────────────────────┐
   │ LLM Wiki                     │
   │                              │
   │ KNOWLEDGE HEALTH             │
   │                              │
   │        38                    │
   │       /100                   │
   │                              │
   │ Integrity        70%         │
   │ Structure       100%         │
   │ Consistency       0%         │
   │ Discoverability   0%         │
   │ Freshness         0%         │
   │                              │
   │ 6 findings need attention    │
   │ [Review health]              │
   │                              │
   ├──────────────────────────────┤
   │ STRUCTURE                    │
   │                              │
   │ 51 documents                 │
   │ 4 connected · 47 isolated    │
   │ 72 references                │
   │                              │
   │ [Explore graph]              │
   │                              │
   ├──────────────────────────────┤
   │ EVOLUTION                    │
   │                              │
   │ 🤖 AI-Oriented              │
   │                              │
   │ Optimizing AI discovery,    │
   │ context efficiency & reuse  │
   │                              │
   │ 3 opportunities              │
   │                              │
   │ [Review evolution]           │
   │                              │
   ├──────────────────────────────┤
   │ SYNC                         │
   │ ● Synced                     │
   │                              │
   │ Last commit · abc1234        │
   └──────────────────────────────┘
   ```
4. Click **`Integrity`** or **`[Review health]`**.
5. The detail drawer slides out smoothly to **`INSIGHTS → Health → Integrity`**.
6. Observe the consolidated findings list:
   - `4 broken references`
   - `2 invalid anchors`
7. Click the contextual action **`[Fix reference]`** on `auth_spec.md#token-refresh`. Select the suggested replacement target.
8. Notice that the Health card updates automatically to `5 findings need attention` and `Integrity: 75%` without full-page reloads or scroll jumps.

---

## Scenario 2: Investigating a Problem-First Structural Opportunity

### Goal
An architect reviews a structural bottleneck on an overloaded document, inspecting human-readable evidence metrics and `ⓘ` informational tooltips.

### Execution Steps
1. On the Manager sidebar, click **`3 opportunities`** or **`[Review evolution]`**.
2. The detail drawer opens directly to **`INSIGHTS → Evolution → Structural Opportunities`**.
3. Observe the top detected problem card:
   ```text
   STRUCTURAL OPPORTUNITY
   Structurally Overloaded Document (High Context Waste)
   Architecture.md · 87% confidence

   WHY
   Only a small portion of this document appears to be relevant
   to most observed knowledge traversals.

   EVIDENCE
   Document size             50k tokens   ⓘ
   Reference concentration   74%          ⓘ
   Navigational complexity   4.8 hops     ⓘ
   Context efficiency        19%          ⓘ
   ```
4. Hover over the **`Reference concentration ⓘ`** icon.
5. A rich tooltip appears:
   > **Reference concentration measures the percentage of inbound links targeting a specific section rather than the overall document.**  
   > - *Higher values*: Agents load the entire document to read just a few paragraphs.  
   > - *Why it matters*: Drives context window saturation and slower LLM response times.  
   > - *Trade-off*: Section addressing requires stable anchor names.

---

## Scenario 3: Evaluating Candidate Strategies & Hybrid Recommendations

### Goal
Compare alternative strategies for resolving the overloaded document without dogmatic prescriptions.

### Execution Steps
1. Beneath the opportunity evidence, inspect the **Candidate Strategies** section:
   ```text
   Candidate Strategies (Evaluated against 🤖 AI-Oriented)

   1. Section Addressing           87% fit   ⓘ
      Expected Benefits:
      ↓ Context surface (50k → 5.1k)
      ↓ Retrieval depth (4.8 → 2.1)
      ↑ Knowledge reuse
      Trade-Off:
      May increase structural granularity and link complexity.
      Effort: Low
      [Preview change]

   2. Lightweight Index Router     79% fit   ⓘ
      Expected Benefits:
      ↑ Hub discoverability
      ↓ Traversal hops
      Trade-Off:
      Requires maintaining index routing table.
      Effort: Medium
      [Preview change]

   3. Recommended Hybrid Approach:
      Section Addressing + Lightweight Index Router
      Synergy: Combines section-level token reduction with entry-point indexing.
      [Preview change]
   ```
2. Note that the UI avoids words like *"Bad organization"* or *"Correct method"*, using *"87% fit under active objectives and constraints"*.

---

## Scenario 4: Impact Preview, User Approval Gate & Post-Change Re-Measurement

### Goal
Verify the safety gate before applying automated changes, and evaluate post-change empirical measurements.

### Execution Steps
1. Click **`[Preview change]`** on the Recommended Hybrid approach.
2. The **Transformation Plan Preview** modal opens:
   ```text
   TRANSFORMATION PLAN PREVIEW
   Target: Architecture.md (Strategy: Hybrid Section Addressing + Index Router)

   Proposed Actions:
   1. Inject frontmatter fragments into Architecture.md (4 addressable sections).
   2. Update 14 incoming references to target specific anchors (e.g. Architecture.md#auth).
   3. Create lightweight router index at Architecture_index.md.

   Expected Impact:
   • Context efficiency:      32% → 81%
   • Navigational complexity: 4.8 → 2.1
   • Total documents:          51 → 52
   • Total references:         72 → 86

   Countervailing Trade-offs:
   • Adds 1 new routing document to the workspace.
   • Introduces anchor dependencies on 4 heading titles.

   [ Cancel ]    [ Apply Changes ]
   ```
3. Click **`[Apply Changes]`**.
4. The extension applies file updates and evaluates post-change re-measurement on an in-memory ephemeral candidate graph (preserving Invariant I until a Git commit occurs).
5. The **Post-Change Re-Measurement** card renders:
   ```text
   STRUCTURAL RE-MEASUREMENT COMPLETED
   The selected objectives improved, with the following structural trade-offs:

   ✓ Context efficiency:        +49%
   ✓ Navigational complexity:   -56%
   ✓ Maintenance coupling:      -23%
   ⚠ Total documents:            +1
   ⚠ Total references:          +14
   ```


---

## Scenario 5: Insights vs Settings Separation & Custom Orientation Tuning

### Goal
Configure Custom Tier 1 objectives and verify that conflict warnings trigger when constraints clash.

### Execution Steps
1. In the right drawer, note the top-level distinction between **`INSIGHTS`** and **`SETTINGS`**:
   - `INSIGHTS`: `Health`, `Structure`, `Evolution`
   - `SETTINGS`: `Knowledge`, `Exclusions`, `Clusters`, `Git / Harness`, `Advanced (⚙)`
2. Click **`Evolution`** and select the **`⚙️ Custom`** orientation card.
3. In **Tier 1 (High-Level Objectives)**, set:
   - `AI Discoverability`: `High`
   - `Context Efficiency`: `High`
4. In **Constraints**, set:
   - `Document Splitting`: `Hard Constraint (Never)`
   - `Context Surface Limit`: `3k tokens`
5. Immediately observe the yellow alert banner:
   > **⚠ Conflicting constraints detected**: Your configuration strongly prioritizes AI retrieval with a 3k context surface limit while strictly prohibiting document splitting on a 50k-token document. Some recommendations cannot be satisfied.
6. Adjust the constraint to `Avoid if possible` (soft constraint); the warning resolves cleanly.
