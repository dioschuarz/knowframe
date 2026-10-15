//! Invariant XIII Monolith Protection Guard.
//! Implements FR-009, SC-006, and Constitution Invariant XIII:
//! Healthy, cohesive monoliths (even >50k tokens) must never be penalized or split
//! if they maintain high internal cohesion, low external coupling, clean section anchors,
//! and adequate internal fragment structure.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonolithMetrics {
    pub tokens: u64,
    pub cohesion: f64,
    pub coupling: f64,
    pub semantic_diversity: f64,
    pub fragment_count: usize,
    pub broken_anchors_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonolithProtection {
    pub is_protected: bool,
    pub reason: String,
}

/// Evaluates whether a document qualifies as a protected Healthy Monolith under Invariant XIII.
pub fn check_monolith_protection(metrics: &MonolithMetrics) -> MonolithProtection {
    let is_cohesive = metrics.cohesion >= 0.70;
    let is_low_coupling = metrics.coupling <= 0.40;
    let is_focused_topic = metrics.semantic_diversity <= 0.50;
    let has_zero_broken_anchors = metrics.broken_anchors_count == 0;
    let is_well_sectioned = metrics.fragment_count >= 5;

    if is_cohesive && is_low_coupling && is_focused_topic && has_zero_broken_anchors && is_well_sectioned {
        MonolithProtection {
            is_protected: true,
            reason: "healthy_monolith".to_string(),
        }
    } else {
        MonolithProtection {
            is_protected: false,
            reason: "unhealthy_or_unfragmented".to_string(),
        }
    }
}
