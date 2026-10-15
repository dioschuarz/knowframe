---
wiki:
  content_hash: 167c53784b7222a936c9781395f67f30bd2ad161fff5986050ff8b8b07f0e4ee
  fragments:
    feature-specification-strategy-selection-combination:
      tokens: 3452
      inbound: 0
      outbound: 0
    user-scenarios-testing-mandatory:
      tokens: 1692
      inbound: 0
      outbound: 0
    user-story-1-multi-strategy-compatibility-strategy-set-generation-priority-p1:
      tokens: 510
      inbound: 0
      outbound: 0
    user-story-2-interactive-multi-strategy-selection-with-incompatibility-guards-priority-p2:
      tokens: 438
      inbound: 0
      outbound: 0
    user-story-3-composite-plan-value-trade-off-valuation-priority-p3:
      tokens: 255
      inbound: 0
      outbound: 0
    edge-cases:
      tokens: 479
      inbound: 0
      outbound: 0
    requirements-mandatory:
      tokens: 1203
      inbound: 0
      outbound: 0
    functional-requirements:
      tokens: 933
      inbound: 0
      outbound: 0
    key-entities:
      tokens: 264
      inbound: 0
      outbound: 0
    success-criteria-mandatory:
      tokens: 262
      inbound: 0
      outbound: 0
    measurable-outcomes:
      tokens: 255
      inbound: 0
      outbound: 0
    assumptions:
      tokens: 178
      inbound: 0
      outbound: 0
---
# Feature Specification: Strategy Selection & Combination

**Feature Branch**: `007-strategy-selection-combination`

**Created**: 2026-09-23

**Status**: Draft

**Input**: User description: "Strategy Selection & Combination — Implementation Spec: Candidate strategies, Fit Score (0–100) with fit < 60 threshold, Fit vs Compatibility vs Role distinction, Strategy relationships (COMPLEMENTARY, ALTERNATIVE, MUTUALLY_EXCLUSIVE), 9-step selection algorithm, Strategy Set output format, and interactive multi-strategy selection UI with compatibility guards."

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Multi-Strategy Compatibility & Strategy Set Generation (Priority: P1)

As a knowledge architect reviewing detected structural bottlenecks in my vault, I want the system to evaluate all candidate strategies against an empirical Fit Score ($0\text{--}100$), filter out low-fit options ($<60$), classify pairwise relationships (`COMPLEMENTARY`, `ALTERNATIVE`, `MUTUALLY_EXCLUSIVE`), and synthesize a structured **Strategy Set** partitioned into `Primary`, `Supporting`, and `Alternative` roles, so that I receive cohesive, multi-faceted recommendations rather than being arbitrarily restricted to a single isolated approach.

**Why this priority**: Core architectural bedrock. Transforming structural recommendations from a single-strategy verdict into a multi-strategy set enables real-world hybrid evolutions (e.g. section addressing inside the document combined with a lightweight index router for navigation) while preventing destructive or nonsensical combinations.

**Independent Test**: Can be tested by providing a structural problem signature (e.g. God Document) and verifying that the engine outputs a valid `StrategySet` with exactly one `PRIMARY` strategy, zero or more `SUPPORTING` complementary strategies, and a list of `ALTERNATIVES`, with zero strategies having `fit < 60`.

**Acceptance Scenarios**:

1. **Given** a detected structural problem with computed fit scores for all 4 candidate strategies, **When** the candidate evaluation runs, **Then** any strategy with `fit < 60` is marked ineligible and omitted from recommendations.
2. **Given** eligible candidate strategies (`fit >= 60`), **When** the engine classifies relationships, **Then** `preserve_document` is classified as `ALTERNATIVE` to all transformation strategies and cannot be combined with them.
3. **Given** `section_addressing` (Fit: 87) and `lightweight_index` (Fit: 81) as eligible strategies, **When** the Strategy Set is generated, **Then** `section_addressing` is designated as `PRIMARY` and `lightweight_index` is designated as `SUPPORTING` because their relationship is `COMPLEMENTARY`.
4. **Given** eligible strategies that are `ALTERNATIVE` (e.g. `faceted_classification` with Fit: 74), **When** the Strategy Set is assembled, **Then** it is placed in `ALTERNATIVES` and NOT combined with the primary solution.

