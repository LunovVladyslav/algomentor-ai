use anyhow::{Context, Result};
use std::path::Path;

use super::models::{Difficulty, TaskDescription};

/// Parse a task.md file into a TaskDescription
pub fn parse_task_file(path: &Path) -> Result<TaskDescription> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read task file: {}", path.display()))?;
    parse_task_content(&content)
}

/// Parse task.md content with optional YAML frontmatter
pub fn parse_task_content(content: &str) -> Result<TaskDescription> {
    let (frontmatter, body) = split_frontmatter(content);

    let mut desc = TaskDescription::default();

    // Parse YAML frontmatter if present
    if let Some(fm) = frontmatter {
        if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(&fm) {
            if let Some(title) = yaml["title"].as_str() {
                desc.title = title.to_string();
            }
            if let Some(diff) = yaml["difficulty"].as_str() {
                desc.difficulty = Some(Difficulty::from_str_loose(diff));
            }
            if let Some(cat) = yaml["category"].as_str() {
                desc.category = Some(cat.to_string());
            }
            if let Some(source) = yaml["source"].as_str() {
                desc.source = Some(source.to_string());
            }
            if let Some(tags) = yaml["tags"].as_sequence() {
                desc.tags = tags
                    .iter()
                    .filter_map(|t| t.as_str().map(String::from))
                    .collect();
            }
        }
    }

    // Parse body content
    desc.body = body.to_string();

    // Extract title from first heading if not set
    if desc.title.is_empty() {
        if let Some(title) = extract_first_heading(&body) {
            desc.title = title;
        }
    }

    // Extract examples
    desc.examples = extract_section(&body, "example");
    desc.constraints = extract_section(&body, "constraint");

    Ok(desc)
}

/// Split content into optional YAML frontmatter and body
fn split_frontmatter(content: &str) -> (Option<String>, String) {
    let trimmed = content.trim_start();

    if !trimmed.starts_with("---") {
        return (None, content.to_string());
    }

    // Find the closing ---
    let after_first = &trimmed[3..];
    if let Some(end_pos) = after_first.find("\n---") {
        let fm = after_first[..end_pos].trim().to_string();
        let body = after_first[end_pos + 4..].trim().to_string();
        (Some(fm), body)
    } else {
        (None, content.to_string())
    }
}

/// Extract the first markdown heading (# Title)
fn extract_first_heading(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(heading) = trimmed.strip_prefix("# ") {
            return Some(heading.trim().to_string());
        }
    }
    None
}

/// Extract content under a section heading (case-insensitive)
fn extract_section(content: &str, section_name: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut in_section = false;
    let mut current = String::new();
    let section_lower = section_name.to_lowercase();

    for line in content.lines() {
        let trimmed = line.trim();

        // Check for section heading
        if trimmed.starts_with('#') {
            if in_section && !current.trim().is_empty() {
                results.push(current.trim().to_string());
                current.clear();
            }

            let heading_text = trimmed.trim_start_matches('#').trim().to_lowercase();
            in_section = heading_text.contains(&section_lower);
            continue;
        }

        if in_section {
            current.push_str(line);
            current.push('\n');
        }
    }

    if in_section && !current.trim().is_empty() {
        results.push(current.trim().to_string());
    }

    results
}

/// Generate a template task.md content
pub fn generate_task_template(name: &str) -> String {
    format!(
        r#"---
title: {name}
difficulty: medium
category: 
source: 
tags: []
---

# {name}

## Description

(Describe the problem here)

## Examples

**Input:** 
**Output:** 

**Input:** 
**Output:** 

## Constraints

- 

## Notes

"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let md = r#"---
title: Two Sum
difficulty: easy
category: Arrays
---
# Two Sum Body"#;
        
        let task = parse_task_content(md).unwrap();
        assert_eq!(task.title, "Two Sum");
        assert_eq!(task.difficulty.unwrap(), Difficulty::Easy);
        assert_eq!(task.category.unwrap(), "Arrays");
        assert_eq!(task.body, "# Two Sum Body");
    }

    #[test]
    fn test_extract_first_heading() {
        let md = "Some text\n# Target Title\nMore text";
        assert_eq!(extract_first_heading(md), Some("Target Title".to_string()));
    }

    #[test]
    fn test_extract_section() {
        let md = "# Title\n## Examples\nInput: [1,2]\nOutput: 3\n## Constraints\n- length > 0";
        let examples = extract_section(md, "example");
        let constraints = extract_section(md, "constraint");
        
        assert_eq!(examples.len(), 1);
        assert!(examples[0].contains("Input: [1,2]"));
        
        assert_eq!(constraints.len(), 1);
        assert!(constraints[0].contains("- length > 0"));
    }
}
