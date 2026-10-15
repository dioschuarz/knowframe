use futures_util::{SinkExt, StreamExt};
use llm_wiki_core::ipc::protocol::{JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
use llm_wiki_core::ipc::server::start_ipc_server;
use serde_json::json;
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, Message};

#[tokio::test]
async fn test_ipc_server_authentication_and_scan() {
    timeout(Duration::from_secs(5), async {
        let auth_token = "test-secret-token-12345";
        let temp_dir = tempfile::tempdir().unwrap();
        std::fs::write(
            temp_dir.path().join("index.md"),
            "# Index\n\nLink to [Doc](doc.md)\n",
        )
        .unwrap();
        std::fs::write(
            temp_dir.path().join("doc.md"),
            "# Doc\n\nSome documentation\n",
        )
        .unwrap();

        let workspace_root = temp_dir.path().to_path_buf();

        println!("[TEST] Starting server...");
        let (addr, shutdown_tx) = start_ipc_server(0, workspace_root, Some(auth_token.to_string()))
            .await
            .expect("Failed to start IPC server");

        let port = addr.port();
        let ws_url = format!("ws://127.0.0.1:{}/ws", port);

        println!("[TEST] Connecting to {}", ws_url);
        let mut req = ws_url.into_client_request().unwrap();
        req.headers_mut().insert(
            "Authorization",
            format!("Bearer {}", auth_token).parse().unwrap(),
        );

        let (mut ws_stream, _) = connect_async(req)
            .await
            .expect("Failed to connect with valid token");
        println!("[TEST] Connected!");

        // 2. Send workspace/scan JSON-RPC request
        let scan_req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "workspace/scan".to_string(),
            params: None,
        };

        let msg = serde_json::to_string(&scan_req).unwrap();
        println!("[TEST] Sending scan request...");
        ws_stream.send(Message::Text(msg)).await.unwrap();

        // 3. Receive scan response
        println!("[TEST] Waiting for response...");
        let response_msg = ws_stream.next().await.unwrap().unwrap();
        let text = response_msg.to_text().unwrap();
        println!("[TEST] Received response: {}", text);
        let rpc_res: JsonRpcResponse =
            serde_json::from_str(text).expect("Failed to deserialize response");

        assert_eq!(rpc_res.id, json!(1));
        assert!(rpc_res.result.is_some());
        assert!(rpc_res.error.is_none());

        // 4. Send unauthenticated loopback git/commitDelta notification on loopback
        let delta_notif = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: "git/commitDelta".to_string(),
            params: json!({
                "repo_path": temp_dir.path().to_string_lossy(),
                "changed_files": []
            }),
        };
        let notif_msg = serde_json::to_string(&delta_notif).unwrap();
        ws_stream.send(Message::Text(notif_msg)).await.unwrap();

        // Clean shutdown
        let _ = shutdown_tx.send(());
    })
    .await
    .expect("Test timed out after 5 seconds");
}

#[tokio::test]
async fn test_traceability_rpc_handlers_t028_t029() {
    use llm_wiki_core::ipc::handlers::{handle_request, AppState};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let temp_dir = tempfile::tempdir().unwrap();
    let root = temp_dir.path();

    // 1. Setup doc citing code
    std::fs::create_dir_all(root.join("docs")).unwrap();
    std::fs::create_dir_all(root.join("src")).unwrap();

    std::fs::write(
        root.join("docs/api.md"),
        r#"---
title: API Docs
code_dependencies:
  - src/service.rs
---
# API Docs
Documentation for API service.
"#,
    )
    .unwrap();
    std::fs::write(root.join("src/service.rs"), "pub fn serve() {}\n").unwrap();

    let state = Arc::new(RwLock::new(AppState::new(root.to_path_buf())));

    // 2. Call trace/getAffectedDocuments
    let trace_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(10),
        method: "trace/getAffectedDocuments".to_string(),
        params: Some(json!({
            "changed_paths": ["src/service.rs"]
        })),
    };

    let trace_res = handle_request(&state, trace_req).await;
    assert_eq!(trace_res.id, json!(10));
    assert!(trace_res.error.is_none());

    let trace_val = trace_res.result.expect("result exists");
    let obligations = trace_val["obligations"].as_array().expect("obligations array");
    assert_eq!(obligations.len(), 1);
    assert_eq!(obligations[0]["doc_id"], "docs/api.md");

    // 3. Call review/resolveDocumentReview
    let resolve_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(11),
        method: "review/resolveDocumentReview".to_string(),
        params: Some(json!({
            "doc_ids": ["docs/api.md"]
        })),
    };

    let resolve_res = handle_request(&state, resolve_req).await;
    assert_eq!(resolve_res.id, json!(11));
    assert!(resolve_res.error.is_none());

    let resolve_val = resolve_res.result.expect("result exists");
    assert_eq!(resolve_val["resolved_count"], 1);
    assert!(resolve_val["receipt_id"].as_str().unwrap().starts_with("RECEIPT-"));

    // 4. Re-query trace/getAffectedDocuments to verify receipt was applied
    let trace_req2 = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: json!(12),
        method: "trace/getAffectedDocuments".to_string(),
        params: Some(json!({
            "changed_paths": ["src/service.rs"],
            "include_unchanged_reviews": true
        })),
    };

    let trace_res2 = handle_request(&state, trace_req2).await;
    let trace_val2 = trace_res2.result.unwrap();
    let obligations2 = trace_val2["obligations"].as_array().unwrap();
    assert_eq!(obligations2.len(), 1);
    assert_eq!(obligations2[0]["doc_id"], "docs/api.md");
    assert_eq!(obligations2[0]["status"], "reviewed_unchanged");
}