---

### User Story 2 - Interactive Multi-Strategy Selection with Incompatibility Guards (Priority: P2)

As a user exploring structural evolution recommendations in the Management & Settings drawer, I want to view the recommended Strategy Set with clear role badges (`PRIMARY`, `SUPPORTING`, `ALTERNATIVES`) and interactive checkboxes, and I want the interface to allow selecting multiple compatible strategies while dynamically disabling incompatible options, so that I maintain full control over the evolution scope without risk of configuring contradictory changes.

**Why this priority**: Essential user experience and agency (Invariant XVI). Users must not be limited to an all-or-nothing toggle; they need to cherry-pick which compatible transformations to apply, while the UI actively guards against invalid combinations.

**Independent Test**: Can be tested in the Webview UI by loading an opportunity recommendation card, verifying that `PRIMARY` and `SUPPORTING` strategies start pre-checked, checking an additional compatible option preserves valid state, and clicking an incompatible alternative prompts or automatically adjusts selections with an explicit explanation.

**Acceptance Scenarios**:

1. **Given** a recommended evolution with Primary `Section Addressing` and Supporting `Lightweight Index`, **When** the card renders, **Then** both compatible options render with enabled checkboxes, while `Faceted Classification` and `Preserve Document` render as alternative options.
2. **Given** the user selects `Preserve Document`, **When** the selection updates, **Then** all transformation strategies (`Section Addressing`, `Lightweight Index`, `Faceted Classification`) are automatically deselected and disabled with an explanation: *"Preserve Document represents no structural transformation and cannot be combined with transformation strategies."*
3. **Given** the user unchecks the supporting strategy `Lightweight Index`, **When** the selection updates, **Then** the active plan recalculates to include only `Section Addressing` without error.
4. **Given** the user checks an `ALTERNATIVE` strategy (e.g. `Faceted Classification`), **When** selected, **Then** conflicting transformation strategies (e.g. `Section Addressing`) are unchecked or swapped with transparent user feedback.

---

### User Story 3 - Composite Plan Value & Trade-Off Valuation (Priority: P3)

As an engineering leader evaluating knowledge base changes, I want the system to compute a total plan value for any active Strategy Set—weighing individual fit scores, complementary synergy bonuses, conceptual overlap penalties, and cumulative transformation costs—so that I can quantitatively compare different strategy combinations before approving dry-run execution.

**Why this priority**: Ensures transparent trade-off accounting (Invariant XV) so users understand whether combining a second strategy provides meaningful marginal value over the primary strategy alone, or merely adds unnecessary maintenance overhead.

**Independent Test**: Can be tested by evaluating plan value for a single primary strategy vs a primary + supporting combination, verifying that the complementary pair earns a synergy bonus while accounting for transformation cost.

**Acceptance Scenarios**:

1. **Given** an active selection of `Section Addressing` + `Lightweight Index`, **When** total plan value is calculated, **Then** the score incorporates a synergy bonus for complementary retrieval pathways while deducting transformation effort.
2. **Given** a candidate strategy set with high conceptual overlap, **When** total plan value is calculated, **Then** an overlap penalty reduces the composite score to prevent redundant documentation artifacts.

---

### Edge Cases

- **Edge Case 1: No strategy achieves `fit >= 60`**:
  If all transformation strategies score below 60 due to high narrative coherence constraints or low problem severity, the system MUST default to `preserve_document` as the sole PRIMARY strategy with an informational advisory explanation: *"No active structural transformation meets the 60% confidence threshold under current profile constraints."*
- **Edge Case 2: Tie in highest Fit Score**:
  If two eligible strategies share the exact same top Fit Score (e.g. both 85%), the engine resolves ties through an exhaustive, deterministic 4-stage hierarchy (Invariants III, XIV):
  1. Highest Fit Score ($0\text{--}100$).
  2. Lowest transformation cost (`transformation_cost` integer).
  3. Highest corresponding objective weight in active `OptimizationProfile` (e.g. `context_efficiency_weight` for `section_addressing`, `discoverability_weight` for `lightweight_index`, `narrative_coherence_weight` for `preserve_document`).
  4. Canonical declaration order (`section_addressing` > `lightweight_index` > `faceted_classification` > `preserve_document`) as absolute mathematical fallback.
