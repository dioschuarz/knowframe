use std::path::Path;
use llm_wiki_core::config::ContextVaultConfig;
use llm_wiki_core::evolution::{EvolutionEngine, detect_all_signatures};
use llm_wiki_core::cli::scan::{execute_scan, ScanOptions};
use llm_wiki_core::model::NoteNode;

fn main() {
    let root = Path::new("/home/ds/projects");
    let scan_res = execute_scan(ScanOptions {
        workspace_root: root.to_path_buf(),
        hub: None,
        pretty: false,
    }).unwrap();
    let config = ContextVaultConfig::load_from_workspace(root);
    let profile = EvolutionEngine::compile_active_profile(
        config.get_orientation(),
        config.custom_profile.as_ref(),
    );
    let nodes: Vec<NoteNode> = scan_res.nodes.into_iter().map(|n| NoteNode {
        id: n.id,
        file_path: n.file_path,
        label: n.label,
        tags: n.tags,
        tokens: n.tokens,
        in_degree: n.in_degree,
        out_degree: n.out_degree,
        betweenness: n.betweenness,
        hop_distance: n.hop_distance,
        status: n.status,
        color: n.color,
        broken_links: n.broken_links,
        fragments: n.fragments,
        freshness: None,
        code_dependencies: vec![],
    }).collect();
    let sigs = detect_all_signatures(&nodes, profile.fragment_size_target);
    println!("Number of signatures: {}", sigs.len());
    for s in &sigs {
        println!("Sig: {} {:?}", s.document_path, s.problem_type);
    }
}
