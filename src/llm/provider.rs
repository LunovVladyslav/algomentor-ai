use anyhow::Result;
use async_trait::async_trait;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use crate::config::settings::AppConfig;

use super::openai::OpenAiProvider;
use super::anthropic::AnthropicProvider;
use super::openrouter::OpenRouterProvider;
use super::local::LocalProvider;

/// Message role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::Tool => write!(f, "tool"),
        }
    }
}

/// A chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_id: Option<String>,
}

impl Message {
    pub fn system(content: &str) -> Self {
        Self {
            role: Role::System,
            content: content.to_string(),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn user(content: &str) -> Self {
        Self {
            role: Role::User,
            content: content.to_string(),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn assistant(content: &str) -> Self {
        Self {
            role: Role::Assistant,
            content: content.to_string(),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn assistant_with_tools(content: &str, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.to_string(),
            tool_calls: Some(tool_calls),
            tool_call_id: None,
        }
    }

    pub fn tool_result(tool_call_id: &str, content: &str) -> Self {
        Self {
            role: Role::Tool,
            content: content.to_string(),
            tool_calls: None,
            tool_call_id: Some(tool_call_id.to_string()),
        }
    }
}

/// Completion options
#[derive(Debug, Clone)]
pub struct CompletionOptions {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub tools: Option<Vec<Tool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub type_: String, // "function"
    pub function: FunctionDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub type_: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String, // JSON string of arguments
}

impl Default for CompletionOptions {
    fn default() -> Self {
        Self {
            model: "gpt-4o".into(),
            temperature: 0.7,
            max_tokens: Some(4096),
            tools: None,
        }
    }
}

pub struct CompletionResponse {
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl CompletionResponse {
    pub fn new(content: String) -> Self {
        Self {
            content,
            tool_calls: None,
        }
    }
    
    pub fn with_tools(content: String, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            content,
            tool_calls: Some(tool_calls),
        }
    }
}

/// The core LLM provider trait
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Complete a chat conversation
    async fn complete(&self, messages: &[Message], options: &CompletionOptions) -> Result<CompletionResponse>;

    /// Stream a response chunk by chunk
    async fn stream(
        &self,
        messages: &[Message],
        options: &CompletionOptions,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>>;

    /// Provider display name
    fn name(&self) -> &str;

    /// Check if the provider is reachable
    async fn health_check(&self) -> Result<bool>;
}

/// Create a provider from the application config
pub fn create_provider(config: &AppConfig) -> Result<Box<dyn LlmProvider>> {
    match config.provider.active.as_str() {
        "openai" => {
            let settings = &config.provider.openai;
            Ok(Box::new(OpenAiProvider::new(
                &settings.api_key,
                &settings.base_url,
            )))
        }
        "anthropic" => {
            let settings = &config.provider.anthropic;
            Ok(Box::new(AnthropicProvider::new(
                &settings.api_key,
                &settings.base_url,
            )))
        }
        "openrouter" => {
            let settings = &config.provider.openrouter;
            Ok(Box::new(OpenRouterProvider::new(
                &settings.api_key,
                &settings.base_url,
            )))
        }
        "ollama" => {
            let settings = &config.provider.ollama;
            Ok(Box::new(LocalProvider::new(
                &settings.base_url,
                "ollama",
            )))
        }
        "lmstudio" => {
            let settings = &config.provider.lmstudio;
            Ok(Box::new(LocalProvider::new(
                &settings.base_url,
                "lmstudio",
            )))
        }
        other => anyhow::bail!("Unknown provider: {}. Use: openai, anthropic, openrouter, ollama, lmstudio", other),
    }
}
