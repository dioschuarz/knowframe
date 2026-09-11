//! Error definitions for LLM Wiki core engine.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, WikiError>;

#[derive(Error, Debug)]
pub enum WikiError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML serialization error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Markdown AST parsing error: {0}")]
    Ast(String),

    #[error("Graph resolution error: {0}")]
    Graph(String),

    #[error("IPC communication error: {0}")]
    Ipc(String),

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Operation timed out: {0}")]
    Timeout(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
