//! Unit test for Invariant XIII guard: protecting cohesive monoliths from size penalties.
//! Validates FR-009, SC-006, and Constitution Invariant XIII.

use llm_wiki_core::health::monolith_guard::{check_monolith_protection, MonolithMetrics};

#[test]
fn test_healthy_monolith_protection_invariant_xiii() {
    let cohesive_monolith = MonolithMetrics {
        tokens: 52_000,
        cohesion: 0.85,
        coupling: 0.20,
        semantic_diversity: 0.30,
        fragment_count: 18,
        broken_anchors_count: 0,
    };

    let protection = check_monolith_protection(&cohesive_monolith);
    assert!(
        protection.is_protected,
        "Cohesive 50k monolith must be protected from splitting per Invariant XIII"
    );
    assert_eq!(protection.reason, "healthy_monolith");
}

#[test]
fn test_unhealthy_monolith_god_document_not_protected() {
    let god_document = MonolithMetrics {
        tokens: 52_000,
        cohesion: 0.20,
        coupling: 0.85,
        semantic_diversity: 0.88,
        fragment_count: 2,
        broken_anchors_count: 5,
    };

    let protection = check_monolith_protection(&god_document);
    assert!(
        !protection.is_protected,
        "High coupling, low cohesion god document must NOT be protected"
    );
}
