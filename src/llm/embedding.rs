use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::settings::AppConfig;

/// Trait for converting text into vector embeddings
#[async_trait::async_trait]
pub trait EmbeddingProvider {
    async fn embed_text(&self, text: &str) -> Result<Vec<f32>>;
}

pub fn create_embedding_provider(config: &AppConfig) -> Result<Box<dyn EmbeddingProvider>> {
    let active = config.provider.active.as_str();
    let settings = config.active_provider_settings();

    match active {
        "ollama" => Ok(Box::new(OllamaEmbeddings::new(
            &settings.base_url,
            "nomic-embed-text",
        ))),
        "openai" | "openrouter" => Ok(Box::new(OpenAiEmbeddings::new(
            &settings.base_url,
            &settings.api_key,
            "text-embedding-3-small",
        ))),
        _ => anyhow::bail!("Embedding not supported for provider: {}", active),
    }
}

pub struct OllamaEmbeddings {
    client: Client,
    base_url: String,
    model: String,
}

impl OllamaEmbeddings {
    pub fn new(base_url: &str, model: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            model: model.to_string(),
        }
    }
}

#[derive(Serialize)]
struct OllamaEmbeddingRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct OllamaEmbeddingResponse {
    embedding: Vec<f32>,
}

#[async_trait::async_trait]
impl EmbeddingProvider for OllamaEmbeddings {
    async fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/api/embeddings", self.base_url);
        let req = OllamaEmbeddingRequest {
            model: &self.model,
            prompt: text,
        };

        let res = self.client
            .post(&url)
            .json(&req)
            .send()
            .await
            .context("Failed to connect to Ollama")?;

        if !res.status().is_success() {
            let status = res.status();
            let err = res.text().await.unwrap_or_default();
            anyhow::bail!("Ollama embedding failed ({}): {}", status, err);
        }

        let data: OllamaEmbeddingResponse = res.json().await?;
        Ok(data.embedding)
    }
}

pub struct OpenAiEmbeddings {
    client: Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl OpenAiEmbeddings {
    pub fn new(base_url: &str, api_key: &str, model: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
        }
    }
}

#[derive(Serialize)]
struct OpenAiEmbeddingRequest<'a> {
    model: &'a str,
    input: &'a str,
}

#[derive(Deserialize)]
struct OpenAiEmbeddingResponse {
    data: Vec<OpenAiEmbeddingData>,
}

#[derive(Deserialize)]
struct OpenAiEmbeddingData {
    embedding: Vec<f32>,
}

#[async_trait::async_trait]
impl EmbeddingProvider for OpenAiEmbeddings {
    async fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/embeddings", self.base_url);
        let req = OpenAiEmbeddingRequest {
            model: &self.model,
            input: text,
        };

        let res = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&req)
            .send()
            .await
            .context("Failed to connect to OpenAI")?;

        if !res.status().is_success() {
            let status = res.status();
            let err = res.text().await.unwrap_or_default();
            anyhow::bail!("OpenAI embedding failed ({}): {}", status, err);
        }

        let mut data: OpenAiEmbeddingResponse = res.json().await?;
        if data.data.is_empty() {
            anyhow::bail!("OpenAI returned empty embeddings array");
        }
        
        Ok(data.data.remove(0).embedding)
    }
}
