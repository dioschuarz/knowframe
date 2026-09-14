use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::evolution::{DocumentationOrientation, ProfileCompiler, Tier1Objectives};

fn bench_profile_compilation(c: &mut Criterion) {
    let compiler = ProfileCompiler::new();
    let orientations = [
        DocumentationOrientation::AiOriented,
        DocumentationOrientation::HumanOriented,
        DocumentationOrientation::Balanced,
        DocumentationOrientation::Custom,
    ];

    c.bench_function("profile_recompilation_all_orientations", |b| {
        b.iter(|| {
            for o in &orientations {
                black_box(compiler.compile_preset(*o));
            }
        });
    });

    let tier1 = Tier1Objectives::default();
    c.bench_function("tier1_to_tier2_mapping", |b| {
        b.iter(|| {
            black_box(compiler.map_tier1_to_tier2(&tier1));
        });
    });

    c.bench_function("custom_profile_compilation", |b| {
        b.iter(|| {
            black_box(compiler.compile_custom(&tier1, None, vec![]));
        });
    });
}

criterion_group!(benches, bench_profile_compilation);
criterion_main!(benches);
