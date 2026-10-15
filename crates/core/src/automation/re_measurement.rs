//! Before/After comparative delta re-measurement engine.
//! Implements FR-040 and SC-006.

use crate::model::{HealthScore, PillarDeltas, ReMeasurementDelta};

/// Computes empirical Before/After delta re-measurement report.
pub fn compute_re_measurement_delta(
    before: &HealthScore,
    after: &HealthScore,
    before_tokens: u64,
    after_tokens: u64,
    graph_diameter_delta: Option<i32>,
) -> ReMeasurementDelta {
    let aggregate_score_delta = after.aggregate_score - before.aggregate_score;
    let raw_token_delta = after_tokens as i64 - before_tokens as i64;

    let pillar_deltas = PillarDeltas {
        integrity: after.integrity.raw_score - before.integrity.raw_score,
        structure: after.structure.raw_score - before.structure.raw_score,
        consistency: after.consistency.raw_score - before.consistency.raw_score,
        discoverability: after.discoverability.raw_score - before.discoverability.raw_score,
        freshness: after.freshness.raw_score - before.freshness.raw_score,
    };

    ReMeasurementDelta {
        aggregate_score_delta: (aggregate_score_delta * 10.0).round() / 10.0,
        raw_token_delta,
        graph_diameter_delta,
        pillar_deltas,
    }
}