- **Edge Case 3: Empty user selection**:
  If a user unchecks all strategies in the UI, the system disables the `[Preview Change]` / `[Generate Plan]` button with a validation tooltip: *"Select at least one strategy to generate a transformation plan."*
- **Edge Case 4: `Preserve Document` as top Fit**:
  When a document is classified as a Healthy Monolith and `preserve_document` achieves the highest fit, it is designated as PRIMARY. All transformation strategies are classified as ALTERNATIVES, and the SUPPORTING list is empty.
- **Edge Case 5: Cumulative transformation complexity**:
  When multiple supporting strategies are complementary, the system displays an aggregate effort notice based on quantified thresholds:
  - Low Effort: cumulative cost $\le 10$ and $\le 1$ file modified.
  - Moderate Effort: cumulative cost $11\text{--}20$ or $2$ files modified.
  - High Effort: cumulative cost $> 20$ or $\ge 3$ files modified.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST compute a deterministic Fit Score ($0\text{--}100$) for every candidate strategy against each detected structural problem and active optimization profile.
- **FR-002**: System MUST evaluate the following four standard candidate strategies:
  1. `section_addressing`
  2. `faceted_classification`
  3. `lightweight_index`
  4. `preserve_document`
- **FR-003**: System MUST treat `fit >= 60` as the strict threshold for candidate eligibility. Any strategy with `fit < 60` MUST be classified as non-recommended and excluded from primary, supporting, and alternative candidate sets.
- **FR-004**: System MUST classify the relationship between every pair of strategies into exactly one of three relationship types:
  1. `COMPLEMENTARY`: Strategies address orthogonal structural aspects and can coexist synergistically.
  2. `ALTERNATIVE`: Strategies offer competing, mutually substitutable approaches to solve the problem.
  3. `MUTUALLY_EXCLUSIVE`: Strategies are fundamentally incompatible and cannot coexist for the same document or problem.
- **FR-005**: System MUST enforce canonical strategy relationships:
  - `section_addressing` + `lightweight_index` $\to$ `COMPLEMENTARY`
  - `faceted_classification` + `lightweight_index` $\to$ `COMPLEMENTARY`
  - `section_addressing` + `faceted_classification` $\to$ `ALTERNATIVE`
  - `preserve_document` + any transformation strategy $\to$ `MUTUALLY_EXCLUSIVE` (represented with role `ALTERNATIVE` in the `StrategySet` presentation)
  - Any strategy paired with itself $(A, A) \to \text{MUTUALLY\_EXCLUSIVE}$ (prevents duplicate selection)
- **FR-006**: System MUST enforce that `preserve_document` represents zero structural transformation, strictly forbidding combination with any transformation strategy for the same target document.
- **FR-007**: System MUST execute the 9-step Strategy Selection Algorithm:
  1. Detect structural problem on target document.
  2. Calculate Fit Score ($0\text{--}100$) for all 4 candidate strategies.
  3. Filter out all strategies with $\text{Fit} < 60$.
  4. Classify remaining strategies by pairwise relationships.
  5. Select the highest-fit eligible strategy as `PRIMARY`.
  6. Add compatible (`COMPLEMENTARY`) high-fit strategies as `SUPPORTING`.
  7. Exclude `ALTERNATIVE` and `MUTUALLY_EXCLUSIVE` strategies from the recommended active combination, placing them in `ALTERNATIVES` with role `ALTERNATIVE`.
  8. Calculate composite plan value incorporating individual fit, compatibility synergy, overlap, and transformation cost.
  9. Present the synthesized Strategy Set to the user.
- **FR-008**: The output of the selection algorithm MUST be a structured `StrategySet` containing:
  - `primary`: exactly one top-ranked primary strategy (`StrategyRecommendationItem`) with integer fit score and rationale.
  - `supporting`: list of supporting strategies (zero or more complementary `StrategyRecommendationItem` objects).
  - `alternatives`: list of alternative strategies (zero or more alternative `StrategyRecommendationItem` objects).
  - `composite_plan_value`: structured `CompositePlanValue` object whose `net_value` represents the aggregate calculated integer score.
