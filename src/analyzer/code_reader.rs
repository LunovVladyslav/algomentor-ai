use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use super::language::Language;

/// A read code file
#[derive(Debug, Clone)]
pub struct CodeFile {
    pub path: PathBuf,
    pub content: String,
    pub language: Language,
    pub line_count: usize,
}

/// Read the primary solution file from a task directory
pub fn read_solution(task_dir: &Path) -> Result<Option<CodeFile>> {
    let solution_files = crate::task::discovery::find_solution_files(task_dir)?;

    if let Some(path) = solution_files.first() {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let language = Language::from_path(path);
        let line_count = content.lines().count();

        Ok(Some(CodeFile {
            path: path.clone(),
            content,
            language,
            line_count,
        }))
    } else {
        Ok(None)
    }
}

/// Read a specific file
pub fn read_file(path: &Path) -> Result<CodeFile> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let language = Language::from_path(path);
    let line_count = content.lines().count();

    Ok(CodeFile {
        path: path.to_path_buf(),
        content,
        language,
        line_count,
    })
}

/// Add line numbers to code content
pub fn with_line_numbers(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let width = lines.len().to_string().len();

    lines
        .iter()
        .enumerate()
        .map(|(i, line)| format!("{:>width$} │ {}", i + 1, line, width = width))
        .collect::<Vec<_>>()
        .join("\n")
}
