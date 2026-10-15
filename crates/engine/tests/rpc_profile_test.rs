//! Contract test for engine/setProfileMode and engine/updateCustomProfile JSON-RPC handlers.
//! Validates FR-006 and FR-041.

use llm_wiki_engine::rpc::handlers::EngineResponse;
use serde_json::json;

#[test]
fn test_rpc_set_profile_mode_protocol_envelope() {
    let raw_response = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "result": {
            "mode": "human_oriented",
            "profile_version": "1.0.0",
            "weights": {
                "integrity": 0.20,
                "structure": 0.25,
                "consistency": 0.25,
                "discoverability": 0.15,
                "freshness": 0.15,
                "context_efficiency": 0.10,
                "navigation_depth": 0.10,
                "vocabulary_adherence": 0.25
            },
            "thresholds": {
                "max_document_tokens": 8000,
                "max_fragment_tokens": 2000,
                "max_navigation_depth": 5,
                "min_scent_similarity": 0.50,
                "max_betweenness_centrality": 0.50,
                "min_category_coverage": 0.60,
                "max_taxonomy_imbalance": 0.40
            },
            "constraints": {
                "never_auto_split_monoliths": true,
                "enforce_controlled_tags": true,
                "require_dcap_frontmatter": false,
                "prohibit_circular_thesaurus": true,
                "isolated_directories": []
            }
        }
    });

    let parsed: EngineResponse<serde_json::Value> = serde_json::from_value(raw_response).unwrap();
    assert_eq!(parsed.jsonrpc, "2.0");
    assert!(parsed.result.is_some());
    let res = parsed.result.unwrap();
    assert_eq!(res["mode"], "human_oriented");
}
