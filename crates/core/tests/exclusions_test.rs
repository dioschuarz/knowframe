use llm_wiki_core::graph::exclusions::ExclusionEngine;

#[test]
fn test_exclusion_engine_two_tier_filtering() {
    let scan_ex = vec![
        "**/.git/**".to_string(),
        "**/target/**".to_string(),
        "**/node_modules/**".to_string(),
    ];
    let graph_ex = vec!["packages/**/README.md".to_string(), "vendor/**".to_string()];

    let engine = ExclusionEngine::new(&scan_ex, &graph_ex).expect("Engine should initialize");

    // Scan tier: physical parsing exclusion
    assert!(engine.is_scan_excluded("packages/ui/node_modules/pkg/index.js"));
    assert!(engine.is_scan_excluded("target/debug/build.rs"));
    assert!(!engine.is_scan_excluded("packages/ui/README.md"));

    // Graph tier: semantic rendering exclusion
    assert!(engine.is_graph_excluded("packages/core/README.md"));
    assert!(engine.is_graph_excluded("packages/nested/deep/README.md"));
    assert!(engine.is_graph_excluded("vendor/third_party/spec.md"));
    assert!(!engine.is_graph_excluded("packages/core/src/lib.md"));
    assert!(!engine.is_graph_excluded("docs/README.md"));
}

#[test]
fn test_exclusion_preview_simulation() {
    let paths = vec![
        "packages/app/README.md".to_string(),
        "packages/core/README.md".to_string(),
        "packages/api/README.md".to_string(),
        "packages/web/README.md".to_string(),
        "packages/cli/README.md".to_string(),
        "packages/extra/README.md".to_string(),
        "docs/overview.md".to_string(),
    ];

    let result = ExclusionEngine::preview("packages/**/README.md", &paths)
        .expect("Preview calculation should succeed");

    assert_eq!(result.matching_count, 6);
    assert_eq!(result.sample_paths.len(), 6);
    assert!(result
        .sample_paths
        .contains(&"packages/app/README.md".to_string()));
    assert!(!result
        .sample_paths
        .contains(&"docs/overview.md".to_string()));

    let non_matching = ExclusionEngine::preview("internal/**", &paths)
        .expect("Preview should succeed with 0 matches");
    assert_eq!(non_matching.matching_count, 0);
    assert!(non_matching.sample_paths.is_empty());
}

#[test]
fn test_noise_detection_clustering_algorithms() {
    // Generate 6 orphan README.md files to trigger stem clustering (threshold >= 5)
    let mut orphan_paths = vec![
        "packages/pkg1/README.md".to_string(),
        "packages/pkg2/README.md".to_string(),
        "packages/pkg3/README.md".to_string(),
        "packages/pkg4/README.md".to_string(),
        "packages/pkg5/README.md".to_string(),
        "packages/pkg6/README.md".to_string(),
    ];

    // And add 5 vendor files to trigger directory prefix clustering (threshold >= 5)
    for i in 1..=5 {
        orphan_paths.push(format!("vendor/lib{}/doc.md", i));
    }

    let suggestions = ExclusionEngine::detect_noise_suggestions(&orphan_paths);

    assert!(!suggestions.is_empty(), "Should generate noise suggestions");

    // Check stem suggestion
    let stem_suggestion = suggestions.iter().find(|s| s.pattern == "**/README.md");
    assert!(stem_suggestion.is_some(), "Should suggest **/README.md");
    let stem = stem_suggestion.unwrap();
    assert_eq!(stem.matching_count, 6);
    assert!(stem.recommended);

    // Check directory suggestion
    let dir_suggestion = suggestions.iter().find(|s| s.pattern == "vendor/**");
    assert!(dir_suggestion.is_some(), "Should suggest vendor/**");
    let dir = dir_suggestion.unwrap();
    assert_eq!(dir.matching_count, 5);
}
