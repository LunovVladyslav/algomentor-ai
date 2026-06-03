use anyhow::{Context, Result};
use async_trait::async_trait;
use futures_util::Stream;
use reqwest::Client;
use serde_json::json;
use std::pin::Pin;

use super::provider::{CompletionOptions, CompletionResponse, LlmProvider, Message};
use super::streaming::parse_openai_sse;

/// OpenRouter provider (OpenAI-compatible API)
pub struct OpenRouterProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OpenRouterProvider {
    pub fn new(api_key: &str, base_url: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    fn build_messages(messages: &[Message]) -> Vec<serde_json::Value> {
        messages
            .iter()
            .map(|m| json!({"role": m.role.to_string(), "content": m.content}))
            .collect()
    }
}

#[async_trait]
impl LlmProvider for OpenRouterProvider {
    async fn complete(&self, messages: &[Message], options: &CompletionOptions) -> Result<CompletionResponse> {
        let mut body = json!({
            "model": options.model,
            "messages": Self::build_messages(messages),
            "temperature": options.temperature,
            "stream": false,
        });

        if let Some(max_tokens) = options.max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://github.com/algomentor")
            .header("X-Title", "AlgoMentor")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to OpenRouter")?;

        let status = resp.status();
        if !status.is_success() {
            let error_body = resp.text().await.unwrap_or_default();
            anyhow::bail!("OpenRouter API error ({}): {}", status, crate::utils::error::parse_api_error(&error_body));
        }

        let json: serde_json::Value = resp.json().await?;
        
        let message = &json["choices"][0]["message"];
        let content = message["content"].as_str().unwrap_or("").to_string();
        
        let mut tool_calls = None;
        if let Some(calls) = message["tool_calls"].as_array() {
            let parsed_calls: Result<Vec<_>, _> = calls.iter().map(|c| serde_json::from_value(c.clone())).collect();
            if let Ok(tc) = parsed_calls {
                tool_calls = Some(tc);
            }
        }
        
        let mut response = CompletionResponse::new(content);
        response.tool_calls = tool_calls;
        Ok(response)
    }

    async fn stream(
        &self,
        messages: &[Message],
        options: &CompletionOptions,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        let mut body = json!({
            "model": options.model,
            "messages": Self::build_messages(messages),
            "temperature": options.temperature,
            "stream": true,
        });

        if let Some(max_tokens) = options.max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://github.com/algomentor")
            .header("X-Title", "AlgoMentor")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to OpenRouter")?;

        let status = resp.status();
        if !status.is_success() {
            let error_body = resp.text().await.unwrap_or_default();
            anyhow::bail!("OpenRouter API error ({}): {}", status, crate::utils::error::parse_api_error(&error_body));
        }

        let byte_stream = Box::pin(resp.bytes_stream());
        Ok(parse_openai_sse(byte_stream))
    }

    fn name(&self) -> &str {
        "OpenRouter"
    }

    async fn health_check(&self) -> Result<bool> {
        let resp = self
            .client
            .get(format!("{}/models", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;
        Ok(resp.map(|r| r.status().is_success()).unwrap_or(false))
    }
}
