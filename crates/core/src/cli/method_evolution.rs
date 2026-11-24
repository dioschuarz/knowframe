//! CLI workflow for profile-driven method evolution.

use crate::cli::scan::{execute_scan, ScanOptions};
use crate::config::{ContextVaultConfig, CustomProfileConfig};
use crate::evolution::{
    detect_all_signatures, evaluate_recommendations_for_signature, DocumentationOrientation,
    EvolutionEngine, ObjectiveLevel, ProfileCompiler, StructuralRecommendation, Tier1Objectives,
    Tier2Parameters, TransformationEngine, TransformationPlan,
};
use crate::graph::builder::GraphTopology;
use crate::model::NoteNode;
use serde_json::json;
use std::path::Path;

fn load_config(
    root: &Path,
) -> Result<ContextVaultConfig, Box<dyn std::error::Error + Send + Sync>> {
    Ok(ContextVaultConfig::load_from_workspace(root))
}

fn build_topology(root: &Path) -> Result<GraphTopology, Box<dyn std::error::Error + Send + Sync>> {
    let scan = execute_scan(ScanOptions {
        workspace_root: root.to_path_buf(),
        hub: None,
        pretty: false,
    })?;
    let mut topology = GraphTopology::new();
    for node in scan.nodes {
        let mut n = NoteNode::new(node.id.clone(), node.file_path, node.label);
        n.tags = node.tags;
        n.tokens = node.tokens;
        n.in_degree = node.in_degree;
        n.out_degree = node.out_degree;
        n.betweenness = node.betweenness;
        n.hop_distance = node.hop_distance;
        n.status = node.status;
        n.color = node.color;
        n.broken_links = node.broken_links;
        n.fragments = node.fragments;
        n.freshness = node.freshness;
        topology.add_node(n);
    }
    for edge in scan.edges {
        topology.add_edge(edge.to_note_edge());
    }
    Ok(topology)
}

fn all_recommendations(
    root: &Path,
    config: &ContextVaultConfig,
) -> Result<Vec<StructuralRecommendation>, Box<dyn std::error::Error + Send + Sync>> {
    let topology = build_topology(root)?;
    let profile = EvolutionEngine::compile_active_profile(
        config.get_orientation(),
        config.custom_profile.as_ref(),
    );
    let nodes: Vec<NoteNode> = topology.nodes.values().cloned().collect();
    let signatures = detect_all_signatures(&nodes, profile.fragment_size_target);
    Ok(signatures
        .iter()
        .flat_map(|signature| evaluate_recommendations_for_signature(signature, &profile))
        .collect())
}

fn parse_profile(value: &str) -> Result<DocumentationOrientation, String> {
    match value.to_ascii_lowercase().replace('_', "-").as_str() {
        "ai-oriented" | "ai" => Ok(DocumentationOrientation::AiOriented),
        "human-oriented" | "human" => Ok(DocumentationOrientation::HumanOriented),
        "balanced" => Ok(DocumentationOrientation::Balanced),
        "custom" => Ok(DocumentationOrientation::Custom),
        _ => Err(format!(
            "Unknown profile '{}'. Use ai-oriented, human-oriented, balanced, or custom.",
            value
        )),
    }
}

fn parse_level(value: &str) -> Result<ObjectiveLevel, String> {
    match value.to_ascii_lowercase().as_str() {
        "low" => Ok(ObjectiveLevel::Low),
        "medium" => Ok(ObjectiveLevel::Medium),
        "high" => Ok(ObjectiveLevel::High),
        _ => Err(format!(
            "Invalid objective level '{}'; use low, medium, or high.",
            value
        )),
    }
}

fn parse_set(value: &str) -> Result<(&str, &str), String> {
    value
        .split_once('=')
        .ok_or_else(|| format!("Invalid setting '{}'; expected key=value.", value))
}

fn apply_custom_setting(
    tier1: &mut Tier1Objectives,
    tier2: &mut Tier2Parameters,
    key: &str,
    value: &str,
) -> Result<(), String> {
    match key {
        "ai_discoverability"
        | "context_efficiency"
        | "human_readability"
        | "knowledge_reuse"
        | "narrative_coherence"
        | "maintenance_simplicity" => {
            let level = parse_level(value)?;
            match key {
                "ai_discoverability" => tier1.ai_discoverability = level,
                "context_efficiency" => tier1.context_efficiency = level,
                "human_readability" => tier1.human_readability = level,
                "knowledge_reuse" => tier1.knowledge_reuse = level,
                "narrative_coherence" => tier1.narrative_coherence = level,
                _ => tier1.maintenance_simplicity = level,
            }
        }
        "cohesion_threshold" => {
            tier2.cohesion_threshold = value.parse().map_err(|_| "Expected a number".to_string())?
        }
        "semantic_diversity_threshold" => {
            tier2.semantic_diversity_threshold =
                value.parse().map_err(|_| "Expected a number".to_string())?
        }
        "reference_concentration_threshold" => {
            tier2.reference_concentration_threshold =
                value.parse().map_err(|_| "Expected a number".to_string())?
        }
        "fragment_size_target" => {
            tier2.fragment_size_target = value
                .parse()
                .map_err(|_| "Expected an integer".to_string())?
        }
        "max_trace_depth" => {
            tier2.max_trace_depth = value
                .parse()
                .map_err(|_| "Expected an integer".to_string())?
        }
        "candidate_expansion_factor" => {
            tier2.candidate_expansion_factor = value
                .parse()
                .map_err(|_| "Expected an integer".to_string())?
        }
        "max_context_surface_tokens" => {
            tier2.max_context_surface_tokens = value
                .parse()
                .map_err(|_| "Expected an integer".to_string())?
        }
        "split_sensitivity" => {
            tier2.split_sensitivity = value.parse().map_err(|_| "Expected a number".to_string())?
        }
        "canonical_ownership_sensitivity" => {
            tier2.canonical_ownership_sensitivity =
                value.parse().map_err(|_| "Expected a number".to_string())?
        }
        "duplicate_tolerance" => {
            tier2.duplicate_tolerance =
                value.parse().map_err(|_| "Expected a number".to_string())?
        }
        "fragmentation_tolerance" => {
            tier2.fragmentation_tolerance =
                value.parse().map_err(|_| "Expected a number".to_string())?
        }
        _ => return Err(format!("Unknown method-evolution parameter '{}'.", key)),
    }
    Ok(())
}

