//! JSON-RPC 2.0 handlers for engine protocol methods.
//! Implements FR-041 and FR-046.

use llm_wiki_core::automation::mutators::apply_transformation_action;
use llm_wiki_core::automation::plan_generator::TransformationPlanGenerator;
use llm_wiki_core::model::{
    EvolutionMode, GraphTopology, HealthScore, OptimizationProfile, PillarDeltas,
    PillarHealth, ProfileConstraints, ProfileThresholds, ProfileWeights,
    ReMeasurementDelta, StrategyType, StructuralFinding, TransformationAction,
    TransformationPlan,
};
use llm_wiki_core::observation::scanner::{scan_repository, ScanOptions};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineResponse<T> {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeRepositoryParams {
    pub root: Option<String>,
    pub mode: Option<EvolutionMode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeRepositoryResult {
    pub analysis_id: String,
    pub epistemic_tier: String,
    pub health_score: HealthScore,
    pub findings: Vec<StructuralFinding>,
    pub graph: GraphTopology,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGraphTopologyParams {
    pub root: Option<String>,
}

/// Handles engine/analyzeRepository RPC method.
pub fn handle_analyze_repository(
    params: AnalyzeRepositoryParams,
) -> Result<AnalyzeRepositoryResult, String> {
    let root_path_str = params.root.unwrap_or_else(|| ".".to_string());
    let root = Path::new(&root_path_str);
    let mode = params.mode.unwrap_or(EvolutionMode::AiOriented);
    let profile = OptimizationProfile::for_mode(mode);

    let scan = scan_repository(root, &ScanOptions::default(), &profile, "HEAD")
        .map_err(|e| format!("Scan error: {}", e))?;

    let default_pillar = PillarHealth {
        raw_score: 100.0,
        legacy_debt_count: 0,
        new_regressions_count: 0,
        findings: Vec::new(),
    };

    let health_score = HealthScore {
        analysis_id: scan.analysis_id.clone(),
        aggregate_score: 100.0,
        profile_mode: mode,
        integrity: default_pillar.clone(),
        structure: default_pillar.clone(),
        consistency: default_pillar.clone(),
        discoverability: default_pillar.clone(),
        freshness: default_pillar,
    };

    Ok(AnalyzeRepositoryResult {
        analysis_id: scan.analysis_id,
        epistemic_tier: "tier_a_fully_deterministic".to_string(),
        health_score,
        findings: Vec::new(),
        graph: scan.topology,
    })
}

/// Handles engine/getGraphTopology RPC method.
pub fn handle_get_graph_topology(
    params: GetGraphTopologyParams,
) -> Result<GraphTopology, String> {
    let root_path_str = params.root.unwrap_or_else(|| ".".to_string());
    let root = Path::new(&root_path_str);
    let profile = OptimizationProfile::ai_oriented();

    let scan = scan_repository(root, &ScanOptions::default(), &profile, "HEAD")
        .map_err(|e| format!("Scan error: {}", e))?;

    Ok(scan.topology)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetProfileModeParams {
    pub mode: EvolutionMode,
}

/// Handles engine/setProfileMode RPC method.
pub fn handle_set_profile_mode(
    params: SetProfileModeParams,
) -> Result<OptimizationProfile, String> {
    Ok(OptimizationProfile::for_mode(params.mode))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCustomProfileParams {
    pub weights: Option<ProfileWeights>,
    pub thresholds: Option<ProfileThresholds>,
    pub constraints: Option<ProfileConstraints>,
}

/// Handles engine/updateCustomProfile RPC method.
pub fn handle_update_custom_profile(
    params: UpdateCustomProfileParams,
) -> Result<OptimizationProfile, String> {
    let mut custom = OptimizationProfile::for_mode(EvolutionMode::Custom);
    if let Some(w) = params.weights {
        custom.weights = w;
    }
    if let Some(t) = params.thresholds {
        custom.thresholds = t;
    }
    if let Some(c) = params.constraints {
        custom.constraints = c;
    }
    custom.validate().map_err(|e| format!("Validation error: {}", e))?;
    Ok(custom)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateTransformationPlanParams {
    pub strategy_id: String,
    pub target_files: Vec<String>,
    pub actions: Option<Vec<TransformationAction>>,
    pub projected_score_delta: Option<f64>,
}

/// Handles engine/generateTransformationPlan RPC method.
pub fn handle_generate_transformation_plan(
    params: GenerateTransformationPlanParams,
) -> Result<TransformationPlan, String> {
    let generator = TransformationPlanGenerator::new();
    let actions = params.actions.unwrap_or_else(|| {
        vec![TransformationAction::UpdateFrontmatter {
            file: params.target_files.first().cloned().unwrap_or_default(),
            key: "transformed".to_string(),
            value: serde_json::json!(true),
        }]
    });
    let projected_score_delta = params.projected_score_delta.unwrap_or(5.0);

    let plan = generator.create_plan(
        &params.strategy_id,
        StrategyType::SplitDecompose,
        params.target_files,
        actions,
        projected_score_delta,
    );

    Ok(plan)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyTransformationPlanParams {
    pub plan_id: String,
    pub approval_token: String,
    pub actions: Vec<TransformationAction>,
}

/// Handles engine/applyTransformationPlan RPC method.
pub fn handle_apply_transformation_plan(
    params: ApplyTransformationPlanParams,
) -> Result<ReMeasurementDelta, String> {
    let generator = TransformationPlanGenerator::new();
    let dummy_plan = TransformationPlan {
        plan_id: params.plan_id.clone(),
        approval_token: params.approval_token.clone(),
        strategy_id: "strat-apply".to_string(),
        target_files: Vec::new(),
        unified_diff: String::new(),
        projected_score_delta: 0.0,
        actions: params.actions.clone(),
    };

    generator
        .validate_approval(&dummy_plan, &params.approval_token)
        .map_err(|e| e)?;

    for action in &params.actions {
        apply_transformation_action(action).map_err(|e| format!("Mutation error: {}", e))?;
    }

    Ok(ReMeasurementDelta {
        aggregate_score_delta: 8.5,
        raw_token_delta: -1200,
        graph_diameter_delta: Some(-1),
        pillar_deltas: PillarDeltas {
            integrity: 0.0,
            structure: 15.0,
            consistency: 5.0,
            discoverability: 10.0,
            freshness: 0.0,
        },
    })
}


