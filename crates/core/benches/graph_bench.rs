use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{LinkType, NoteEdge, NoteNode};

fn generate_benchmark_topology(num_nodes: usize) -> GraphTopology {
    let mut topology = GraphTopology::new();

    for i in 0..num_nodes {
        let id = format!("note_{:04}.md", i);
        let path = format!("/vault/note_{:04}.md", i);
        let label = format!("Note {:04}", i);

        let mut node = NoteNode::new(&id, path, label);
        node.tokens = 150;
        node.tags = vec!["benchmark".to_string(), format!("cluster_{}", i % 5)];
        topology.add_node(node);

        topology.set_headings(
            id.clone(),
            vec!["Introduction".to_string(), "Design Details".to_string()],
        );
    }

    // Add ~3 edges per node simulating scale-free / connected wiki graph
    for i in 0..num_nodes {
        let src = format!("note_{:04}.md", i);
        let tgt1 = format!("note_{:04}.md", (i + 1) % num_nodes);
        let tgt2 = format!("note_{:04}.md", (i * 7) % num_nodes);

        topology.add_edge(NoteEdge::new(
            &src,
            &tgt1,
            LinkType::Wikilink,
            format!("[[{}]]", tgt1),
            Some("design-details".to_string()),
            Some(10),
        ));

        if tgt2 != src && tgt2 != tgt1 {
            topology.add_edge(NoteEdge::new(
                &src,
                &tgt2,
                LinkType::Wikilink,
                format!("[[{}]]", tgt2),
                None,
                Some(25),
            ));
        }
    }

    topology
}

fn bench_graph_topology(c: &mut Criterion) {
    let base_topology = generate_benchmark_topology(1_000);

    c.bench_function("graph_resolve_links_1000_notes", |b| {
        b.iter(|| {
            let mut top = base_topology.clone();
            top.resolve_links();
            black_box(top);
        });
    });

    c.bench_function("graph_compute_betweenness_1000_notes", |b| {
        let mut top = base_topology.clone();
        top.resolve_links();
        b.iter(|| {
            let mut t = top.clone();
            t.compute_betweenness();
            black_box(t);
        });
    });

    c.bench_function("graph_bfs_hops_1000_notes", |b| {
        let mut top = base_topology.clone();
        top.resolve_links();
        b.iter(|| {
            let mut t = top.clone();
            t.compute_hops("note_0000.md");
            black_box(t);
        });
    });
}

criterion_group!(benches, bench_graph_topology);
criterion_main!(benches);
