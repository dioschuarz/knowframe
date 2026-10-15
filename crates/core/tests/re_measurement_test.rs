//! Integration test for post-apply Before/After delta re-measurement.
//! Validates FR-040 and SC-006.

use llm_wiki_core::automation::re_measurement::compute_re_measurement_delta;
use llm_wiki_core::model::{EvolutionMode, HealthScore, PillarHealth};

#[test]
fn test_before_after_delta_re_measurement() {
    let make_score = |agg: f64, integrity: f64, structure: f64| HealthScore {
        analysis_id: "test-run".to_string(),
        aggregate_score: agg,
        profile_mode: EvolutionMode::AiOriented,
        integrity: PillarHealth {
            raw_score: integrity,
            legacy_debt_count: 0,
            new_regressions_count: 0,
            findings: Vec::new(),
        },
        structure: PillarHealth {
            raw_score: structure,
            legacy_debt_count: 0,
            new_regressions_count: 0,
            findings: Vec::new(),
        },
        consistency: PillarHealth {
            raw_score: 90.0,
            legacy_debt_count: 0,
            new_regressions_count: 0,
            findings: Vec::new(),
        },
        discoverability: PillarHealth {
            raw_score: 85.0,
            legacy_debt_count: 0,
            new_regressions_count: 0,
            findings: Vec::new(),
        },
        freshness: PillarHealth {
            raw_score: 80.0,
            legacy_debt_count: 0,
            new_regressions_count: 0,
            findings: Vec::new(),
        },
    };

    let before = make_score(75.0, 80.0, 60.0);
    let after = make_score(88.0, 95.0, 85.0);

    let delta = compute_re_measurement_delta(&before, &after, 50000, 42000, Some(-1));

    assert!((delta.aggregate_score_delta - 13.0).abs() < 1e-4);
    assert_eq!(delta.raw_token_delta, -8000);
    assert_eq!(delta.graph_diameter_delta, Some(-1));
    assert!((delta.pillar_deltas.integrity - 15.0).abs() < 1e-4);
    assert!((delta.pillar_deltas.structure - 25.0).abs() < 1e-4);
}
