//! Core knowledge graph engine for LLM Wiki VS Code extension.

pub mod ast;
pub mod candidate;
pub mod cli;
pub mod collision;
pub mod config;
pub mod debt;
pub mod delta;
pub mod drift;
pub mod error;
pub mod evolution;
pub mod fragment;
pub mod graph;
pub mod ipc;
pub mod model;
pub mod quality;
pub mod tokens;
pub mod trace;
