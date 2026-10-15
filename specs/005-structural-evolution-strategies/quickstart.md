---
wiki:
  content_hash: 1383eac2c7d7c4e48b53dc9f3fbe3cdbdffe4235253a6e88ccc25de8ba29708f
  fragments:
    quickstart-knowledge-organization-systems-structural-evolution-scenarios:
      tokens: 1225
      inbound: 0
      outbound: 0
    scenario-1-switching-documentation-orientations-real-time-re-weighting:
      tokens: 233
      inbound: 0
      outbound: 0
    objective:
      tokens: 62
      inbound: 0
      outbound: 0
    steps:
      tokens: 153
      inbound: 0
      outbound: 0
    scenario-2-two-tier-custom-configuration-conflict-detection:
      tokens: 286
      inbound: 0
      outbound: 0
    objective-1:
      tokens: 31
      inbound: 0
      outbound: 0
    steps-1:
      tokens: 241
      inbound: 0
      outbound: 0
    scenario-3-10-dimensional-structural-problem-detection-god-document-vs-cohesive-monolith:
      tokens: 320
      inbound: 0
      outbound: 0
    objective-2:
      tokens: 32
      inbound: 0
      outbound: 0
    steps-2:
      tokens: 265
      inbound: 0
      outbound: 0
    scenario-4-kos-strategy-evaluation-user-approval-re-measurement:
      tokens: 343
      inbound: 0
      outbound: 0
    objective-3:
      tokens: 31
      inbound: 0
      outbound: 0
    steps-3:
      tokens: 294
      inbound: 0
      outbound: 0
---
# Quickstart: Knowledge Organization Systems & Structural Evolution Scenarios

**Feature Branch:** `005-structural-evolution-strategies`  
**Target Delivery:** `llm_wiki_ext4vsc`  

---

## Scenario 1: Switching Documentation Orientations & Real-Time Re-Weighting

### Objective
Verify that toggling the Documentation Orientation between `AI-Oriented` (default), `Human-Oriented`, `Balanced`, and `Custom` updates `.context-vault.yaml` and dynamically adjusts the internal `OptimizationProfile` in $\le 20\text{ ms}$.

### Steps
1. Open the LLM Wiki Settings Drawer in VS Code and navigate to the **Documentation Orientation** selector.
2. The initial default is selected: `🤖 AI-Oriented`.
3. Switch orientation to `👤 Human-Oriented`.
4. Check `.context-vault.yaml` in the workspace root:
   ```yaml
   orientation: human_oriented
   ```
5. Inspect the diagnostic log in VS Code Output Channel:
   ```text
   [EXTENSION] Orientation changed to 'human_oriented'. OptimizationProfile recompiled in 4.2ms.
   ```
6. Verify that narrative coherence and document continuity weights increase to $\ge 0.80$, while context surface bounds are relaxed.

---

## Scenario 2: Two-Tier Custom Configuration & Conflict Detection

### Objective
Verify that setting Custom mode allows tuning Tier 1 high-level objectives with informative tooltips (`ⓘ`) and flags mathematically contradictory constraints.

### Steps
1. In the Orientation selector, pick `⚙️ Custom`.
2. The UI expands into two tiers: **High-Level Objectives (Tier 1)** and **Advanced Parameters (Tier 2)**.
3. Hover over the tooltip `ⓘ` beside **Context Efficiency**:
   - Tooltip displays:
     > Controls how strongly LLM Wiki favors reducing irrelevant content traversed or loaded to answer a task.
     > Higher values favor focused knowledge units, section-level addressing, and shorter Context Traces.
     > Trade-off: may increase structural fragmentation.
     > Affects: max_context_surface, fragment_size_target, section_addressability_weight.
4. Set `Context Efficiency` to `High` and configure a Hard Constraint: `Never automatically split documents`.
5. Also set `Max Context Surface` to `5k tokens` on a repository with a 50k-token document.
6. Observe the immediate warning banner:
   ```text
   ⚠️ Conflicting Constraints Detected
   Your configuration strongly prioritizes context efficiency (max 5k tokens) while prohibiting document splitting. Some structural recommendations may become impossible to satisfy.
   ```

---

## Scenario 3: 10-Dimensional Structural Problem Detection (God Document vs. Cohesive Monolith)

### Objective
Verify that LLM Wiki distinguishes an overloaded "God Document" from a healthy, cohesive large reference document without relying on raw line/token count.

### Steps
1. Create a cohesive 50k-token reference document `docs/api_reference.md` covering one continuous schema with uniform section usage:
   - Cohesion: 0.85
   - Semantic Diversity: 0.20
   - Reference Concentration: 0.25
   - Context Efficiency: 0.70
2. Create an overloaded 6k-token note `docs/everything.md` covering architecture, deployment scripts, HR onboarding, and legal terms with 80% of inbound links targeting only the deployment snippet:
   - Cohesion: 0.22
   - Semantic Diversity: 0.85
   - Reference Concentration: 0.88
   - Context Efficiency: 0.18
   - Coupling: 0.80
3. Run structural problem analysis:
   ```bash
   target/release/llm-wiki-core analytics --problems
   ```
4. Verify results:
   - `docs/api_reference.md` is classified as `HealthyMonolith` (Confidence: 0.92) — NO decomposition recommended.
   - `docs/everything.md` is classified as `GodDocument` (Confidence: 0.89) with itemized evidence of high diversity, concentration, and low efficiency.

---

## Scenario 4: KOS Strategy Evaluation, User Approval & Re-Measurement

### Objective
Verify that the engine proposes candidate strategies with transparent trade-offs, requires explicit user confirmation before modifying files, and measures the empirical delta after execution.

### Steps
1. Inspect the recommendation for `docs/everything.md` in the Settings Drawer:
   - Candidate Strategies:
     1. **Section-Level Addressing + Lightweight Index** (Confidence: 87% — Recommended)
     2. **Topic Decomposition** (Confidence: 74%)
     3. **Preserve Document** (Confidence: 30%)
   - Trade-off matrix:
     - Section Addressing + Index: High efficiency gain, low fragmentation.
     - Topic Decomposition: Maximum cohesion, but adds 3 new document nodes.
2. Click `[Review Transformation Plan]`:
   - Inspect diff preview:
     - Extraction of deployment script section to `docs/deployment.md`.
     - Creation of index router `docs/everything_index.md`.
     - Rewriting of inbound references to `docs/deployment.md`.
     - Verification: 0 Hard Constraints violated.
3. Click `[Apply Transformation]`.
4. Check immediate Before-vs-After verification output in UI:
   ```text
   Transformation applied successfully:
   • Context Efficiency: 18% → 79% (+61% ↑)
   • Context Trace Depth: 4.2 → 1.8 (-2.4 hops ↓)
   • Maintenance Coupling: 80% → 42% (-38% ↓)
   • Document Count: 1 → 3 (+2 notes)
   ```
