//! Integration, unit, and contract tests for Strategy Selection & Combination (Feature 007).
//! Implements T006, T007, T008, T016.

use llm_wiki_core::evolution::compiler::ProfileCompiler;
use llm_wiki_core::evolution::metrics::StructuralMetrics;
use llm_wiki_core::evolution::profile::DocumentationOrientation;
use llm_wiki_core::evolution::signatures::{
    ProblemSignatureType, StructuralProblemSignature,
};
use llm_wiki_core::evolution::strategies::{
    calculate_integer_fit_score, get_strategy_relationship, is_strategy_eligible,
    select_strategy_set, KnowledgeOrganizationStrategy, StrategyRelationshipType, StrategyRole,
    FIT_ELIGIBILITY_THRESHOLD, STANDARD_CANDIDATE_STRATEGIES,
};

fn mock_god_document_signature() -> StructuralProblemSignature {
    StructuralProblemSignature {
        signature_id: "sig_god_doc_001".to_string(),
        document_path: "architecture/core_monolith.md".to_string(),
        problem_type: ProblemSignatureType::GodDocument,
        confidence: 0.95,
        evidence: StructuralMetrics {
            cohesion: 0.22,
            coupling: 0.65,
            reference_concentration: 0.85,
            granularity: 52000.0,
            context_efficiency: 0.18,
            discoverability: 0.40,
            reusability: 0.35,
            semantic_diversity: 0.88,
            navigational_complexity: 0.72,
            maintenance_coupling: 0.60,
        },
        rationale: "Oversized document with extreme semantic diversity and high reference concentration.".to_string(),
    }
}

fn mock_healthy_monolith_signature() -> StructuralProblemSignature {
    StructuralProblemSignature {
        signature_id: "sig_healthy_mono_001".to_string(),
        document_path: "specs/stable_spec.md".to_string(),
        problem_type: ProblemSignatureType::HealthyMonolith,
        confidence: 0.90,
        evidence: StructuralMetrics {
            cohesion: 0.92,
            coupling: 0.15,
            reference_concentration: 0.10,
            granularity: 45000.0,
            context_efficiency: 0.85,
            discoverability: 0.80,
            reusability: 0.75,
            semantic_diversity: 0.15,
            navigational_complexity: 0.20,
            maintenance_coupling: 0.10,
        },
        rationale: "Large monolithic document with high internal cohesion and narrative integrity.".to_string(),
    }
}

#[test]
fn test_candidate_strategy_evaluation_and_eligibility_filter_t006() {
    let compiler = ProfileCompiler::new();
    let profile = compiler.compile_preset(DocumentationOrientation::AiOriented);
    let sig = mock_god_document_signature();

    assert_eq!(STANDARD_CANDIDATE_STRATEGIES.len(), 4);
    assert_eq!(FIT_ELIGIBILITY_THRESHOLD, 60);

    for &strat in &STANDARD_CANDIDATE_STRATEGIES {
        let fit = calculate_integer_fit_score(strat, &sig, &profile);
        assert!(fit <= 100, "Fit score must be <= 100, got {}", fit);
        if fit >= 60 {
            assert!(is_strategy_eligible(fit));
        } else {
            assert!(!is_strategy_eligible(fit));
        }
    }
}

