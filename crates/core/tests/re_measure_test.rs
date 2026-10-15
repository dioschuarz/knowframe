use llm_wiki_core::evolution::*;

#[test]
fn test_empirical_re_measurement_calculation() {
    let before_metrics = vec![
        StructuralMetrics {
            cohesion: 0.40,
            coupling: 0.70,
            reference_concentration: 0.80,
            granularity: 0.30,
            context_efficiency: 0.25,
            discoverability: 0.50,
            reusability: 0.40,
            semantic_diversity: 0.75,
            navigational_complexity: 3.5,
            maintenance_coupling: 0.65,
        },
    ];

    let after_metrics = vec![
        StructuralMetrics {
            cohesion: 0.80,
            coupling: 0.40,
            reference_concentration: 0.30,
            granularity: 0.75,
            context_efficiency: 0.70,
            discoverability: 0.85,
            reusability: 0.80,
            semantic_diversity: 0.35,
            navigational_complexity: 1.8,
            maintenance_coupling: 0.30,
        },
    ];

    let delta = ReMeasurementEngine::compute_delta(&before_metrics, &after_metrics, 1, 3);
    assert!(delta.efficiency_delta > 0.0, "Efficiency delta should be positive improvement");
    assert!(delta.trace_depth_delta < 0.0, "Navigational complexity delta should decrease");
    assert!(delta.coupling_delta < 0.0, "Coupling delta should decrease");
    assert_eq!(delta.doc_count_delta, 2);
}
