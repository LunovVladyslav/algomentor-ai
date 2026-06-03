use anyhow::{Context, Result};
use colored::Colorize;
use futures_util::StreamExt;
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

use crate::analyzer::code_reader;
use crate::config::settings::AppConfig;
use crate::llm::prompts;
use crate::llm::provider::{CompletionOptions, LlmProvider, Message};
use crate::memory::database::Database;
use crate::task::parser;

/// Code file extensions to watch
const WATCH_EXTENSIONS: &[&str] = &[
    "py", "js", "ts", "java", "cpp", "cc", "c", "go", "rs", "rb", "cs",
];

/// File watcher that monitors code changes and provides real-time feedback
pub struct FileWatcher {
    task_dir: PathBuf,
    config: AppConfig,
}

impl FileWatcher {
    pub fn new(task_dir: PathBuf, config: AppConfig) -> Self {
        Self { task_dir, config }
    }

    /// Start watching for file changes
    pub async fn start(
        &self,
        provider: Box<dyn LlmProvider>,
        _db: Database,
    ) -> Result<()> {
        let task_md = self.task_dir.join("task.md");
        let task_desc = parser::parse_task_file(&task_md).ok();
        let task_name = task_desc
            .as_ref()
            .map(|d| d.title.as_str())
            .unwrap_or_else(|| {
                self.task_dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
            });

        let task_display = task_name;
        let dir_display   = self.task_dir.display().to_string();
        let prov_display  = provider.name();

        let rule = "─".repeat(54);

        println!();
        println!("  ╔{}╗", "═".repeat(54).cyan());
        println!(
            "  ║  {}{}  ║",
            "👁  AlgoMentor  ·  Watch Mode".white().bold(),
            " ".repeat(54usize.saturating_sub(30))
        );
        println!("  ╠{}╣", "═".repeat(54).cyan());
        println!(
            "  ║  {}  {}{}  ║",
            "task".dimmed(),
            task_display.yellow().bold(),
            " ".repeat(54usize.saturating_sub(6 + task_display.len()))
        );
        println!(
            "  ║  {}  {}{}  ║",
            "dir ".dimmed(),
            dir_display.dimmed(),
            " ".repeat(54usize.saturating_sub(6 + dir_display.len().min(44)))
        );
        println!(
            "  ║  {}  {}{}  ║",
            "via ".dimmed(),
            prov_display.white(),
            " ".repeat(54usize.saturating_sub(6 + prov_display.len()))
        );
        println!("  ╠{}╣", "═".repeat(54).cyan());
        println!(
            "  ║  {}{}  ║",
            "Press Ctrl+C to stop".dimmed(),
            " ".repeat(54usize.saturating_sub(22))
        );
        println!("  ╚{}╝", "═".repeat(54).cyan());
        println!();
        println!("  {}  {}", "·".dimmed(), "Watching for file changes...".dimmed());
        let _ = rule;

        // Set up file watcher with debouncing
        let (tx, rx) = mpsc::channel();
        let debounce_duration = Duration::from_millis(self.config.watcher.debounce_ms);
        let mut debouncer = new_debouncer(debounce_duration, tx)
            .context("Failed to create file watcher")?;

        debouncer
            .watcher()
            .watch(&self.task_dir, RecursiveMode::NonRecursive)
            .context("Failed to watch directory")?;

        let ignore_patterns = &self.config.watcher.ignore_patterns;
        let system_prompt = prompts::get_mentor_system_prompt(
            &self.config.general.level,
            &self.config.general.language,
        );

        // Watch loop
        loop {
            match rx.recv() {
                Ok(Ok(events)) => {
                    for event in events {
                        let path = &event.path;

                        // Only process code files
                        if !is_code_file(path) {
                            continue;
                        }

                        // Check ignore patterns
                        if should_ignore(path, ignore_patterns) {
                            continue;
                        }

                        if event.kind == DebouncedEventKind::Any {
                            let filename = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown");

                            println!();
                            println!(
                                "  {}  {} {}",
                                "save".yellow().bold(),
                                "·".dimmed(),
                                filename.white()
                            );

                            // Read the changed file
                            match code_reader::read_file(path) {
                                Ok(code_file) => {
                                    if code_file.content.trim().is_empty() {
                                        continue;
                                    }

                                    let task_body =
                                        task_desc.as_ref().map(|d| d.body.as_str());
                                    let prompt = prompts::get_watch_analysis_prompt(
                                        &code_file.content,
                                        &code_file.language.to_string(),
                                        task_body,
                                    );

                                    let messages = vec![
                                        Message::system(&system_prompt),
                                        Message::user(&prompt),
                                    ];

                                    let options = CompletionOptions {
                                        model: self.config.active_model().to_string(),
                                        temperature: self.config.mentor.temperature,
                                        max_tokens: Some(512),
                                        tools: None,
                                    };

                                    // Stream response
                                    print!("\n  {} {}\n  ", "mentor".cyan().bold(), "›".cyan().dimmed());
                                    match provider.stream(&messages, &options).await {
                                        Ok(mut stream) => {
                                            while let Some(chunk) = stream.next().await {
                                                match chunk {
                                                    Ok(text) => {
                                                        print!("{}", text);
                                                        std::io::Write::flush(
                                                            &mut std::io::stdout(),
                                                        )
                                                        .ok();
                                                    }
                                                    Err(e) => {
                                                        eprintln!(
                                                            "\n{} Stream error: {}",
                                                            "❌".red(),
                                                            e
                                                        );
                                                        break;
                                                    }
                                                }
                                            }
                                            println!();
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "\n{} Failed to get analysis: {}",
                                                "❌".red(),
                                                e
                                            );
                                        }
                                    }

                                    println!();
                                    println!("  {}", "─".repeat(54).dimmed());
                                    println!();
                                    println!("  {}  {}", "·".dimmed(), "Watching for file changes...".dimmed());
                                }
                                Err(e) => {
                                    eprintln!("{} Failed to read file: {}", "⚠️".yellow(), e);
                                }
                            }
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("{} Watcher error: {:?}", "⚠️".yellow(), e);
                }
                Err(e) => {
                    // Channel disconnected
                    eprintln!("{} Watcher channel closed: {}", "❌".red(), e);
                    break;
                }
            }
        }

        Ok(())
    }
}

fn is_code_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|ext| WATCH_EXTENSIONS.contains(&ext))
        .unwrap_or(false)
}

fn should_ignore(path: &Path, patterns: &[String]) -> bool {
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    for pattern in patterns {
        let pattern = pattern.trim();
        if pattern.starts_with("*.") {
            // Extension pattern
            let ext = &pattern[2..];
            if filename.ends_with(ext) {
                return true;
            }
        } else if filename.starts_with(pattern) || filename == pattern {
            return true;
        }
    }
    false
}
