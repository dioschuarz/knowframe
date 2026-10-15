//! BLAKE3 cryptographic digest engine deriving canonical analysis_id.
//! FR-002: analysis_id = BLAKE3(GitCommitHash || CanonicalConfigBytes || CanonicalProfileBytes || MethodologyVersion)

use blake3::Hasher;

/// Current methodology version for deterministic knowledge engine.
pub const METHODOLOGY_VERSION: &str = "1.0.0";

/// Computes a canonical BLAKE3 cryptographic digest identifying an analysis run.
pub fn compute_analysis_id(
    git_commit_hash: &str,
    canonical_config_bytes: &[u8],
    canonical_profile_bytes: &[u8],
    methodology_version: &str,
) -> String {
    let mut hasher = Hasher::new();
    hasher.update(git_commit_hash.as_bytes());
    hasher.update(canonical_config_bytes);
    hasher.update(canonical_profile_bytes);
    hasher.update(methodology_version.as_bytes());
    hasher.finalize().to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_analysis_id_reproducibility() {
        let commit = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let config = b"mode: strict\nscan: true";
        let profile = b"orientation: ai_oriented\nversion: 1.0.0";
        let version = METHODOLOGY_VERSION;

        let id1 = compute_analysis_id(commit, config, profile, version);
        let id2 = compute_analysis_id(commit, config, profile, version);

        assert_eq!(id1, id2);
        assert_eq!(id1.len(), 64);
    }

    #[test]
    fn test_compute_analysis_id_sensitivity() {
        let commit = "commit1";
        let config = b"config1";
        let profile = b"profile1";
        let version = "1.0.0";

        let id_base = compute_analysis_id(commit, config, profile, version);
        let id_diff_commit = compute_analysis_id("commit2", config, profile, version);
        let id_diff_config = compute_analysis_id(commit, b"config2", profile, version);
        let id_diff_profile = compute_analysis_id(commit, config, b"profile2", version);
        let id_diff_version = compute_analysis_id(commit, config, profile, "1.0.1");

        assert_ne!(id_base, id_diff_commit);
        assert_ne!(id_base, id_diff_config);
        assert_ne!(id_base, id_diff_profile);
        assert_ne!(id_base, id_diff_version);
    }
}
