//! Unit test for Saaty AHP matrix priority solver, Fit Score, and Implementation Cost.
//! Validates FR-032, FR-033, and SC-006.

use llm_wiki_core::strategies::ahp::{
    compute_ahp_priority_vector, compute_implementation_cost, compute_strategy_fit_score,
    AhpMatrix,
};

#[test]
fn test_ahp_matrix_priority_vector_and_consistency() {
    // 3x3 pairwise comparison matrix where row 0 is 2x row 1, 4x row 2
    let matrix = AhpMatrix {
        size: 3,
        data: vec![
            vec![1.0, 2.0, 4.0],
            vec![0.5, 1.0, 2.0],
            vec![0.25, 0.5, 1.0],
        ],
    };

    let result = compute_ahp_priority_vector(&matrix).expect("Valid matrix should solve");
    assert_eq!(result.weights.len(), 3);
    assert!((result.weights.iter().sum::<f64>() - 1.0).abs() < 1e-4);
    assert!(result.weights[0] > result.weights[1]);
    assert!(result.weights[1] > result.weights[2]);
    assert!(result.consistency_ratio < 0.10, "Consistent matrix must have CR < 0.10");
}

#[test]
fn test_implementation_cost_calculation() {
    // 2 target files, 100 estimated line churn
    // Cost = min(1.0, (0.6 * 2 + 0.4 * (100 / 50)) / 10.0) = min(1.0, (1.2 + 0.8) / 10.0) = 2.0 / 10.0 = 0.20
    let cost = compute_implementation_cost(2, 100);
    assert!((cost - 0.20).abs() < 1e-4);
}

#[test]
fn test_strategy_fit_score_utility() {
    let criteria_weights = vec![0.50, 0.30, 0.20];
    let strategy_evaluations = vec![0.80, 0.90, 0.60];
    let penalty = 0.05;

    let fit = compute_strategy_fit_score(&criteria_weights, &strategy_evaluations, penalty);
    // Utility = (0.50 * 0.80 + 0.30 * 0.90 + 0.20 * 0.60) - 0.05 = (0.40 + 0.27 + 0.12) - 0.05 = 0.79 - 0.05 = 0.74
    assert!((fit - 0.74).abs() < 1e-4);
}
