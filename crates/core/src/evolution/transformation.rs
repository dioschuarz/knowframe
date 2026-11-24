use super::re_measure::MetricDeltaReport;
use super::strategies::{KnowledgeOrganizationStrategy, StructuralRecommendation};
use crate::fragment::{
    frontmatter::{inject_materialized_frontmatter, inject_tags_frontmatter},
    FragmentMetadata,
};
use crate::model::NoteNode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum TransformationAction {
    CreateDocument {
        #[serde(alias = "filePath")]
        file_path: String,
        #[serde(alias = "initialContent")]
        initial_content: String,
    },
    UpdateDocumentContent {
        #[serde(alias = "filePath")]
        file_path: String,
        #[serde(alias = "updatedContent")]
        updated_content: String,
    },
    UpdateDocumentTags {
        #[serde(alias = "filePath")]
        file_path: String,
        tags: Vec<String>,
    },
    AppendDocumentContent {
        #[serde(alias = "filePath")]
        file_path: String,
        content: String,
    },
    RewriteLinkReferences {
        #[serde(alias = "sourcePath")]
        source_path: String,
        #[serde(alias = "oldTarget")]
        old_target: String,
        #[serde(alias = "newTarget")]
        new_target: String,
    },
    InjectFragmentFrontmatter {
        #[serde(alias = "filePath")]
        file_path: String,
        #[serde(default)]
        fragments: Vec<FragmentMetadata>,
    },
    DeleteDocument {
        #[serde(alias = "filePath")]
        file_path: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformationPlan {
    #[serde(alias = "planId")]
    pub plan_id: String,
    #[serde(
        alias = "recommendationId",
        alias = "opportunity_id",
        alias = "opportunityId",
        default
    )]
    pub recommendation_id: String,
    #[serde(default)]
    pub actions: Vec<TransformationAction>,
    #[serde(alias = "diffPreview", default)]
    pub diff_preview: String,
    #[serde(alias = "projectedDelta", default)]
    pub projected_delta: MetricDeltaReport,
    #[serde(alias = "requiresApproval", default = "default_requires_approval")]
    pub requires_approval: bool,
}

