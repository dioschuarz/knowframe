//! Unit test for 5-pillar profile-relative health scoring.
//! Validates FR-024, FR-028, FR-029, and SC-006.

use llm_wiki_core::health::scorer::compute_health_score;
use llm_wiki_core::model::{
    EpistemicTier, FindingSeverity, Methodology, OptimizationProfile, QualityPillar,
    StructuralFinding,
};

#[test]
fn test_health_scoring_profile_relative_adaptation() {
    let findings = vec![
        StructuralFinding {
            id: "f-struct-1".to_string(),
            methodology: Methodology::InformationArchitecture,
            pillar: QualityPillar::Structure,
            tier: EpistemicTier::TierAFullyDeterministic,
            severity: FindingSeverity::High,
            document_id: "docs/monolith.md".to_string(),
            fragment_anchor: None,
            line_number: 1,
            rule_key: "structure_oversized".to_string(),
            message: "Document exceeds token limits".to_string(),
            evidence_metrics: serde_json::json!({ "tokens": 9000 }),
            remediation_hint: "Decompose document".to_string(),
        },
        StructuralFinding {
            id: "f-disc-1".to_string(),
            methodology: Methodology::KnowledgeGraphNetworkAnalysis,
            pillar: QualityPillar::Discoverability,
            tier: EpistemicTier::TierAFullyDeterministic,
            severity: FindingSeverity::Medium,
            document_id: "docs/orphan.md".to_string(),
            fragment_anchor: None,
            line_number: 1,
            rule_key: "orphan_isolated".to_string(),
            message: "Document is isolated".to_string(),
            evidence_metrics: serde_json::json!({ "in_degree": 0 }),
            remediation_hint: "Add references".to_string(),
        },
    ];

    let ai_profile = OptimizationProfile::ai_oriented();
    let human_profile = OptimizationProfile::human_oriented();

    let score_ai = compute_health_score("run-1", &findings, &ai_profile, 0, 0);
    let score_human = compute_health_score("run-1", &findings, &human_profile, 0, 0);

    // AI-Oriented weights Structure more heavily, so score_ai should reflect different weighting
    assert!(
        (score_ai.aggregate_score - score_human.aggregate_score).abs() > 0.01,
        "Aggregate score must adapt according to active optimization profile weights"
    );
}
