//! Unit tests for Concept Maps, Ontology Shapes, and DCAP Metadata KOS methodologies.
//! Validates FR-016, FR-017, and FR-019.

use llm_wiki_core::methodologies::metadata_profiles::{
    evaluate_dcap_metadata, evaluate_ontology_shapes, DcapShape, DocumentMetadataRecord,
};
use llm_wiki_core::methodologies::semantic_networks::{
    evaluate_concept_map, ConceptProposition,
};

#[test]
fn test_concept_map_proposition_density() {
    let propositions = vec![
        ConceptProposition {
            subject: "Compiler".to_string(),
            predicate: "generates".to_string(),
            object: "Bytecode".to_string(),
        },
        ConceptProposition {
            subject: "VM".to_string(),
            predicate: "executes".to_string(),
            object: "Bytecode".to_string(),
        },
    ];

    let total_words = 100;
    let eval = evaluate_concept_map(&propositions, total_words);
    assert_eq!(eval.proposition_count, 2);
    assert!((eval.proposition_density - 0.02).abs() < 1e-4);
}

#[test]
fn test_ontology_shacl_shapes_and_dcap_metadata() {
    let mut shape = DcapShape::new("adr_note");
    shape.add_mandatory_field("title");
    shape.add_mandatory_field("status");
    shape.add_mandatory_field("date");

    let doc_valid = DocumentMetadataRecord {
        document_id: "ADR-001.md".to_string(),
        fields: vec![
            ("title".to_string(), "Use Rust".to_string()),
            ("status".to_string(), "Accepted".to_string()),
            ("date".to_string(), "2026-10-06".to_string()),
        ],
    };

    let doc_invalid = DocumentMetadataRecord {
        document_id: "ADR-002.md".to_string(),
        fields: vec![
            ("title".to_string(), "Use WebAssembly".to_string()),
            // Missing status and date
        ],
    };

    let records = vec![doc_valid, doc_invalid];
    let dcap_eval = evaluate_dcap_metadata(&shape, &records);
    assert_eq!(dcap_eval.total_documents, 2);
    assert_eq!(dcap_eval.compliant_documents, 1);
    assert!((dcap_eval.compliance_ratio - 0.50).abs() < 1e-4);
    assert!((dcap_eval.completeness_ratio - (2.0 / 3.0)).abs() < 1e-4);

    let ontology_eval = evaluate_ontology_shapes(&shape, &records);
    assert_eq!(ontology_eval.violating_documents.len(), 1);
    assert_eq!(ontology_eval.violating_documents[0], "ADR-002.md");
}
