//! Contract test for engine/generateTransformationPlan and engine/applyTransformationPlan.
//! Validates FR-041 and JSON-RPC 2.0 contract.

use llm_wiki_engine::rpc::handlers::EngineResponse;
use serde_json::json;

#[test]
fn test_rpc_generate_plan_protocol_envelope() {
    let raw_response = json!({
        "jsonrpc": "2.0",
        "id": 10,
        "result": {
            "plan_id": "plan-xyz",
            "approval_token": "token-hash-123",
            "strategy_id": "strat-split",
            "target_files": ["docs/monolith.md"],
            "unified_diff": "--- a/docs/monolith.md\n+++ b/docs/monolith.md\n@@ -1 +1 @@\n-old\n+new\n",
            "projected_score_delta": 8.5,
            "actions": [
                {
                    "action_type": "update_frontmatter",
                    "file": "docs/monolith.md",
                    "key": "decomposed",
                    "value": true
                }
            ]
        }
    });

    let parsed: EngineResponse<serde_json::Value> = serde_json::from_value(raw_response).unwrap();
    assert_eq!(parsed.jsonrpc, "2.0");
    assert!(parsed.result.is_some());
    let res = parsed.result.unwrap();
    assert_eq!(res["plan_id"], "plan-xyz");
    assert_eq!(res["approval_token"], "token-hash-123");
}

#[test]
fn test_rpc_apply_plan_protocol_envelope() {
    let raw_response = json!({
        "jsonrpc": "2.0",
        "id": 11,
        "result": {
            "aggregate_score_delta": 8.5,
            "raw_token_delta": -1200,
            "graph_diameter_delta": -1,
            "pillar_deltas": {
                "integrity": 0.0,
                "structure": 15.0,
                "consistency": 5.0,
                "discoverability": 10.0,
                "freshness": 0.0
            }
        }
    });

    let parsed: EngineResponse<serde_json::Value> = serde_json::from_value(raw_response).unwrap();
    assert_eq!(parsed.jsonrpc, "2.0");
    assert!(parsed.result.is_some());
    let res = parsed.result.unwrap();
    assert_eq!(res["aggregate_score_delta"], 8.5);
    assert_eq!(res["raw_token_delta"], -1200);
}
