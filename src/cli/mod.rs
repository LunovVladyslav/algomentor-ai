pub mod commands;

use clap::Subcommand;

/// Configuration subcommands
#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// Configuration key (e.g., provider, model, api-key, language, level)
        key: String,
        /// Value to set
        value: String,
    },
    /// Set the active LLM provider
    Provider {
        /// Provider name: openai, anthropic, openrouter, ollama, lmstudio
        name: String,
    },
    /// Set the model to use
    Model {
        /// Model name (e.g., gpt-4o, claude-sonnet-4-20250514, llama3.2)
        name: String,
    },
    /// Set API key for the active provider
    #[command(name = "api-key")]
    ApiKey {
        /// API key value
        key: String,
    },
}

/// Knowledge base subcommands
#[derive(Subcommand, Debug)]
pub enum KnowledgeAction {
    /// Ingest a directory of markdown files into the knowledge base
    Ingest {
        /// Directory containing markdown files
        dir: String,
    },
}
