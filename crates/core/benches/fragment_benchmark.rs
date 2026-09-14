use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::fragment::extractor::extract_fragments_from_document;

fn generate_sample_documents(count: usize) -> Vec<String> {
    let mut docs = Vec::with_capacity(count);
    for i in 0..count {
        let content = format!(
            r#"# Document {:03} Architecture

Overview of system module {:03}.
This section provides a high-level overview of components, data flow, and interactions.

## Authentication & Authorization
Detailed authentication guide for module {:03}.
Ensures token validity, session termination, and identity verification against OAuth providers.

### Token Verification Pipeline
Sub-pipeline for cryptographic signature verification, claim assertion, and expiration checks.

## Data Storage & Replication
Information regarding persistence layers, read replicas, and distributed synchronization.

### Sharding Strategy
Consistent hashing partitioning across 16 database partitions.

## Observability & Metrics
Telemetry collection, OpenTelemetry traces, Prometheus metrics, and alert triggers.
"#,
            i, i, i
        );
        docs.push(content);
    }
    docs
}

fn bench_fragment_extraction_100_docs(c: &mut Criterion) {
    let docs = generate_sample_documents(100);

    c.bench_function("fragment_extraction_100_documents", |b| {
        b.iter(|| {
            let mut total_fragments = 0;
            for doc in &docs {
                let frags = extract_fragments_from_document(black_box(doc), &[]);
                total_fragments += frags.len();
            }
            black_box(total_fragments);
        });
    });
}

criterion_group!(benches, bench_fragment_extraction_100_docs);
criterion_main!(benches);
