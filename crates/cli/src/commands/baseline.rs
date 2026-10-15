//! Baseline CLI command managing .context-vault/baseline.json.
//! Implements FR-028, FR-039, and Invariant XI.

use anyhow::Result;
use llm_wiki_core::health::baseline::BaselineManager;
use std::path::Path;

pub fn execute_baseline(root: &Path) -> Result<()> {
    let baseline_path = root.join(".context-vault/baseline.json");
    let manager = if baseline_path.exists() {
        BaselineManager::load(&baseline_path)?
    } else {
        let m = BaselineManager::new(&baseline_path);
        m.save()?;
        m
    };

    println!(
        "{{\"baseline_file\":\"{}\",\"total_baselined_items\":{}}}",
        baseline_path.display(),
        manager.items().len()
    );
    Ok(())
}
