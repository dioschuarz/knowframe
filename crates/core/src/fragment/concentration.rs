//! Reference Concentration calculation for document fragments.
//!
//! Formula:
//! C(f) = inbound(f) / sum_{f' in D} inbound(f')
//!
//! If total inbound references across all fragments in document == 0, C(f) = 0.0.

use super::FragmentMetadata;

/// Computes reference concentration C(f) for a specific fragment relative to all fragments in the document.
/// Returns a value in 0.0 ..= 1.0.
pub fn calculate_reference_concentration(
    fragment: &FragmentMetadata,
    all_fragments: &[FragmentMetadata],
) -> f64 {
    let total_inbound: u32 = all_fragments.iter().map(|f| f.inbound).sum();
    if total_inbound == 0 {
        0.0
    } else {
        fragment.inbound as f64 / total_inbound as f64
    }
}

/// Returns true if any fragment on the document absorbs >= 70% of all inbound references
/// and total document tokens >= 50,000.
pub fn has_high_reference_concentration(
    doc_tokens: u32,
    fragments: &[FragmentMetadata],
) -> Option<(&FragmentMetadata, f64)> {
    if doc_tokens < 50_000 {
        return None;
    }
    let total_inbound: u32 = fragments.iter().map(|f| f.inbound).sum();
    if total_inbound == 0 {
        return None;
    }

    for frag in fragments {
        let conc = frag.inbound as f64 / total_inbound as f64;
        if conc >= 0.70 {
            return Some((frag, conc));
        }
    }
    None
}
