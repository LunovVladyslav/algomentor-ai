use anyhow::Result;
use std::path::{Path, PathBuf};

use super::models::TaskInfo;
use super::parser;

/// Code file extensions to look for in task directories
const CODE_EXTENSIONS: &[&str] = &[
    "py", "js", "ts", "java", "cpp", "cc", "c", "go", "rs", "rb", "cs",
];

/// Discover all task directories under the project root
pub fn discover_tasks(root: &Path, max_depth: usize) -> Result<Vec<TaskInfo>> {
    let mut tasks = Vec::new();
    scan_directory(root, root, &mut tasks, 0, max_depth)?;
    tasks.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(tasks)
}

fn scan_directory(
    root: &Path,
    dir: &Path,
    tasks: &mut Vec<TaskInfo>,
    depth: usize,
    max_depth: usize,
) -> Result<()> {
    if depth > max_depth {
        return Ok(());
    }

    // Skip hidden directories and common noise
    if let Some(name) = dir.file_name().and_then(|n| n.to_str()) {
        if name.starts_with('.') || name == "node_modules" || name == "__pycache__" || name == "target" {
            return Ok(());
        }
    }

    let task_md_path = dir.join("task.md");
    if task_md_path.exists() {
        // This is a task directory
        let name = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let solution_files = find_solution_files(dir)?;
        let description = parser::parse_task_file(&task_md_path).ok();

        tasks.push(TaskInfo {
            name,
            directory: dir.to_path_buf(),
            has_task_md: true,
            solution_files,
            description,
        });

        // Don't recurse into task directories
        return Ok(());
    }

    // Recurse into subdirectories
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_directory(root, &path, tasks, depth + 1, max_depth)?;
            }
        }
    }

    Ok(())
}

/// Find solution files in a task directory
pub fn find_solution_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if CODE_EXTENSIONS.contains(&ext) {
                        files.push(path);
                    }
                }
            }
        }
    }

    // Sort: prefer files named "solution.*"
    files.sort_by(|a, b| {
        let a_is_solution = a
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.starts_with("solution"))
            .unwrap_or(false);
        let b_is_solution = b
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.starts_with("solution"))
            .unwrap_or(false);
        b_is_solution.cmp(&a_is_solution)
    });

    Ok(files)
}

/// Initialize a new task directory with a template task.md
pub fn init_task(parent_dir: &Path, name: &str) -> Result<PathBuf> {
    let task_dir = parent_dir.join(name);
    std::fs::create_dir_all(&task_dir)?;

    let task_md_path = task_dir.join("task.md");
    if !task_md_path.exists() {
        let template = parser::generate_task_template(name);
        std::fs::write(&task_md_path, template)?;
    }

    Ok(task_dir)
}

/// Resolve a task name to its directory path.
/// Searches for a directory matching the name that contains task.md.
pub fn resolve_task_dir(project_dir: &Path, task_name: &str) -> Result<Option<PathBuf>> {
    let tasks = discover_tasks(project_dir, 3)?;
    for task in &tasks {
        if task.name == task_name {
            return Ok(Some(task.directory.clone()));
        }
    }
    // Try as a relative path
    let direct_path = project_dir.join(task_name);
    if direct_path.join("task.md").exists() {
        return Ok(Some(direct_path));
    }
    Ok(None)
}
