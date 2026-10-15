//! Skill structure and capability auditor.
//!
//! Enforces FR-021, FR-022, and FR-024:
//! - Validates SKILL.md presence, script/template disk existence, and references.
//! - Detects unreferenced/orphan skills.
//! - Computes capability overlap between skills (proposing consolidation when overlap >= 80%).

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use super::{KnowledgeIndexRoute, SkillAsset, SkillAssetType, SkillDefinition, SkillValidationStatus};

pub struct SkillAuditor;

impl SkillAuditor {
    /// Discovers and audits all skills in the repository.
    pub fn audit_repository(
        root_path: &Path,
        harness_routes: &[KnowledgeIndexRoute],
        vault_files: &[String],
    ) -> Vec<SkillDefinition> {
        let mut skill_dirs = Vec::new();

        // Standard locations for agent skills
        let candidate_parent_dirs = [
            root_path.join(".agents").join("skills"),
            root_path.join(".claude").join("skills"),
            root_path.join("skills"),
        ];

        for parent in &candidate_parent_dirs {
            if parent.is_dir() {
                if let Ok(entries) = fs::read_dir(parent) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            skill_dirs.push(path);
                        }
                    }
                }
            }
        }

        let mut skills = Vec::new();

        for skill_dir in skill_dirs {
            let skill_id = skill_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            let rel_dir = skill_dir
                .strip_prefix(root_path)
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| skill_id.clone());

            let manifest_path_buf = skill_dir.join("SKILL.md");
            let rel_manifest = format!("{}/SKILL.md", rel_dir);

            if !manifest_path_buf.is_file() {
                // Missing manifest
                skills.push(SkillDefinition {
                    skill_id: skill_id.clone(),
                    dir_path: rel_dir,
                    manifest_path: rel_manifest,
                    name: skill_id,
                    description: String::new(),
                    assets: Vec::new(),
                    status: SkillValidationStatus::MissingManifest,
                    unreferenced: true,
                    overlap_ids: Vec::new(),
                });
                continue;
            }

            // Manifest exists, read and parse
            let content = fs::read_to_string(&manifest_path_buf).unwrap_or_default();
            let (name, description) = Self::parse_manifest_metadata(&content, &skill_id);

            // Audit assets
            let mut assets = Vec::new();
            let mut seen_asset_paths = HashSet::new();

            // 1. Scan filesystem for assets in standard subdirectories
            for sub in &["scripts", "templates", "references", "resources"] {
                let sub_dir = skill_dir.join(sub);
                if sub_dir.is_dir() {
                    if let Ok(entries) = fs::read_dir(&sub_dir) {
                        for entry in entries.flatten() {
                            let asset_p = entry.path();
                            if asset_p.is_file() {
                                let file_name = asset_p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                                let rel_asset = format!("{}/{}", sub, file_name);
                                seen_asset_paths.insert(rel_asset.clone());

                                let asset_type = match *sub {
                                    "scripts" => SkillAssetType::Script,
                                    "templates" => SkillAssetType::Template,
                                    _ => SkillAssetType::Reference,
                                };
                                let byte_size = entry.metadata().map(|m| m.len()).unwrap_or(0);

                                assets.push(SkillAsset {
                                    asset_type,
                                    file_path: rel_asset,
                                    exists_on_disk: true,
                                    byte_size,
                                });
                            }
                        }
                    }
                }
            }

            // 2. Scan markdown content for explicit asset references (e.g. [deploy.sh](scripts/deploy.sh))
            let referenced_assets = Self::extract_referenced_assets(&content);
            for (rel_path, asset_type) in referenced_assets {
                if !seen_asset_paths.contains(&rel_path) {
                    seen_asset_paths.insert(rel_path.clone());
                    let disk_path = skill_dir.join(&rel_path);
                    let exists = disk_path.is_file();
                    let byte_size = if exists {
                        fs::metadata(&disk_path).map(|m| m.len()).unwrap_or(0)
                    } else {
                        0
                    };

                    assets.push(SkillAsset {
                        asset_type,
                        file_path: rel_path,
                        exists_on_disk: exists,
                        byte_size,
                    });
                }
            }

            // Determine status
            let has_broken_assets = assets.iter().any(|a| !a.exists_on_disk);
            let status = if has_broken_assets {
                SkillValidationStatus::BrokenAssetReference
            } else {
                SkillValidationStatus::Valid
            };

            // Check if unreferenced
            let unreferenced = Self::is_unreferenced(&skill_id, &name, &rel_manifest, harness_routes, vault_files, root_path);

            skills.push(SkillDefinition {
                skill_id,
                dir_path: rel_dir,
                manifest_path: rel_manifest,
                name,
                description,
                assets,
                status,
                unreferenced,
                overlap_ids: Vec::new(),
            });
        }

        // Overlap detection pass (FR-024)
        Self::detect_overlaps(&mut skills);

        skills.sort_by(|a, b| a.skill_id.cmp(&b.skill_id));
        skills
    }

    fn parse_manifest_metadata(content: &str, fallback_id: &str) -> (String, String) {
        let mut name = fallback_id.to_string();
        let mut description = String::new();

        if let Some(frontmatter) = extract_yaml_frontmatter(content) {
            for line in frontmatter.lines() {
                let trimmed = line.trim();
                if let Some(val) = trimmed.strip_prefix("name:") {
                    let n = val.trim().trim_matches('"').trim_matches('\'');
                    if !n.is_empty() {
                        name = n.to_string();
                    }
                } else if let Some(val) = trimmed.strip_prefix("description:") {
                    let d = val.trim().trim_matches('"').trim_matches('\'');
                    if !d.is_empty() {
                        description = d.to_string();
                    }
                }
            }
        }

        if description.is_empty() {
            // Fall back to first non-heading paragraph
            for line in content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with("---") {
                    description = trimmed.to_string();
                    break;
                }
            }
        }

        (name, description)
    }

    fn extract_referenced_assets(content: &str) -> Vec<(String, SkillAssetType)> {
        let mut results = Vec::new();
        // Scan for markdown links and paths
        for line in content.lines() {
            // Find patterns like (scripts/...) or (templates/...) or (references/...)
            let mut remaining = line;
            while let Some(start) = remaining.find('(') {
                if let Some(end) = remaining[start..].find(')') {
                    let target = &remaining[start + 1..start + end];
                    let normalized = target.trim().replace('\\', "/");
                    if normalized.starts_with("scripts/")
                        || normalized.starts_with("templates/")
                        || normalized.starts_with("references/")
                        || normalized.starts_with("resources/")
                    {
                        let asset_type = if normalized.starts_with("scripts/") {
                            SkillAssetType::Script
                        } else if normalized.starts_with("templates/") {
                            SkillAssetType::Template
                        } else {
                            SkillAssetType::Reference
                        };
                        results.push((normalized, asset_type));
                    }
                    remaining = &remaining[start + end + 1..];
                } else {
                    break;
                }
            }
        }
        results
    }

    fn is_unreferenced(
        skill_id: &str,
        name: &str,
        manifest_path: &str,
        harness_routes: &[KnowledgeIndexRoute],
        vault_files: &[String],
        root_path: &Path,
    ) -> bool {
        // 1. Check routes
        for route in harness_routes {
            if route.target_file == manifest_path
                || route.target_file.contains(skill_id)
                || route.label.eq_ignore_ascii_case(name)
            {
                return false;
            }
        }

        // 2. Check vault file contents
        let needle_id = skill_id.to_lowercase();
        let needle_manifest = manifest_path.to_lowercase();

        for vf in vault_files {
            let full_p = root_path.join(vf);
            if let Ok(c) = fs::read_to_string(&full_p) {
                let lower = c.to_lowercase();
                if lower.contains(&needle_id) || lower.contains(&needle_manifest) {
                    return false;
                }
            }
        }

        true
    }

    fn detect_overlaps(skills: &mut [SkillDefinition]) {
        let n = skills.len();
        let mut overlap_pairs = Vec::new();

        for i in 0..n {
            for j in (i + 1)..n {
                let sim = Self::calculate_similarity(&skills[i], &skills[j]);
                if sim >= 0.80 {
                    overlap_pairs.push((i, j));
                }
            }
        }

        for (i, j) in overlap_pairs {
            let id_j = skills[j].skill_id.clone();
            let id_i = skills[i].skill_id.clone();

            if !skills[i].overlap_ids.contains(&id_j) {
                skills[i].overlap_ids.push(id_j);
            }
            if !skills[j].overlap_ids.contains(&id_i) {
                skills[j].overlap_ids.push(id_i);
            }

            if skills[i].status == SkillValidationStatus::Valid {
                skills[i].status = SkillValidationStatus::DuplicateCapability;
            }
            if skills[j].status == SkillValidationStatus::Valid {
                skills[j].status = SkillValidationStatus::DuplicateCapability;
            }
        }
    }

    fn calculate_similarity(a: &SkillDefinition, b: &SkillDefinition) -> f32 {
        if a.name == b.name && a.description == b.description && !a.name.is_empty() {
            return 1.0;
        }

        let words_a = Self::tokenize(&format!("{} {}", a.name, a.description));
        let words_b = Self::tokenize(&format!("{} {}", b.name, b.description));

        if words_a.is_empty() || words_b.is_empty() {
            return 0.0;
        }

        let intersection: HashSet<_> = words_a.intersection(&words_b).cloned().collect();
        let union: HashSet<_> = words_a.union(&words_b).cloned().collect();

        if union.is_empty() {
            0.0
        } else {
            (intersection.len() as f32) / (union.len() as f32)
        }
    }

    fn tokenize(text: &str) -> HashSet<String> {
        text.split_whitespace()
            .map(|w| {
                w.to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            })
            .filter(|w| w.len() > 2)
            .collect()
    }
}

fn extract_yaml_frontmatter(content: &str) -> Option<&str> {
    if !content.starts_with("---") {
        return None;
    }
    let rest = &content[3..];
    let end = rest.find("\n---")?;
    Some(&rest[..end])
}
