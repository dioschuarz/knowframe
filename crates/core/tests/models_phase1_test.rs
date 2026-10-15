use llm_wiki_core::model::*;
use std::collections::HashSet;

#[test]
fn test_debt_baseline_serialization_roundtrip() {
    let mut baseline = KnowledgeDebtBaseline::default();
    baseline.baseline_commit = "abc1234".to_string();
    baseline.broken_links.push(BrokenLinkSignature {
        source_path: "notes/doc_a.md".to_string(),
        target_ref: "doc_b".to_string(),
        line_number: 14,
    });
    baseline.orphan_nodes.insert("notes/orphan.md".to_string());
    baseline
        .collisions
        .push(CollisionSignature::new("notes/beta.md", "notes/alpha.md"));
    baseline.technical_drifts.push(DriftSignature {
        doc_path: "notes/arch.md".to_string(),
        referenced_code_path: "src/auth.ts".to_string(),
        symbol: "AuthService".to_string(),
    });
    baseline.total_debt = 4;

    // Verify alphabetical ordering of CollisionSignature
    assert_eq!(baseline.collisions[0].doc_a, "notes/alpha.md");
    assert_eq!(baseline.collisions[0].doc_b, "notes/beta.md");

    let json = serde_json::to_string_pretty(&baseline).expect("Serialize baseline");
    let deserialized: KnowledgeDebtBaseline =
        serde_json::from_str(&json).expect("Deserialize baseline");
    assert_eq!(baseline, deserialized);
}

#[test]
fn test_candidate_report_serialization() {
    let report = CandidateValidationReport {
        base_commit: "commit_xyz".to_string(),
        decision: AdmissionDecision::Block,
        exit_code: 1,
        summary: DebtAccountingSummary {
            total_debt: 10,
            legacy_debt: 8,
            new_debt: 2,
            resolved_debt: 0,
            regressions: 2,
        },
        regressions: vec![RegressionItem {
            id: "reg-1".to_string(),
            category: RegressionCategory::BrokenReference,
            severity: RegressionSeverity::Blocker,
            file_path: "notes/new.md".to_string(),
            line_number: Some(25),
            message: "Target doc_missing.md does not exist".to_string(),
            remediation_hint: "Create doc_missing.md or remove link".to_string(),
        }],
        resolved_findings: vec![ResolvedItem {
            id: "res-1".to_string(),
            category: "broken_link".to_string(),
            file_path: "notes/fixed.md".to_string(),
            description: "Resolved reference to target.md".to_string(),
        }],
        warnings: vec![WarningItem {
            message: "Potential fragmentation detected".to_string(),
            file_path: Some("notes/fragment.md".to_string()),
        }],
    };

    let json = serde_json::to_string(&report).expect("Serialize report");
    let deserialized: CandidateValidationReport =
        serde_json::from_str(&json).expect("Deserialize report");
    assert_eq!(report, deserialized);
    assert_eq!(deserialized.decision, AdmissionDecision::Block);
    assert_eq!(deserialized.exit_code, 1);
}

#[test]
fn test_fingerprint_and_collision_structures() {
    let mut fp = DocumentFingerprint::new("notes/sample.md");
    fp.title_tokens.insert("sample".to_string());
    fp.heading_tokens.insert("architecture".to_string());
    fp.tags.insert("arch".to_string());
    fp.ngrams.insert("sam".to_string());
    fp.term_frequencies.insert("system".to_string(), 1.5);

    let collision = SemanticCollision {
        source_id: "notes/a.md".to_string(),
        target_id: "notes/b.md".to_string(),
        composite_similarity: 0.89,
        title_similarity: 0.90,
        heading_similarity: 0.85,
        bm25_similarity: 0.88,
        ngram_similarity: 0.92,
        tag_similarity: 0.80,
        classification: CollisionClassification::PotentialDuplicate,
        shared_key_terms: vec!["architecture".to_string(), "system".to_string()],
        canonical_candidate: Some("notes/b.md".to_string()),
        canonical_confidence: 0.95,
        canonical_evidence: vec![
            "Earlier commit".to_string(),
            "Higher betweenness".to_string(),
        ],
    };

    let json = serde_json::to_string(&collision).expect("Serialize collision");
    assert!(json.contains("potential_duplicate"));
}

#[test]
fn test_reverse_impact_and_drift_structures() {
    let mut rev = ReverseImpactIndex::default();
    let mut docs = HashSet::new();
    docs.insert("notes/auth_flow.md".to_string());
    rev.code_to_docs
        .insert("src/auth/service.ts".to_string(), docs.clone());
    rev.symbol_to_docs
        .insert("AuthService.login".to_string(), docs);

    let drift = TechnicalDriftFinding {
        doc_id: "notes/auth_flow.md".to_string(),
        code_target: "src/auth/service.ts".to_string(),
        symbol_name: Some("AuthService.login".to_string()),
        citation_line: 42,
        drift_type: DriftType::DeletedSymbol,
        confidence: DriftConfidence::High,
        explanation: "Symbol deleted in staged diff".to_string(),
    };

    let json = serde_json::to_string(&drift).expect("Serialize drift");
    assert!(json.contains("deleted_symbol"));
    assert!(json.contains("high"));
}

#[test]
fn test_context_trace_and_quality_score_structures() {
    let trace = ContextTraceResult {
        query: "authentication flow".to_string(),
        entrypoint: "AGENTS.md".to_string(),
        target_node: "notes/auth.md".to_string(),
        path: vec![
            "AGENTS.md".to_string(),
            "notes/security.md".to_string(),
            "notes/auth.md".to_string(),
        ],
        depth: 2,
        path_quality_score: 92.5,
        cumulative_bpe_tokens: 3450,
        relevant_tokens: 1200,
        weak_intermediate_nodes: vec![],
        latency_ms: 12,
    };

    let quality = KnowledgeQualityScore {
        overall_score: 84.5,
        integrity_score: 90.0,
        structure_score: 85.0,
        consistency_score: 80.0,
        discoverability_score: 85.0,
        freshness_score: 83.0,
        legacy_debt_count: 12,
        new_regressions_count: 0,
        resolved_debt_count: 3,
        central_hub_id: Some("00_meta/wiki_index.md".to_string()),
        top_degradation_factors: vec![QualityDegradationFactor {
            category: "legacy_broken_link".to_string(),
            affected_node: "notes/old.md".to_string(),
            betweenness_weight: 1.25,
            deduction_points: 2.5,
            description: "Broken wikilink to missing target".to_string(),
        }],
        ..Default::default()
    };

    assert_eq!(trace.depth, 2);
    assert!(quality.overall_score > 80.0);
}
