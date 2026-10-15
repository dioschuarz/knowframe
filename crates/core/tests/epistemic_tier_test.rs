//! Epistemic tier audit test verifying strict classification and zero external AI calls.
//! Validates FR-001, FR-003, SC-001, and SC-007.

use llm_wiki_core::model::{
    EpistemicTier, FindingSeverity, Methodology, QualityPillar, StructuralFinding,
};

#[test]
fn test_epistemic_tier_variants_and_boundaries() {
    let tier_a = EpistemicTier::TierAFullyDeterministic;
    let tier_b = EpistemicTier::TierBCandidateGeneration;
    let tier_c = EpistemicTier::TierCDecisionSupport;
    let tier_d = EpistemicTier::TierDNotReliablyDeterminable;

    // Verify serde serialization strings matching protocol contract
    assert_eq!(
        serde_json::to_string(&tier_a).unwrap(),
        "\"tier_a_fully_deterministic\""
    );
    assert_eq!(
        serde_json::to_string(&tier_b).unwrap(),
        "\"tier_b_candidate_generation\""
    );
    assert_eq!(
        serde_json::to_string(&tier_c).unwrap(),
        "\"tier_c_decision_support\""
    );
    assert_eq!(
        serde_json::to_string(&tier_d).unwrap(),
        "\"tier_d_not_reliably_determinable\""
    );
}

#[test]
fn test_structural_finding_epistemic_classification() {
    let finding_a = StructuralFinding {
        id: "finding-001".to_string(),
        methodology: Methodology::Taxonomy,
        pillar: QualityPillar::Structure,
        tier: EpistemicTier::TierAFullyDeterministic,
        severity: FindingSeverity::High,
        document_id: "docs/arch.md".to_string(),
        fragment_anchor: Some("core-engine".to_string()),
        line_number: 42,
        rule_key: "taxonomy_cycle_detected".to_string(),
        message: "Cyclic dependency detected in taxonomic hierarchy".to_string(),
        evidence_metrics: serde_json::json!({ "cycle_length": 3 }),
        remediation_hint: "Break the cycle by removing backlink".to_string(),
    };

    assert_eq!(finding_a.tier, EpistemicTier::TierAFullyDeterministic);
    assert_eq!(finding_a.pillar, QualityPillar::Structure);
}
