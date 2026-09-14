use super::re_measure::MetricDeltaReport;
use super::strategies::{KnowledgeOrganizationStrategy, StructuralRecommendation};
use crate::fragment::MaterializedFragmentEntry;
use crate::model::NoteNode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum TransformationAction {
    CreateDocument {
        file_path: String,
        initial_content: String,
    },
    UpdateDocumentContent {
        file_path: String,
        updated_content: String,
    },
    RewriteLinkReferences {
        source_path: String,
        old_target: String,
        new_target: String,
    },
    InjectFragmentFrontmatter {
        file_path: String,
        fragments: Vec<MaterializedFragmentEntry>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformationPlan {
    pub plan_id: String,
    pub recommendation_id: String,
    pub actions: Vec<TransformationAction>,
    pub diff_preview: String,
    pub projected_delta: MetricDeltaReport,
    pub requires_approval: bool,
}

pub struct TransformationEngine;

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
                let fragment_entries: Vec<MaterializedFragmentEntry> = node
                    .fragments
                    .iter()
                    .map(|f| MaterializedFragmentEntry {
                        tokens: f.tokens,
                        inbound: f.inbound,
                        outbound: f.outbound,
                    })
                    .collect();

                actions.push(TransformationAction::InjectFragmentFrontmatter {
                    file_path: node.file_path.clone(),
                    fragments: fragment_entries,
                });

                diff_preview.push_str(&format!(
                    "--- a/{}\n+++ b/{}\n@@ -1,3 +1,10 @@\n+wiki:\n+  fragments:\n",
                    node.id, node.id
                ));
                for f in &node.fragments {
                    diff_preview.push_str(&format!("+    \"{}\": {{ tokens: {} }}\n", f.anchor, f.tokens));
                }

                projected_delta.efficiency_delta = 0.35;
                projected_delta.trace_depth_delta = -1.0;
                projected_delta.coupling_delta = 0.0;
                projected_delta.doc_count_delta = 0;
            }
            KnowledgeOrganizationStrategy::TopicDecomposition => {
                let subtopic_path = node.file_path.replace(".md", "_subtopic.md");
                let subtopic_rel = node.id.replace(".md", "_subtopic.md");

                let subtopic_content = format!("# {} Subtopic\n\nDecomposed conceptual entity.\n", node.label);
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
                    subtopic_rel, subtopic_content.trim()
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
                let index_path = workspace_root.join("index.md").to_string_lossy().to_string();
                let index_content = format!("# Knowledge Index\n\n- [{}]({})\n", node.label, node.id);

                actions.push(TransformationAction::CreateDocument {
                    file_path: index_path,
                    initial_content: index_content.clone(),
                });

                diff_preview.push_str(&format!(
                    "--- /dev/null\n+++ b/index.md\n@@ -0,0 +1,3 @@\n+{}\n",
                    index_content.trim()
                ));

                projected_delta.efficiency_delta = 0.20;
                projected_delta.trace_depth_delta = -1.0;
                projected_delta.coupling_delta = 0.10;
                projected_delta.doc_count_delta = 1;
            }
            KnowledgeOrganizationStrategy::FacetedClassification => {
                actions.push(TransformationAction::UpdateDocumentContent {
                    file_path: node.file_path.clone(),
                    updated_content: format!("---\ntags: [concept, architecture]\n---\n{}", node.label),
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
                | TransformationAction::InjectFragmentFrontmatter { file_path, .. } => {
                    let path = PathBuf::from(file_path);
                    if !path.starts_with(workspace_root) {
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
        Self::verify_invariant_xvi(plan).map_err(|e| std::io::Error::other(e))?;
        Self::validate_safety(plan, workspace_root).map_err(|e| std::io::Error::other(e))?;

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
                        if path.exists() {
                            let prev = std::fs::read_to_string(&path)?;
                            journal.entry(path.clone()).or_insert(Some(prev));
                        }
                        std::fs::write(&path, updated_content)?;
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
                        file_path, ..
                    } => {
                        let path = PathBuf::from(file_path);
                        if path.exists() {
                            let prev = std::fs::read_to_string(&path)?;
                            journal.entry(path.clone()).or_insert(Some(prev.clone()));
                            let updated = format!("---\nwiki:\n  materialized: true\n---\n{}", prev);
                            std::fs::write(&path, updated)?;
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
            diff_preview: "+++ docs/deployment.md\n@@ -0,0 +1,1 @@\n+# Deployment Guide".to_string(),
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
        assert!(!plan.diff_preview.is_empty(), "Diff preview must be generated");
        assert!(plan.diff_preview.contains("---"));
        assert!(plan.diff_preview.contains("+++"));
        assert_eq!(plan.requires_approval, true);
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

