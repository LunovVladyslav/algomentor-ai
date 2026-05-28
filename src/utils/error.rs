/// Parse a raw JSON error string into a user-friendly message
pub fn parse_api_error(raw: &str) -> String {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(raw) {
        if let Some(msg) = json.pointer("/error/message").and_then(|v| v.as_str()) {
            return msg.to_string();
        }
        if let Some(msg) = json.pointer("/error").and_then(|v| v.as_str()) {
            return msg.to_string();
        }
        if let Some(msg) = json.pointer("/message").and_then(|v| v.as_str()) {
            return msg.to_string();
        }
    }
    
    // Truncate if it's too long
    if raw.len() > 300 {
        format!("{}...", &raw[..300])
    } else {
        raw.to_string()
    }
}
