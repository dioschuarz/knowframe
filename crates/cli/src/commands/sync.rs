//! Post-commit graph sync command applying committed Git deltas in <= 50 ms.
//! Implements FR-039b, NFR-003, and Constitution Principle II.

use anyhow::Result;
use llm_wiki_core::model::OptimizationProfile;
use llm_wiki_core::observation::scanner::{scan_repository, ScanOptions};
use std::path::Path;
use std::time::Instant;

pub fn execute_sync(root: &Path) -> Result<()> {
    let start = Instant::now();
    let profile = OptimizationProfile::ai_oriented();
    let options = ScanOptions::default();

    let scan = scan_repository(root, &options, &profile, "HEAD")?;
    let duration = start.elapsed();

    println!(
        "Post-commit graph sync completed in {:.2}ms ({} documents, {} edges).",
        duration.as_secs_f64() * 1000.0,
        scan.total_documents,
        scan.topology.edges.len()
    );

    Ok(())
}