#[test]
fn test_pairwise_relationship_matrix_classification_t007() {
    use KnowledgeOrganizationStrategy::*;
    use StrategyRelationshipType::*;

    // Canonical complementary pairs
    assert_eq!(
        get_strategy_relationship(SectionAddressing, LightweightIndex),
        Complementary
    );
    assert_eq!(
        get_strategy_relationship(LightweightIndex, SectionAddressing),
        Complementary
    );
    assert_eq!(
        get_strategy_relationship(FacetedClassification, LightweightIndex),
        Complementary
    );
    assert_eq!(
        get_strategy_relationship(LightweightIndex, FacetedClassification),
        Complementary
    );

    // Canonical alternative pair
    assert_eq!(
        get_strategy_relationship(SectionAddressing, FacetedClassification),
        Alternative
    );
    assert_eq!(
        get_strategy_relationship(FacetedClassification, SectionAddressing),
        Alternative
    );

    // PreserveDocument is strictly MutuallyExclusive with all transformation strategies
    assert_eq!(
        get_strategy_relationship(PreserveDocument, SectionAddressing),
        MutuallyExclusive
    );
    assert_eq!(
        get_strategy_relationship(SectionAddressing, PreserveDocument),
        MutuallyExclusive
    );
    assert_eq!(
        get_strategy_relationship(PreserveDocument, LightweightIndex),
        MutuallyExclusive
    );
    assert_eq!(
        get_strategy_relationship(PreserveDocument, FacetedClassification),
        MutuallyExclusive
    );

    // Self-pair is MutuallyExclusive (cannot combine with itself)
    assert_eq!(
        get_strategy_relationship(SectionAddressing, SectionAddressing),
        MutuallyExclusive
    );
    assert_eq!(
        get_strategy_relationship(PreserveDocument, PreserveDocument),
        MutuallyExclusive
    );
}

#[test]
fn test_strategy_set_generation_god_document_t009() {
    let compiler = ProfileCompiler::new();
    let profile = compiler.compile_preset(DocumentationOrientation::AiOriented);
    let sig = mock_god_document_signature();

    let strategy_set = select_strategy_set(
        "opp_001",
        "architecture/core_monolith.md",
        &sig,
        &profile,
    );

    assert_eq!(strategy_set.opportunity_id, "opp_001");
    assert_eq!(strategy_set.target_document, "architecture/core_monolith.md");

    // Exactly one PRIMARY
    assert_eq!(strategy_set.primary.role, StrategyRole::Primary);
    assert!(strategy_set.primary.fit_score >= 60);

    // Supporting strategies must be COMPLEMENTARY to primary
    for sup in &strategy_set.supporting {
        assert_eq!(sup.role, StrategyRole::Supporting);
        assert!(sup.fit_score >= 60);
        assert_eq!(
            get_strategy_relationship(strategy_set.primary.strategy, sup.strategy),
            StrategyRelationshipType::Complementary
        );
    }

    // Alternatives must not contain complementary strategies
    for alt in &strategy_set.alternatives {
        assert_eq!(alt.role, StrategyRole::Alternative);
        assert!(alt.fit_score >= 60);
        let rel = get_strategy_relationship(strategy_set.primary.strategy, alt.strategy);
        assert!(
            rel == StrategyRelationshipType::Alternative
                || rel == StrategyRelationshipType::MutuallyExclusive
        );
    }

    // Active selection defaults to Primary + Supporting
    assert!(strategy_set.active_selection.contains(&strategy_set.primary.strategy));
    for sup in &strategy_set.supporting {
        assert!(strategy_set.active_selection.contains(&sup.strategy));
    }
}

#[test]
fn test_healthy_monolith_preserve_document_fallback_t010() {
    let compiler = ProfileCompiler::new();
    let profile = compiler.compile_preset(DocumentationOrientation::Balanced);
    let sig = mock_healthy_monolith_signature();

    let strategy_set = select_strategy_set(
        "opp_healthy_001",
        "specs/stable_spec.md",
        &sig,
        &profile,
    );

    // Edge Case 4: Healthy Monolith has PreserveDocument as Primary
    assert_eq!(
        strategy_set.primary.strategy,
        KnowledgeOrganizationStrategy::PreserveDocument
    );
    assert_eq!(strategy_set.primary.role, StrategyRole::Primary);

    // Supporting is strictly empty when PreserveDocument is Primary (FR-006)
    assert!(
        strategy_set.supporting.is_empty(),
        "PreserveDocument cannot have supporting transformation strategies"
    );

    // Transformation strategies appear as alternatives
    for alt in &strategy_set.alternatives {
        assert_ne!(
            alt.strategy,
            KnowledgeOrganizationStrategy::PreserveDocument
        );
    }
}

