use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

/// Task difficulty level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Unknown,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
            Difficulty::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Difficulty {
    pub fn from_str_loose(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "easy" | "simple" => Self::Easy,
            "medium" | "moderate" | "med" => Self::Medium,
            "hard" | "difficult" | "expert" => Self::Hard,
            _ => Self::Unknown,
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            Difficulty::Easy => "🟢",
            Difficulty::Medium => "🟡",
            Difficulty::Hard => "🔴",
            Difficulty::Unknown => "⚪",
        }
    }
}

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    InProgress,
    Completed,
    Abandoned,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Abandoned => write!(f, "Abandoned"),
        }
    }
}

impl TaskStatus {
    pub fn emoji(&self) -> &str {
        match self {
            TaskStatus::InProgress => "🔄",
            TaskStatus::Completed => "✅",
            TaskStatus::Abandoned => "❌",
        }
    }

    pub fn from_str_loose(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "completed" | "done" | "solved" => Self::Completed,
            "abandoned" | "skipped" => Self::Abandoned,
            _ => Self::InProgress,
        }
    }
}

/// A task stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub directory: String,
    pub difficulty: Option<String>,
    pub category: Option<String>,
    pub status: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub time_complexity: Option<String>,
    pub space_complexity: Option<String>,
    pub attempts: i32,
    pub language: Option<String>,
}

/// Parsed content from a task.md file
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskDescription {
    pub title: String,
    pub difficulty: Option<Difficulty>,
    pub category: Option<String>,
    pub source: Option<String>,
    pub tags: Vec<String>,
    pub body: String,
    pub examples: Vec<String>,
    pub constraints: Vec<String>,
}

/// Info about a discovered task directory
#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub name: String,
    pub directory: PathBuf,
    pub has_task_md: bool,
    pub solution_files: Vec<PathBuf>,
    pub description: Option<TaskDescription>,
}
