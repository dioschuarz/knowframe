use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::candidate::evaluator::{evaluate_candidate, AdmissionConfig};
use llm_wiki_core::candidate::overlay::CandidateOverlay;
use llm_wiki_core::debt::baseline::capture_initial_baseline;
use llm_wiki_core::delta::updater::{FileDeltaItem, FileDeltaStatus};
use llm_wiki_core::delta::GitDelta;
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{LinkType, NoteEdge, NoteNode};
use tempfile::tempdir;

fn generate_benchmark_topology(num_nodes: usize) -> GraphTopology {
    let mut topology = GraphTopology::new();

    for i in 0..num_nodes {
        let id = format!("note_{:04}.md", i);
        let path = format!("/vault/note_{:04}.md", i);
        let label = format!("Note {:04}", i);

        let mut node = NoteNode::new(&id, path, label);
        node.tokens = 200;
        topology.add_node(node);
    }

    for i in 0..num_nodes {
        let src = format!("note_{:04}.md", i);
        let tgt = format!("note_{:04}.md", (i + 1) % num_nodes);
        topology.add_edge(NoteEdge::new(
            &src,
            &tgt,
            LinkType::Wikilink,
            format!("[[{}]]", tgt),
            None,
            Some(10),
        ));
    }

    topology.resolve_links();
    topology
}

fn bench_candidate_evaluation(c: &mut Criterion) {
    let base_graph = generate_benchmark_topology(1_000);
    let baseline = capture_initial_baseline(&base_graph, "base_sha", vec![], vec![]);

    let dir = tempdir().unwrap();
    let root = dir.path();

    let staged_delta = GitDelta {
        commit_sha: Some("cand_sha".to_string()),
        items: vec![
            FileDeltaItem {
                path: "note_0005.md".to_string(),
                status: FileDeltaStatus::Modified,
            },
            FileDeltaItem {
                path: "note_new.md".to_string(),
                status: FileDeltaStatus::Added,
            },
        ],
    };

    let overlay = CandidateOverlay::new(&base_graph, root, &staged_delta);
    let config = AdmissionConfig::default();

    c.bench_function("candidate_evaluation_1000_notes", |b| {
        b.iter(|| {
            let report = evaluate_candidate(
                black_box(&overlay),
                black_box(&baseline),
                black_box(vec![]),
                black_box(vec![]),
                black_box(config),
            );
            black_box(report);
        });
    });
}

criterion_group!(benches, bench_candidate_evaluation);
criterion_main!(benches);
