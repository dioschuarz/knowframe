use llm_wiki_core::evolution::*;
use llm_wiki_core::model::NoteNode;

#[test]
fn test_invariant_xiii_healthy_monolith_protection() {
    // Monolithic note with 60,000 tokens (well over 50k threshold)
    let mut node = NoteNode::new(
        "arch_spec.md".to_string(),
        "/vault/arch_spec.md".to_string(),
        "Architecture Specification",
    );
    node.tokens = 60_000;

    // High cohesion (0.85 >= 0.80) and low reference concentration (0.20 <= 0.30)
    let metrics = StructuralMetrics {
        cohesion: 0.85,
        coupling: 0.30,
        reference_concentration: 0.20,
        granularity: 0.70,
        context_efficiency: 0.65,
        discoverability: 0.75,
        reusability: 0.60,
        semantic_diversity: 0.30,
        navigational_complexity: 1.5,
        maintenance_coupling: 0.25,
    };

    // 1. Must be detected as HealthyMonolith
    let signature = detect_healthy_monolith(&node, &metrics);
    assert!(signature.is_some(), "Healthy monolith must be detected");
    let sig = signature.unwrap();
    assert_eq!(sig.problem_type, ProblemSignatureType::HealthyMonolith);
    assert!(sig.confidence >= 0.75);

    // 2. Candidate strategies must NEVER include TopicDecomposition (splitting)
    let strategies = generate_candidate_strategies(&sig);
    assert!(
        !strategies.contains(&KnowledgeOrganizationStrategy::TopicDecomposition),
        "Invariant XIII violation: HealthyMonolith must NEVER be recommended for TopicDecomposition (splitting)"
    );
    assert!(
        strategies.contains(&KnowledgeOrganizationStrategy::PreserveDocument),
        "PreserveDocument must be among recommendations for HealthyMonolith"
    );

    // 3. In build_structural_opportunities, HealthyMonolith must NOT be returned as an opportunity
    let profile = OptimizationProfile::ai_oriented();
    let opportunities = build_structural_opportunities(&[node], &profile);
    assert!(
        opportunities.is_empty(),
        "HealthyMonolith documents must not be listed as problem opportunities to split"
    );
}
