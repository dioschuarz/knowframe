//! Deterministic signature hashing algorithms for knowledge debt findings.

use sha2::{Digest, Sha256};
use super::{BrokenLinkSignature, CollisionSignature, DriftSignature};

/// Produces a deterministic SHA-256 digest for a broken link signature.
pub fn hash_broken_link(sig: &BrokenLinkSignature) -> String {
    let mut hasher = Sha256::new();
    hasher.update(sig.source_path.as_bytes());
    hasher.update(b"::");
    hasher.update(sig.target_ref.as_bytes());
    hasher.update(b"::");
    hasher.update(sig.line_number.to_string().as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Produces a deterministic SHA-256 digest for an orphan note.
pub fn hash_orphan(path: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Produces a deterministic SHA-256 digest for an unordered collision pair.
pub fn hash_collision(sig: &CollisionSignature) -> String {
    let mut hasher = Sha256::new();
    let (a, b) = if sig.doc_a <= sig.doc_b {
        (&sig.doc_a, &sig.doc_b)
    } else {
        (&sig.doc_b, &sig.doc_a)
    };
    hasher.update(a.as_bytes());
    hasher.update(b"::");
    hasher.update(b.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Produces a deterministic SHA-256 digest for a technical drift finding.
pub fn hash_drift(sig: &DriftSignature) -> String {
    let mut hasher = Sha256::new();
    hasher.update(sig.doc_path.as_bytes());
    hasher.update(b"::");
    hasher.update(sig.referenced_code_path.as_bytes());
    hasher.update(b"::");
    hasher.update(sig.symbol.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_hashing_consistency() {
        let bl = BrokenLinkSignature {
            source_path: "notes/a.md".to_string(),
            target_ref: "target".to_string(),
            line_number: 10,
        };
        assert_eq!(hash_broken_link(&bl), hash_broken_link(&bl));

        let col1 = CollisionSignature::new("notes/b.md", "notes/a.md");
        let col2 = CollisionSignature::new("notes/a.md", "notes/b.md");
        assert_eq!(hash_collision(&col1), hash_collision(&col2));
    }
}
