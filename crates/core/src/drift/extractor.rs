//! AST code citation extractor for Markdown documents.
//! Extracts referenced code file paths and symbols without requiring multi-language compiler frontends.

use std::path::Path;

const CODE_EXTENSIONS: &[&str] = &[
    "rs", "ts", "tsx", "js", "jsx", "py", "go", "java", "cpp", "c", "h", "hpp", "rb", "php",
    "cs", "swift", "kt", "sh", "sql", "toml", "yaml", "yml", "json",
];

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
        .split('#')
        .next()
        .unwrap_or("")
        .split('?')
        .next()
        .unwrap_or("");

    if let Some(ext) = Path::new(clean).extension().and_then(|e| e.to_str()) {
        CODE_EXTENSIONS.contains(&ext.to_lowercase().as_str())
    } else {
        false
    }
}

/// Normalizes a referenced code path relative to workspace root.
pub fn normalize_code_path(referencing_doc: &str, target: &str) -> String {
    let clean = target
        .trim()
        .trim_start_matches("file://")
        .split('#')
        .next()
        .unwrap_or("")
        .split('?')
        .next()
        .unwrap_or("")
        .replace('\\', "/");

    if clean.starts_with('/') {
        return clean.trim_start_matches('/').to_string();
    }

    if clean.starts_with("./") || clean.starts_with("../") {
        if let Some(parent) = Path::new(referencing_doc).parent() {
            let parent_str = parent.to_string_lossy().replace('\\', "/");
            if !parent_str.is_empty() && parent_str != "." {
                let combined = format!("{}/{}", parent_str, clean);
                return normalize_path_components(&combined);
            }
        }
    }

    normalize_path_components(&clean)
}

fn normalize_path_components(path: &str) -> String {
    let mut parts = Vec::new();
    for seg in path.split('/') {
        if seg.is_empty() || seg == "." {
            continue;
        }
        if seg == ".." {
            parts.pop();
        } else {
            parts.push(seg);
        }
    }
    parts.join("/")
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
