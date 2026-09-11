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

        let (mut ws_stream, _) = connect_async(req).await.expect("Failed to connect with valid token");
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
        let rpc_res: JsonRpcResponse = serde_json::from_str(text).expect("Failed to deserialize response");

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
