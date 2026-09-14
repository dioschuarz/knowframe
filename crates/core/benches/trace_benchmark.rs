use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::fragment::FragmentMetadata;
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{LinkType, NoteEdge, NoteNode};
use llm_wiki_core::trace::beam::trace_context_section;
use llm_wiki_core::trace::traversal::execute_context_trace;

fn generate_benchmark_topology(num_nodes: usize) -> GraphTopology {
    let mut topology = GraphTopology::new();

    // Entrypoint node
    let mut entry = NoteNode::new("AGENTS.md", "/vault/AGENTS.md", "Agent Operating System");
    entry.tokens = 500;
    topology.add_node(entry);

    for i in 0..num_nodes {
        let id = format!("note_{:04}.md", i);
        let path = format!("/vault/note_{:04}.md", i);
        let label = format!("Service Architecture Module {:04}", i);

        let mut node = NoteNode::new(&id, path, label);
        node.tokens = 2500;
        node.tags = vec!["architecture".to_string(), format!("cluster_{}", i % 10)];
        node.fragments = vec![
            FragmentMetadata {
                anchor: "overview".to_string(),
                heading: "Overview".to_string(),
                start_line: 1,
                end_line: 25,
                tokens: 300,
                inbound: 2,
                outbound: 1,
            },
            FragmentMetadata {
                anchor: format!("technical-specification-{:04}", i),
                heading: format!("Technical Specification {:04}", i),
                start_line: 26,
                end_line: 80,
                tokens: 450,
                inbound: 5,
                outbound: 3,
            },
        ];
        topology.add_node(node);

        topology.set_headings(
            id.clone(),
            vec![
                "Overview".to_string(),
                format!("Technical Specification {:04}", i),
            ],
        );
    }

    // Connect entrypoint to first 5 nodes
    for i in 0..5 {
        let tgt = format!("note_{:04}.md", i);
        topology.add_edge(NoteEdge::new(
            "AGENTS.md",
            &tgt,
            LinkType::Wikilink,
            format!("[[{}]]", tgt),
            None,
            Some(1),
        ));
    }

    // Create a chain and branch network across 1,000 nodes
    for i in 0..num_nodes {
        let src = format!("note_{:04}.md", i);
        let tgt1 = format!("note_{:04}.md", (i + 1) % num_nodes);
        let tgt2 = format!("note_{:04}.md", (i * 13) % num_nodes);

        topology.add_edge(NoteEdge::new(
            &src,
            &tgt1,
            LinkType::Wikilink,
            format!("[[{}]]", tgt1),
            None,
            Some(10),
        ));

        if tgt2 != src && tgt2 != tgt1 {
            topology.add_edge(NoteEdge::new(
                &src,
                &tgt2,
                LinkType::Wikilink,
                format!("[[{}]]", tgt2),
                None,
                Some(20),
            ));
        }
    }

    topology.resolve_links();
    topology
}

fn bench_trace_traversal(c: &mut Criterion) {
    let topology = generate_benchmark_topology(1_000);

    c.bench_function("trace_dijkstra_1000_notes", |b| {
        b.iter(|| {
            let res = execute_context_trace(
                black_box(&topology),
                black_box("Technical Specification 0500"),
                black_box(Some("AGENTS.md")),
            );
            black_box(res);
        });
    });

    c.bench_function("trace_beam_section_1000_notes", |b| {
        b.iter(|| {
            let res = trace_context_section(
                black_box(&topology),
                black_box("Technical Specification 0500"),
                black_box(Some("AGENTS.md")),
            );
            black_box(res);
        });
    });
}

criterion_group!(benches, bench_trace_traversal);
criterion_main!(benches);