pub fn execute_profile(
    root: &Path,
    profile: Option<&str>,
    settings: &[String],
    json_output: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = ContextVaultConfig::canonical_config_path(root);
    let mut config = load_config(root)?;
    if profile.is_none() && settings.is_empty() {
        let active = EvolutionEngine::compile_active_profile(
            config.get_orientation(),
            config.custom_profile.as_ref(),
        );
        let output = json!({ "profile": config.get_orientation(), "parameters": active });
        if json_output {
            println!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            println!(
                "Active method-evolution profile: {:?}\n{}",
                config.get_orientation(),
                serde_json::to_string_pretty(&active)?
            );
        }
        return Ok(());
    }

    let selected = profile
        .map(parse_profile)
        .transpose()
        .map_err(std::io::Error::other)?
        .unwrap_or(DocumentationOrientation::Custom);
    config.orientation = Some(selected);
    if selected == DocumentationOrientation::Custom || !settings.is_empty() {
        let existing = config.custom_profile.clone().unwrap_or_default();
        let mut tier1 = existing.tier1.unwrap_or_default();
        let mut tier2 = existing
            .tier2
            .unwrap_or_else(|| ProfileCompiler::new().map_tier1_to_tier2(&tier1));
        for item in settings {
            let (key, value) = parse_set(item).map_err(std::io::Error::other)?;
            apply_custom_setting(&mut tier1, &mut tier2, key, value)
                .map_err(std::io::Error::other)?;
        }
        config.custom_profile = Some(CustomProfileConfig {
            tier1: Some(tier1),
            tier2: Some(tier2),
            constraints: existing.constraints,
        });
    }
    config.save_to_file(&path)?;
    println!(
        "Saved method-evolution profile {:?} to {}",
        selected,
        path.display()
    );
    Ok(())
}

pub fn execute_suggestions(
    root: &Path,
    limit: Option<usize>,
    json_output: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = load_config(root)?;
    let mut recommendations = all_recommendations(root, &config)?;
    if let Some(limit) = limit {
        recommendations.truncate(limit);
    }
    if json_output {
        println!("{}", serde_json::to_string_pretty(&recommendations)?);
        return Ok(());
    }
    if recommendations.is_empty() {
        println!("No method-evolution suggestions found.");
        return Ok(());
    }
    for recommendation in recommendations {
        println!(
            "{} | {:?} | confidence {:.2} | {}",
            recommendation.recommendation_id,
            recommendation.strategy,
            recommendation.confidence,
            recommendation.expected_benefits.join("; ")
        );
    }
    Ok(())
}

fn find_plan(
    root: &Path,
    recommendation_id: &str,
) -> Result<TransformationPlan, Box<dyn std::error::Error + Send + Sync>> {
    let config = load_config(root)?;
    let topology = build_topology(root)?;
    let profile = EvolutionEngine::compile_active_profile(
        config.get_orientation(),
        config.custom_profile.as_ref(),
    );
    let nodes: Vec<NoteNode> = topology.nodes.values().cloned().collect();
    for signature in detect_all_signatures(&nodes, profile.fragment_size_target) {
        for recommendation in evaluate_recommendations_for_signature(&signature, &profile) {
            if recommendation.recommendation_id == recommendation_id {
                let node = nodes
                    .iter()
                    .find(|n| {
                        n.id == signature.document_path || n.file_path == signature.document_path
                    })
                    .ok_or_else(|| std::io::Error::other("Recommendation document not found"))?;
                return Ok(TransformationEngine::build_plan(
                    &recommendation,
                    node,
                    root,
                ));
            }
        }
    }
    Err(std::io::Error::other(format!("Recommendation '{}' not found", recommendation_id)).into())
}

pub fn execute_plan(
    root: &Path,
    recommendation_id: &str,
    json_output: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let plan = find_plan(root, recommendation_id)?;
    if json_output {
        println!("{}", serde_json::to_string_pretty(&plan)?);
    } else {
        println!(
            "Plan {}\n{}\nActions: {}",
            plan.plan_id,
            plan.diff_preview,
            plan.actions.len()
        );
    }
    Ok(())
}

pub fn execute_apply(
    root: &Path,
    recommendation_id: &str,
    approve: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !approve {
        return Err(std::io::Error::other("Applying a plan requires --approve").into());
    }
    let plan = find_plan(root, recommendation_id)?;
    let delta = TransformationEngine::apply_plan_atomic(&plan, root)?;
    println!("Applied and validated {} ({:?})", plan.plan_id, delta);
    Ok(())
}
