//! Saaty Analytic Hierarchy Process (AHP) matrix solver and implementation cost calculation.
//! Implements FR-033 and Section 4 & 5 of technical research.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AhpMatrix {
    pub size: usize,
    pub data: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AhpPriorityResult {
    pub weights: Vec<f64>,
    pub consistency_ratio: f64,
}

/// Saaty Random Index lookup table (n = 1..=10).
const RANDOM_INDEX: [f64; 11] = [0.0, 0.0, 0.0, 0.58, 0.90, 1.12, 1.24, 1.32, 1.41, 1.45, 1.49];

/// Computes the priority vector and consistency ratio using Saaty's geometric mean method.
pub fn compute_ahp_priority_vector(matrix: &AhpMatrix) -> Result<AhpPriorityResult, String> {
    let n = matrix.size;
    if n == 0 || matrix.data.len() != n {
        return Err("Matrix dimensions do not match size".to_string());
    }

    // Geometric mean of each row
    let mut geom_means = Vec::with_capacity(n);
    for row in &matrix.data {
        if row.len() != n {
            return Err("Matrix row length mismatch".to_string());
        }
        let mut prod = 1.0;
        for &val in row {
            if val <= 0.0 {
                return Err("Pairwise comparison values must be strictly positive".to_string());
            }
            prod *= val;
        }
        geom_means.push(prod.powf(1.0 / n as f64));
    }

    let sum_geom: f64 = geom_means.iter().sum();
    if sum_geom <= 0.0 {
        return Err("Sum of geometric means must be positive".to_string());
    }

    let weights: Vec<f64> = geom_means.iter().map(|&gm| gm / sum_geom).collect();

    // Approximate principal eigenvalue lambda_max = (1/n) * sum((A * w)_i / w_i)
    let mut lambda_max = 0.0;
    for i in 0..n {
        let mut row_sum = 0.0;
        for j in 0..n {
            row_sum += matrix.data[i][j] * weights[j];
        }
        if weights[i] > 0.0 {
            lambda_max += row_sum / weights[i];
        }
    }
    lambda_max /= n as f64;

    // Consistency Index (CI) and Consistency Ratio (CR)
    let ci = if n > 1 {
        (lambda_max - n as f64) / (n as f64 - 1.0)
    } else {
        0.0
    };

    let ri = if n < RANDOM_INDEX.len() {
        RANDOM_INDEX[n]
    } else {
        1.49
    };

    let consistency_ratio = if ri > 0.0 {
        (ci / ri).max(0.0)
    } else {
        0.0
    };

    Ok(AhpPriorityResult {
        weights,
        consistency_ratio,
    })
}

/// Computes normalized implementation cost:
/// Cost_impl(S) = min(1.0, (0.6 * TargetFiles + 0.4 * (LineChurn / 50)) / 10.0)
pub fn compute_implementation_cost(target_files_count: usize, estimated_line_churn: usize) -> f64 {
    let files_term = 0.6 * target_files_count as f64;
    let churn_term = 0.4 * (estimated_line_churn as f64 / 50.0);
    let raw_cost = (files_term + churn_term) / 10.0;
    raw_cost.clamp(0.0, 1.0)
}

/// Computes strategy utility / Fit Score:
/// Fit(S) = sum(w_k * mu_k) - penalty
pub fn compute_strategy_fit_score(
    criteria_weights: &[f64],
    strategy_evaluations: &[f64],
    penalty: f64,
) -> f64 {
    let mut utility = 0.0;
    for (w, u) in criteria_weights.iter().zip(strategy_evaluations.iter()) {
        utility += w * u;
    }
    (utility - penalty).clamp(0.0, 1.0)
}
