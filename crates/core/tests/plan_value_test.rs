//! Tests for Composite Plan Value & Trade-Off Valuation (Feature 007).
//! Implements T023 and T024.

use llm_wiki_core::evolution::plan_value::{
    calculate_composite_plan_value, get_pairwise_overlap, get_pairwise_synergy,
    get_transformation_cost, CompositePlanValue,
};
use llm_wiki_core::evolution::strategies::KnowledgeOrganizationStrategy::*;
use llm_wiki_core::ipc::handlers::{handle_request, AppState};
use llm_wiki_core::ipc::protocol::JsonRpcRequest;
use llm_wiki_core::model::NoteNode;
use serde_json::json;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::sync::RwLock;

#[test]
fn test_transformation_cost_and_synergy_constants_t023() {
    assert_eq!(get_transformation_cost(PreserveDocument), 0);
    assert_eq!(get_transformation_cost(SectionAddressing), 5);
    assert_eq!(get_transformation_cost(LightweightIndex), 10);
    assert_eq!(get_transformation_cost(FacetedClassification), 15);

    // Synergy bonuses
    assert_eq!(get_pairwise_synergy(SectionAddressing, LightweightIndex), 15);
    assert_eq!(get_pairwise_synergy(LightweightIndex, SectionAddressing), 15);
    assert_eq!(get_pairwise_synergy(FacetedClassification, LightweightIndex), 12);
    assert_eq!(get_pairwise_synergy(SectionAddressing, FacetedClassification), 0);

    // Overlap penalties
    assert_eq!(get_pairwise_overlap(SectionAddressing, LightweightIndex), 5);
    assert_eq!(get_pairwise_overlap(LightweightIndex, SectionAddressing), 5);
    assert_eq!(get_pairwise_overlap(FacetedClassification, LightweightIndex), 3);
    assert_eq!(get_pairwise_overlap(SectionAddressing, FacetedClassification), 0);
}

#[test]
fn test_composite_plan_value_calculation_t023() {
    let fit_lookup = vec![
        (SectionAddressing, 85),
        (LightweightIndex, 75),
        (FacetedClassification, 65),
        (PreserveDocument, 30),
    ];

    // Single strategy: SectionAddressing
    let val_single = calculate_composite_plan_value(&[SectionAddressing], &fit_lookup);
    assert_eq!(val_single.base_fit_sum, 85);
    assert_eq!(val_single.synergy_bonus, 0);
    assert_eq!(val_single.overlap_penalty, 0);
    assert_eq!(val_single.transformation_cost, 5);
    assert_eq!(val_single.net_value, 80); // 85 - 5

    // Combined pair: SectionAddressing + LightweightIndex
    let val_pair = calculate_composite_plan_value(&[SectionAddressing, LightweightIndex], &fit_lookup);
    assert_eq!(val_pair.base_fit_sum, 85 + 75); // 160
    assert_eq!(val_pair.synergy_bonus, 15);
    assert_eq!(val_pair.overlap_penalty, 5);
    assert_eq!(val_pair.transformation_cost, 5 + 10); // 15
    // net_value = 160 + 15 - 5 - 15 = 155
    assert_eq!(val_pair.net_value, 155);
    assert!(val_pair.narrative.contains("synergy bonus"));

    // Empty selection
    let val_empty = calculate_composite_plan_value(&[], &fit_lookup);
    assert_eq!(val_empty, CompositePlanValue::default());
}

#[tokio::test]
async fn test_calculate_plan_value_rpc_handler_t024() {
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

    // Call calculatePlanValue with complementary combination
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(10),
        method: "evolution/calculatePlanValue".to_string(),
        params: Some(json!({
            "opportunity_id": "silo_docs_orphan.md",
            "selected_strategies": ["section_addressing", "lightweight_index"]
        })),
    };

    let res = handle_request(&state, req).await;
    assert!(res.error.is_none(), "RPC should succeed: {:?}", res.error);
    let result = res.result.expect("Expected result payload");

    assert_eq!(result.get("is_valid").and_then(|v| v.as_bool()), Some(true));
    let cpv = result.get("composite_plan_value").expect("composite_plan_value required");
    assert!(cpv.get("net_value").is_some());
    assert!(cpv.get("synergy_bonus").is_some());
    assert_eq!(cpv.get("synergy_bonus").and_then(|v| v.as_u64()), Some(15));
    assert_eq!(cpv.get("overlap_penalty").and_then(|v| v.as_u64()), Some(5));

    // Call calculatePlanValue with incompatible combination
    let req_incompat = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(11),
        method: "evolution/calculatePlanValue".to_string(),
        params: Some(json!({
            "opportunity_id": "silo_docs_orphan.md",
            "selected_strategies": ["section_addressing", "faceted_classification"]
        })),
    };

    let res_incompat = handle_request(&state, req_incompat).await;
    assert!(res_incompat.error.is_none());
    let res_inc_val = res_incompat.result.unwrap();
    assert_eq!(res_inc_val.get("is_valid").and_then(|v| v.as_bool()), Some(false));
    assert!(res_inc_val.get("conflict_message").is_some());
}
