//! Unit test for deterministic AST and filesystem mutators generating unified diffs.
//! Validates FR-038.

use llm_wiki_core::automation::mutators::{
    apply_transformation_action, generate_unified_diff,
};
use llm_wiki_core::model::TransformationAction;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_update_frontmatter_mutator() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("doc.md");
    fs::write(&file_path, "---\ntitle: Old Title\n---\n# Content").unwrap();

    let action = TransformationAction::UpdateFrontmatter {
        file: file_path.to_str().unwrap().to_string(),
        key: "category".to_string(),
        value: serde_json::json!("architecture"),
    };

    apply_transformation_action(&action).expect("Should apply frontmatter mutation");
    let updated = fs::read_to_string(&file_path).unwrap();
    assert!(updated.contains("category: architecture"));
    assert!(updated.contains("# Content"));
}

#[test]
fn test_retarget_link_mutator() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("source.md");
    fs::write(&file_path, "See [[old_target]] for details.").unwrap();

    let action = TransformationAction::RetargetLink {
        file: file_path.to_str().unwrap().to_string(),
        line: 1,
        old_target: "old_target".to_string(),
        new_target: "new_canonical_target".to_string(),
    };

    apply_transformation_action(&action).expect("Should retarget link");
    let updated = fs::read_to_string(&file_path).unwrap();
    assert!(updated.contains("[[new_canonical_target]]"));
    assert!(!updated.contains("[[old_target]]"));
}

#[test]
fn test_generate_unified_diff() {
    let old = "line 1\nline 2\nline 3\n";
    let new = "line 1\nline 2 modified\nline 3\n";
    let diff = generate_unified_diff("test.md", old, new);
    assert!(diff.contains("--- a/test.md"));
    assert!(diff.contains("+++ b/test.md"));
    assert!(diff.contains("-line 2"));
    assert!(diff.contains("+line 2 modified"));
}
