//! Criterion benchmark asserting 1,000 documents cold scan in <= 1,500 ms.
//! Implements T065, NFR-004, and SC-002.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::model::OptimizationProfile;
use llm_wiki_core::observation::scanner::{scan_repository, ScanOptions};
use std::time::Instant;
use tempfile::tempdir;

fn bench_cold_scan_1000(c: &mut Criterion) {
    let dir = tempdir().expect("Failed to create tempdir for cold scan benchmark");
    let root = dir.path();

    // Create 1,000 markdown documents with headings, frontmatter, and references
    for i in 0..1_000 {
        let content = format!(
            "---\ntitle: Document {:04}\ntags: [benchmark, cluster_{}]\n---\n\n# Document {:04}\n\nThis is document {} with references to [[doc_{:04}.md]] and [[doc_{:04}.md]].\n\n## Section Alpha\nDetailed design content here.\n\n## Section Beta\nAdditional architectural notes.\n",
            i,
            i % 10,
            i,
            i,
            (i + 1) % 1_000,
            (i + 13) % 1_000
        );
        let path = root.join(format!("doc_{:04}.md", i));
        std::fs::write(path, content).expect("Failed to write benchmark file");
    }

    let profile = OptimizationProfile::ai_oriented();
    let options = ScanOptions::default();

    // Verification assertion: cold scan of 1,000 docs MUST complete in <= 1,500 ms
    let start = Instant::now();
    let result = scan_repository(root, &options, &profile, "commit_cold_bench")
        .expect("Cold scan failed during benchmark verification");
    let elapsed = start.elapsed();

    assert_eq!(result.total_documents, 1_000);
    assert!(
        elapsed.as_millis() <= 1500,
        "Cold scan exceeded 1,500ms budget: actual elapsed {}ms",
        elapsed.as_millis()
    );

    let mut group = c.benchmark_group("cold_scan");
    group.sample_size(10);
    group.bench_function("cold_scan_1000_documents", |b| {
        b.iter(|| {
            let res = scan_repository(
                black_box(root),
                black_box(&options),
                black_box(&profile),
                black_box("commit_cold_bench"),
            )
            .unwrap();
            black_box(res);
        })
    });
    group.finish();
}

criterion_group!(benches, bench_cold_scan_1000);
criterion_main!(benches);
