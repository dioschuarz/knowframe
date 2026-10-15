use llm_wiki_core::evolution::*;
use llm_wiki_core::model::NoteNode;

#[test]
fn test_hybrid_strategy_recommendation_and_synergy() {
    let mut node = NoteNode::new(
        "large_module.md".to_string(),
        "/vault/large_module.md".to_string(),
        "Large Module",
    );
    node.tokens = 12_000;
    node.out_degree = 30;
    node.betweenness = 0.65;

    let metrics = StructuralMetrics {
        cohesion: 0.50,
        coupling: 0.75,
        reference_concentration: 0.80,
        granularity: 0.40,
        context_efficiency: 0.30,
        discoverability: 0.60,
        reusability: 0.45,
        semantic_diversity: 0.80,
        navigational_complexity: 2.8,
        maintenance_coupling: 0.70,
    };

    let sig = detect_god_document(&node, &metrics).expect("God document signature expected");
    let strategies = generate_candidate_strategies(&sig);
    assert!(
        strategies.contains(&KnowledgeOrganizationStrategy::Hybrid),
        "God document should support Hybrid strategy"
    );

    let profile = OptimizationProfile::ai_oriented();
    let fitness = calculate_fitness(KnowledgeOrganizationStrategy::Hybrid, &sig, &profile);
    assert!(fitness > 0.0 && fitness <= 1.0);

    let matrix = generate_trade_off_matrix(KnowledgeOrganizationStrategy::Hybrid, &sig);
    assert!(!matrix.expected_benefits.is_empty(), "Benefits must be itemized");
    assert!(!matrix.trade_offs.is_empty(), "Countervailing trade-offs must be itemized (Invariant XV)");
}
