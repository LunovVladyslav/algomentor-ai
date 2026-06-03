use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod cli;

use cli::{ConfigAction, KnowledgeAction};

#[derive(Parser)]
#[command(
    name = "algomentor",
    about = "🧠 AI Mentor for algorithmic problem solving & interview preparation",
    long_about = "AlgoMentor is an AI-powered CLI mentor that helps you solve algorithmic problems.\n\
    It analyzes your code in real-time, provides hints (not solutions), evaluates Big O complexity,\n\
    and tracks your progress. Supports LeetCode, CodeWars, and technical interview prep.",
    version,
    author = "AlgoMentor"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize AlgoMentor in the current project directory
    Init,

    /// Show the interactive guide and help
    Guide,

    /// Add a new task directory with a template
    Add {
        /// Name of the task (used for the directory name)
        name: String,

        /// Category of the task (e.g., arrays, graphs)
        #[arg(short, long)]
        category: Option<String>,

        /// Difficulty (e.g., Easy, Medium, Hard)
        #[arg(short, long)]
        difficulty: Option<String>,
    },

    /// Start an interactive chat with the mentor
    Chat {
        /// Task name or directory to focus on
        #[arg(value_name = "TASK")]
        task: Option<String>,
    },

    /// Watch files for changes and provide real-time feedback
    Watch {
        /// Task name or directory to watch
        #[arg(value_name = "TASK")]
        task: Option<String>,
    },

    /// Start a project session with MCP support (Context7)
    Project,

    /// Analyze a specific file for complexity and issues
    Analyze {
        /// Path to the file to analyze
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Open the TUI dashboard with progress and analytics
    Dashboard,

    /// List all tasks and their statuses
    Tasks,

    /// View chat history
    History {
        /// Task name to view history for
        #[arg(value_name = "TASK")]
        task: Option<String>,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
    },

    /// Manage the RAG knowledge base
    Knowledge {
        #[command(subcommand)]
        action: KnowledgeAction,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let cli = Cli::parse();
    let project_dir = std::env::current_dir().context("Failed to get current directory")?;

    match cli.command {
        None => cli::commands::run_chat(&project_dir, None).await,
        Some(Commands::Init) => cli::commands::run_init(&project_dir).await,
        Some(Commands::Guide) => cli::commands::run_guide().await,
        Some(Commands::Add { name, category, difficulty }) => cli::commands::run_add(&project_dir, &name, category.as_deref(), difficulty.as_deref()).await,
        Some(Commands::Chat { task }) => cli::commands::run_chat(&project_dir, task.as_deref()).await,
        Some(Commands::Watch { task }) => cli::commands::run_watch(&project_dir, task.as_deref()).await,
        Some(Commands::Project) => cli::commands::run_project(&project_dir).await,
        Some(Commands::Analyze { file }) => cli::commands::run_analyze(&project_dir, &file).await,
        Some(Commands::Dashboard) => cli::commands::run_dashboard(&project_dir).await,
        Some(Commands::Tasks) => cli::commands::run_tasks(&project_dir).await,
        Some(Commands::History { task }) => cli::commands::run_history(&project_dir, task.as_deref()).await,
        Some(Commands::Config { action }) => cli::commands::run_config(&project_dir, action).await,
        Some(Commands::Knowledge { action }) => cli::commands::run_knowledge(&project_dir, action).await,
    }
}
