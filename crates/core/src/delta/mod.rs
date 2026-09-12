pub mod updater;

use crate::delta::updater::FileDeltaItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GitDelta {
    pub commit_sha: Option<String>,
    pub items: Vec<FileDeltaItem>,
}
