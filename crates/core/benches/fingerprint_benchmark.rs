use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::collision::fingerprint::extract_fingerprint;

fn bench_fingerprint_extraction(c: &mut Criterion) {
    let doc_id = "architecture_spec.md";
    let headings = vec![
        "Introduction and Background".to_string(),
        "Constitutional Quality Admission Boundaries".to_string(),
        "Deterministic Collision Detection Engine".to_string(),
        "Performance and Token Economics".to_string(),
        "Conclusion and Next Steps".to_string(),
    ];
    let tags = vec![
        "#architecture".to_string(),
        "#admission".to_string(),
        "#quality".to_string(),
    ];

    let content = r#"
# Architecture Specification: Knowledge Admission Control

This document defines the strict pre-commit admission control boundaries.
Any staged changes that introduce new broken links or high-confidence duplicates
exceeding the recorded baseline are rejected before committing to the repository.

## Constitutional Quality Admission Boundaries

The Dual-Process separation mandates that the pre-commit hook runs entirely in-memory
against an ephemeral candidate graph without modifying persistent storage.
The post-commit hook receives the committed git delta and updates the Knowledge Graph.

## Deterministic Collision Detection Engine

Using character 3-grams, BM25 term frequencies, and Jaccard title similarities,
the engine computes composite similarity scores between notes without relying on external AI.

## Performance and Token Economics

Under strict non-blocking fail-open guarantees, the pre-commit hook aborts at 500ms
with exit code 0 to prevent blocking the developer's Git workflow.
"#;

    c.bench_function("fingerprint_extraction_standard_note", |b| {
        b.iter(|| {
            let fp = extract_fingerprint(
                black_box(doc_id),
                black_box(content),
                black_box(&headings),
                black_box(&tags),
            );
            black_box(fp);
        });
    });
}

criterion_group!(benches, bench_fingerprint_extraction);
criterion_main!(benches);
