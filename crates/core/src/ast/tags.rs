//! Hashtag and YAML frontmatter tag extractor normalizing to lowercase.

use regex::Regex;
use serde_yaml::Value;
use std::collections::HashSet;

pub fn extract_frontmatter_tags(content: &str) -> (Vec<String>, Option<String>) {
    let mut tags = Vec::new();
    let mut title = None;

    if !content.starts_with("---") {
        return (tags, title);
    }

    if let Some(end_idx) = content[3..].find("\n---") {
        let yaml_str = &content[3..3 + end_idx];
        if let Ok(Value::Mapping(map)) = serde_yaml::from_str::<Value>(yaml_str) {
            // Check for title in frontmatter
            if let Some(Value::String(t)) = map.get(&Value::String("title".to_string())) {
                title = Some(t.clone());
            }

            // Check for tags
            if let Some(val) = map.get(&Value::String("tags".to_string())) {
                match val {
                    Value::Sequence(seq) => {
                        for item in seq {
                            if let Value::String(s) = item {
                                tags.push(s.to_lowercase().trim().trim_start_matches('#').to_string());
                            }
                        }
                    }
                    Value::String(s) => {
                        for item in s.split(',') {
                            let t = item.trim().to_lowercase().trim_start_matches('#').to_string();
                            if !t.is_empty() {
                                tags.push(t);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    (tags, title)
}

pub fn extract_inline_hashtags(text: &str) -> Vec<String> {
    // Matches #tag, #sub/tag, #tag-name, but not inside URLs or headings (# Header)
    // Needs word boundary before # or whitespace/punctuation
    let re = Regex::new(r"(?:^|[\s(\[<{])#([a-zA-Z0-9_/-]+)").unwrap();
    let mut tags = HashSet::new();

    for cap in re.captures_iter(text) {
        if let Some(mat) = cap.get(1) {
            let tag_str = mat.as_str().to_lowercase();
            // Don't treat hex colors like #fff or markdown headings as tags
            if !tag_str.is_empty() && !tag_str.chars().all(|c| c.is_ascii_hexdigit()) {
                tags.insert(tag_str);
            }
        }
    }

    let mut result: Vec<String> = tags.into_iter().collect();
    result.sort();
    result
}
