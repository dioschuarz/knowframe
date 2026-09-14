use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::evolution::extract_metrics_parallel;
use llm_wiki_core::model::NoteNode;

fn generate_sample_nodes(count: usize) -> Vec<NoteNode> {
    let mut nodes = Vec::with_capacity(count);
    for i in 0..count {
        let mut node = NoteNode::new(
            format!("docs/node_{:04}.md", i),
            format!("/workspace/docs/node_{:04}.md", i),
            format!("Node {:04}", i),
        );
        node.tokens = 1000 + (i % 5000) as u32;
        node.in_degree = (i % 25) as u32;
        node.out_degree = ((i * 3) % 30) as u32;
        node.betweenness = ((i % 100) as f64) / 100.0;
        node.hop_distance = (i % 5) as i32;
        nodes.push(node);
    }
    nodes
}

fn bench_metrics_extraction_1000_files(c: &mut Criterion) {
    let nodes = generate_sample_nodes(1000);

    c.bench_function("metrics_extraction_1000_files_parallel", |b| {
        b.iter(|| {
            black_box(extract_metrics_parallel(&nodes, 5000));
        });
    });
}

criterion_group!(benches, bench_metrics_extraction_1000_files);
criterion_main!(benches);
