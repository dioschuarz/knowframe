//! Unit test for cryptographic approval_token verification and hard constraint gate.
//! Validates FR-036, FR-037, SC-005, and Constitution Invariant XVI.

use llm_wiki_core::automation::plan_generator::{
    generate_approval_token, verify_approval_token, TransformationPlanGenerator,
};
use llm_wiki_core::model::{StrategyType, TransformationAction};

#[test]
fn test_approval_token_cryptographic_verification() {
    let actions = vec![TransformationAction::UpdateFrontmatter {
        file: "docs/guide.md".to_string(),
        key: "status".to_string(),
        value: serde_json::json!("published"),
    }];

    let plan_id = "plan-001";
    let token = generate_approval_token(plan_id, &actions);
    assert!(!token.is_empty(), "Approval token must be generated");

    // Token must verify successfully with matching plan_id and actions
    assert!(
        verify_approval_token(plan_id, &actions, &token),
        "Cryptographically generated token must verify cleanly"
    );

    // Mismatched token must be rejected
    assert!(
        !verify_approval_token(plan_id, &actions, "invalid_token_xyz"),
        "Invalid token must be rejected"
    );

    // Tampered actions must be rejected
    let tampered_actions = vec![TransformationAction::MoveFile {
        source: "a.md".to_string(),
        destination: "b.md".to_string(),
    }];
    assert!(
        !verify_approval_token(plan_id, &tampered_actions, &token),
        "Tampered actions must fail verification"
    );
}

#[test]
fn test_plan_generator_requires_approval_and_enforces_gate() {
    let generator = TransformationPlanGenerator::new();
    let actions = vec![TransformationAction::MoveFile {
        source: "old.md".to_string(),
        destination: "new.md".to_string(),
    }];

    let plan = generator.create_plan(
        "strat-1",
        StrategyType::SplitDecompose,
        vec!["old.md".to_string()],
        actions.clone(),
        5.0,
    );

    assert!(!plan.approval_token.is_empty());

    // Validation passes with correct token
    assert!(generator.validate_approval(&plan, &plan.approval_token).is_ok());

    // Validation fails with wrong token
    assert!(generator.validate_approval(&plan, "bad_token").is_err());
}
