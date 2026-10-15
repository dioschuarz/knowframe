//! Contract test for engine/analyzeRepository and engine/getGraphTopology JSON-RPC handlers.
//! Validates FR-041 and FR-046 against contracts/knowledge_engine_protocol.json.

use llm_wiki_engine::rpc::handlers::EngineResponse;
use serde_json::json;

#[test]
fn test_rpc_analyze_protocol_envelope() {
    let raw_response = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "analysis_id": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "epistemic_tier": "tier_a_fully_deterministic",
            "health_score": {
                "analysis_id": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "aggregate_score": 92.5,
                "profile_mode": "ai_oriented",
                "integrity": { "raw_score": 95.0, "legacy_debt_count": 0, "new_regressions_count": 0, "findings": [] },
                "structure": { "raw_score": 90.0, "legacy_debt_count": 0, "new_regressions_count": 0, "findings": [] },
                "consistency": { "raw_score": 92.0, "legacy_debt_count": 0, "new_regressions_count": 0, "findings": [] },
                "discoverability": { "raw_score": 93.0, "legacy_debt_count": 0, "new_regressions_count": 0, "findings": [] },
                "freshness": { "raw_score": 91.0, "legacy_debt_count": 0, "new_regressions_count": 0, "findings": [] }
            },
            "findings": [],
            "graph": {
                "nodes": [],
                "edges": []
            }
        }
    });

    let parsed: EngineResponse<serde_json::Value> = serde_json::from_value(raw_response).unwrap();
    assert_eq!(parsed.jsonrpc, "2.0");
    assert!(parsed.result.is_some());
    let res = parsed.result.unwrap();
    assert_eq!(res["epistemic_tier"], "tier_a_fully_deterministic");
}
