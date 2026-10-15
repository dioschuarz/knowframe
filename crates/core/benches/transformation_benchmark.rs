use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::evolution::{
    KnowledgeOrganizationStrategy, ReMeasurementEngine, StructuralMetrics,
    StructuralRecommendation, TransformationEngine,
};
use llm_wiki_core::model::NoteNode;
use std::path::Path;

fn bench_plan_generation_and_re_measurement(c: &mut Criterion) {
    let mut node = NoteNode::new("docs/god.md", "/workspace/docs/god.md", "God");
    node.tokens = 12000;
    let rec = StructuralRecommendation {
        recommendation_id: "rec_bench".to_string(),
        signature_id: "sig_bench".to_string(),
        strategy: KnowledgeOrganizationStrategy::TopicDecomposition,
        confidence: 0.95,
        expected_benefits: vec!["Topic isolation".to_string()],
        trade_offs: vec!["File addition".to_string()],
        transformation_plan: None,
    };
    let root = Path::new("/workspace");

    c.bench_function("transformation_plan_dry_run_generation", |b| {
        b.iter(|| {
            black_box(TransformationEngine::build_plan(&rec, &node, root));
        });
    });

    let before_metrics = vec![StructuralMetrics {
        cohesion: 0.5,
        coupling: 0.8,
        reference_concentration: 0.75,
        granularity: 0.4,
        context_efficiency: 0.3,
        discoverability: 0.6,
        reusability: 0.3,
        semantic_diversity: 0.8,
        navigational_complexity: 0.6,
        maintenance_coupling: 0.7,
    }];
    let after_metrics = vec![
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

    c.bench_function("post_transformation_re_measurement", |b| {
        b.iter(|| {
            black_box(ReMeasurementEngine::compute_delta(
                &before_metrics,
                &after_metrics,
                1,
                2,
            ));
        });
    });
}

criterion_group!(benches, bench_plan_generation_and_re_measurement);
criterion_main!(benches);
