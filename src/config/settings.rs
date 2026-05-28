use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Name of the project config directory
pub const CONFIG_DIR: &str = ".algomentor";
/// Name of the local config file
pub const CONFIG_FILE: &str = "config.toml";
/// Name of the database file
pub const DB_FILE: &str = "algomentor.db";
/// Name of the global config file
pub const GLOBAL_CONFIG_FILE: &str = "global_config.toml";

/// Top-level application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub provider: ProviderConfig,
    #[serde(default)]
    pub watcher: WatcherConfig,
    #[serde(default)]
    pub mentor: MentorConfig,
    #[serde(default)]
    pub mcp: std::collections::HashMap<String, McpServerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Interface language: "en", "ru", or "auto"
    #[serde(default = "default_language")]
    pub language: String,
    /// User skill level: "beginner", "intermediate", "advanced"
    #[serde(default = "default_level")]
    pub level: String,
    /// Preferred programming language (e.g., "python", "rust")
    #[serde(default = "default_programming_language")]
    pub programming_language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Active provider: "openai", "anthropic", "openrouter", "ollama", "lmstudio"
    #[serde(default = "default_provider")]
    pub active: String,
    /// Default model to use
    #[serde(default = "default_model")]
    pub model: String,
    /// Provider-specific settings
    #[serde(default)]
    pub openai: ProviderSettings,
    #[serde(default)]
    pub anthropic: ProviderSettings,
    #[serde(default)]
    pub openrouter: ProviderSettings,
    #[serde(default)]
    pub ollama: ProviderSettings,
    #[serde(default)]
    pub lmstudio: ProviderSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderSettings {
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatcherConfig {
    /// Debounce delay in milliseconds
    #[serde(default = "default_debounce")]
    pub debounce_ms: u64,
    /// File patterns to ignore
    #[serde(default = "default_ignore_patterns")]
    pub ignore_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentorConfig {
    /// Maximum messages in context window
    #[serde(default = "default_max_context")]
    pub max_context_messages: usize,
    /// LLM temperature
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    /// Auto-analyze on file save in watch mode
    #[serde(default = "default_true")]
    pub auto_analyze_on_save: bool,
}

// Default value functions
fn default_language() -> String { "auto".into() }
fn default_level() -> String { "intermediate".into() }
fn default_programming_language() -> String { "python".into() }
fn default_provider() -> String { "openai".into() }
fn default_model() -> String { "gpt-4o".into() }
fn default_debounce() -> u64 { 1000 }
fn default_max_context() -> usize { 20 }
fn default_temperature() -> f32 { 0.7 }
fn default_true() -> bool { true }
fn default_ignore_patterns() -> Vec<String> {
    vec![
        "*.swp".into(), "*.tmp".into(), "~*".into(), ".git".into(),
        "node_modules".into(), "__pycache__".into(), "*.pyc".into(),
        ".DS_Store".into(), "Thumbs.db".into(),
    ]
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            language: default_language(),
            level: default_level(),
            programming_language: default_programming_language(),
        }
    }
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            active: default_provider(),
            model: default_model(),
            openai: ProviderSettings {
                base_url: "https://api.openai.com/v1".into(),
                ..Default::default()
            },
            anthropic: ProviderSettings {
                base_url: "https://api.anthropic.com".into(),
                ..Default::default()
            },
            openrouter: ProviderSettings {
                base_url: "https://openrouter.ai/api/v1".into(),
                ..Default::default()
            },
            ollama: ProviderSettings {
                base_url: "http://localhost:11434".into(),
                model: "llama3.2".into(),
                ..Default::default()
            },
            lmstudio: ProviderSettings {
                base_url: "http://localhost:1234".into(),
                ..Default::default()
            },
        }
    }
}

impl Default for WatcherConfig {
    fn default() -> Self {
        Self {
            debounce_ms: default_debounce(),
            ignore_patterns: default_ignore_patterns(),
        }
    }
}

