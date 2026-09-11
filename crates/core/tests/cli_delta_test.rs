use std::time::Instant;
use tempfile::tempdir;

#[tokio::test]
async fn test_git_delta_cli_offline_daemon_fail_open() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let start = Instant::now();

    // Call delta dispatch on a directory without active daemon port
    let result = llm_wiki_core::cli::delta::execute_git_delta(
        root,
        &["docs/note.md".to_string()],
        std::time::Duration::from_millis(500),
    ).await;

    let elapsed = start.elapsed();

    // Must return Ok(()) (fail-open exit code 0)
    assert!(result.is_ok(), "Git delta must fail open with Ok(())");
    // Must complete in under 550ms
    assert!(
        elapsed.as_millis() <= 550,
        "Git delta must not block beyond 500ms timeout, took {}ms",
        elapsed.as_millis()
    );
}
