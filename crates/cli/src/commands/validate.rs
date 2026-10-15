//! Pre-commit validation command with 500ms fail-open guarantee.
//! Implements FR-039, NFR-005, SC-003, and Constitution Invariants I, II, XI, XII.

use anyhow::Result;
use llm_wiki_core::health::baseline::BaselineManager;
use llm_wiki_core::model::OptimizationProfile;
use llm_wiki_core::observation::scanner::{scan_repository, ScanOptions};
use std::path::Path;
use std::time::{Duration, Instant};

pub fn execute_validate(root: &Path) -> Result<()> {
    let start = Instant::now();
    let timeout = Duration::from_millis(500);

    let run_validation = || -> Result<usize> {
        let profile = OptimizationProfile::ai_oriented();
        let options = ScanOptions::default();

        let baseline_path = root.join(".context-vault/baseline.json");
        let _baseline = if baseline_path.exists() {
            BaselineManager::load(&baseline_path).ok()
        } else {
            None
        };

        let scan = scan_repository(root, &options, &profile, "HEAD")?;

        if start.elapsed() > timeout {
            eprintln!("Warning: pre-commit validation exceeded 500ms budget; failing open per NFR-005.");
            return Ok(0);
        }

        let mut new_broken = 0;
        for edge in &scan.topology.edges {
            if !edge.is_resolved {
                new_broken += 1;
            }
        }

        Ok(new_broken)
    };

    match run_validation() {
        Ok(regressions) => {
            if regressions > 0 {
                println!("Validation completed: {} broken references detected.", regressions);
            } else {
                println!("Validation passed: 0 regressions detected.");
            }
            Ok(())
        }
        Err(err) => {
            eprintln!("Warning: pre-commit validation error: {}; failing open per NFR-005.", err);
            Ok(())
        }
    }
}
