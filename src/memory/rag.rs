use anyhow::Result;
use std::cmp::Ordering;

use super::database::Database;
use crate::llm::embedding::EmbeddingProvider;

#[derive(Debug)]
pub struct KnowledgeChunk {
    pub id: i64,
    pub source: String,
    pub chunk_index: usize,
    pub content: String,
    pub embedding: Vec<f32>,
}

#[derive(Debug)]
pub struct SearchResult {
    pub chunk: KnowledgeChunk,
    pub score: f32,
}

/// Calculate cosine similarity between two vectors
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..a.len() {
        dot_product += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a.sqrt() * norm_b.sqrt())
}

/// Convert Vec<f32> to bytes for SQLite BLOB storage
pub fn f32_vec_to_bytes(vec: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(vec.len() * 4);
    for &f in vec {
        bytes.extend_from_slice(&f.to_ne_bytes());
    }
    bytes
}

/// Convert bytes from SQLite BLOB to Vec<f32>
pub fn bytes_to_f32_vec(bytes: &[u8]) -> Vec<f32> {
    let mut vec = Vec::with_capacity(bytes.len() / 4);
    for chunk in bytes.chunks_exact(4) {
        vec.push(f32::from_ne_bytes(chunk.try_into().unwrap()));
    }
    vec
}

/// Simple markdown chunker that splits by double newlines or headers
pub fn chunk_markdown(content: &str, max_chars: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();

    for line in content.lines() {
        let is_header = line.starts_with('#');
        let will_overflow = current_chunk.len() + line.len() > max_chars;

        if (is_header && !current_chunk.is_empty()) || will_overflow {
            chunks.push(current_chunk.trim().to_string());
            current_chunk.clear();
        }

        current_chunk.push_str(line);
        current_chunk.push('\n');
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }

    chunks
}

pub struct RagSystem<'a> {
    db: &'a Database,
    provider: Box<dyn EmbeddingProvider>,
}

impl<'a> RagSystem<'a> {
    pub fn new(db: &'a Database, provider: Box<dyn EmbeddingProvider>) -> Self {
        Self { db, provider }
    }

    /// Check if database is empty and auto-ingest knowledge base
    pub async fn init_knowledge_base(&self) -> Result<()> {
        let count: i64 = self.db.with_conn(|conn| {
            let mut stmt = conn.prepare("SELECT COUNT(*) FROM knowledge_base")?;
            let count: i64 = stmt.query_row([], |row| row.get(0))?;
            Ok(count)
        })?;

        if count > 0 {
            return Ok(());
        }

        use crate::assets::Assets;
        
        let files: Vec<_> = Assets::iter().filter(|p| p.starts_with("knowledge/")).collect();
        if files.is_empty() {
            return Ok(());
        }

        crate::chat::renderer::render_system_message("Initializing Knowledge Base (RAG) for the first time...");
        
        for (i, file_path) in files.iter().enumerate() {
            if let Some(file) = Assets::get(file_path) {
                if let Ok(content) = std::str::from_utf8(file.data.as_ref()) {
                    let spinner = crate::chat::renderer::start_spinner(&format!("Indexing {}/{} - {}...", i + 1, files.len(), file_path));
                    
                    if let Err(e) = self.ingest_file(file_path, content).await {
                        spinner.finish_and_clear();
                        crate::chat::renderer::render_error(&format!("Failed to ingest {}: {}", file_path, e));
                    } else {
                        spinner.finish_and_clear();
                    }
                }
            }
        }
        
        crate::chat::renderer::render_success("Knowledge Base successfully initialized!");
        Ok(())
    }

    /// Ingest a file into the knowledge base
    pub async fn ingest_file(&self, source: &str, content: &str) -> Result<usize> {
        let chunks = chunk_markdown(content, 1500); // ~300-400 tokens per chunk
        let mut count = 0;

        for (index, text) in chunks.iter().enumerate() {
            if text.len() < 50 {
                continue; // Skip very small chunks
            }

            let embedding = self.provider.embed_text(text).await?;
            let embedding_bytes = f32_vec_to_bytes(&embedding);

            self.db.with_conn(|conn| {
                conn.execute(
                    "INSERT INTO knowledge_base (source, chunk_index, content, embedding)
                     VALUES (?1, ?2, ?3, ?4)",
                    rusqlite::params![source, index, text, embedding_bytes],
                )?;
                Ok(())
            })?;
            count += 1;
        }

        Ok(count)
    }

    /// Search for the most relevant chunks given a query
    pub async fn search(&self, query: &str, top_k: usize) -> Result<Vec<SearchResult>> {
        let query_embedding = self.provider.embed_text(query).await?;

        // Fetch all embeddings from DB
        let chunks = self.db.with_conn(|conn| {
            let mut stmt = conn.prepare("SELECT id, source, chunk_index, content, embedding FROM knowledge_base")?;
            let rows = stmt.query_map([], |row| {
                let id: i64 = row.get(0)?;
                let source: String = row.get(1)?;
                let chunk_index: usize = row.get(2)?;
                let content: String = row.get(3)?;
                let embedding_bytes: Vec<u8> = row.get(4)?;

                Ok(KnowledgeChunk {
                    id,
                    source,
                    chunk_index,
                    content,
                    embedding: bytes_to_f32_vec(&embedding_bytes),
                })
            })?;

            let mut chunks = Vec::new();
            for r in rows {
                chunks.push(r?);
            }
            Ok(chunks)
        })?;

        let mut results: Vec<SearchResult> = chunks.into_iter()
            .map(|chunk| {
                let score = cosine_similarity(&query_embedding, &chunk.embedding);
                SearchResult { chunk, score }
            })
            .collect();

        // Sort descending by score
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));

        results.truncate(top_k);

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < f32::EPSILON);

        let c = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&a, &c) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_chunk_markdown() {
        let md = "# Title\n\nSome text here\n# Header 2\nMore text";
        let chunks = chunk_markdown(md, 50);
        
        assert_eq!(chunks.len(), 2);
        assert!(chunks[0].contains("# Title"));
        assert!(chunks[1].contains("# Header 2"));
    }
}

