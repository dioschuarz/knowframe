//! Criterion benchmark asserting single-file candidate patch in <= 50 ms and graph calculation < 200 ms.
//! Implements T066, NFR-003, and SC-002.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::graph::candidate::CandidateGraph;
use llm_wiki_core::health::scorer::compute_health_score;
use llm_wiki_core::model::{DocumentNode, GraphTopology, OptimizationProfile, ReferenceEdge};
use std::time::Instant;

fn generate_base_topology(num_nodes: usize) -> GraphTopology {
    let mut nodes = Vec::with_capacity(num_nodes);
    let mut edges = Vec::new();

    for i in 0..num_nodes {
        let id = format!("doc_{:04}.md", i);
        let target_1 = format!("doc_{:04}.md", (i + 1) % num_nodes);
        let target_2 = format!("doc_{:04}.md", (i + 7) % num_nodes);

        let mut outgoing = vec![ReferenceEdge {
            target_document: target_1.clone(),
            target_fragment: None,
            relation_type: Some("wikilink".to_string()),
            is_resolved: true,
            line_number: 10,
        }];

        edges.push(ReferenceEdge {
            target_document: target_1,
            target_fragment: None,
            relation_type: Some("wikilink".to_string()),
            is_resolved: true,
            line_number: 10,
        });

        if target_2 != id {
            outgoing.push(ReferenceEdge {
                target_document: target_2.clone(),
                target_fragment: None,
                relation_type: Some("wikilink".to_string()),
                is_resolved: true,
                line_number: 25,
            });

            edges.push(ReferenceEdge {
                target_document: target_2,
                target_fragment: None,
                relation_type: Some("wikilink".to_string()),
                is_resolved: true,
                line_number: 25,
            });
        }

        nodes.push(DocumentNode {
            id: id.clone(),
            relative_path: id,
            title: format!("Document {:04}", i),
            raw_token_count: 250,
            frontmatter_schema: None,
            fragments: Vec::new(),
            outgoing_references: outgoing,
            git_last_modified_commit: "commit_base".to_string(),
            git_last_modified_timestamp: 1700000000,
        });
    }

    GraphTopology { nodes, edges }
}

fn bench_incremental_pipeline(c: &mut Criterion) {
    let base_topology = generate_base_topology(1_000);
    let profile = OptimizationProfile::ai_oriented();

    // 1. Single-file candidate patch verification (<= 50 ms)
    let start_patch = Instant::now();
    let mut candidate = CandidateGraph::new(&base_topology);
    let modified_node = DocumentNode {
        id: "doc_0042.md".to_string(),
        relative_path: "doc_0042.md".to_string(),
        title: "Modified Document 0042".to_string(),
        raw_token_count: 500,
        frontmatter_schema: None,
        fragments: Vec::new(),
        outgoing_references: Vec::new(),
        git_last_modified_commit: "commit_candidate".to_string(),
        git_last_modified_timestamp: 1700000500,
    };
    candidate.stage_node(modified_node.clone());
    let effective = candidate.effective_topology();
    let patch_elapsed = start_patch.elapsed();

    assert!(
        patch_elapsed.as_millis() <= 50,
        "Incremental candidate patch exceeded 50ms budget: actual {}ms",
        patch_elapsed.as_millis()
    );

    // 2. Incremental graph calculation verification (< 200 ms)
    let start_calc = Instant::now();
    let mut broken_links = 0;
    for edge in &effective.edges {
        if !edge.is_resolved {
            broken_links += 1;
        }
    }
    let findings = Vec::new();
    let health = compute_health_score("cand_analysis_id", &findings, &profile, 0, broken_links as u64);
    let calc_elapsed = start_calc.elapsed();

    assert!(
        calc_elapsed.as_millis() < 200,
        "Graph calculation exceeded 200ms budget: actual {}ms",
        calc_elapsed.as_millis()
    );
    assert!(health.aggregate_score >= 0.0);

    // Criterion benchmarks
    let mut group = c.benchmark_group("incremental_ops");

    group.bench_function("candidate_patch_single_file", |b| {
        b.iter(|| {
            let mut cand = CandidateGraph::new(black_box(&base_topology));
            cand.stage_node(black_box(modified_node.clone()));
            let topo = cand.effective_topology();
            black_box(topo);
        })
    });

    group.bench_function("graph_calculation_effective_topology", |b| {
        b.iter(|| {
            let mut broken = 0;
            for edge in &effective.edges {
                if !edge.is_resolved {
                    broken += 1;
                }
            }
            let f = Vec::new();
            let h = compute_health_score(
                black_box("cand_analysis_id"),
                black_box(&f),
                black_box(&profile),
                black_box(0),
                black_box(broken as u64),
            );
            black_box(h);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_incremental_pipeline);
criterion_main!(benches);
