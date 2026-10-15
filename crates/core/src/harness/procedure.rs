//! Procedure candidate extractor and candidacy scorer.
//!
//! Enforces FR-021, FR-022, and FR-023:
//! - Distinguishes one-off task steps from reusable multi-step capabilities.
//! - Extracts ordered procedural sequences across markdown documents.
//! - Computes recurrence scores (>= 0.70 qualifies as a ProcedureCandidate for promotion to Skill).

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use super::ProcedureCandidate;

pub struct ProcedureExtractor;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct StepSequence {
    signatures: Vec<String>,
}

impl ProcedureExtractor {
    /// Extracts procedure candidates from an in-memory map of document paths to content.
    pub fn extract_candidates(documents: &HashMap<String, String>) -> Vec<ProcedureCandidate> {
        let mut sequence_occurrences: HashMap<StepSequence, HashSet<String>> = HashMap::new();
        let mut sequence_scripts: HashMap<StepSequence, HashSet<String>> = HashMap::new();

        for (doc_path, content) in documents {
            let sequences = Self::find_step_sequences(content);
            for (seq, scripts) in sequences {
                sequence_occurrences
                    .entry(seq.clone())
                    .or_default()
                    .insert(doc_path.clone());

                let script_set = sequence_scripts.entry(seq).or_default();
                for s in scripts {
                    script_set.insert(s);
                }
            }
        }

        let mut candidates = Vec::new();

        for (seq, doc_set) in sequence_occurrences {
            let occurrence_count = doc_set.len();
            let recurrence_score = Self::calculate_recurrence_score(occurrence_count);

            // Invariant & FR-023: Only procedures with recurrence_score >= 0.70 qualify
            if recurrence_score >= 0.70 {
                let mut source_documents: Vec<String> = doc_set.into_iter().collect();
                source_documents.sort();

                let mut associated_scripts: Vec<String> = sequence_scripts
                    .remove(&seq)
                    .unwrap_or_default()
                    .into_iter()
                    .collect();
                associated_scripts.sort();

                let candidate_id = format!(
                    "proc_{:x}",
                    seahash_u64(&seq.signatures.join("||")) & 0xffffffffffff
                );
                let recommended_skill_name = Self::generate_recommended_skill_name(&seq.signatures);

                candidates.push(ProcedureCandidate {
                    candidate_id,
                    step_signatures: seq.signatures,
                    occurrence_count,
                    source_documents,
                    associated_scripts,
                    recurrence_score,
                    recommended_skill_name,
                });
            }
        }

        candidates.sort_by(|a, b| {
            b.occurrence_count
                .cmp(&a.occurrence_count)
                .then_with(|| a.candidate_id.cmp(&b.candidate_id))
        });
        candidates
    }

    /// Extracts procedure candidates from a vault directory on disk.
    pub fn extract_candidates_from_vault(
        root_path: &Path,
        vault_files: &[String],
    ) -> Vec<ProcedureCandidate> {
        let mut documents = HashMap::new();
        for file in vault_files {
            let full_p = root_path.join(file);
            if let Ok(content) = fs::read_to_string(&full_p) {
                documents.insert(file.clone(), content);
            }
        }
        Self::extract_candidates(&documents)
    }

    fn calculate_recurrence_score(distinct_doc_count: usize) -> f32 {
        match distinct_doc_count {
            0 => 0.0,
            1 => 0.35,
            2 => 0.70,
            3 => 0.85,
            _ => 1.0,
        }
    }

    fn find_step_sequences(content: &str) -> Vec<(StepSequence, Vec<String>)> {
        let mut sequences = Vec::new();
        let mut current_steps = Vec::new();
        let mut current_scripts = Vec::new();
        let mut expected_num = 1;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if let Some((num, rest)) = Self::parse_numbered_step(trimmed) {
                if num == expected_num {
                    let sig = Self::normalize_step_text(rest);
                    if !sig.is_empty() {
                        current_steps.push(sig);
                        Self::extract_scripts(rest, &mut current_scripts);
                        expected_num += 1;
                    }
                } else if num == 1 {
                    // Start of a new sequence
                    if current_steps.len() >= 3 {
                        sequences.push((
                            StepSequence {
                                signatures: current_steps,
                            },
                            current_scripts,
                        ));
                    }
                    current_steps = Vec::new();
                    current_scripts = Vec::new();
                    let sig = Self::normalize_step_text(rest);
                    if !sig.is_empty() {
                        current_steps.push(sig);
                        Self::extract_scripts(rest, &mut current_scripts);
                        expected_num = 2;
                    } else {
                        expected_num = 1;
                    }
                }
            } else if !trimmed.starts_with('-') && !trimmed.starts_with('*') {
                // Non-list line breaks numbered sequence
                if current_steps.len() >= 3 {
                    sequences.push((
                        StepSequence {
                            signatures: current_steps,
                        },
                        current_scripts,
                    ));
                }
                current_steps = Vec::new();
                current_scripts = Vec::new();
                expected_num = 1;
            }
        }

        if current_steps.len() >= 3 {
            sequences.push((
                StepSequence {
                    signatures: current_steps,
                },
                current_scripts,
            ));
        }

        sequences
    }

    fn parse_numbered_step(line: &str) -> Option<(usize, &str)> {
        let dot_pos = line.find('.')?;
        let num_str = line[..dot_pos].trim();
        let num: usize = num_str.parse().ok()?;
        let rest = line[dot_pos + 1..].trim();
        Some((num, rest))
    }

    fn normalize_step_text(text: &str) -> String {
        // Strip markdown backticks and formatting
        let clean = text.replace(['`', '*', '_'], "");
        clean
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn extract_scripts(text: &str, scripts: &mut Vec<String>) {
        // Scan for backticked paths e.g. `scripts/db_backup.sh`
        let mut rest = text;
        while let Some(start) = rest.find('`') {
            if let Some(end) = rest[start + 1..].find('`') {
                let token = &rest[start + 1..start + 1 + end];
                if token.contains(".sh")
                    || token.contains(".py")
                    || token.contains(".js")
                    || token.contains(".ts")
                    || token.starts_with("scripts/")
                {
                    scripts.push(token.to_string());
                }
                rest = &rest[start + 1 + end + 1..];
            } else {
                break;
            }
        }
    }

    fn generate_recommended_skill_name(signatures: &[String]) -> String {
        if let Some(first) = signatures.first() {
            let words: Vec<&str> = first.split_whitespace().take(4).collect();
            let joined = words.join("-");
            let filtered: String = joined
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
                .collect();
            if !filtered.is_empty() {
                return filtered;
            }
        }
        "custom-procedure".to_string()
    }
}

fn seahash_u64(s: &str) -> u64 {
    let mut h: u64 = 0x16f11ee4b87a6f01;
    for b in s.bytes() {
        h = h.wrapping_mul(0x100000001b3) ^ (b as u64);
    }
    h
}