#[tokio::test]
async fn test_get_strategy_set_rpc_handler_t008() {
    use llm_wiki_core::ipc::handlers::{handle_request, AppState};
    use llm_wiki_core::ipc::protocol::JsonRpcRequest;
    use llm_wiki_core::model::NoteNode;
    use serde_json::json;
    use std::sync::Arc;
    use tempfile::tempdir;
    use tokio::sync::RwLock;

    let temp = tempdir().unwrap();
    let state = Arc::new(RwLock::new(AppState::new(temp.path().to_path_buf())));

    // Create an orphan node to trigger an IsolatedSilo opportunity
    let mut node = NoteNode::new(
        "docs/orphan.md".to_string(),
        temp.path().join("docs/orphan.md").to_string_lossy().to_string(),
        "Orphan Doc".to_string(),
    );
    node.hop_distance = -1;
    {
        let mut st = state.write().await;
        st.topology.add_node(node);
    }

    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(1),
        method: "evolution/getStrategySet".to_string(),
        params: Some(json!({
            "opportunity_id": "silo_docs_orphan.md"
        })),
    };

    let res = handle_request(&state, req).await;
    assert!(res.error.is_none(), "Handler should succeed: {:?}", res.error);
    let result = res.result.expect("Expected result payload");

    assert_eq!(result.get("opportunity_id").and_then(|v| v.as_str()), Some("silo_docs_orphan.md"));
    assert!(result.get("primary").is_some(), "Must have primary strategy");
    assert!(result.get("supporting").is_some(), "Must have supporting list");
    assert!(result.get("alternatives").is_some(), "Must have alternatives list");
    assert!(result.get("composite_plan_value").is_some(), "Must have composite plan value");
    assert!(result.get("active_selection").is_some(), "Must have active selection");
}

#[tokio::test]
async fn test_validate_strategy_selection_rpc_handler_t016() {
    use llm_wiki_core::ipc::handlers::{handle_request, AppState};
    use llm_wiki_core::ipc::protocol::JsonRpcRequest;
    use serde_json::json;
    use std::sync::Arc;
    use tempfile::tempdir;
    use tokio::sync::RwLock;

    let temp = tempdir().unwrap();
    let state = Arc::new(RwLock::new(AppState::new(temp.path().to_path_buf())));

    // Test 1: Complementary pair -> valid
    let req_valid = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(2),
        method: "evolution/validateStrategySelection".to_string(),
        params: Some(json!({
            "selected_strategies": ["section_addressing", "lightweight_index"]
        })),
    };
    let res_valid = handle_request(&state, req_valid).await;
    assert!(res_valid.error.is_none());
    let res_valid_val = res_valid.result.unwrap();
    assert_eq!(res_valid_val.get("is_valid").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(res_valid_val.get("incompatible_pairs").and_then(|v| v.as_array()).map(|a| a.len()), Some(0));

    // Test 2: Incompatible pair -> invalid with pair details
    let req_invalid = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(3),
        method: "evolution/validateStrategySelection".to_string(),
        params: Some(json!({
            "selected_strategies": ["section_addressing", "faceted_classification"]
        })),
    };
    let res_invalid = handle_request(&state, req_invalid).await;
    assert!(res_invalid.error.is_none());
    let res_invalid_val = res_invalid.result.unwrap();
    assert_eq!(res_invalid_val.get("is_valid").and_then(|v| v.as_bool()), Some(false));
    let pairs = res_invalid_val.get("incompatible_pairs").and_then(|v| v.as_array()).unwrap();
    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs[0].get("relationship").and_then(|v| v.as_str()), Some("alternative"));
}

