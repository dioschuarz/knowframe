//! Brownfield baseline isolation (.context-vault/baseline.json).
//! Implements FR-028, FR-039, SC-003, and Constitution Invariant XI.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DebtItem {
    pub rule_key: String,
    pub target_file: String,
    pub line_number: u32,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaselineData {
    pub version: String,
    pub items: Vec<DebtItem>,
}

#[derive(Debug, Clone)]
pub struct BaselineManager {
    path: PathBuf,
    items: Vec<DebtItem>,
    fingerprints: HashSet<String>,
}

impl BaselineManager {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            items: Vec::new(),
            fingerprints: HashSet::new(),
        }
    }

    pub fn add_item(&mut self, item: DebtItem) {
        self.fingerprints.insert(item.fingerprint.clone());
        self.items.push(item);
    }

    pub fn items(&self) -> &[DebtItem] {
        &self.items
    }

    pub fn is_baselined(&self, fingerprint: &str) -> bool {
        self.fingerprints.contains(fingerprint)
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = BaselineData {
            version: "1.0.0".to_string(),
            items: self.items.clone(),
        };
        let json = serde_json::to_string_pretty(&data)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let data: BaselineData = serde_json::from_str(&content)?;
        let mut fingerprints = HashSet::new();
        for item in &data.items {
            fingerprints.insert(item.fingerprint.clone());
        }
        Ok(Self {
            path: path.to_path_buf(),
            items: data.items,
            fingerprints,
        })
    }
}
