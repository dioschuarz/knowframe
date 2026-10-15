//! Ontology Engineering and DCAP Metadata Profile shape validators.
//! Implements FR-017 and FR-019.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcapShape {
    pub shape_name: String,
    pub mandatory_fields: Vec<String>,
}

impl DcapShape {
    pub fn new(shape_name: &str) -> Self {
        Self {
            shape_name: shape_name.to_string(),
            mandatory_fields: Vec::new(),
        }
    }

    pub fn add_mandatory_field(&mut self, field_name: &str) {
        self.mandatory_fields.push(field_name.to_string());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadataRecord {
    pub document_id: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcapEvaluation {
    pub total_documents: usize,
    pub compliant_documents: usize,
    pub compliance_ratio: f64,
    pub completeness_ratio: f64,
}

pub fn evaluate_dcap_metadata(
    shape: &DcapShape,
    records: &[DocumentMetadataRecord],
) -> DcapEvaluation {
    if records.is_empty() {
        return DcapEvaluation {
            total_documents: 0,
            compliant_documents: 0,
            compliance_ratio: 1.0,
            completeness_ratio: 1.0,
        };
    }

    let mut compliant = 0;
    let mut total_score = 0.0;
    let mandatory_count = shape.mandatory_fields.len();

    for doc in records {
        let present_fields: HashSet<_> = doc.fields.iter().map(|(k, _)| k.as_str()).collect();
        let present_mandatory = shape
            .mandatory_fields
            .iter()
            .filter(|f| present_fields.contains(f.as_str()))
            .count();

        if mandatory_count > 0 {
            let score = present_mandatory as f64 / mandatory_count as f64;
            total_score += score;
            if present_mandatory == mandatory_count {
                compliant += 1;
            }
        } else {
            total_score += 1.0;
            compliant += 1;
        }
    }

    let compliance_ratio = compliant as f64 / records.len() as f64;

    DcapEvaluation {
        total_documents: records.len(),
        compliant_documents: compliant,
        compliance_ratio,
        completeness_ratio: total_score / records.len() as f64,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyShapeEvaluation {
    pub total_documents: usize,
    pub conforming_documents: usize,
    pub conformance_ratio: f64,
    pub violating_documents: Vec<String>,
}

pub fn evaluate_ontology_shapes(
    shape: &DcapShape,
    records: &[DocumentMetadataRecord],
) -> OntologyShapeEvaluation {
    let mut violating = Vec::new();
    let mut conforming = 0;

    for doc in records {
        let present_fields: HashSet<_> = doc.fields.iter().map(|(k, _)| k.as_str()).collect();
        let is_valid = shape
            .mandatory_fields
            .iter()
            .all(|f| present_fields.contains(f.as_str()));

        if is_valid {
            conforming += 1;
        } else {
            violating.push(doc.document_id.clone());
        }
    }

    let total = records.len();
    let conformance_ratio = if total > 0 {
        conforming as f64 / total as f64
    } else {
        1.0
    };

    OntologyShapeEvaluation {
        total_documents: total,
        conforming_documents: conforming,
        conformance_ratio,
        violating_documents: violating,
    }
}
