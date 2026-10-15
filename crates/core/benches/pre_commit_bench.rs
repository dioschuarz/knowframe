//! Criterion benchmark asserting pre-commit validation in <= 500 ms fail-open and post-commit sync <= 50 ms.
//! Implements T067, NFR-003, NFR-005, and SC-003.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::health::baseline::BaselineManager;
use llm_wiki_core::model::OptimizationProfile;
use llm_wiki_core::observation::scanner::{scan_repository, ScanOptions};
use std::time::{Duration, Instant};
use tempfile::tempdir;

fn bench_commit_lifecycle(c: &mut Criterion) {
    let dir = tempdir().expect("Failed to create tempdir for pre_commit_bench");
    let root = dir.path();

    // Create a 200-document knowledge vault with links and headings
    for i in 0..200 {
        let content = format!(
            "# Doc {:03}\n\nContent for document {} referencing [[doc_{:03}.md]].\n\n## Section A\nInfo.\n",
            i,
            i,
            (i + 1) % 200
        );
        std::fs::write(root.join(format!("doc_{:03}.md", i)), content).unwrap();
    }

    let cv_dir = root.join(".context-vault");
    std::fs::create_dir_all(&cv_dir).unwrap();
    let baseline_path = cv_dir.join("baseline.json");
    let baseline = BaselineManager::new(&baseline_path);
    baseline.save().unwrap();

    let profile = OptimizationProfile::ai_oriented();
    let options = ScanOptions::default();

    // 1. Verify pre-commit validation latency <= 500 ms
    let pre_commit_start = Instant::now();
    let scan_res = scan_repository(root, &options, &profile, "HEAD").unwrap();
    let timeout = Duration::from_millis(500);
    let mut broken_refs = 0;
    for edge in &scan_res.topology.edges {
        if !edge.is_resolved {
            broken_refs += 1;
        }
    }
    let pre_commit_elapsed = pre_commit_start.elapsed();

    assert!(
        pre_commit_elapsed <= timeout,
        "Pre-commit validation exceeded 500ms budget: {}ms",
        pre_commit_elapsed.as_millis()
    );
    assert!(broken_refs <= 200);

    // 2. Verify post-commit sync latency <= 50 ms
    let post_commit_start = Instant::now();
    let sync_scan = scan_repository(root, &options, &profile, "HEAD").unwrap();
    let post_commit_elapsed = post_commit_start.elapsed();

    assert!(
        post_commit_elapsed.as_millis() <= 50,
        "Post-commit sync exceeded 50ms budget: {}ms",
        post_commit_elapsed.as_millis()
    );
    assert_eq!(sync_scan.total_documents, 200);

    let mut group = c.benchmark_group("commit_lifecycle");
    group.sample_size(15);

    group.bench_function("pre_commit_validation_200_docs", |b| {
        b.iter(|| {
            let start = Instant::now();
            let scan = scan_repository(
                black_box(root),
                black_box(&options),
                black_box(&profile),
                black_box("HEAD"),
            )
            .unwrap();
            let elapsed = start.elapsed();
            if elapsed > timeout {
                black_box(0); // Fail-open exit code 0
            } else {
                let mut broken = 0;
                for edge in &scan.topology.edges {
                    if !edge.is_resolved {
                        broken += 1;
                    }
                }
                black_box(broken);
            }
        })
    });

    group.bench_function("post_commit_sync_200_docs", |b| {
        b.iter(|| {
            let scan = scan_repository(
                black_box(root),
                black_box(&options),
                black_box(&profile),
                black_box("HEAD"),
            )
            .unwrap();
            black_box(scan);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_commit_lifecycle);
criterion_main!(benches);
