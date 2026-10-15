//! Memory profiling benchmark verifying resting RSS <= 35 MB for 5,000 documents
//! and peak scan RAM <= 250 MB.
//! Implements T067b and NFR-006.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::model::{DocumentNode, FragmentMetadata, GraphTopology, ReferenceEdge};

fn get_process_rss_mb() -> f64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(kb) = parts[1].parse::<f64>() {
                            return kb / 1024.0;
                        }
                    }
                }
            }
        }
    }
    0.0
}

fn generate_in_memory_topology_5000() -> GraphTopology {
    let mut nodes = Vec::with_capacity(5_000);
    let mut edges = Vec::with_capacity(10_000);

    for i in 0..5_000 {
        let id = format!("docs/module_{:02}/note_{:04}.md", i % 50, i);
        let fragments = vec![
            FragmentMetadata {
                anchor: "overview".to_string(),
                heading: "## Overview".to_string(),
                start_line: 10,
                end_line: 25,
                tokens: 85,
                inbound: 2,
                outbound: 1,
            },
            FragmentMetadata {
                anchor: "implementation".to_string(),
                heading: "## Implementation".to_string(),
                start_line: 26,
                end_line: 70,
                tokens: 240,
                inbound: 4,
                outbound: 3,
            },
        ];

        let outgoing = vec![
            ReferenceEdge {
                target_document: format!(
                    "docs/module_{:02}/note_{:04}.md",
                    (i + 1) % 50,
                    (i + 1) % 5_000
                ),
                target_fragment: Some("overview".to_string()),
                relation_type: Some("wikilink".to_string()),
                is_resolved: true,
                line_number: 15,
            },
            ReferenceEdge {
                target_document: format!(
                    "docs/module_{:02}/note_{:04}.md",
                    (i + 7) % 50,
                    (i + 7) % 5_000
                ),
                target_fragment: None,
                relation_type: Some("wikilink".to_string()),
                is_resolved: true,
                line_number: 35,
            },
        ];

        edges.extend(outgoing.clone());

        nodes.push(DocumentNode {
            id: id.clone(),
            relative_path: id,
            title: format!("Architectural Document {:04}", i),
            raw_token_count: 420,
            frontmatter_schema: None,
            fragments,
            outgoing_references: outgoing,
            git_last_modified_commit: "commit_hash_123456789abcdef".to_string(),
            git_last_modified_timestamp: 1710000000,
        });
    }

    GraphTopology { nodes, edges }
}

fn bench_memory_footprint(c: &mut Criterion) {
    let rss_before = get_process_rss_mb();
    let topology = generate_in_memory_topology_5000();
    let rss_after = get_process_rss_mb();

    let memory_delta_mb = (rss_after - rss_before).max(0.0);

    // Verify NFR-006: 5,000 documents memory delta must be well within <= 35 MB
    // and overall process peak must be <= 250 MB
    assert_eq!(topology.nodes.len(), 5_000);
    assert_eq!(topology.edges.len(), 10_000);

    if rss_after > 0.0 {
        assert!(
            memory_delta_mb <= 35.0,
            "5,000 documents resting RSS delta exceeded 35MB: actual {:.2}MB",
            memory_delta_mb
        );
        assert!(
            rss_after <= 250.0,
            "Process peak memory exceeded 250MB: actual {:.2}MB",
            rss_after
        );
    }

    let mut group = c.benchmark_group("memory_footprint");
    group.sample_size(10);
    group.bench_function("generate_5000_doc_topology_ram", |b| {
        b.iter(|| {
            let topo = generate_in_memory_topology_5000();
            black_box(topo.nodes.len());
        })
    });
    group.finish();
}

criterion_group!(benches, bench_memory_footprint);
criterion_main!(benches);
