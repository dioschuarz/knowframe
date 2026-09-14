use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::evolution::{
    evaluate_recommendations_for_signature, DocumentationOrientation, ProblemSignatureType,
    ProfileCompiler, StructuralMetrics, StructuralProblemSignature,
};

fn generate_sample_signatures(count: usize) -> Vec<StructuralProblemSignature> {
    let mut sigs = Vec::with_capacity(count);
    for i in 0..count {
        let prob_type = match i % 4 {
            0 => ProblemSignatureType::GodDocument,
            1 => ProblemSignatureType::ImplicitRouter,
            2 => ProblemSignatureType::FragmentedConcept,
            _ => ProblemSignatureType::IsolatedSilo,
        };
        sigs.push(StructuralProblemSignature {
            signature_id: format!("sig_{:03}", i),
            document_path: format!("docs/file_{:03}.md", i),
            problem_type: prob_type,
            confidence: 0.85,
            evidence: StructuralMetrics {
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
            },
            rationale: "Synthetic problem signature for benchmark".to_string(),
        });
    }
    sigs
}

fn bench_strategy_evaluation_50_signatures(c: &mut Criterion) {
    let compiler = ProfileCompiler::new();
    let profile = compiler.compile_preset(DocumentationOrientation::AiOriented);
    let signatures = generate_sample_signatures(50);

    c.bench_function("strategy_evaluation_50_signatures", |b| {
        b.iter(|| {
            for sig in &signatures {
                black_box(evaluate_recommendations_for_signature(sig, &profile));
            }
        });
    });
}

criterion_group!(benches, bench_strategy_evaluation_50_signatures);
criterion_main!(benches);