#[tokio::test]
async fn test_get_structural_problems_attaches_strategy_set() {
    use llm_wiki_core::ipc::handlers::{handle_request, AppState};
    use llm_wiki_core::ipc::protocol::JsonRpcRequest;
    use llm_wiki_core::model::NoteNode;
    use serde_json::json;
    use std::sync::Arc;
    use tempfile::tempdir;
    use tokio::sync::RwLock;

    let temp = tempdir().unwrap();
    let state = Arc::new(RwLock::new(AppState::new(temp.path().to_path_buf())));

    let mut node = NoteNode::new(
        "docs/orphan.md".to_string(),
        temp.path().join("docs/orphan.md").to_string_lossy().to_string(),
        "Orphan Doc".to_string(),
    );
    node.hop_distance = -1;
    {
        let mut st = state.write().await;
        st.topology.add_node(node);
    }

    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(4),
        method: "analytics/getStructuralProblems".to_string(),
        params: None,
    };

    let res = handle_request(&state, req).await;
    assert!(res.error.is_none());
    let val = res.result.unwrap();
    let sigs = val.get("signatures").and_then(|v| v.as_array()).expect("Must have signatures");
    assert!(!sigs.is_empty(), "Must have detected orphan as isolated silo");

    let first_sig = &sigs[0];
    assert!(first_sig.get("strategy_set").is_some(), "Signature must have strategy_set attached");
    let strat_set = first_sig.get("strategy_set").unwrap();
    assert!(strat_set.get("primary").is_some(), "Strategy set must have primary");
    assert!(strat_set.get("supporting").is_some(), "Strategy set must have supporting");
    assert!(strat_set.get("alternatives").is_some(), "Strategy set must have alternatives");
    assert!(strat_set.get("composite_plan_value").is_some(), "Strategy set must have composite_plan_value");
}

#[tokio::test]
async fn test_transformation_plan_deserialization_and_apply_robustness() {
    use llm_wiki_core::ipc::handlers::{handle_request, AppState};
    use llm_wiki_core::ipc::protocol::JsonRpcRequest;
    use llm_wiki_core::model::NoteNode;
    use serde_json::json;
    use std::sync::Arc;
    use tempfile::tempdir;
    use tokio::sync::RwLock;

    let temp = tempdir().unwrap();
    let root = temp.path().to_path_buf();
    let doc_path = root.join("specs/doc.md");
    std::fs::create_dir_all(doc_path.parent().unwrap()).unwrap();
    std::fs::write(&doc_path, "# Document\n\nContent here.\n").unwrap();

    let state = Arc::new(RwLock::new(AppState::new(root.clone())));
    let node = NoteNode::new(
        "specs/doc.md".to_string(),
        doc_path.to_string_lossy().to_string(),
        "Document".to_string(),
    );
    {
        let mut st = state.write().await;
        st.topology.add_node(node);
    }

    // 1. Generate plan using multi-strategy
    let gen_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(10),
        method: "evolution/generatePlan".to_string(),
        params: Some(json!({
            "opportunityId": "silo_specs_doc.md",
            "selectedStrategies": ["section_addressing", "lightweight_index", "faceted_classification"]
        })),
    };
    let gen_res = handle_request(&state, gen_req).await;
    assert!(gen_res.error.is_none(), "Generation must succeed: {:?}", gen_res.error);
    let plan_json = gen_res.result.unwrap();

    // 2. Apply plan passing the JSON object directly (with camelCase and without recommendation_id)
    let apply_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(11),
        method: "transformation/applyPlan".to_string(),
        params: Some(json!({
            "planId": plan_json.get("planId").or_else(|| plan_json.get("plan_id")).unwrap(),
            "plan": plan_json
        })),
    };
    let apply_res = handle_request(&state, apply_req).await;
    assert!(apply_res.error.is_none(), "Apply must succeed without -32004: {:?}", apply_res.error);
    let apply_val = apply_res.result.unwrap();
    assert_eq!(apply_val.get("success").and_then(|v| v.as_bool()), Some(true));

    // 3. Test on-the-fly regeneration fallback when cache is cleared and plan payload is missing
    {
        let mut st = state.write().await;
        st.cached_plans.clear(); // Simulate sidecar process restart!
    }
    let fallback_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(12),
        method: "transformation/applyPlan".to_string(),
        params: Some(json!({
            "planId": "plan_silo_specs_doc.md"
        })),
    };
    let fallback_res = handle_request(&state, fallback_req).await;
    assert!(fallback_res.error.is_none(), "Fallback on-the-fly regeneration must succeed: {:?}", fallback_res.error);
    let fallback_val = fallback_res.result.unwrap();
    assert_eq!(fallback_val.get("success").and_then(|v| v.as_bool()), Some(true));
}


