use anyhow::{Context, Result};
use async_trait::async_trait;
use futures_util::Stream;
use reqwest::Client;
use serde_json::json;
use std::pin::Pin;

use super::provider::{CompletionOptions, CompletionResponse, LlmProvider, Message};
use super::streaming::parse_openai_sse;

/// Local LLM provider (Ollama / LM Studio) via OpenAI-compatible API
pub struct LocalProvider {
    client: Client,
    base_url: String,
    kind: String,
}

impl LocalProvider {
    pub fn new(base_url: &str, kind: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            kind: kind.to_string(),
        }
    }

    fn api_url(&self) -> String {
        match self.kind.as_str() {
            "ollama" => format!("{}/v1", self.base_url),
            _ => format!("{}/v1", self.base_url),
        }
    }

    fn build_messages(messages: &[Message]) -> Vec<serde_json::Value> {
        messages
            .iter()
            .map(|m| json!({"role": m.role.to_string(), "content": m.content}))
            .collect()
    }

    /// List available models from the local server
    pub async fn list_models(&self) -> Result<Vec<String>> {
        let resp = self
            .client
            .get(format!("{}/models", self.api_url()))
            .send()
            .await
            .context("Failed to connect to local LLM server")?;

        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        let json: serde_json::Value = resp.json().await?;
        let models = json["data"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|m| m["id"].as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        Ok(models)
    }
}

#[async_trait]
impl LlmProvider for LocalProvider {
    async fn complete(&self, messages: &[Message], options: &CompletionOptions) -> Result<CompletionResponse> {
        let body = json!({
            "model": options.model,
            "messages": Self::build_messages(messages),
            "temperature": options.temperature,
            "stream": false,
        });

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.api_url()))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .with_context(|| format!("Failed to connect to {} at {}", self.kind, self.base_url))?;

        let status = resp.status();
        if !status.is_success() {
            let error_body = resp.text().await.unwrap_or_default();
            anyhow::bail!("{} API error ({}): {}", self.kind, status, error_body);
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
        let body = json!({
            "model": options.model,
            "messages": Self::build_messages(messages),
            "temperature": options.temperature,
            "stream": true,
        });

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.api_url()))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .with_context(|| format!("Failed to connect to {} at {}", self.kind, self.base_url))?;

        let status = resp.status();
        if !status.is_success() {
            let error_body = resp.text().await.unwrap_or_default();
            anyhow::bail!("{} API error ({}): {}", self.kind, status, error_body);
        }

        let byte_stream = Box::pin(resp.bytes_stream());
        Ok(parse_openai_sse(byte_stream))
    }

    fn name(&self) -> &str {
        match self.kind.as_str() {
            "ollama" => "Ollama (Local)",
            "lmstudio" => "LM Studio (Local)",
            _ => "Local LLM",
        }
    }

    async fn health_check(&self) -> Result<bool> {
        let resp = self.client.get(&self.base_url).send().await;
        Ok(resp.map(|r| r.status().is_success()).unwrap_or(false))
    }
}
