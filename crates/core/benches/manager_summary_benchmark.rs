use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::evolution::{compute_manager_summary, DocumentationOrientation};
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{LinkType, NoteEdge, NoteNode};
use tempfile::tempdir;

fn bench_manager_summary(c: &mut Criterion) {
    let mut topology = GraphTopology::new();

    // 1,000 documents
    for i in 0..1000 {
        let node = NoteNode::new(
            format!("doc_{}.md", i),
            format!("/vault/doc_{}.md", i),
            format!("Document {}", i),
        );
        topology.add_node(node);
    }

    // 2,000 references
    for i in 0..1000 {
        let target = (i + 1) % 1000;
        let edge = NoteEdge::new(
            format!("doc_{}.md", i),
            format!("doc_{}.md", target),
            LinkType::Wikilink,
            "wikilink".to_string(),
            None,
            Some(5),
        );
        topology.add_edge(edge);
    }

    let temp = tempdir().unwrap();
    let root = temp.path();

    c.bench_function("manager_summary_1000_docs", |b| {
        b.iter(|| {
            black_box(compute_manager_summary(
                &topology,
                root,
                DocumentationOrientation::AiOriented,
                &[],
            ));
        });
    });
}

criterion_group!(benches, bench_manager_summary);
criterion_main!(benches);