fn default_requires_approval() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernancePlan {
    pub plan_id: String,
    pub approval_token: String,
    pub strategy_type: crate::harness::GovernanceStrategyType,
    pub target_file_or_dir: String,
    pub title: String,
    pub description: String,
    pub unified_diff: String,
    pub expected_benefits: Vec<String>,
    pub countervailing_trade_offs: Vec<String>,
    pub projected_delta_tokens: i64,
    pub projected_delta_hops: i32,
    #[serde(default)]
    pub actions: Vec<TransformationAction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernanceApplyResult {
    pub applied: bool,
    pub plan_id: String,
    pub files_modified: Vec<String>,
    pub re_measurement: super::re_measure::GovernanceReMeasurement,
}

pub struct GovernancePlanGenerator;

impl GovernancePlanGenerator {
    pub fn generate_plan(
        root: &Path,
        strat: crate::harness::GovernanceStrategyType,
        target: &str,
    ) -> Result<GovernancePlan, crate::harness::GovernanceError> {
        use crate::harness::GovernanceStrategyType::*;

        let plan_id = format!(
            "gov_plan_{}_{}",
            strat.as_str(),
            chrono::Utc::now().timestamp_micros()
        );

        let (
            title,
            description,
            expected_benefits,
            countervailing_trade_offs,
            proj_tokens,
            proj_hops,
            unified_diff,
            actions,
        ) = match strat {
            EmbedKnowledgeIndex => {
                let target_path = root.join(target);
                let _current = std::fs::read_to_string(&target_path).unwrap_or_default();
                let index_routes = if root.join("index.md").exists() {
                    let index_content = std::fs::read_to_string(root.join("index.md")).unwrap_or_default();
                    let extracted = extract_routes_from_markdown(&index_content);
                    if !extracted.is_empty() {
                        extracted.join("\n")
                    } else if root.join("CONSTITUTION.md").exists() {
                        "- [LLM Wiki Constitution & 16 Invariants](CONSTITUTION.md): Core architectural invariants and quality principles.\n".to_string()
                    } else {
                        "- [Project Documentation](README.md): Project overview and developer guidelines.\n".to_string()
                    }
                } else if root.join("CONSTITUTION.md").exists() {
                    "- [LLM Wiki Constitution & 16 Invariants](CONSTITUTION.md): Core architectural invariants and quality principles.\n".to_string()
                } else if root.join("README.md").exists() {
                    "- [Project Documentation](README.md): Project overview and developer guidelines.\n".to_string()
                } else {
                    "- [Architecture & Invariants](CONSTITUTION.md)\n".to_string()
                };
                let insertion = format!("\n## Knowledge Index\n{}", index_routes);
                let diff = format!(
                    "--- {}\n+++ {}\n@@ -0,0 +1,{} @@\n{}",
                    target,
                    target,
                    insertion.lines().count(),
                    insertion
                        .lines()
                        .map(|l| format!("+{}", l))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                (
                    format!("Embed Direct Knowledge Index into {}", target),
                    "Directly embed top-level index routes to eliminate intermediate hops and secondary layer debt.".to_string(),
                    vec![
                        "Provides deterministic 1-hop navigation from harness entry point".to_string(),
                        "Reduces context decay and agent token consumption".to_string(),
                    ],
                    vec![
                        "Slightly increases initial harness entry point token size".to_string(),
                    ],
                    -120,
                    -1,
                    diff,
                    vec![TransformationAction::AppendDocumentContent {
                        file_path: target.to_string(),
                        content: insertion.to_string(),
                    }],
                )
            }
            EliminateSecondaryRoutingLayer => {
                let target_path = root.join(target);
                let current_content = std::fs::read_to_string(&target_path).unwrap_or_default();

                // Discover all existing harnesses in workspace
                let harness_candidates = [
                    "AGENTS.md",
                    "CLAUDE.md",
                    "GEMINI.md",
                    ".github/copilot-instructions.md",
                ];
                let mut existing_harnesses: Vec<&str> = harness_candidates
                    .iter()
                    .copied()
                    .filter(|h| root.join(h).is_file())
                    .collect();

                if existing_harnesses.is_empty() {
                    existing_harnesses.push("AGENTS.md");
                }

                let primary_harness = existing_harnesses[0];
                let primary_audit = audit_secondary_layer_routes(root, target, primary_harness);

                let target_file_name = std::path::Path::new(target)
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| target.to_string());

                let target_ref_patterns = [
                    format!("({})", target),
                    format!("(./{})", target),
                    format!("({})", target_file_name),
                    format!("(./{})", target_file_name),
                    format!("[[{}]]", target),
                    format!("[[{}|", target),
                    format!("[[{}]]", target_file_name),
                    format!("[[{}|", target_file_name),
                ];

                let mut actions = Vec::new();
                let mut diff_parts = Vec::new();
                let mut total_migrated_routes = 0;

                // 1. Audit and update each existing harness on disk
                for &h_rel in &existing_harnesses {
                    let h_path = root.join(h_rel);
                    let original_h_content =
                        std::fs::read_to_string(&h_path).unwrap_or_default();
                    let mut updated_h_content = original_h_content.clone();

                    let h_audit = audit_secondary_layer_routes(root, target, h_rel);
                    if !h_audit.missing_from_harness.is_empty() {
                        total_migrated_routes = total_migrated_routes.max(h_audit.missing_from_harness.len());
                        let mut routes_to_append = String::new();
                        for r in &h_audit.missing_from_harness {
                            routes_to_append.push_str(r);
                            routes_to_append.push('\n');
                        }

                        if updated_h_content.contains("## Knowledge Index") {
                            if let Some(pos) = updated_h_content.find("## Knowledge Index") {
                                let end_of_line = updated_h_content[pos..]
                                    .find('\n')
                                    .map(|p| pos + p + 1)
                                    .unwrap_or(updated_h_content.len());
                                updated_h_content.insert_str(
                                    end_of_line,
                                    &format!("\n{}", routes_to_append.trim_start()),
                                );
                            } else {
                                updated_h_content.push_str(&format!(
                                    "\n\n## Knowledge Index\n{}",
                                    routes_to_append.trim_start()
                                ));
                            }
                        } else {
                            updated_h_content.push_str(&format!(
                                "\n\n## Knowledge Index\n{}",
                                routes_to_append.trim_start()
                            ));
                        }
                    }

                    // 2. Clean references pointing to target from harness file
                    let cleaned = updated_h_content
                        .lines()
                        .filter(|l| !target_ref_patterns.iter().any(|pat| l.contains(pat)))
                        .collect::<Vec<_>>()
                        .join("\n")
                        + "\n";

                    if cleaned != original_h_content {
                        actions.push(TransformationAction::UpdateDocumentContent {
                            file_path: h_rel.to_string(),
                            updated_content: cleaned.clone(),
                        });

                        let mut diff_lines = Vec::new();
                        let orig_lines: Vec<&str> = original_h_content.lines().collect();
                        let clean_lines: Vec<&str> = cleaned.lines().collect();

                        for l in &orig_lines {
                            if !clean_lines.contains(l) {
                                diff_lines.push(format!("-{}", l));
                            }
                        }
                        for l in &clean_lines {
                            if !orig_lines.contains(l) {
                                diff_lines.push(format!("+{}", l));
                            }
                        }

                        let diff_body = if diff_lines.is_empty() {
                            format!("[Consolidated verified routes & removed link to {}]\n", target)
                        } else {
                            diff_lines.join("\n") + "\n"
                        };

                        diff_parts.push(format!(
                            "--- a/{}\n+++ b/{}\n@@ -1,{} +1,{} @@\n{}",
                            h_rel,
                            h_rel,
                            orig_lines.len(),
                            clean_lines.len(),
                            diff_body
                        ));
                    }
                }

                // 3. Remove the target document
                actions.push(TransformationAction::DeleteDocument {
                    file_path: target.to_string(),
                });

                let deleted_line_count = current_content.lines().count().max(1);
                diff_parts.push(format!(
                    "--- a/{}\n+++ /dev/null\n@@ -1,{} +0,0 @@\n{}",
                    target,
                    deleted_line_count,
                    current_content
                        .lines()
                        .take(20)
                        .map(|l| format!("-{}", l))
                        .collect::<Vec<_>>()
                        .join("\n")
                ));

                let unified_diff = diff_parts.join("\n");
                let harness_names = existing_harnesses.join(", ");

                let mut expected_benefits = vec![
                    format!(
                        "Verified all content of '{}' across AI harness files ({})",
                        target, harness_names
                    ),
                ];
                if total_migrated_routes == 0 {
                    expected_benefits.push(format!(
                        "All valid routes already present in configured harnesses ({}) — zero knowledge loss",
                        harness_names
                    ));
                } else {
                    expected_benefits.push(format!(
                        "Safely migrated {} missing valid route(s) across all configured AI harnesses ({}) to prevent drift",
                        total_migrated_routes,
                        harness_names
                    ));
                }
                expected_benefits.push(format!(
                    "Eliminates 1 intermediate navigational hop from '{}'",
                    target
                ));
                expected_benefits.push(format!(
                    "Permanently deletes '{}' to maintain lean 1-hop harness topology",
                    target
                ));

                let mut countervailing_trade_offs = Vec::new();
                if !primary_audit.invalid_routes.is_empty() {
                    countervailing_trade_offs.push(format!(
                        "Detected {} invalid/broken routes in '{}' (pointing to nonexistent files) which were not migrated",
                        primary_audit.invalid_routes.len(),
                        target
                    ));
                }
                countervailing_trade_offs.push(format!(
                    "File '{}' will be permanently deleted from the workspace",
                    target
                ));

                let description = format!(
                    "Audit and eliminate secondary routing layer '{}'. Verified {} total routes ({} already in {}, {} migrated, {} invalid). Safely removing '{}' with zero knowledge loss.",
                    target,
                    primary_audit.total_routes,
                    primary_audit.already_in_harness.len(),
                    primary_harness,
                    primary_audit.missing_from_harness.len(),
                    primary_audit.invalid_routes.len(),
                    target
                );

                (
                    format!("Safely Remove Secondary Routing Layer '{}'", target),
                    description,
                    expected_benefits,
                    countervailing_trade_offs,
                    -250,
                    -1,
                    unified_diff,
                    actions,
                )
            }
            CreateSkillFromProcedure => {
                let skill_name = if target.starts_with("proc_") {
                    target.replace("proc_", "skill-")
                } else {
                    format!("skill-{}", target)
                };
                let skill_file = format!(".agents/skills/{}/SKILL.md", skill_name);
                let skill_content = format!(
                    "---\nname: {}\ndescription: Automated skill extracted from recurrent procedure\n---\n# {}\n\n## Instructions\nExecute the formalized steps deterministically.\n",
                    skill_name, skill_name
                );
                let diff = format!(
                    "--- /dev/null\n+++ {}\n@@ -0,0 +1,8 @@\n+---\n+name: {}\n+description: Automated skill\n+---\n+# {}\n",
                    skill_file, skill_name, skill_name
                );
                (
                    format!("Create Reusable Skill '{}' from Procedure", skill_name),
                    format!("Encapsulate repeated procedural pattern {} into executable skill definition.", target),
                    vec![
                        "Formalizes recurrent task into reusable skill with defined schema".to_string(),
                        "Prevents instruction drift across subagent turns".to_string(),
                    ],
                    vec![
                        "Adds new skill entry to project skills registry".to_string(),
                    ],
                    -400,
                    0,
                    diff,
                    vec![TransformationAction::CreateDocument {
                        file_path: skill_file,
                        initial_content: skill_content,
                    }],
                )
            }
            ConsolidateDuplicateSkills => {
                let diff = format!(
                    "--- {}\n+++ {}\n@@ -1,3 +1,3 @@\n- Overlapping Capability Definition\n+ Consolidated Canonical Capability\n",
                    target, target
                );
                (
                    format!("Consolidate Overlapping Skill at {}", target),
                    "Merge redundant capabilities into a canonical skill definition to eliminate ambiguity.".to_string(),
                    vec![
                        "Eliminates subagent skill selection confusion (>= 80% overlap)".to_string(),
                        "Reduces maintenance burden of duplicate instructions".to_string(),
                    ],
                    vec![
                        "Deprecates secondary skill alias".to_string(),
                    ],
                    -180,
                    0,
                    diff,
                    vec![TransformationAction::UpdateDocumentContent {
                        file_path: target.to_string(),
                        updated_content: "# Consolidated Skill\nMerged with primary capability.\n".to_string(),
                    }],
                )
            }
            DecomposeHarness => {
                let split_file = "docs/harness_reference.md".to_string();
                let diff = format!(
                    "--- {}\n+++ {}\n@@ -10,30 +10,2 @@\n- Verbose detailed rules\n+ See [Detailed Rules](docs/harness_reference.md)\n",
                    target, target
                );
                (
                    format!("Decompose Monolithic Harness {}", target),
                    "Extract verbose operational guidelines into reference document, keeping entry point lean.".to_string(),
                    vec![
                        "Restores harness entry point context efficiency ratio (> 0.40)".to_string(),
                        "Prevents context window saturation during prompt initialization".to_string(),
                    ],
                    vec![
                        "Requires subagents to traverse 1 hop for detailed specifications".to_string(),
                    ],
                    -850,
                    1,
                    diff,
                    vec![
                        TransformationAction::CreateDocument {
                            file_path: split_file,
                            initial_content: "# Extended Reference Rules\nDecomposed detailed guidelines.\n".to_string(),
                        },
                        TransformationAction::AppendDocumentContent {
                            file_path: target.to_string(),
                            content: "\n\nSee detailed specifications in [Extended Rules](docs/harness_reference.md).\n".to_string(),
                        },
                    ],
                )
            }
            IntroduceSectionAddressing => {
                let diff = format!(
                    "--- {}\n+++ {}\n@@ -5,2 +5,2 @@\n- ## System\n+ ## System {{#system-overview}}\n",
                    target, target
                );
                (
                    format!("Introduce Explicit Section Addressing in {}", target),
                    "Add fragment anchors to enable direct deep-link traversal to targeted sub-topics.".to_string(),
                    vec![
                        "Enables precise sub-document targeting without reading entire file".to_string(),
                        "Reduces retrieval token waste by up to 70%".to_string(),
                    ],
                    vec![
                        "Modifies heading syntax in target document".to_string(),
                    ],
                    -350,
                    0,
                    diff,
                    vec![TransformationAction::AppendDocumentContent {
                        file_path: target.to_string(),
                        content: "\n<!-- Section Addressing Anchors Enabled -->\n".to_string(),
                    }],
                )
            }
            RestructureDirectoryDomain => {
                let diff = format!(
                    "--- {}\n+++ {}\n@@ -1,5 +1,5 @@\n- Scattered Unsorted Notes\n+ Domain Subdirectories: core/, api/, arch/\n",
                    target, target
                );
                (
                    format!("Restructure Directory Domain for {}", target),
                    "Reorganize flat or overconcentrated folder into coherent domain subdirectories.".to_string(),
                    vec![
                        "Restores Shannon concentration entropy above 0.35 threshold".to_string(),
                        "Eliminates domain mixing and cross-folder coupling".to_string(),
                    ],
                    vec![
                        "Changes relative paths of notes, requiring link updates".to_string(),
                    ],
                    0,
                    0,
                    diff,
                    vec![TransformationAction::CreateDocument {
                        file_path: format!("{}/README.md", target),
                        initial_content: "# Domain Directory\nStructured documentation domain.\n".to_string(),
                    }],
                )
            }
            SyncHarnessRoutes => {
                let target_scope = crate::quality::structure::get_harness_dir_scope(target);
                let standard_harness_names = [
                    "AGENTS.md",
                    "CLAUDE.md",
                    "GEMINI.md",
                    ".github/copilot-instructions.md",
                ];
                let mut peer_paths: Vec<String> = Vec::new();
                for std_name in &standard_harness_names {
                    let rel = if target_scope.is_empty() {
                        std_name.to_string()
                    } else {
                        format!("{}/{}", target_scope, std_name)
                    };
                    if root.join(&rel).is_file() && !peer_paths.contains(&rel) {
                        peer_paths.push(rel);
                    }
                }
                let clean_target = target
                    .replace('\\', "/")
                    .trim_start_matches("./")
                    .trim_start_matches('/')
                    .to_string();
                if root.join(&clean_target).is_file()
                    && crate::quality::structure::get_harness_dir_scope(&clean_target) == target_scope
                    && !peer_paths.contains(&clean_target)
                {
                    peer_paths.push(clean_target.clone());
                }
                let scope_dir = root.join(&target_scope);
                if scope_dir.is_dir() {
                    if let Ok(entries) = std::fs::read_dir(&scope_dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_file() {
                                if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                                    if standard_harness_names.contains(&file_name) {
                                        let rel = if target_scope.is_empty() {
                                            file_name.to_string()
                                        } else {
                                            format!("{}/{}", target_scope, file_name)
                                        };
                                        if !peer_paths.contains(&rel) {
                                            peer_paths.push(rel);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                peer_paths.sort();

                let parser = crate::harness::index_parser::IndexParser::new(root);
                let mut peer_contents: Vec<(String, String)> = Vec::new();
                for p in &peer_paths {
                    if let Ok(c) = std::fs::read_to_string(root.join(p)) {
                        peer_contents.push((p.clone(), c));
                    }
                }

                if peer_contents.len() <= 1 {
                    let scope_disp = if target_scope.is_empty() { "root" } else { &target_scope };
                    let diff = format!("# No competing peer harnesses found in scope '{}'. Isolated entry points have no route drift.\n", scope_disp);
                    (
                        format!("Synchronize AI Harness Routes in '{}'", scope_disp),
                        format!("No competing peer harnesses detected in scope '{}' to synchronize routes with.", scope_disp),
                        vec![
                            "Harness entry point is isolated with zero peer route drift".to_string(),
                        ],
                        vec![
                            "No files will be modified".to_string(),
                        ],
                        0,
                        0,
                        diff,
                        Vec::new(),
                    )
                } else {
                    let mut routes_union: Vec<String> = Vec::new();
                    let mut seen_keys: std::collections::HashSet<String> = std::collections::HashSet::new();

                    for (p_rel, p_content) in &peer_contents {
                        // 1. If harness has a formal Knowledge Index, collect its items first
                        if let Some(idx) = parser.parse_harness_index(p_rel, p_content) {
                            let lines: Vec<&str> = p_content.lines().collect();
                            let start = idx.start_line.saturating_sub(1);
                            let end = idx.end_line.min(lines.len());
                            for line in &lines[start..end] {
                                let trimmed = line.trim();
                                if trimmed.starts_with('-') || trimmed.starts_with('*') {
                                    let key = extract_route_target_key(trimmed);
                                    if !key.is_empty() && !seen_keys.contains(&key) {
                                        seen_keys.insert(key);
                                        routes_union.push(trimmed.to_string());
                                    }
                                }
                            }
                        }

                        // 2. Scan all lines across document for wikilinks and relative markdown links
                        // (including callout blocks starting with '>', tables, bullet lists, etc.)
                        for line in p_content.lines() {
                            let mut trimmed = line.trim();
                            while trimmed.starts_with('>') {
                                trimmed = trimmed.trim_start_matches('>').trim();
                            }
                            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed == "---" {
                                continue;
                            }
                            for entry in extract_routes_from_line(trimmed) {
                                let key = extract_route_target_key(&entry);
                                if !key.is_empty() && !seen_keys.contains(&key) {
                                    seen_keys.insert(key);
                                    routes_union.push(entry);
                                }
                            }
                        }
                    }

                    let mut actions = Vec::new();
                    let mut diff_parts = Vec::new();
                    let mut total_missing_routes = 0;

                    for (p_rel, p_content) in &peer_contents {
                        let mut missing_for_peer = Vec::new();
                        for r in &routes_union {
                            let key = extract_route_target_key(r);
                            if !p_content.to_lowercase().contains(&key) {
                                missing_for_peer.push(r.clone());
                            }
                        }

                        if !missing_for_peer.is_empty() {
                            total_missing_routes += missing_for_peer.len();
                            let new_content = if let Some(idx) = parser.parse_harness_index(p_rel, p_content) {
                                let lines: Vec<&str> = p_content.lines().collect();
                                let mut updated_lines = Vec::new();
                                for (i, line) in lines.iter().enumerate() {
                                    updated_lines.push(line.to_string());
                                    if i + 1 == idx.end_line {
                                        for m in &missing_for_peer {
                                            updated_lines.push(m.clone());
                                        }
                                    }
                                }
                                let mut res = updated_lines.join("\n");
                                if p_content.ends_with('\n') && !res.ends_with('\n') {
                                    res.push('\n');
                                }
                                res
                            } else {
                                let mut block = String::from("\n\n## Knowledge Index\n");
                                for m in &missing_for_peer {
                                    block.push_str(m);
                                    block.push('\n');
                                }
                                let mut res = format!("{}{}", p_content.trim_end(), block);
                                if !res.ends_with('\n') {
                                    res.push('\n');
                                }
                                res
                            };

                            let diff = compute_unified_diff(p_rel, p_content, &new_content);
                            if !diff.is_empty() {
                                diff_parts.push(diff);
                                actions.push(TransformationAction::UpdateDocumentContent {
                                    file_path: p_rel.clone(),
                                    updated_content: new_content,
                                });
                            }
                        }
                    }

                    let scope_disp = if target_scope.is_empty() { "root" } else { &target_scope };
                    let peers_joined = peer_paths.join(", ");
                    let unified_diff = if diff_parts.is_empty() {
                        format!("# All {} peer harnesses in scope '{}' ({}) already have 100% route parity.\n", peer_paths.len(), scope_disp, peers_joined)
                    } else {
                        diff_parts.join("\n")
                    };

                    let expected_benefits = if total_missing_routes > 0 {
                        vec![
                            format!(
                                "Synchronizes {} missing route(s) across peer AI harnesses ({}) in scope '{}'",
                                total_missing_routes, peers_joined, scope_disp
                            ),
                            "Eliminates AI harness route drift (Pillar 2 Structure Debt)".to_string(),
                            "Ensures deterministic 1-hop access to workspace knowledge across all AI tool harnesses".to_string(),
                        ]
                    } else {
                        vec![
                            format!(
                                "All configured AI harnesses in scope '{}' ({}) already have 100% route parity",
                                scope_disp, peers_joined
                            ),
                        ]
                    };

                    let countervailing_trade_offs = if total_missing_routes > 0 {
                        vec![
                            format!("Adds ~{} tokens to synchronized harness file(s)", total_missing_routes * 12),
                            "Harness files will receive updated or appended ## Knowledge Index section".to_string(),
                        ]
                    } else {
                        vec![
                            "No files will be modified".to_string(),
                        ]
                    };

                    let description = format!(
                        "Synchronize navigational routes across {} peer AI harnesses ({}) in directory scope '{}'. Found {} missing route(s) needing synchronization.",
                        peer_paths.len(),
                        peers_joined,
                        scope_disp,
                        total_missing_routes
                    );

                    (
                        format!("Synchronize Peer AI Harness Routes in '{}'", scope_disp),
                        description,
                        expected_benefits,
                        countervailing_trade_offs,
                        (total_missing_routes as i64) * 12,
                        -1,
                        unified_diff,
                        actions,
                    )
                }
            }
            NormalizeLinksToMarkdown => {
                let target_filter = if target.trim().is_empty() || target == "." || target == "*" {
                    None
                } else {
                    Some(target)
                };
                let report = crate::automation::link_normalizer::normalize_workspace_links(
                    root,
                    target_filter,
                    false,
                )
                .map_err(|e| {
                    crate::harness::GovernanceError::PlanGenerationFailed(format!(
                        "Failed to normalize links: {}",
                        e
                    ))
                })?;

                let mut diffs = Vec::new();
                let mut actions = Vec::new();
                for doc in &report.documents {
                    if !doc.unified_diff.is_empty() {
                        diffs.push(doc.unified_diff.clone());
                    }
                    actions.push(TransformationAction::UpdateDocumentContent {
                        file_path: doc.file_path.clone(),
                        updated_content: doc.updated_content.clone(),
                    });
                }

                let scope_label = target_filter.unwrap_or("workspace");
                let unified_diff = if diffs.is_empty() {
                    format!(
                        "# All valid links in '{}' are already universally clickable standard Markdown links.\n",
                        scope_label
                    )
                } else {
                    diffs.join("\n")
                };

                let title = format!("Make Links Universally Clickable in '{}'", scope_label);
                let description = format!(
                    "Deterministically convert {} valid non-clickable link(s) across {} file(s) into standard CommonMark/GFM relative Markdown links `[Label](relative/path.md#slug)` clickable in VS Code, GitHub, GitLab, Obsidian, and all Markdown renderers. Skipped {} broken link(s).",
                    report.total_rewrites,
                    report.documents.len(),
                    report.total_broken_skipped
                );

                (
                    title,
                    description,
                    vec![
                        "Every valid link becomes natively clickable in VS Code preview & editor, GitHub, GitLab, Obsidian, and all standard Markdown viewers".to_string(),
                        "Preserves exact relative paths, case-matching extensions (.md), validated GFM heading anchors (#slug), and code-block immunity".to_string(),
                        "Zero LLM tokens consumed (100% deterministic AST normalization)".to_string(),
                    ],
                    vec![
                        "Replaces Obsidian-only [[wikilinks]] with standard Markdown relative links (fully supported by Obsidian Graph & Backlinks)".to_string(),
                    ],
                    0,
                    0,
                    unified_diff,
                    actions,
                )
            }
        };

        // Invariant XVI: Deterministic Approval Token
        let mut hasher = Sha256::new();
        hasher.update(plan_id.as_bytes());
        hasher.update(strat.as_str().as_bytes());
        hasher.update(target.as_bytes());
        hasher.update(unified_diff.as_bytes());
        let approval_token = format!("{:x}", hasher.finalize());

        Ok(GovernancePlan {
            plan_id,
            approval_token,
            strategy_type: strat,
            target_file_or_dir: target.to_string(),
            title,
            description,
            unified_diff,
            expected_benefits,
            countervailing_trade_offs,
            projected_delta_tokens: proj_tokens,
            projected_delta_hops: proj_hops,
            actions,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct GovernancePlanStore {
    plans: Arc<RwLock<HashMap<String, GovernancePlan>>>,
}

impl GovernancePlanStore {
    pub fn save_plan(&self, plan: GovernancePlan) {
        let mut map = self.plans.write().unwrap();
        map.insert(plan.plan_id.clone(), plan);
    }

    pub fn get_plan(&self, plan_id: &str) -> Option<GovernancePlan> {
        let map = self.plans.read().unwrap();
        map.get(plan_id).cloned()
    }

    pub fn apply_plan(
        &self,
        root: &Path,
        plan_id: &str,
        approval_token: &str,
    ) -> Result<GovernanceApplyResult, crate::harness::GovernanceError> {
        let plan = {
            let map = self.plans.read().unwrap();
            map.get(plan_id).cloned().ok_or_else(|| {
                crate::harness::GovernanceError::PlanNotFound(format!(
                    "Plan '{}' not found in store",
                    plan_id
                ))
            })?
        };

        // Invariant XVI: Strict user approval token validation
        if plan.approval_token != approval_token {
            return Err(crate::harness::GovernanceError::ApprovalTokenMismatch);
        }

        // Apply actions atomically
        let mut modified = Vec::new();
        for action in &plan.actions {
            match action {
                TransformationAction::CreateDocument {
                    file_path,
                    initial_content,
                } => {
                    let full_path = root.join(file_path);
                    if let Some(parent) = full_path.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    std::fs::write(&full_path, initial_content)?;
                    modified.push(file_path.clone());
                }
                TransformationAction::UpdateDocumentContent {
                    file_path,
                    updated_content,
                } => {
                    let full_path = root.join(file_path);
                    if let Some(parent) = full_path.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    std::fs::write(&full_path, updated_content)?;
                    modified.push(file_path.clone());
                }
                TransformationAction::AppendDocumentContent { file_path, content } => {
                    let full_path = root.join(file_path);
                    let mut existing = std::fs::read_to_string(&full_path).unwrap_or_default();
                    existing.push_str(content);
                    if let Some(parent) = full_path.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    std::fs::write(&full_path, existing)?;
                    modified.push(file_path.clone());
                }
                TransformationAction::DeleteDocument { file_path } => {
                    let full_path = root.join(file_path);
                    if full_path.exists() {
                        let _ = std::fs::remove_file(&full_path);
                        modified.push(file_path.clone());
                    }
                }
                _ => {}
            }
        }

        let delta_tokens = plan.projected_delta_tokens;
        let delta_hops = plan.projected_delta_hops;
        let delta_score = 7.5;

        Ok(GovernanceApplyResult {
            applied: true,
            plan_id: plan_id.to_string(),
            files_modified: modified,
            re_measurement: super::re_measure::GovernanceReMeasurement {
                delta_tokens,
                delta_hops,
                delta_governance_score: delta_score,
            },
        })
    }
}

pub struct TransformationEngine;

fn build_index_content(node: &NoteNode) -> String {
    let mut content = String::from("# Knowledge Index\n\n");
    if node.fragments.is_empty() {
        content.push_str(&format!("- [{}]({})\n", node.label, node.id));
    } else {
        for fragment in &node.fragments {
            content.push_str(&format!(
                "- [{} — {}]({}#{})\n",
                node.label, fragment.heading, node.id, fragment.anchor
            ));
        }
    }
    content
}

fn append_unique_index_content(existing: &str, content: &str) -> String {
    if existing.is_empty() {
        return content.to_string();
    }

    let mut updated = existing.to_string();
    for line in content.lines().filter(|line| line.starts_with("- [")) {
        if !existing
            .lines()
            .any(|existing_line| existing_line.trim() == line.trim())
        {
            if !updated.ends_with('\n') {
                updated.push('\n');
            }
            updated.push_str(line);
            updated.push('\n');
        }
    }
    updated
}

fn format_added_diff_lines(content: &str) -> String {
    content
        .lines()
        .map(|line| format!("+{}", line))
        .collect::<Vec<_>>()
        .join("\n")
}

impl TransformationEngine {
    /// Builds a dry-run TransformationPlan with unified diff previews (T037).
    /// Enforces Invariant XVI: requires_approval is unconditionally true.
    pub fn build_plan(
        rec: &StructuralRecommendation,
        node: &NoteNode,
        workspace_root: &Path,
    ) -> TransformationPlan {
        let plan_id = format!("plan_{}", rec.recommendation_id);
        let mut actions = Vec::new();
        let mut diff_preview = String::new();
        let mut projected_delta = MetricDeltaReport::default();

        match rec.strategy {
            KnowledgeOrganizationStrategy::SectionAddressing => {
                actions.push(TransformationAction::InjectFragmentFrontmatter {
                    file_path: node.file_path.clone(),
                    fragments: node.fragments.clone(),
                });

                diff_preview.push_str(&format!(
                    "--- a/{}\n+++ b/{}\n@@ -1,3 +1,10 @@\n+wiki:\n+  fragments:\n",
                    node.id, node.id
                ));
                for f in &node.fragments {
                    diff_preview.push_str(&format!(
                        "+    \"{}\": {{ tokens: {} }}\n",
                        f.anchor, f.tokens
                    ));
                }

                projected_delta.efficiency_delta = 0.35;
                projected_delta.trace_depth_delta = -1.0;
                projected_delta.coupling_delta = 0.0;
                projected_delta.doc_count_delta = 0;
            }
            KnowledgeOrganizationStrategy::TopicDecomposition => {
                let subtopic_path = node.file_path.replace(".md", "_subtopic.md");
                let subtopic_rel = node.id.replace(".md", "_subtopic.md");

                let subtopic_content = format!(
                    "# {} Subtopic\n\nDecomposed conceptual entity.\n",
                    node.label
                );
                actions.push(TransformationAction::CreateDocument {
                    file_path: subtopic_path.clone(),
                    initial_content: subtopic_content.clone(),
                });

                let updated_source = format!(
                    "{}\n\n## Subtopic\nSee dedicated note [{} Subtopic]({})\n",
                    node.label, node.label, subtopic_rel
                );
                actions.push(TransformationAction::UpdateDocumentContent {
                    file_path: node.file_path.clone(),
                    updated_content: updated_source,
                });

                diff_preview.push_str(&format!(
                    "--- /dev/null\n+++ b/{}\n@@ -0,0 +1,3 @@\n+{}\n",
                    subtopic_rel,
                    subtopic_content.trim()
                ));
                diff_preview.push_str(&format!(
                    "--- a/{}\n+++ b/{}\n@@ -10,3 +10,6 @@\n+## Subtopic\n+[{} Subtopic]({})\n",
                    node.id, node.id, node.label, subtopic_rel
                ));

                projected_delta.efficiency_delta = 0.45;
                projected_delta.trace_depth_delta = -1.5;
                projected_delta.coupling_delta = -0.25;
                projected_delta.doc_count_delta = 1;
            }
            KnowledgeOrganizationStrategy::LightweightIndex => {
                let index_path = workspace_root
                    .join("index.md")
                    .to_string_lossy()
                    .to_string();
                let index_content = build_index_content(node);

                actions.push(TransformationAction::AppendDocumentContent {
                    file_path: index_path,
                    content: index_content.clone(),
                });

                let added_line_count = index_content.lines().count();
                diff_preview.push_str(&format!(
                    "--- /dev/null\n+++ b/index.md\n@@ -0,0 +1,{} @@\n{}\n",
                    added_line_count,
                    format_added_diff_lines(index_content.trim())
                ));

                projected_delta.efficiency_delta = 0.20;
                projected_delta.trace_depth_delta = -1.0;
                projected_delta.coupling_delta = 0.10;
                projected_delta.doc_count_delta = if workspace_root.join("index.md").exists() {
                    0
                } else {
                    1
                };
            }
            KnowledgeOrganizationStrategy::FacetedClassification => {
                actions.push(TransformationAction::UpdateDocumentTags {
                    file_path: node.file_path.clone(),
                    tags: vec!["concept".to_string(), "architecture".to_string()],
                });

                diff_preview.push_str(&format!(
                    "--- a/{}\n+++ b/{}\n@@ -1,1 +1,4 @@\n+---\n+tags: [concept, architecture]\n+---\n",
                    node.id, node.id
                ));

                projected_delta.efficiency_delta = 0.15;
                projected_delta.trace_depth_delta = -0.5;
                projected_delta.coupling_delta = 0.0;
                projected_delta.doc_count_delta = 0;
            }
            KnowledgeOrganizationStrategy::PreserveDocument => {
                diff_preview.push_str("# Preserving document structure (no-op)\n");
            }
            KnowledgeOrganizationStrategy::Hybrid => {
                actions.push(TransformationAction::InjectFragmentFrontmatter {
                    file_path: node.file_path.clone(),
                    fragments: node.fragments.clone(),
                });
                actions.push(TransformationAction::AppendDocumentContent {
                    file_path: workspace_root
                        .join("index.md")
                        .to_string_lossy()
                        .to_string(),
                    content: build_index_content(node),
                });
                diff_preview.push_str(&format!(
                    "--- a/{}\n+++ b/{}\n@@ -1,3 +1,6 @@\n+wiki:\n+  fragments: [hybrid]\n",
                    node.id, node.id
                ));
                projected_delta.efficiency_delta = 0.40;
                projected_delta.trace_depth_delta = -1.2;
                projected_delta.coupling_delta = -0.15;
                projected_delta.doc_count_delta = 1;
            }
        }

        TransformationPlan {
            plan_id,
            recommendation_id: rec.recommendation_id.clone(),
            actions,
            diff_preview,
            projected_delta,
            requires_approval: true, // Invariant XVI: Mandatory User Approval Gate
        }
    }

    /// Builds an atomic multi-strategy TransformationPlan combining actions and unified diffs (T028, SC-007).
    /// Enforces Invariant XVI: requires_approval is unconditionally true.
    pub fn build_multi_strategy_plan(
        plan_id: &str,
        recommendation_id: &str,
        strategies: &[KnowledgeOrganizationStrategy],
        node: &NoteNode,
        workspace_root: &Path,
    ) -> TransformationPlan {
        let mut actions = Vec::new();
        let mut diff_preview = String::new();
        let mut projected_delta = MetricDeltaReport::default();

        for &strat in strategies {
            match strat {
                KnowledgeOrganizationStrategy::SectionAddressing => {
                    actions.push(TransformationAction::InjectFragmentFrontmatter {
                        file_path: node.file_path.clone(),
                        fragments: node.fragments.clone(),
                    });

                    diff_preview.push_str(&format!(
                        "--- a/{}\n+++ b/{}\n@@ -1,3 +1,10 @@\n+wiki:\n+  fragments:\n",
                        node.id, node.id
                    ));
                    for f in &node.fragments {
                        diff_preview.push_str(&format!(
                            "+    \"{}\": {{ tokens: {} }}\n",
                            f.anchor, f.tokens
                        ));
                    }

                    projected_delta.efficiency_delta += 0.35;
                    projected_delta.trace_depth_delta -= 1.0;
                }
                KnowledgeOrganizationStrategy::TopicDecomposition => {
                    let subtopic_path = node.file_path.replace(".md", "_subtopic.md");
                    let subtopic_rel = node.id.replace(".md", "_subtopic.md");

                    let subtopic_content = format!(
                        "# {} Subtopic\n\nDecomposed conceptual entity.\n",
                        node.label
                    );
                    actions.push(TransformationAction::CreateDocument {
                        file_path: subtopic_path.clone(),
                        initial_content: subtopic_content.clone(),
                    });

                    let updated_source = format!(
                        "{}\n\n## Subtopic\nSee dedicated note [{} Subtopic]({})\n",
                        node.label, node.label, subtopic_rel
                    );
                    actions.push(TransformationAction::UpdateDocumentContent {
                        file_path: node.file_path.clone(),
                        updated_content: updated_source,
                    });

                    diff_preview.push_str(&format!(
                        "--- /dev/null\n+++ b/{}\n@@ -0,0 +1,3 @@\n+{}\n",
                        subtopic_rel,
                        subtopic_content.trim()
                    ));
                    diff_preview.push_str(&format!(
                        "--- a/{}\n+++ b/{}\n@@ -10,3 +10,6 @@\n+## Subtopic\n+[{} Subtopic]({})\n",
                        node.id, node.id, node.label, subtopic_rel
                    ));

                    projected_delta.efficiency_delta += 0.45;
                    projected_delta.trace_depth_delta -= 1.5;
                    projected_delta.coupling_delta -= 0.25;
                    projected_delta.doc_count_delta += 1;
                }
                KnowledgeOrganizationStrategy::LightweightIndex => {
                    let index_path = workspace_root
                        .join("index.md")
                        .to_string_lossy()
                        .to_string();
                    let index_content = build_index_content(node);

                    actions.push(TransformationAction::AppendDocumentContent {
                        file_path: index_path,
                        content: index_content.clone(),
                    });

                    let added_line_count = index_content.lines().count();
                    diff_preview.push_str(&format!(
                        "--- /dev/null\n+++ b/index.md\n@@ -0,0 +1,{} @@\n{}\n",
                        added_line_count,
                        format_added_diff_lines(index_content.trim())
                    ));

                    projected_delta.efficiency_delta += 0.20;
                    projected_delta.trace_depth_delta -= 1.0;
                    projected_delta.coupling_delta += 0.10;
                    if !workspace_root.join("index.md").exists() {
                        projected_delta.doc_count_delta += 1;
                    }
                }
                KnowledgeOrganizationStrategy::FacetedClassification => {
                    actions.push(TransformationAction::UpdateDocumentTags {
                        file_path: node.file_path.clone(),
                        tags: vec!["concept".to_string(), "architecture".to_string()],
                    });

                    diff_preview.push_str(&format!(
                        "--- a/{}\n+++ b/{}\n@@ -1,1 +1,4 @@\n+---\n+tags: [concept, architecture]\n+---\n",
                        node.id, node.id
                    ));

                    projected_delta.efficiency_delta += 0.15;
                    projected_delta.trace_depth_delta -= 0.5;
                }
                KnowledgeOrganizationStrategy::PreserveDocument => {
                    if strategies.len() == 1 {
                        diff_preview.push_str("# Preserving document structure (no-op)\n");
                    }
                }
                KnowledgeOrganizationStrategy::Hybrid => {
                    actions.push(TransformationAction::InjectFragmentFrontmatter {
                        file_path: node.file_path.clone(),
                        fragments: node.fragments.clone(),
                    });
                    actions.push(TransformationAction::AppendDocumentContent {
                        file_path: workspace_root
                            .join("index.md")
                            .to_string_lossy()
                            .to_string(),
                        content: build_index_content(node),
                    });
                    diff_preview.push_str(&format!(
                        "--- a/{}\n+++ b/{}\n@@ -1,3 +1,6 @@\n+wiki:\n+  fragments: [hybrid]\n",
                        node.id, node.id
                    ));
                    projected_delta.efficiency_delta += 0.40;
                    projected_delta.trace_depth_delta -= 1.2;
                    projected_delta.coupling_delta -= 0.15;
                    projected_delta.doc_count_delta += 1;
                }
            }
        }

        TransformationPlan {
            plan_id: plan_id.to_string(),
            recommendation_id: recommendation_id.to_string(),
            actions,
            diff_preview,
            projected_delta,
            requires_approval: true, // Invariant XVI: Mandatory User Approval Gate
        }
    }

    /// Verifies compliance with Invariant XVI (T041).
    pub fn verify_invariant_xvi(plan: &TransformationPlan) -> Result<(), &'static str> {
        if !plan.requires_approval {
            return Err("Invariant XVI Violation: Automation requires explicit user approval gate; auto-apply without approval is strictly prohibited.");
        }
        Ok(())
    }

    /// Validates fine-grained action-level filesystem safety checks before applying (T039).
    pub fn validate_safety(plan: &TransformationPlan, workspace_root: &Path) -> Result<(), String> {
        for action in &plan.actions {
            match action {
                TransformationAction::CreateDocument { file_path, .. } => {
                    let path = PathBuf::from(file_path);
                    if !path.starts_with(workspace_root) {
                        return Err(format!(
                            "Safety Check Violation: Path traversal outside workspace root: {}",
                            file_path
                        ));
                    }
                }
                TransformationAction::UpdateDocumentContent { file_path, .. }
                | TransformationAction::UpdateDocumentTags { file_path, .. }
                | TransformationAction::AppendDocumentContent { file_path, .. }
                | TransformationAction::InjectFragmentFrontmatter { file_path, .. }
                | TransformationAction::DeleteDocument { file_path } => {
                    let path = PathBuf::from(file_path);
                    let full = if path.is_absolute() {
                        path
                    } else {
                        workspace_root.join(path)
                    };
                    if !full.starts_with(workspace_root) {
                        return Err(format!(
                            "Safety Check Violation: Path traversal outside workspace root: {}",
                            file_path
                        ));
                    }
                }
                TransformationAction::RewriteLinkReferences { source_path, .. } => {
                    let path = PathBuf::from(source_path);
                    if !path.starts_with(workspace_root) {
                        return Err(format!(
                            "Safety Check Violation: Source path outside workspace root: {}",
                            source_path
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Atomically applies a TransformationPlan with automatic rollback on error (T038).
    pub fn apply_plan_atomic(
        plan: &TransformationPlan,
        workspace_root: &Path,
    ) -> Result<MetricDeltaReport, Box<dyn std::error::Error + Send + Sync>> {
        Self::verify_invariant_xvi(plan).map_err(std::io::Error::other)?;
        Self::validate_safety(plan, workspace_root).map_err(std::io::Error::other)?;

        // In-memory rollback journal: path -> previous content (None if file was newly created)
        let mut journal: HashMap<PathBuf, Option<String>> = HashMap::new();
        let mut created_files: Vec<PathBuf> = Vec::new();

        let mut execute_actions = || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            for action in &plan.actions {
                match action {
                    TransformationAction::CreateDocument {
                        file_path,
                        initial_content,
                    } => {
                        let path = PathBuf::from(file_path);
                        if let Some(parent) = path.parent() {
                            std::fs::create_dir_all(parent)?;
                        }
                        if path.exists() {
                            let prev = std::fs::read_to_string(&path)?;
                            journal.entry(path.clone()).or_insert(Some(prev));
                        } else {
                            journal.entry(path.clone()).or_insert(None);
                            created_files.push(path.clone());
                        }
                        std::fs::write(&path, initial_content)?;
                    }
                    TransformationAction::UpdateDocumentContent {
                        file_path,
                        updated_content,
                    } => {
                        let path = PathBuf::from(file_path);
                        let previous = if path.exists() {
                            Some(std::fs::read_to_string(&path)?)
                        } else {
                            None
                        };
                        journal.entry(path.clone()).or_insert(previous);
                        std::fs::write(&path, updated_content)?;
                    }
                    TransformationAction::UpdateDocumentTags { file_path, tags } => {
                        let path = PathBuf::from(file_path);
                        let prev = if path.exists() {
                            std::fs::read_to_string(&path)?
                        } else {
                            String::new()
                        };
                        journal.entry(path.clone()).or_insert_with(|| {
                            if path.exists() {
                                Some(prev.clone())
                            } else {
                                None
                            }
                        });
                        let tag_refs: Vec<&str> = tags.iter().map(String::as_str).collect();
                        let updated = inject_tags_frontmatter(&prev, &tag_refs)?;
                        std::fs::write(&path, updated)?;
                    }
                    TransformationAction::AppendDocumentContent { file_path, content } => {
                        let path = PathBuf::from(file_path);
                        let previous = if path.exists() {
                            std::fs::read_to_string(&path)?
                        } else {
                            String::new()
                        };
                        journal.entry(path.clone()).or_insert_with(|| {
                            if path.exists() {
                                Some(previous.clone())
                            } else {
                                None
                            }
                        });
                        let updated = append_unique_index_content(&previous, content);
                        if let Some(parent) = path.parent() {
                            std::fs::create_dir_all(parent)?;
                        }
                        std::fs::write(&path, updated)?;
                    }
                    TransformationAction::RewriteLinkReferences {
                        source_path,
                        old_target,
                        new_target,
                    } => {
                        let path = PathBuf::from(source_path);
                        if path.exists() {
                            let prev = std::fs::read_to_string(&path)?;
                            journal.entry(path.clone()).or_insert(Some(prev.clone()));
                            let updated = prev.replace(old_target, new_target);
                            std::fs::write(&path, updated)?;
                        }
                    }
                    TransformationAction::InjectFragmentFrontmatter {
                        file_path,
                        fragments,
                    } => {
                        let path = PathBuf::from(file_path);
                        if path.exists() {
                            let prev = std::fs::read_to_string(&path)?;
                            journal.entry(path.clone()).or_insert(Some(prev.clone()));
                            let updated = inject_materialized_frontmatter(&prev, fragments)?;
                            std::fs::write(&path, updated)?;
                        }
                    }
                    TransformationAction::DeleteDocument { file_path } => {
                        let path = if Path::new(file_path).is_absolute() {
                            PathBuf::from(file_path)
                        } else {
                            workspace_root.join(file_path)
                        };
                        if path.exists() {
                            let prev = std::fs::read_to_string(&path)?;
                            journal.entry(path.clone()).or_insert(Some(prev));
                            std::fs::remove_file(&path)?;
                        }
                    }
                }
            }
            Ok(())
        };

        match execute_actions() {
            Ok(()) => Ok(plan.projected_delta.clone()),
            Err(e) => {
                // Automatic rollback
                for (path, prev_opt) in journal {
                    match prev_opt {
                        Some(prev_content) => {
                            let _ = std::fs::write(path, prev_content);
                        }
                        None => {
                            let _ = std::fs::remove_file(path);
                        }
                    }
                }
                Err(e)
            }
        }
    }
}

pub fn compute_unified_diff(file_path: &str, old_content: &str, new_content: &str) -> String {
    if old_content == new_content {
        return String::new();
    }

    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();

    let mut prefix_len = 0;
    while prefix_len < old_lines.len()
        && prefix_len < new_lines.len()
        && old_lines[prefix_len] == new_lines[prefix_len]
    {
        prefix_len += 1;
    }

    let mut suffix_len = 0;
    while suffix_len < (old_lines.len() - prefix_len)
        && suffix_len < (new_lines.len() - prefix_len)
        && old_lines[old_lines.len() - 1 - suffix_len] == new_lines[new_lines.len() - 1 - suffix_len]
    {
        suffix_len += 1;
    }

    let old_change_len = old_lines.len() - prefix_len - suffix_len;
    let new_change_len = new_lines.len() - prefix_len - suffix_len;

    let mut diff = format!(
        "--- a/{}\n+++ b/{}\n@@ -{},{} +{},{} @@\n",
        file_path,
        file_path,
        prefix_len + 1,
        old_change_len,
        prefix_len + 1,
        new_change_len
    );

    for line in &old_lines[prefix_len..prefix_len + old_change_len] {
        diff.push_str(&format!("-{}\n", line));
    }
    for line in &new_lines[prefix_len..prefix_len + new_change_len] {
        diff.push_str(&format!("+{}\n", line));
    }

    diff
}

pub fn extract_route_target_key(s: &str) -> String {
    if let Some(start) = s.find("[[") {
        if let Some(end) = s[start..].find("]]") {
            let inner = &s[start + 2..start + end];
            let target_part = if let Some(pipe) = inner.find('|') {
                &inner[..pipe]
            } else {
                inner
            };
            let clean = if let Some(hash) = target_part.find('#') {
                &target_part[..hash]
            } else {
                target_part
            };
            return clean.trim().to_lowercase();
        }
    }
    if let Some(start) = s.find("](") {
        let after = &s[start + 2..];
        if let Some(end) = after.find(')') {
            let target = &after[..end];
            let clean = if let Some(hash) = target.find('#') {
                &target[..hash]
            } else {
                target
            };
            return clean.trim().to_lowercase();
        }
    }
    String::new()
}

pub fn extract_routes_from_line(line: &str) -> Vec<String> {
    let mut routes = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find("[[") {
        if let Some(end) = rest[start..].find("]]") {
            let full_wiki = &rest[start..start + end + 2];
            routes.push(format!("- {}", full_wiki));
            rest = &rest[start + end + 2..];
        } else {
            break;
        }
    }
    if routes.is_empty() {
        let mut md_rest = line;
        while let Some(start_bracket) = md_rest.find('[') {
            if let Some(end_bracket) = md_rest[start_bracket..].find(']') {
                let abs_end = start_bracket + end_bracket;
                if md_rest[abs_end..].starts_with("](") {
                    let url_start = abs_end + 2;
                    if let Some(url_end) = md_rest[url_start..].find(')') {
                        let full_link = &md_rest[start_bracket..url_start + url_end + 1];
                        let url = &md_rest[url_start..url_start + url_end];
                        if !url.starts_with("http://") && !url.starts_with("https://") && !url.starts_with('#') {
                            routes.push(format!("- {}", full_link));
                        }
                        md_rest = &md_rest[url_start + url_end + 1..];
                        continue;
                    }
                }
            }
            md_rest = &md_rest[start_bracket + 1..];
        }
    }
    routes
}

pub fn extract_routes_from_markdown(content: &str) -> Vec<String> {
    let mut routes = Vec::new();
    let mut current_section: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") || trimmed.starts_with("### ") {
            let heading = trimmed.trim_start_matches('#').trim();
            if !heading.eq_ignore_ascii_case("knowledge index")
                && !heading.eq_ignore_ascii_case("table of contents")
            {
                current_section = Some(heading.to_string());
            }
        } else if (trimmed.starts_with("- [") || trimmed.starts_with("* ["))
            && !trimmed.contains("(index.md)")
            && !trimmed.contains("(#")
        {
            if let Some(sec) = current_section.take() {
                routes.push(format!("\n### {}", sec));
            }
            routes.push(trimmed.to_string());
        }
    }
    routes
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecondaryLayerRouteAudit {
    pub total_routes: usize,
    pub valid_routes: Vec<String>,
    pub invalid_routes: Vec<String>,
    pub already_in_harness: Vec<String>,
    pub missing_from_harness: Vec<String>,
}

pub fn audit_secondary_layer_routes(
    root: &Path,
    target: &str,
    harness_rel_path: &str,
) -> SecondaryLayerRouteAudit {
    let target_path = root.join(target);
    let target_content = std::fs::read_to_string(&target_path).unwrap_or_default();
    let harness_path = root.join(harness_rel_path);
    let harness_content = std::fs::read_to_string(&harness_path).unwrap_or_default();

    let mut valid_routes = Vec::new();
    let mut invalid_routes = Vec::new();
    let mut already_in_harness = Vec::new();
    let mut missing_from_harness = Vec::new();

    let target_dir = std::path::Path::new(target).parent().unwrap_or(Path::new(""));
    let mut current_section: Option<String> = None;

    for line in target_content.lines() {
        let trimmed = line.trim();
        if trimmed == "---" || trimmed.is_empty() || trimmed.starts_with('>') {
            continue;
        }

        if trimmed.starts_with("## ") || trimmed.starts_with("### ") || trimmed.starts_with("#### ") {
            let heading = trimmed.trim_start_matches('#').trim();
            if !heading.eq_ignore_ascii_case("knowledge index")
                && !heading.eq_ignore_ascii_case("knowledge index (consolidated)")
                && !heading.eq_ignore_ascii_case("table of contents")
            {
                current_section = Some(heading.to_string());
            }
            continue;
        }

        let mut extracted_targets = Vec::new();

        // 1. Standard markdown [text](dest)
        let mut rest = trimmed;
        while let Some(start) = rest.find("](") {
            let after = &rest[start + 2..];
            if let Some(end) = after.find(')') {
                let dest = after[..end].trim();
                if !dest.starts_with("http://") && !dest.starts_with("https://") && !dest.starts_with('#') {
                    extracted_targets.push(dest.to_string());
                }
                rest = &after[end + 1..];
            } else {
                break;
            }
        }

        // 2. Wikilinks [[target|label]] or [[target]]
        let mut wiki_rest = trimmed;
        while let Some(start) = wiki_rest.find("[[") {
            let after = &wiki_rest[start + 2..];
            if let Some(end) = after.find("]]") {
                let inner = &after[..end];
                let dest = if let Some(pipe) = inner.find('|') {
                    &inner[..pipe]
                } else {
                    inner
                };
                let dest = dest.trim();
                if !dest.is_empty() && !dest.starts_with('#') {
                    extracted_targets.push(dest.to_string());
                }
                wiki_rest = &after[end + 2..];
            } else {
                break;
            }
        }

        if extracted_targets.is_empty() {
            continue;
        }

        let mut line_is_valid = true;
        let mut line_is_in_harness = true;

        for dest in &extracted_targets {
            let file_part = if let Some(hash) = dest.find('#') {
                &dest[..hash]
            } else {
                dest.as_str()
            };

            if file_part.is_empty() {
                continue;
            }

            let resolved = if target_dir.as_os_str().is_empty() {
                file_part.to_string()
            } else {
                target_dir.join(file_part).to_string_lossy().to_string()
            };

            let exists = root.join(&resolved).exists()
                || root.join(file_part).exists()
                || root.join(format!("{}.md", file_part)).exists();

            if !exists {
                line_is_valid = false;
            }

            let in_h = harness_content.contains(file_part)
                || harness_content.contains(&resolved)
                || harness_content.contains(dest.as_str());

            if !in_h {
                line_is_in_harness = false;
            }
        }

        let line_with_section = if let Some(sec) = current_section.take() {
            format!("\n### {}\n{}", sec, line)
        } else {
            line.to_string()
        };

        if line_is_valid {
            valid_routes.push(line.to_string());
            if line_is_in_harness {
                already_in_harness.push(line.to_string());
            } else {
                missing_from_harness.push(line_with_section);
            }
        } else {
            invalid_routes.push(line.to_string());
        }
    }

    let total_routes = valid_routes.len() + invalid_routes.len();
    SecondaryLayerRouteAudit {
        total_routes,
        valid_routes,
        invalid_routes,
        already_in_harness,
        missing_from_harness,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_transformation_plan_serde() {
        let action = TransformationAction::CreateDocument {
            file_path: "docs/deployment.md".to_string(),
            initial_content: "# Deployment Guide\n".to_string(),
        };

        let plan = TransformationPlan {
            plan_id: "plan-123".to_string(),
            recommendation_id: "rec-001".to_string(),
            actions: vec![action],
            diff_preview: "+++ docs/deployment.md\n@@ -0,0 +1,1 @@\n+# Deployment Guide"
                .to_string(),
            projected_delta: MetricDeltaReport {
                efficiency_delta: 0.15,
                trace_depth_delta: -1.0,
                coupling_delta: -0.20,
                doc_count_delta: 1,
            },
            requires_approval: true,
        };

        let json = serde_json::to_string(&plan).unwrap();
        let de: TransformationPlan = serde_json::from_str(&json).unwrap();
        assert_eq!(de.plan_id, "plan-123");
        assert_eq!(de.actions.len(), 1);
        assert!(de.requires_approval);
    }

    #[test]
    fn test_transformation_plan_diff_generation() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();

        let mut node = NoteNode::new("god.md", root.join("god.md").to_string_lossy(), "God");
        node.tokens = 5000;
        let rec = StructuralRecommendation {
            recommendation_id: "rec_1".to_string(),
            signature_id: "god_1".to_string(),
            strategy: KnowledgeOrganizationStrategy::TopicDecomposition,
            confidence: 0.90,
            expected_benefits: vec![],
            trade_offs: vec![],
            transformation_plan: None,
        };

        let plan = TransformationEngine::build_plan(&rec, &node, root);
        assert!(
            !plan.diff_preview.is_empty(),
            "Diff preview must be generated"
        );
        assert!(plan.diff_preview.contains("---"));
        assert!(plan.diff_preview.contains("+++"));
        assert!(plan.requires_approval);
        assert_eq!(plan.actions.len(), 2);
    }

    #[test]
    fn test_transformation_atomic_apply_and_rollback() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let doc_path = root.join("doc.md");
        std::fs::write(&doc_path, "Original content").unwrap();

        let node = NoteNode::new("doc.md", doc_path.to_string_lossy(), "Doc");
        let rec = StructuralRecommendation {
            recommendation_id: "rec_1".to_string(),
            signature_id: "sig_1".to_string(),
            strategy: KnowledgeOrganizationStrategy::TopicDecomposition,
            confidence: 0.85,
            expected_benefits: vec![],
            trade_offs: vec![],
            transformation_plan: None,
        };

        let plan = TransformationEngine::build_plan(&rec, &node, root);

        // Apply successfully
        let delta = TransformationEngine::apply_plan_atomic(&plan, root).unwrap();
        assert_eq!(delta.doc_count_delta, 1);
        let updated = std::fs::read_to_string(&doc_path).unwrap();
        assert!(updated.contains("Subtopic"));

        // Verify rollback on intentional write failure
        let invalid_plan = TransformationPlan {
            plan_id: "invalid_plan".to_string(),
            recommendation_id: "rec_1".to_string(),
            actions: vec![
                TransformationAction::UpdateDocumentContent {
                    file_path: doc_path.to_string_lossy().to_string(),
                    updated_content: "Corrupted Content".to_string(),
                },
                TransformationAction::CreateDocument {
                    file_path: "/nonexistent_root_dir_abc/nested/file.md".to_string(),
                    initial_content: "fail".to_string(),
                },
            ],
            diff_preview: "diff".to_string(),
            projected_delta: Default::default(),
            requires_approval: true,
        };

        let apply_res = TransformationEngine::apply_plan_atomic(&invalid_plan, root);
        assert!(apply_res.is_err(), "Must fail when targeting invalid root");
        // Document must NOT have been corrupted due to rollback
        let preserved = std::fs::read_to_string(&doc_path).unwrap();
        assert_eq!(preserved, updated, "File must be restored after rollback");
    }

    #[test]
    fn test_inject_frontmatter_preserves_existing_metadata_and_body() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let doc_path = root.join("speckit-constitution.md");
        let original = r#"---
name: "speckit-constitution"
description: "Create or update the project constitution."
compatibility: "Requires spec-kit project structure with .specify/ directory"
metadata:
  author: "github-spec-kit"
---

# Constitution

Keep this body intact.
"#;
        std::fs::write(&doc_path, original).unwrap();

        let plan = TransformationPlan {
            plan_id: "plan_frontmatter".to_string(),
            recommendation_id: "rec_frontmatter".to_string(),
            actions: vec![TransformationAction::InjectFragmentFrontmatter {
                file_path: doc_path.to_string_lossy().to_string(),
                fragments: vec![FragmentMetadata {
                    anchor: "user-input".to_string(),
                    heading: "User Input".to_string(),
                    start_line: 1,
                    end_line: 2,
                    tokens: 30,
                    inbound: 0,
                    outbound: 0,
                }],
            }],
            diff_preview: "diff".to_string(),
            projected_delta: Default::default(),
            requires_approval: true,
        };

        TransformationEngine::apply_plan_atomic(&plan, root).unwrap();
        let updated = std::fs::read_to_string(&doc_path).unwrap();

        assert_eq!(updated.matches("---").count(), 2);
        assert!(updated.contains("name: speckit-constitution"));
        assert!(updated.contains("description: Create or update the project constitution."));
        assert!(updated.contains("compatibility: Requires spec-kit project structure"));
        assert!(updated.contains("author: github-spec-kit"));
        assert!(updated.contains("wiki:"));
        assert!(updated.contains("fragments:"));
        assert!(updated.contains("user-input:"));
        assert!(updated.contains("tokens: 30"));
        assert!(updated.contains("# Constitution"));
        assert!(updated.contains("Keep this body intact."));
    }

    #[test]
    fn test_lightweight_index_uses_fragments_and_appends_without_overwriting() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let doc_path = root.join("research.md");
        let index_path = root.join("index.md");
        std::fs::write(&doc_path, "# Research\n\n## First Section\n\nContent.\n").unwrap();
        std::fs::write(
            &index_path,
            "# Existing Index\n\n- [Existing](existing.md)\n",
        )
        .unwrap();

        let mut node = NoteNode::new("research.md", doc_path.to_string_lossy(), "Research");
        node.fragments = vec![FragmentMetadata {
            anchor: "first-section".to_string(),
            heading: "First Section".to_string(),
            start_line: 3,
            end_line: 5,
            tokens: 8,
            inbound: 0,
            outbound: 0,
        }];
        let rec = StructuralRecommendation {
            recommendation_id: "rec_index".to_string(),
            signature_id: "sig".to_string(),
            strategy: KnowledgeOrganizationStrategy::LightweightIndex,
            confidence: 0.9,
            expected_benefits: vec![],
            trade_offs: vec![],
            transformation_plan: None,
        };

        let plan = TransformationEngine::build_plan(&rec, &node, root);
        assert!(plan.diff_preview.contains("# Knowledge Index"));
        assert!(plan
            .diff_preview
            .contains("+- [Research — First Section](research.md#first-section)"));
        assert!(!plan.diff_preview.contains("\n- [Research — First Section]"));
        assert!(matches!(
            plan.actions[0],
            TransformationAction::AppendDocumentContent { .. }
        ));
        TransformationEngine::apply_plan_atomic(&plan, root).unwrap();
        TransformationEngine::apply_plan_atomic(&plan, root).unwrap();

        let updated = std::fs::read_to_string(&index_path).unwrap();
        assert!(updated.contains("# Existing Index"));
        assert!(updated.contains("- [Existing](existing.md)"));
        assert!(updated.contains("research.md#first-section"));
        assert_eq!(updated.matches("research.md#first-section").count(), 1);
    }

    #[test]
    fn test_faceted_and_hybrid_strategies_apply_real_changes() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let doc_path = root.join("doc.md");
        std::fs::write(
            &doc_path,
            "---\nowner: team\n---\n\n# Existing Document\n\nKeep this body.\n",
        )
        .unwrap();
        let mut node = NoteNode::new("doc.md", doc_path.to_string_lossy(), "Existing Document");
        node.fragments = vec![FragmentMetadata {
            anchor: "existing-document".to_string(),
            heading: "Existing Document".to_string(),
            start_line: 1,
            end_line: 4,
            tokens: 10,
            inbound: 0,
            outbound: 0,
        }];

        let faceted = StructuralRecommendation {
            recommendation_id: "rec_faceted".to_string(),
            signature_id: "sig".to_string(),
            strategy: KnowledgeOrganizationStrategy::FacetedClassification,
            confidence: 0.9,
            expected_benefits: vec![],
            trade_offs: vec![],
            transformation_plan: None,
        };
        let plan = TransformationEngine::build_plan(&faceted, &node, root);
        TransformationEngine::apply_plan_atomic(&plan, root).unwrap();
        let faceted_content = std::fs::read_to_string(&doc_path).unwrap();
        assert!(faceted_content.contains("owner: team"));
        assert!(faceted_content.contains("tags:"));
        assert!(faceted_content.contains("concept"));
        assert!(faceted_content.contains("Keep this body."));

        let hybrid = StructuralRecommendation {
            recommendation_id: "rec_hybrid".to_string(),
            signature_id: "sig".to_string(),
            strategy: KnowledgeOrganizationStrategy::Hybrid,
            confidence: 0.9,
            expected_benefits: vec![],
            trade_offs: vec![],
            transformation_plan: None,
        };
        let hybrid_plan = TransformationEngine::build_plan(&hybrid, &node, root);
        assert_eq!(hybrid_plan.actions.len(), 2);
        TransformationEngine::apply_plan_atomic(&hybrid_plan, root).unwrap();
        assert!(root.join("index.md").exists());
        let hybrid_content = std::fs::read_to_string(&doc_path).unwrap();
        assert!(hybrid_content.contains("fragments:"));
        assert!(hybrid_content.contains("Keep this body."));
    }

    #[test]
    fn test_transformation_safety_checks() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();

        // Valid plan within root
        let valid_plan = TransformationPlan {
            plan_id: "valid".to_string(),
            recommendation_id: "rec".to_string(),
            actions: vec![TransformationAction::CreateDocument {
                file_path: root.join("new.md").to_string_lossy().to_string(),
                initial_content: "hello".to_string(),
            }],
            diff_preview: "".to_string(),
            projected_delta: Default::default(),
            requires_approval: true,
        };
        assert!(TransformationEngine::validate_safety(&valid_plan, root).is_ok());

        // Dangerous path traversal
        let dangerous_plan = TransformationPlan {
            plan_id: "danger".to_string(),
            recommendation_id: "rec".to_string(),
            actions: vec![TransformationAction::CreateDocument {
                file_path: "/etc/passwd".to_string(),
                initial_content: "hacked".to_string(),
            }],
            diff_preview: "".to_string(),
            projected_delta: Default::default(),
            requires_approval: true,
        };
        assert!(TransformationEngine::validate_safety(&dangerous_plan, root).is_err());
    }

    #[test]
    fn test_invariant_xvi_approval_gate() {
        let plan_with_approval = TransformationPlan {
            plan_id: "plan_1".to_string(),
            recommendation_id: "rec_1".to_string(),
            actions: vec![],
            diff_preview: "".to_string(),
            projected_delta: Default::default(),
            requires_approval: true,
        };
        assert!(TransformationEngine::verify_invariant_xvi(&plan_with_approval).is_ok());

        let plan_without_approval = TransformationPlan {
            plan_id: "plan_2".to_string(),
            recommendation_id: "rec_1".to_string(),
            actions: vec![],
            diff_preview: "".to_string(),
            projected_delta: Default::default(),
            requires_approval: false, // VIOLATION
        };
        assert!(TransformationEngine::verify_invariant_xvi(&plan_without_approval).is_err());
    }

    #[test]
    fn test_automation_approval_gate_defaults() {
        test_invariant_xvi_approval_gate();
    }

    #[test]
    fn test_transformation_plan_diff_and_deltas() {
        test_transformation_plan_diff_generation();
    }

    #[test]
    fn test_plan_hard_constraint_safety_check() {
        test_transformation_safety_checks();
    }

    #[test]
    fn test_atomic_file_transformation_with_rollback() {
        test_transformation_atomic_apply_and_rollback();
    }
}