impl Default for MentorConfig {
    fn default() -> Self {
        Self {
            max_context_messages: default_max_context(),
            temperature: default_temperature(),
            auto_analyze_on_save: true,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            provider: ProviderConfig::default(),
            watcher: WatcherConfig::default(),
            mentor: MentorConfig::default(),
            mcp: std::collections::HashMap::new(),
        }
    }
}

impl AppConfig {
    /// Load config from the project directory, falling back to defaults
    pub fn load(project_dir: &Path) -> Result<Self> {
        let config_path = project_dir.join(CONFIG_DIR).join(CONFIG_FILE);

        let mut config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .with_context(|| format!("Failed to read config from {}", config_path.display()))?;
            toml::from_str::<AppConfig>(&content)
                .with_context(|| "Failed to parse config.toml")?
        } else {
            AppConfig::default()
        };

        // Merge global config for API keys
        config.merge_global()?;
        Ok(config)
    }

    /// Merge API keys from global config (~/.algomentor/global_config.toml)
    fn merge_global(&mut self) -> Result<()> {
        if let Some(home) = dirs::home_dir() {
            let global_path = home.join(CONFIG_DIR).join(GLOBAL_CONFIG_FILE);
            if global_path.exists() {
                let content = std::fs::read_to_string(&global_path)?;
                let global: AppConfig = toml::from_str(&content).unwrap_or_default();

                // Only fill in empty API keys from global config
                if self.provider.openai.api_key.is_empty() {
                    self.provider.openai.api_key = global.provider.openai.api_key;
                }
                if self.provider.anthropic.api_key.is_empty() {
                    self.provider.anthropic.api_key = global.provider.anthropic.api_key;
                }
                if self.provider.openrouter.api_key.is_empty() {
                    self.provider.openrouter.api_key = global.provider.openrouter.api_key;
                }
            }
        }
        Ok(())
    }

    /// Save config to the project directory
    pub fn save(&self, project_dir: &Path) -> Result<()> {
        let config_dir = project_dir.join(CONFIG_DIR);
        std::fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join(CONFIG_FILE);
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        std::fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config to {}", config_path.display()))?;
        Ok(())
    }

    /// Save API key to global config
    pub fn save_api_key_global(provider_name: &str, api_key: &str) -> Result<()> {
        let home = dirs::home_dir().context("Cannot find home directory")?;
        let global_dir = home.join(CONFIG_DIR);
        std::fs::create_dir_all(&global_dir)?;
        let global_path = global_dir.join(GLOBAL_CONFIG_FILE);

        let mut config = if global_path.exists() {
            let content = std::fs::read_to_string(&global_path)?;
            toml::from_str::<AppConfig>(&content).unwrap_or_default()
        } else {
            AppConfig::default()
        };

        match provider_name {
            "openai" => config.provider.openai.api_key = api_key.to_string(),
            "anthropic" => config.provider.anthropic.api_key = api_key.to_string(),
            "openrouter" => config.provider.openrouter.api_key = api_key.to_string(),
            _ => {}
        }

        let content = toml::to_string_pretty(&config)?;
        std::fs::write(&global_path, content)?;
        Ok(())
    }

    /// Get the active provider settings
    pub fn active_provider_settings(&self) -> &ProviderSettings {
        match self.provider.active.as_str() {
            "anthropic" => &self.provider.anthropic,
            "openrouter" => &self.provider.openrouter,
            "ollama" => &self.provider.ollama,
            "lmstudio" => &self.provider.lmstudio,
            _ => &self.provider.openai,
        }
    }

    /// Get the model to use (provider-specific override or default)
    pub fn active_model(&self) -> &str {
        let provider_model = &self.active_provider_settings().model;
        if provider_model.is_empty() {
            &self.provider.model
        } else {
            provider_model
        }
    }

    /// Get the database path for this project
    pub fn db_path(project_dir: &Path) -> PathBuf {
        project_dir.join(CONFIG_DIR).join(DB_FILE)
    }

    /// Check if the project is initialized
    pub fn is_initialized(project_dir: &Path) -> bool {
        project_dir.join(CONFIG_DIR).exists()
    }
}