- **FR-009**: The user interface MUST render the Strategy Set with distinct visual groupings for `PRIMARY`, `SUPPORTING`, and `ALTERNATIVES`.
- **FR-010**: The user interface MUST provide interactive checkboxes (`☑` / `☐`) allowing users to customize which compatible strategies are included in the transformation.
- **FR-011**: The user interface MUST prevent incompatible combinations in real time: selecting a strategy MUST immediately disable or deselect incompatible strategies, accompanied by a plain-language explanation of why they cannot be combined.
- **FR-012**: System MUST pass the user's finalized multi-strategy selection into transformation plan generation, producing an atomic plan incorporating actions for all selected strategies.
- **FR-013**: System MUST ensure that `fit >= 60` is treated strictly as "eligible candidate", never as an automatic guarantee of inclusion in the active selection set.
- **FR-014**: System MUST recalculate the composite plan value dynamically whenever the user modifies strategy selections in the interface.

---

### Key Entities

- **`StrategyFitScore`**: Evaluated numerical fit ($0\text{--}100$) of a specific strategy for a detected problem under the active optimization profile.
- **`StrategyRelationshipType`**: Enumerated compatibility classification (`COMPLEMENTARY`, `ALTERNATIVE`, `MUTUALLY_EXCLUSIVE`).
- **`StrategyRole`**: Assigned role of an eligible strategy within a recommendation (`PRIMARY`, `SUPPORTING`, `ALTERNATIVE`).
- **`StrategyRecommendationItem`**: An individual strategy entry containing its strategy key, display label, integer Fit Score ($0\text{--}100$), role, rationale, expected benefits, countervailing trade-offs, and transformation cost.
- **`StrategySet`**: The synthesized combination bundle containing `primary` (`StrategyRecommendationItem`), `supporting` (`Vec<StrategyRecommendationItem>`), `alternatives` (`Vec<StrategyRecommendationItem>`), `composite_plan_value` (`CompositePlanValue`), and `active_selection`.
- **`CompositePlanValue`**: Total evaluated utility score of an active strategy combination, factoring in base fit scores, complementary synergy coefficients, overlap deductions, and effort costs with `net_value` as the aggregate integer score.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of candidate strategies evaluated produce an integer Fit Score in the range $0\text{--}100$, with zero strategies having $\text{fit} < 60$ surfacing as recommended or eligible.
- **SC-002**: 100% of generated structural evolution recommendations output a structured `StrategySet` cleanly partitioned into `PRIMARY`, `SUPPORTING`, and `ALTERNATIVES`.
- **SC-003**: Exactly 0% of generated or user-selected Strategy Sets contain `ALTERNATIVE` or `MUTUALLY_EXCLUSIVE` strategies combined on the same target document.
- **SC-004**: When `preserve_document` is selected, 100% of transformation strategies are disabled and excluded from the generated transformation plan.
- **SC-005**: Users can inspect fit scores and toggle compatible supporting strategies in $\le 2$ clicks within the evolution recommendation card.
- **SC-006**: Candidate strategy evaluation, relationship classification, and Strategy Set synthesis execute in $\le 10\text{ ms}$ per detected structural problem.
- **SC-007**: Multi-strategy transformation plan generation produces a unified diff preview that executes atomically without conflicting filesystem operations.

---

## Assumptions

- The 4 core strategies (`section_addressing`, `faceted_classification`, `lightweight_index`, `preserve_document`) cover all canonical structural patterns required for Markdown knowledge vaults. Future specialized strategies (e.g. `topic_decomposition`) can be added by declaring their pairwise relationship mappings.
- Fit scores are calculated deterministically from observable 10-D metrics and the active `OptimizationProfile` (Invariant XIV), ensuring identical repository states produce identical Fit Scores (Invariant III).
- Pairwise strategy relationships are symmetric: if $A$ is `COMPLEMENTARY` to $B$, then $B$ is `COMPLEMENTARY` to $A$.
- In the Webview UI, users are presented with sensible defaults (`PRIMARY` and `SUPPORTING` pre-checked) so common workflows require zero manual configuration, while preserving full agency to customize.
