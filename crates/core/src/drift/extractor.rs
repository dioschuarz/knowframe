//! AST code citation extractor for Markdown documents.
//! Extracts referenced code file paths and symbols without requiring multi-language compiler frontends.

use std::path::Path;
use crate::model::{CodeDependency, DependencySource};

const CODE_EXTENSIONS: &[&str] = &[
    "rs", "ts", "tsx", "js", "jsx", "py", "go", "java", "cpp", "c", "h", "hpp", "rb", "php", "cs",
    "swift", "kt", "sh", "sql", "toml", "yaml", "yml", "json",
];

pub const PATH_TRAVERSAL_PROHIBITED: &str = "PATH_TRAVERSAL_PROHIBITED";

/// Extracted reference from a markdown document to a source code file or symbol.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodeCitation {
    pub code_path: String,
    pub symbol: Option<String>,
    pub line_number: u32,
}

/// Checks if a path string appears to be a source code or config file.
pub fn is_code_path(target: &str) -> bool {
    let clean = target
        .trim()
        .trim_start_matches("file://")
        .trim_start_matches("file:")
        .split('#')
        .next()
        .unwrap_or("")
        .split('?')
        .next()
        .unwrap_or("");

    if clean.starts_with("http://") || clean.starts_with("https://") {
        return false;
    }

    if let Some(ext) = Path::new(clean).extension().and_then(|e| e.to_str()) {
        CODE_EXTENSIONS.contains(&ext.to_lowercase().as_str())
    } else {
        false
    }
}

/// Normalizes and validates a referenced code path ensuring strict containment within workspace root.
/// Returns Ok(normalized_relative_posix_path), or Err(PATH_TRAVERSAL_PROHIBITED) if the path
/// attempts to escape outside the workspace root via `..`.
pub fn validate_and_normalize_code_path(
    referencing_doc: Option<&str>,
    target: &str,
) -> Result<String, &'static str> {
    let clean = target
        .trim()
        .trim_start_matches("file://")
        .trim_start_matches("file:")
        .split('#')
        .next()
        .unwrap_or("")
        .split('?')
        .next()
        .unwrap_or("")
        .replace('\\', "/");

    if clean.is_empty() {
        return Ok(String::new());
    }

    let combined = if (clean.starts_with("./") || clean.starts_with("../")) && referencing_doc.is_some() {
        let doc = referencing_doc.unwrap();
        if let Some(parent) = Path::new(doc).parent() {
            let parent_str = parent.to_string_lossy().replace('\\', "/");
            if !parent_str.is_empty() && parent_str != "." {
                format!("{}/{}", parent_str, clean)
            } else {
                clean
            }
        } else {
            clean
        }
    } else {
        clean
    };

    let mut parts = Vec::new();
    for seg in combined.split('/') {
        if seg.is_empty() || seg == "." {
            continue;
        }
        if seg == ".." {
            if parts.is_empty() {
                return Err(PATH_TRAVERSAL_PROHIBITED);
            }
            parts.pop();
        } else {
            parts.push(seg);
        }
    }

    Ok(parts.join("/"))
}

/// Validates containment of a standalone path without document context.
pub fn validate_path_containment(target: &str) -> Result<String, &'static str> {
    validate_and_normalize_code_path(None, target)
}

/// Normalizes a referenced code path relative to workspace root (fallback for legacy callers).
pub fn normalize_code_path(referencing_doc: &str, target: &str) -> String {
    validate_and_normalize_code_path(Some(referencing_doc), target)
        .unwrap_or_else(|_| target.replace('\\', "/"))
}

/// Extracts all code citations (files and backticked symbols) from Markdown content.
pub fn extract_code_citations(doc_id: &str, content: &str) -> Vec<CodeCitation> {
    let mut citations = Vec::new();

    for (line_idx, line) in content.lines().enumerate() {
        let line_num = (line_idx + 1) as u32;

        // 1. Scan standard markdown links: [text](target)
        let mut rest = line;
        while let Some(start_bracket) = rest.find('[') {
            if let Some(end_bracket) = rest[start_bracket..].find(']') {
                let text = &rest[start_bracket + 1..start_bracket + end_bracket];
                let after_bracket = &rest[start_bracket + end_bracket + 1..];
                if after_bracket.starts_with('(') {
                    if let Some(end_paren) = after_bracket.find(')') {
                        let target = &after_bracket[1..end_paren];
                        if is_code_path(target) {
                            let norm = normalize_code_path(doc_id, target);
                            citations.push(CodeCitation {
                                code_path: norm,
                                symbol: None,
                                line_number: line_num,
                            });
                        } else if is_code_path(text) {
                            let norm = normalize_code_path(doc_id, text);
                            citations.push(CodeCitation {
                                code_path: norm,
                                symbol: None,
                                line_number: line_num,
                            });
                        }
                        rest = &after_bracket[end_paren + 1..];
                        continue;
                    }
                }
            }
            rest = &rest[start_bracket + 1..];
        }

        // 2. Scan inline code spans: `...`
        let mut rest_code = line;
        while let Some(start_tick) = rest_code.find('`') {
            let after_start = &rest_code[start_tick + 1..];
            if let Some(end_tick) = after_start.find('`') {
                let span = &after_start[..end_tick].trim();
                if is_code_path(span) {
                    let norm = normalize_code_path(doc_id, span);
                    citations.push(CodeCitation {
                        code_path: norm,
                        symbol: None,
                        line_number: line_num,
                    });
                } else if span.contains("::") || span.contains('.') || span.ends_with("()") {
                    // Likely a code symbol identifier (e.g. AuthService.login, crate::service)
                    let clean_sym = span.trim_end_matches("()").to_string();
                    if clean_sym.len() >= 3 && !clean_sym.contains(' ') {
                        citations.push(CodeCitation {
                            code_path: String::new(),
                            symbol: Some(clean_sym),
                            line_number: line_num,
                        });
                    }
                }
                rest_code = &after_start[end_tick + 1..];
            } else {
                break;
            }
        }
    }

    citations
}

/// Extracts all code dependencies (both frontmatter declarations and inline citations).
pub fn extract_all_code_dependencies(doc_id: &str, content: &str) -> (Vec<CodeDependency>, Vec<String>) {
    let mut dependencies = Vec::new();
    let mut errors = Vec::new();

    // 1. Frontmatter code_dependencies
    match crate::fragment::frontmatter::parse_code_dependencies(content) {
        Ok(fm_deps) => {
            for dep in fm_deps {
                match validate_path_containment(&dep) {
                    Ok(norm_path) => {
                        dependencies.push(CodeDependency {
                            code_path: norm_path,
                            source: DependencySource::Frontmatter,
                            line_number: None,
                        });
                    }
                    Err(err) => {
                        errors.push(format!("{}: {}", err, dep));
                    }
                }
            }
        }
        Err(err) => {
            errors.push(err);
        }
    }

    // 2. Inline markdown links and citations
    for citation in extract_code_citations(doc_id, content) {
        if !citation.code_path.is_empty() {
            match validate_and_normalize_code_path(Some(doc_id), &citation.code_path) {
                Ok(norm_path) => {
                    let source = if citation.symbol.is_some() {
                        DependencySource::InlineSymbolCitation
                    } else {
                        DependencySource::InlineMarkdownLink
                    };
                    dependencies.push(CodeDependency {
                        code_path: norm_path,
                        source,
                        line_number: Some(citation.line_number),
                    });
                }
                Err(err) => {
                    errors.push(format!("{}: {}", err, citation.code_path));
                }
            }
        }
    }

    (dependencies, errors)
}
