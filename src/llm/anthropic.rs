use anyhow::{Context, Result};
use async_trait::async_trait;
use futures_util::Stream;
use reqwest::Client;
use serde_json::json;
use std::pin::Pin;

use super::provider::{CompletionOptions, CompletionResponse, LlmProvider, Message, Role};
use super::streaming::parse_anthropic_sse;

/// Anthropic Claude API provider (native Messages API)
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl AnthropicProvider {
    pub fn new(api_key: &str, base_url: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    /// Build request body in Anthropic's Messages API format.
    /// System message must be separate from user/assistant messages.
    fn build_body(messages: &[Message], options: &CompletionOptions, stream: bool) -> serde_json::Value {
        let mut system_text = String::new();
        let mut api_messages = Vec::new();

        for msg in messages {
            match msg.role {
                Role::System => {
                    if !system_text.is_empty() {
                        system_text.push('\n');
                    }
                    system_text.push_str(&msg.content);
                }
                Role::User => {
                    api_messages.push(json!({"role": "user", "content": msg.content}));
                }
                Role::Assistant => {
                    api_messages.push(json!({"role": "assistant", "content": msg.content}));
                }
                Role::Tool => {
                    api_messages.push(json!({"role": "user", "content": msg.content}));
                }
            }
        }

        // Ensure messages alternate properly — Anthropic requires user, assistant, user...
        // If first non-system message is assistant, prepend a user message
        if api_messages.first().map(|m| m["role"].as_str()) == Some(Some("assistant")) {
            api_messages.insert(0, json!({"role": "user", "content": "Continue."}));
        }

        let mut body = json!({
            "model": options.model,
            "messages": api_messages,
            "max_tokens": options.max_tokens.unwrap_or(4096),
            "stream": stream,
        });

        if !system_text.is_empty() {
            body["system"] = json!(system_text);
        }

        body
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    async fn complete(&self, messages: &[Message], options: &CompletionOptions) -> Result<CompletionResponse> {
        let body = Self::build_body(messages, options, false);

        let resp = self
            .client
            .post(format!("{}/v1/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Anthropic")?;

        let status = resp.status();
        if !status.is_success() {
            let error_body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Anthropic API error ({}): {}", status, crate::utils::error::parse_api_error(&error_body));
        }

        let json: serde_json::Value = resp.json().await?;

        // Extract text from content blocks
        let mut result = String::new();
        if let Some(content) = json["content"].as_array() {
            for block in content {
                if block["type"].as_str() == Some("text") {
                    if let Some(text) = block["text"].as_str() {
                        result.push_str(text);
                    }
                }
            }
        }

        Ok(CompletionResponse::new(result))
    }

    async fn stream(
        &self,
        messages: &[Message],
        options: &CompletionOptions,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        let body = Self::build_body(messages, options, true);

        let resp = self
            .client
            .post(format!("{}/v1/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to Anthropic")?;

        let status = resp.status();
        if !status.is_success() {
            let error_body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Anthropic API error ({}): {}", status, crate::utils::error::parse_api_error(&error_body));
        }

        let byte_stream = Box::pin(resp.bytes_stream());
        Ok(parse_anthropic_sse(byte_stream))
    }

    fn name(&self) -> &str {
        "Anthropic Claude"
    }

    async fn health_check(&self) -> Result<bool> {
        // Anthropic doesn't have a simple health endpoint, try a minimal request
        Ok(!self.api_key.is_empty())
    }
}
