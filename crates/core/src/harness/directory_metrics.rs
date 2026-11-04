//! Physical directory and file structure metrics.
//! Implements FR-025, FR-026, FR-027, Invariant XIII.

use super::{
    DirectoryStructureMetrics, OrganizationalSignal, OrganizationalSignalType,
    CONCENTRATION_ENTROPY_THRESHOLD, CROSS_FOLDER_COUPLING_THRESHOLD,
    DOMAIN_MIXING_THRESHOLD, MAX_NAVIGATION_DEPTH_THRESHOLD,
};
use crate::graph::builder::GraphTopology;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

pub struct DirectoryMetricsAnalyzer;

impl DirectoryMetricsAnalyzer {
    pub fn analyze(root: &Path, topology: Option<&GraphTopology>) -> DirectoryStructureMetrics {
        let mut depths: Vec<u32> = Vec::new();
        let mut dir_counts: HashMap<String, usize> = HashMap::new();
        let mut dir_depths: HashMap<String, u32> = HashMap::new();
        let mut dir_tags: HashMap<String, HashSet<String>> = HashMap::new();

        // 1. Walk directory tree
        for entry in WalkDir::new(root)
            .into_iter()
            .filter_entry(|e| {
                if e.depth() == 0 {
                    return true;
                }
                let name = e.file_name().to_string_lossy();
                !name.starts_with('.') && name != "target" && name != "node_modules"
            })
            .flatten()
        {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "md" {
                        if let Ok(rel) = entry.path().strip_prefix(root) {
                            let parent = rel.parent().unwrap_or_else(|| Path::new(""));
                            let dir_str = parent.to_string_lossy().replace('\\', "/");
                            let depth = if dir_str.is_empty() {
                                0
                            } else {
                                parent.components().count() as u32
                            };

                            depths.push(depth);
                            *dir_counts.entry(dir_str.clone()).or_insert(0) += 1;
                            dir_depths.entry(dir_str).or_insert(depth);
                        }
                    }
                }
            }
        }

        let total_docs = depths.len();
        let max_directory_depth = depths.iter().copied().max().unwrap_or(0);
        let average_directory_depth = if depths.is_empty() {
            0.0
        } else {
            depths.iter().sum::<u32>() as f32 / total_docs as f32
        };

        // 2. Concentration entropy (Shannon entropy over folder distribution)
        let num_dirs = dir_counts.len();
        let concentration_entropy = if num_dirs <= 1 || total_docs == 0 {
            0.0
        } else {
            let mut h = 0.0f32;
            for &count in dir_counts.values() {
                let p = count as f32 / total_docs as f32;
                if p > 0.0 {
                    h -= p * p.ln();
                }
            }
            let max_h = (num_dirs as f32).ln();
            if max_h > 0.0 {
                (h / max_h).clamp(0.0, 1.0)
            } else {
                0.0
            }
        };

        // 3. Topology-based metrics (cross-folder coupling & domain mixing)
        let mut cross_folder_coupling = 0.0f32;
        let mut domain_mixing_score = 0.0f32;
        let mut signals = Vec::new();

        if let Some(topo) = topology {
            // Collect tags per directory
            for node in topo.nodes.values() {
                let p = Path::new(&node.file_path);
                let dir_str = p
                    .parent()
                    .unwrap_or_else(|| Path::new(""))
                    .to_string_lossy()
                    .replace('\\', "/");
                let tag_set = dir_tags.entry(dir_str).or_default();
                for tag in &node.tags {
                    tag_set.insert(tag.clone());
                }
            }

            // Cross-folder coupling
            let mut total_edges = 0usize;
            let mut cross_edges = 0usize;
            let mut dir_outgoing: HashMap<String, usize> = HashMap::new();
            let mut dir_cross_outgoing: HashMap<String, usize> = HashMap::new();

            for edge in &topo.edges {
                total_edges += 1;
                let s_dir = Path::new(&edge.source)
                    .parent()
                    .unwrap_or_else(|| Path::new(""))
                    .to_string_lossy()
                    .replace('\\', "/");
                let t_dir = Path::new(&edge.target)
                    .parent()
                    .unwrap_or_else(|| Path::new(""))
                    .to_string_lossy()
                    .replace('\\', "/");

                *dir_outgoing.entry(s_dir.clone()).or_insert(0) += 1;
                if s_dir != t_dir {
                    cross_edges += 1;
                    *dir_cross_outgoing.entry(s_dir).or_insert(0) += 1;
                }
            }

            if total_edges > 0 {
                cross_folder_coupling = cross_edges as f32 / total_edges as f32;
            }

            // Check high navigational coupling when repo-level cross-folder coupling exceeds threshold
            if cross_folder_coupling > CROSS_FOLDER_COUPLING_THRESHOLD {
                if let Some((dir, &total_out)) = dir_outgoing
                    .iter()
                    .filter(|(_, &out)| out >= 3)
                    .max_by_key(|(d, &out)| {
                        let cross = dir_cross_outgoing.get(*d).copied().unwrap_or(0);
                        (cross, out, d.as_str())
                    })
                {
                    let cross_out = dir_cross_outgoing.get(dir).copied().unwrap_or(0);
                    let ratio = cross_out as f32 / total_out as f32;
                    if ratio > CROSS_FOLDER_COUPLING_THRESHOLD {
                        signals.push(OrganizationalSignal {
                            target_directory: dir.clone(),
                            signal_type: OrganizationalSignalType::HighNavigationalCoupling,
                            message: format!(
                                "High cross-folder navigational coupling in '{}' ({:.1}% outgoing references cross folder boundaries)",
                                dir,
                                ratio * 100.0
                            ),
                            cost_evidence: format!(
                                "Cross-folder reference ratio {:.2} > threshold {:.2}",
                                ratio, CROSS_FOLDER_COUPLING_THRESHOLD
                            ),
                        });
                    }
                }
            }

            // Check domain mixing
            let mut total_dir_diversity = 0.0f32;
            let mut evaluated_dirs = 0usize;
            for (dir, tags) in &dir_tags {
                let doc_count = dir_counts.get(dir).copied().unwrap_or(1);
                if doc_count > 1 && !tags.is_empty() {
                    let div = tags.len() as f32 / doc_count as f32;
                    total_dir_diversity += div.min(1.0);
                    evaluated_dirs += 1;
                }
            }
            if evaluated_dirs > 0 {
                domain_mixing_score = (total_dir_diversity / evaluated_dirs as f32).clamp(0.0, 1.0);
            }
        }

        // 4. Check excessive depth (report deepest directory when max depth exceeds threshold)
        if max_directory_depth > MAX_NAVIGATION_DEPTH_THRESHOLD {
            if let Some((dir, &depth)) = dir_depths
                .iter()
                .filter(|(_, &d)| d > MAX_NAVIGATION_DEPTH_THRESHOLD)
                .max_by_key(|(dir, &d)| (d, dir.as_str()))
            {
                signals.push(OrganizationalSignal {
                    target_directory: dir.clone(),
                    signal_type: OrganizationalSignalType::ExcessiveDepth,
                    message: format!(
                        "Excessive directory nesting depth ({}) exceeds threshold ({})",
                        depth, MAX_NAVIGATION_DEPTH_THRESHOLD
                    ),
                    cost_evidence: format!(
                        "Directory depth {} > threshold {}",
                        depth, MAX_NAVIGATION_DEPTH_THRESHOLD
                    ),
                });
            }
        }

        // 5. Check document overconcentration
        if concentration_entropy < CONCENTRATION_ENTROPY_THRESHOLD && total_docs >= 10 {
            if let Some((max_dir, &max_count)) = dir_counts.iter().max_by_key(|(_, &c)| c) {
                let pct = (max_count as f32 / total_docs as f32) * 100.0;
                signals.push(OrganizationalSignal {
                    target_directory: max_dir.clone(),
                    signal_type: OrganizationalSignalType::DocumentOverconcentration,
                    message: format!(
                        "Document overconcentration detected in '{}': {:.1}% of repository documents are in a single folder",
                        max_dir, pct
                    ),
                    cost_evidence: format!(
                        "Shannon entropy {:.2} < threshold {:.2}",
                        concentration_entropy, CONCENTRATION_ENTROPY_THRESHOLD
                    ),
                });
            }
        }

        // 6. Check domain mixing / scattered related knowledge
        if domain_mixing_score > DOMAIN_MIXING_THRESHOLD {
            signals.push(OrganizationalSignal {
                target_directory: "".to_string(),
                signal_type: OrganizationalSignalType::ScatteredRelatedKnowledge,
                message: "High domain mixing detected across repository directories".to_string(),
                cost_evidence: format!(
                    "Domain mixing score {:.2} > threshold {:.2}",
                    domain_mixing_score, DOMAIN_MIXING_THRESHOLD
                ),
            });
        }

        DirectoryStructureMetrics {
            max_directory_depth,
            average_directory_depth: (average_directory_depth * 100.0).round() / 100.0,
            concentration_entropy: (concentration_entropy * 100.0).round() / 100.0,
            domain_mixing_score: (domain_mixing_score * 100.0).round() / 100.0,
            cross_folder_coupling: (cross_folder_coupling * 100.0).round() / 100.0,
            organizational_signals: signals,
        }
    }
}
