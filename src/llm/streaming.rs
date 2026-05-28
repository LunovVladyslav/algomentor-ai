use anyhow::Result;
use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use std::pin::Pin;

/// Parse an SSE stream (OpenAI-style) into text chunks.
///
/// Expects lines in the format:
/// ```text
/// data: {"choices":[{"delta":{"content":"hello"}}]}
/// ```
pub fn parse_openai_sse(
    byte_stream: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
) -> Pin<Box<dyn Stream<Item = Result<String>> + Send>> {
    let stream = async_stream::try_stream! {
        let mut buffer = String::new();

        tokio::pin!(byte_stream);

        while let Some(chunk) = byte_stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);
            buffer.push_str(&text);

            // Process complete lines
            while let Some(newline_pos) = buffer.find('\n') {
                let line = buffer[..newline_pos].trim().to_string();
                buffer = buffer[newline_pos + 1..].to_string();

                if line.is_empty() || line.starts_with(':') {
                    continue;
                }

                if let Some(data) = line.strip_prefix("data: ") {
                    let data = data.trim();

                    if data == "[DONE]" {
                        return;
                    }

                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                        // OpenAI / OpenRouter format
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            if !content.is_empty() {
                                yield content.to_string();
                            }
                        }
                    }
                }
            }
        }
    };

    Box::pin(stream)
}

/// Parse an SSE stream (Anthropic-style) into text chunks.
///
/// Anthropic uses event types like:
/// ```text
/// event: content_block_delta
/// data: {"type":"content_block_delta","delta":{"type":"text_delta","text":"hello"}}
/// ```
pub fn parse_anthropic_sse(
    byte_stream: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
) -> Pin<Box<dyn Stream<Item = Result<String>> + Send>> {
    let stream = async_stream::try_stream! {
        let mut buffer = String::new();
        let mut current_event = String::new();

        tokio::pin!(byte_stream);

        while let Some(chunk) = byte_stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);
            buffer.push_str(&text);

            while let Some(newline_pos) = buffer.find('\n') {
                let line = buffer[..newline_pos].trim().to_string();
                buffer = buffer[newline_pos + 1..].to_string();

                if line.is_empty() {
                    current_event.clear();
                    continue;
                }

                if let Some(event_type) = line.strip_prefix("event: ") {
                    current_event = event_type.trim().to_string();
                    continue;
                }

                if let Some(data) = line.strip_prefix("data: ") {
                    if current_event == "content_block_delta" {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                            if let Some(text) = json["delta"]["text"].as_str() {
                                if !text.is_empty() {
                                    yield text.to_string();
                                }
                            }
                        }
                    } else if current_event == "message_stop" {
                        return;
                    }
                }
            }
        }
    };

    Box::pin(stream)
}
