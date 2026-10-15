use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::evolution::{
    select_strategy_set, DocumentationOrientation, ProblemSignatureType, ProfileCompiler,
    StructuralMetrics, StructuralProblemSignature,
};

fn generate_sample_signatures(count: usize) -> Vec<StructuralProblemSignature> {
    let mut sigs = Vec::with_capacity(count);
    for i in 0..count {
        let prob_type = match i % 5 {
            0 => ProblemSignatureType::GodDocument,
            1 => ProblemSignatureType::ImplicitRouter,
            2 => ProblemSignatureType::FragmentedConcept,
            3 => ProblemSignatureType::IsolatedSilo,
            _ => ProblemSignatureType::HealthyMonolith,
        };
        sigs.push(StructuralProblemSignature {
            signature_id: format!("sig_{:03}", i),
            document_path: format!("docs/file_{:03}.md", i),
            problem_type: prob_type,
            confidence: 0.88,
            evidence: StructuralMetrics {
                cohesion: 0.45,
                coupling: 0.75,
                reference_concentration: 0.70,
                granularity: 5000.0,
                context_efficiency: 0.35,
                discoverability: 0.55,
                reusability: 0.40,
                semantic_diversity: 0.80,
                navigational_complexity: 2.5,
                maintenance_coupling: 0.65,
            },
            rationale: "Synthetic problem signature for microbenchmark".to_string(),
        });
    }
    sigs
}

fn bench_strategy_set_selection_50_signatures(c: &mut Criterion) {
    let compiler = ProfileCompiler::new();
    let profile = compiler.compile_preset(DocumentationOrientation::AiOriented);
    let signatures = generate_sample_signatures(50);

    c.bench_function("strategy_set_selection_50_signatures", |b| {
        b.iter(|| {
            for sig in &signatures {
                black_box(select_strategy_set(
                    &sig.signature_id,
                    &sig.document_path,
                    sig,
                    &profile,
                ));
            }
        });
    });
}

criterion_group!(benches, bench_strategy_set_selection_50_signatures);
criterion_main!(benches);
