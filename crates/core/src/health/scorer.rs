//! 5-Pillar profile-relative health scoring aggregator.
//! Implements FR-024, FR-028, and FR-029.

use crate::model::{
    FindingSeverity, HealthScore, OptimizationProfile, PillarHealth, QualityPillar,
    StructuralFinding,
};

/// Computes penalty deduction for a finding based on severity.
fn severity_penalty(severity: FindingSeverity) -> f64 {
    match severity {
        FindingSeverity::Critical => 25.0,
        FindingSeverity::High => 15.0,
        FindingSeverity::Medium => 8.0,
        FindingSeverity::Low => 3.0,
        FindingSeverity::Informational => 0.0,
    }
}

/// Computes profile-relative 5-pillar health score.
pub fn compute_health_score(
    analysis_id: &str,
    findings: &[StructuralFinding],
    profile: &OptimizationProfile,
    legacy_debt_count: u64,
    new_regressions_count: u64,
) -> HealthScore {
    let mut integrity_findings = Vec::new();
    let mut structure_findings = Vec::new();
    let mut consistency_findings = Vec::new();
    let mut discoverability_findings = Vec::new();
    let mut traceability_findings = Vec::new();

    for f in findings {
        match f.pillar {
            QualityPillar::Integrity => integrity_findings.push(f.clone()),
            QualityPillar::Structure => structure_findings.push(f.clone()),
            QualityPillar::Consistency => consistency_findings.push(f.clone()),
            QualityPillar::Discoverability => discoverability_findings.push(f.clone()),
            QualityPillar::Traceability => traceability_findings.push(f.clone()),
        }
    }

    let compute_pillar = |p_findings: Vec<StructuralFinding>| {
        let mut raw = 100.0;
        for f in &p_findings {
            raw -= severity_penalty(f.severity);
        }
        let score = raw.clamp(0.0, 100.0);
        PillarHealth {
            raw_score: score,
            legacy_debt_count,
            new_regressions_count,
            findings: p_findings,
        }
    };

    let integrity = compute_pillar(integrity_findings);
    let structure = compute_pillar(structure_findings);
    let consistency = compute_pillar(consistency_findings);
    let discoverability = compute_pillar(discoverability_findings);
    let traceability = compute_pillar(traceability_findings);

    let weights = &profile.weights;
    let total_weight = weights.integrity
        + weights.structure
        + weights.consistency
        + weights.discoverability
        + weights.freshness;

    let aggregate = if total_weight > 0.0 {
        (integrity.raw_score * weights.integrity
            + structure.raw_score * weights.structure
            + consistency.raw_score * weights.consistency
            + discoverability.raw_score * weights.discoverability
            + traceability.raw_score * weights.freshness)
            / total_weight
    } else {
        100.0
    };

    HealthScore {
        analysis_id: analysis_id.to_string(),
        aggregate_score: (aggregate * 10.0).round() / 10.0,
        profile_mode: profile.mode,
        integrity,
        structure,
        consistency,
        discoverability,
        freshness: traceability,
    }
}
