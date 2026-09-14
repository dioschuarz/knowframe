use llm_wiki_core::fragment::extractor::{slugify_heading, AnchorDisambiguator};

#[test]
fn test_slugify_heading_basic() {
    assert_eq!(slugify_heading("Introduction"), "introduction");
    assert_eq!(slugify_heading("System Architecture"), "system-architecture");
    assert_eq!(
        slugify_heading("2.1 Authentication & Tokens (OAuth2)"),
        "21-authentication-tokens-oauth2"
    );
}

#[test]
fn test_slugify_heading_punctuation_and_whitespace() {
    assert_eq!(slugify_heading("  Getting Started -- Quick!  "), "getting-started-quick");
    assert_eq!(slugify_heading("API: v2 / Endpoints"), "api-v2-endpoints");
    assert_eq!(slugify_heading("`code_snippet()` in heading"), "code-snippet-in-heading");
    assert_eq!(slugify_heading("???"), "section");
}

#[test]
fn test_anchor_disambiguation() {
    let mut disambiguator = AnchorDisambiguator::new();
    assert_eq!(disambiguator.disambiguate("configuration"), "configuration");
    assert_eq!(disambiguator.disambiguate("configuration"), "configuration-1");
    assert_eq!(disambiguator.disambiguate("configuration"), "configuration-2");
    assert_eq!(disambiguator.disambiguate("architecture"), "architecture");
    assert_eq!(disambiguator.disambiguate("architecture"), "architecture-1");
}
