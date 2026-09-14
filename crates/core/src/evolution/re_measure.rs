use serde::{Deserialize, Serialize};
use super::metrics::StructuralMetrics;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDeltaReport {
    pub efficiency_delta: f32,
    pub trace_depth_delta: f32,
    pub coupling_delta: f32,
    pub doc_count_delta: i32,
}

impl Default for MetricDeltaReport {
    fn default() -> Self {
        Self {
            efficiency_delta: 0.0,
            trace_depth_delta: 0.0,
            coupling_delta: 0.0,
            doc_count_delta: 0,
        }
    }
}

pub struct ReMeasurementEngine;

impl ReMeasurementEngine {
    /// Computes Before-vs-After structural delta report.
    /// Executes incrementally in <= 100ms per NFR-006.
    pub fn compute_delta(
        before_metrics: &[StructuralMetrics],
        after_metrics: &[StructuralMetrics],
        before_doc_count: usize,
        after_doc_count: usize,
    ) -> MetricDeltaReport {
        let avg_eff_before: f32 = if before_metrics.is_empty() {
            0.0
        } else {
            before_metrics.iter().map(|m| m.context_efficiency).sum::<f32>() / before_metrics.len() as f32
        };
        let avg_eff_after: f32 = if after_metrics.is_empty() {
            0.0
        } else {
            after_metrics.iter().map(|m| m.context_efficiency).sum::<f32>() / after_metrics.len() as f32
        };

        let avg_nav_before: f32 = if before_metrics.is_empty() {
            0.0
        } else {
            before_metrics.iter().map(|m| m.navigational_complexity).sum::<f32>() / before_metrics.len() as f32
        };
        let avg_nav_after: f32 = if after_metrics.is_empty() {
            0.0
        } else {
            after_metrics.iter().map(|m| m.navigational_complexity).sum::<f32>() / after_metrics.len() as f32
        };

        let avg_coup_before: f32 = if before_metrics.is_empty() {
            0.0
        } else {
            before_metrics.iter().map(|m| m.coupling).sum::<f32>() / before_metrics.len() as f32
        };
        let avg_coup_after: f32 = if after_metrics.is_empty() {
            0.0
        } else {
            after_metrics.iter().map(|m| m.coupling).sum::<f32>() / after_metrics.len() as f32
        };

        MetricDeltaReport {
            efficiency_delta: ((avg_eff_after - avg_eff_before) * 1000.0).round() / 1000.0,
            trace_depth_delta: ((avg_nav_after - avg_nav_before) * 1000.0).round() / 1000.0,
            coupling_delta: ((avg_coup_after - avg_coup_before) * 1000.0).round() / 1000.0,
            doc_count_delta: after_doc_count as i32 - before_doc_count as i32,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_incremental_post_apply_re_measurement() {
        let before_m = vec![
            StructuralMetrics {
                cohesion: 0.6,
                coupling: 0.75,
                reference_concentration: 0.8,
                granularity: 0.4,
                context_efficiency: 0.3,
                discoverability: 0.5,
                reusability: 0.2,
                semantic_diversity: 0.8,
                navigational_complexity: 0.7,
                maintenance_coupling: 0.6,
            },
        ];

        let after_m = vec![
            StructuralMetrics {
                cohesion: 0.85,
                coupling: 0.35,
                reference_concentration: 0.3,
                granularity: 0.8,
                context_efficiency: 0.75,
                discoverability: 0.8,
                reusability: 0.6,
                semantic_diversity: 0.4,
                navigational_complexity: 0.4,
                maintenance_coupling: 0.3,
            },
            StructuralMetrics {
                cohesion: 0.90,
                coupling: 0.30,
                reference_concentration: 0.25,
                granularity: 0.85,
                context_efficiency: 0.80,
                discoverability: 0.85,
                reusability: 0.65,
                semantic_diversity: 0.35,
                navigational_complexity: 0.35,
                maintenance_coupling: 0.25,
            },
        ];

        let start = Instant::now();
        let delta = ReMeasurementEngine::compute_delta(&before_m, &after_m, 1, 2);
        let elapsed = start.elapsed();

        assert!(elapsed.as_millis() <= 100, "Re-measurement must be <= 100ms per NFR-006, took {:?}", elapsed);
        assert_eq!(delta.doc_count_delta, 1);
        assert!(delta.efficiency_delta > 0.4, "Efficiency must have improved");
        assert!(delta.coupling_delta < -0.3, "Coupling must have decreased");
        assert!(delta.trace_depth_delta < -0.2, "Navigational complexity / trace depth must have decreased");
    }
}
