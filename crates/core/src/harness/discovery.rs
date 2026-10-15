//! Harness detection and template initialization.
//!
//! Enforces deterministic detection of AI Harness Entry Points (FR-001)
//! and provides one-click template initialization (FR-002) for AGENTS.md,
//! CLAUDE.md, and GEMINI.md.

use super::{HarnessEntryPoint, HarnessQualityRatings, HarnessType};
use std::fs;
use std::path::Path;

/// Generates a template AI harness entry point containing conventions and an embedded `## Knowledge Index`.
pub fn generate_template_harness(
    harness_type: HarnessType,
    project_name: Option<&str>,
    custom_routes: Option<&[String]>,
) -> String {
    let name = project_name.unwrap_or("Project");
    let routes_str = match custom_routes {
        Some(routes) if !routes.is_empty() => routes.join("\n"),
        _ => "### Architecture\n- [System Architecture](CONSTITUTION.md): Core architectural invariants and quality principles.\n\n### Development & Operations\n- [Development Guide](README.md): Project overview and developer guidelines.".to_string(),
    };

    match harness_type {
        HarnessType::ClaudeMd => {
            format!(
                r#"# CLAUDE.md — Claude Code Instructions & Knowledge Index

Welcome to {name}. This file serves as the deterministic knowledge and instructions entry point for Claude Code.

## 1. Operating Conventions & Guidelines
- **Files & Notes**: Use `snake_case` for all markdown notes and documentation.
- **Code Symbols**: Use `CamelCase` for types, classes, and components.

## Knowledge Index

{routes_str}
"#
            )
        }
        HarnessType::GeminiMd => {
            format!(
                r#"# GEMINI.md — Gemini CLI & Agent Instructions

Welcome to {name}. This file serves as the deterministic knowledge entry point for Gemini CLI and Antigravity agents.

## 1. Naming Standards & Conventions
- **Files & Notes**: Use `snake_case` for all markdown notes and documentation.
- **Code Symbols**: Use `CamelCase` for types, classes, and components.

## Knowledge Index

{routes_str}
"#
            )
        }
        _ => {
            format!(
                r#"# AGENTS.md — AI Harness Entry Point

Welcome to {name}. This file serves as the deterministic knowledge entry point for AI agents.

## 1. Naming Standards & Conventions
- **Files & Notes**: Use `snake_case` for all markdown notes and documentation.
- **Code Symbols**: Use `CamelCase` for types, classes, and components.

## Knowledge Index

{routes_str}
"#
            )
        }
    }
}

/// Generates a standard `AGENTS.md` template containing an embedded `## Knowledge Index`.
pub fn generate_template_agents_md(project_name: Option<&str>) -> String {
    generate_template_harness(HarnessType::AgentsMd, project_name, None)
}

/// Detector for identifying configured or existing AI Harness Entry Points in a workspace.
pub struct HarnessDetector;

impl HarnessDetector {
    /// Detects all supported AI Harness Entry Points with active harness precedence.
    pub fn detect_all_with_active(
        workspace_root: &Path,
        custom_paths: &[String],
        active_harness: Option<&str>,
    ) -> Vec<HarnessEntryPoint> {
        let mut harnesses = Vec::new();

        let standard_candidates = [
            ("AGENTS.md", HarnessType::AgentsMd),
            ("notes/AGENTS.md", HarnessType::AgentsMd),
            ("CLAUDE.md", HarnessType::ClaudeMd),
            ("notes/CLAUDE.md", HarnessType::ClaudeMd),
            ("GEMINI.md", HarnessType::GeminiMd),
            ("notes/GEMINI.md", HarnessType::GeminiMd),
            (
                ".github/copilot-instructions.md",
                HarnessType::CopilotInstructions,
            ),
        ];

        let mut found_candidates = Vec::new();

        for (rel_path, harness_type) in standard_candidates {
            let full_path = workspace_root.join(rel_path);
            if full_path.is_file() {
                found_candidates.push((full_path, rel_path.to_string(), harness_type));
            }
        }

        for custom in custom_paths {
            let full_path = workspace_root.join(custom);
            if full_path.is_file() && !found_candidates.iter().any(|(_, r, _)| r == custom) {
                let harness_type = HarnessType::from_path(custom);
                found_candidates.push((full_path, custom.clone(), harness_type));
            }
        }

        // Determine which harness is primary:
        // 1. If active_harness matches an existing entry, mark that one primary.
        // 2. Otherwise mark the first one primary.
        let primary_path = active_harness
            .and_then(|act| {
                found_candidates
                    .iter()
                    .find(|(_, r, _)| r == act)
                    .map(|(_, r, _)| r.clone())
            })
            .or_else(|| found_candidates.first().map(|(_, r, _)| r.clone()));

        for (full_path, rel_path, harness_type) in found_candidates {
            let is_primary = Some(&rel_path) == primary_path.as_ref();
            let entry = Self::build_entry_stub(&full_path, &rel_path, harness_type, is_primary);
            harnesses.push(entry);
        }

        harnesses
    }

    /// Detects all supported AI Harness Entry Points in the given workspace root.
    pub fn detect_all(workspace_root: &Path, custom_paths: &[String]) -> Vec<HarnessEntryPoint> {
        Self::detect_all_with_active(workspace_root, custom_paths, None)
    }

    fn build_entry_stub(
        full_path: &Path,
        rel_path: &str,
        harness_type: HarnessType,
        is_primary: bool,
    ) -> HarnessEntryPoint {
        let metadata = fs::metadata(full_path).ok();
        let last_modified = metadata
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64);

        let content = fs::read_to_string(full_path).unwrap_or_default();
        let heading_count = content.lines().filter(|l| l.trim_start().starts_with('#')).count();
        let total_tokens = crate::tokens::counter::count_bpe_tokens(&content).unwrap_or(0) as usize;

        HarnessEntryPoint {
            file_path: rel_path.to_string(),
            harness_type,
            is_primary,
            total_tokens,
            heading_count,
            embedded_index: None,
            secondary_layers_detected: Vec::new(),
            quality_ratings: HarnessQualityRatings::default(),
            last_modified,
        }
    }
}
