use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_wiki_core::harness::directory_metrics::DirectoryMetricsAnalyzer;
use llm_wiki_core::harness::discovery::HarnessDetector;
use llm_wiki_core::harness::index_parser::IndexParser;
use llm_wiki_core::harness::matrix::AIOrientedCapabilityMatrixEvaluator;
use llm_wiki_core::harness::secondary_layer::SecondaryLayerDetector;
use llm_wiki_core::harness::skill::SkillAuditor;
use std::fs;
use tempfile::tempdir;

fn setup_large_workspace(num_files: usize) -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create AGENTS.md with Knowledge Index
    let mut agents_content = String::from("# AGENTS.md\nAI Operating System\n\n## Knowledge Index\n");
    for i in 0..10 {
        agents_content.push_str(&format!("- [Domain {}](docs/domain_{}/note_000.md)\n", i, i));
    }
    fs::write(root.join("AGENTS.md"), agents_content).unwrap();

    // Create 10 domain directories with num_files / 10 notes each
    let per_domain = num_files / 10;
    for d in 0..10 {
        let domain_dir = root.join("docs").join(format!("domain_{}", d));
        fs::create_dir_all(&domain_dir).unwrap();
        for i in 0..per_domain {
            let note_path = domain_dir.join(format!("note_{:03}.md", i));
            let note_content = format!(
                "# Note {}:{}\nConcept description and cross-reference to [AGENTS.md](../../AGENTS.md).\n",
                d, i
            );
            fs::write(note_path, note_content).unwrap();
        }
    }

    // Create a skill
    let skill_dir = root.join(".agents").join("skills").join("bench-skill");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: bench-skill\ndescription: Benchmark test skill\n---\n# Instructions\n",
    )
    .unwrap();

    dir
}

fn bench_full_governance_audit(c: &mut Criterion) {
    let temp_workspace = setup_large_workspace(1000);
    let root = temp_workspace.path();

    c.bench_function("governance_audit_1000_files", |b| {
        b.iter(|| {
            // 1. Detect harness entry points
            let harnesses = HarnessDetector::detect_all(black_box(root), &[]);
            assert!(!harnesses.is_empty());

            // 2. Parse embedded index routes
            let primary = &harnesses[0];
            let content = fs::read_to_string(root.join(&primary.file_path)).unwrap_or_default();
            let index_parser = IndexParser::new(root);
            let parsed_index = index_parser.parse_harness_index(&primary.file_path, black_box(&content));

            // 3. Detect secondary layers
            let detector = SecondaryLayerDetector::new(root);
            let secondary_layers = detector.detect_secondary_layers(black_box(&primary.file_path));

            // 4. Audit reusable skills
            let routes = parsed_index.as_ref().map(|i| i.routes.clone()).unwrap_or_default();
            let skills = SkillAuditor::audit_repository(black_box(root), &routes, &[]);

            // 5. Calculate physical directory metrics
            let dir_metrics = DirectoryMetricsAnalyzer::analyze(black_box(root), None);

            // 6. Evaluate 4-capability matrix
            let matrix = AIOrientedCapabilityMatrixEvaluator::evaluate(
                Some(primary),
                &skills,
                &[],
                &[],
            );

            black_box((primary, parsed_index, secondary_layers, skills, dir_metrics, matrix));
        });
    });
}

criterion_group!(benches, bench_full_governance_audit);
criterion_main!(benches);
