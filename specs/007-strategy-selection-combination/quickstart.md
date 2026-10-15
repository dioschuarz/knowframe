---
wiki:
  content_hash: 7e2d25b930b6b225d9a510b6446137e3ace878cb6c25d30f2444d294eca16219
  fragments:
    quickstart-runnable-validation-scenarios-strategy-selection-combination-007-strategy-selection-combination:
      tokens: 1474
      inbound: 0
      outbound: 2
    prerequisites:
      tokens: 78
      inbound: 0
      outbound: 0
    scenario-1-multi-strategy-candidate-evaluation-strategyset-partitioning:
      tokens: 349
      inbound: 0
      outbound: 0
    execution-steps:
      tokens: 239
      inbound: 0
      outbound: 0
    expected-outcome:
      tokens: 39
      inbound: 0
      outbound: 0
    scenario-2-interactive-checkbox-customization-incompatibility-guards:
      tokens: 430
      inbound: 0
      outbound: 0
    execution-steps-1:
      tokens: 353
      inbound: 0
      outbound: 0
    expected-outcome-1:
      tokens: 37
      inbound: 0
      outbound: 0
    scenario-3-composite-plan-value-calculation-trade-off-valuation:
      tokens: 273
      inbound: 0
      outbound: 0
    execution-steps-2:
      tokens: 206
      inbound: 0
      outbound: 0
    expected-outcome-2:
      tokens: 25
      inbound: 0
      outbound: 0
    scenario-4-edge-cases-healthy-monolith-preserve-document-fallback:
      tokens: 260
      inbound: 0
      outbound: 0
    execution-steps-3:
      tokens: 186
      inbound: 0
      outbound: 0
    expected-outcome-3:
      tokens: 26
      inbound: 0
      outbound: 0
---
# Quickstart & Runnable Validation Scenarios: Strategy Selection & Combination (`007-strategy-selection-combination`)

**Feature Branch**: `007-strategy-selection-combination`  
**Date**: 2026-09-23  
**Status**: Completed  
**Source Inputs**: [`spec.md`](spec.md), [`data-model.md`](data-model.md), `contracts/strategy_selection_rpc_protocol.json`  

---

## Prerequisites

- Rust toolchain (`cargo` $\ge 1.75$) and Node.js ($\ge 20$) with npm.
- VS Code $\ge 1.90.0$ running with `llm-wiki` extension active.
- Workspace containing Markdown documentation vault (e.g. `/home/ds/projects` with `notes/`).

---

## Scenario 1: Multi-Strategy Candidate Evaluation & StrategySet Partitioning

**Objective**: Verify that the Rust core evaluates all 4 candidate strategies against a detected structural bottleneck, applies the $\text{Fit} \ge 60$ eligibility gate, and partitions results cleanly into `PRIMARY`, `SUPPORTING`, and `ALTERNATIVES`.

### Execution Steps:
1. Run the Rust integration test suite for strategy combination:
   ```bash
   cargo test --test strategy_combination_test -- --nocapture
   ```
2. Verify that for a detected God Document problem (`granularity: 52000`, `semantic_diversity: 0.82`, `reference_concentration: 0.74`):
   - All 4 candidate strategies are evaluated: `section_addressing`, `faceted_classification`, `lightweight_index`, `preserve_document`.
   - Strategies scoring $\text{fit} < 60$ are strictly omitted from recommendations.
   - The strategy with the highest fit (e.g. `section_addressing` with Fit 87) is designated as `PRIMARY`.
   - Compatible strategies (`lightweight_index` with Fit 81) are classified as `SUPPORTING` because their relationship is `COMPLEMENTARY`.
   - Competing strategies (`faceted_classification` with Fit 72) are placed in `ALTERNATIVES`.
3. Check the benchmark execution time: candidate evaluation and `StrategySet` synthesis complete in $\le 10\text{ ms}$ (SC-006).

### Expected Outcome:
`StrategySet` is generated with exactly one `PRIMARY` strategy, zero or more `SUPPORTING` strategies, and zero strategies below the 60% threshold.

---

## Scenario 2: Interactive Checkbox Customization & Incompatibility Guards

