use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::delta::updater::{apply_file_delta, FileDeltaItem, FileDeltaStatus};
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
        node.tokens = 150;
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

fn bench_post_commit_sync(c: &mut Criterion) {
    let base_topology = generate_benchmark_topology(1_000);
    let dir = tempdir().unwrap();
    let root = dir.path();

    let items = vec![
        FileDeltaItem {
            path: "note_0010.md".to_string(),
            status: FileDeltaStatus::Modified,
        },
        FileDeltaItem {
            path: "note_0020.md".to_string(),
            status: FileDeltaStatus::Deleted,
        },
    ];

    c.bench_function("post_commit_sync_delta_1000_notes", |b| {
        b.iter(|| {
            let mut top = base_topology.clone();
            apply_file_delta(
                black_box(&mut top),
                black_box(root),
                black_box(&items),
                black_box(Some("note_0000.md")),
            );
            black_box(top);
        });
    });
}

criterion_group!(benches, bench_post_commit_sync);
criterion_main!(benches);