**Objective**: Verify that the Management & Settings drawer allows selecting multiple compatible strategies while dynamically guarding against incompatible combinations in real time.

### Execution Steps:
1. In VS Code, open the Management & Settings drawer (`Ctrl+Shift+P` $\to$ **`LLM Wiki: Open Management & Settings`**).
2. Switch to the **Evolution** tab.
3. Locate an active structural opportunity recommendation card.
4. Verify the visual layout:
   - **PRIMARY**: Pre-checked (`☑`), cyan badge, displays Fit Score (e.g. `87%`).
   - **SUPPORTING**: Pre-checked (`☑`), green badge, displays Fit Score (e.g. `81%`) and synergy rationale.
   - **ALTERNATIVES**: Unchecked (`☐`), amber badge, displays individual Fit Score.
5. Uncheck the `SUPPORTING` strategy (`Lightweight Index`):
   - Verify the card immediately updates the Total Plan Value without errors.
6. Click the checkbox for an `ALTERNATIVE` strategy (`Faceted Classification`):
   - Verify the incompatibility guard activates instantly:
   - `Section Addressing` is automatically deselected and disabled.
   - An explanatory notification appears: *"Faceted Classification is an alternative to Section Addressing. Swapped active strategy to prevent conflicting restructuring."*
7. Click the checkbox for `Preserve Document`:
   - Verify all transformation strategies (`Section Addressing`, `Lightweight Index`, `Faceted Classification`) are automatically deselected and disabled.
   - Verbatim explanatory notice appears: *"Preserve Document represents zero structural transformation and cannot be combined with transformation strategies."*
8. Uncheck all strategies:
   - Verify the `[Generate Plan]` button becomes disabled with tooltip: *"Select at least one strategy to generate a transformation plan."*

### Expected Outcome:
Zero incompatible strategies can be simultaneously checked (SC-003, SC-004); feedback renders within $16.6\text{ ms}$ (60 FPS).

---

## Scenario 3: Composite Plan Value Calculation & Trade-Off Valuation

**Objective**: Verify that the composite plan value mathematically reflects base fit, synergy bonuses, overlap deductions, and transformation effort.

### Execution Steps:
1. Run the plan value unit tests:
   ```bash
   cargo test --test plan_value_test -- --nocapture
   ```
2. Verify test case for `section_addressing` (Fit: 87, Cost: 5) alone:
   $$\text{Net Value} = 87 - 5 = 82$$
3. Verify test case for combination of `section_addressing` (Fit: 87, Cost: 5) + `lightweight_index` (Fit: 81, Cost: 10):
   $$\text{Net Value} = (87 + 81) + 15\ (\text{synergy}) - 5\ (\text{overlap}) - (5 + 10)\ (\text{cost}) = 168 + 15 - 5 - 15 = 163$$
4. Verify that marginal plan value delta is positive ($\Delta = +81 > 0$), justifying the hybrid approach.

### Expected Outcome:
Composite plan value is computed deterministically as an integer score with a complete itemized trade-off breakdown.

---

## Scenario 4: Edge Cases & Healthy Monolith `Preserve Document` Fallback

**Objective**: Verify that the engine gracefully handles tie scores and honors Invariant XIII (non-prescriptive organization: healthy monoliths are preserved).

### Execution Steps:
1. Trigger evaluation on a Healthy Monolith document (`tokens: 45000`, `semantic_diversity: 0.22`, `cohesion: 0.91`):
   - Verify `preserve_document` achieves top fit score ($\ge 90$).
   - Verify `preserve_document` is designated as `PRIMARY`.
   - Verify the `SUPPORTING` list is empty, and transformation strategies are marked `ALTERNATIVES`.
2. Trigger evaluation on a document where all transformation strategies score $< 60$ due to high narrative coherence weights:
   - Verify the engine defaults to `preserve_document` as `PRIMARY` with advisory explanation: *"No active structural transformation meets the 60% confidence threshold under current profile constraints."*
3. Verify tie-breaking logic: when two complementary strategies share the exact same top score, the one with lower transformation cost becomes `PRIMARY`.

### Expected Outcome:
System strictly avoids dogmatic splitting of healthy monoliths (Invariant XIII) and enforces reliable fallback behavior.
