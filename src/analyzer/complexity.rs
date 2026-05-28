use super::language::Language;

/// Result of complexity analysis
#[derive(Debug, Clone)]
pub struct ComplexityResult {
    pub time_best: String,
    pub time_average: String,
    pub time_worst: String,
    pub space: String,
    pub explanation: String,
    pub is_optimal: bool,
    pub suggestion: String,
}

/// Build the complexity analysis prompt (to send to LLM)
pub fn build_complexity_prompt(code: &str, language: &Language, task_description: Option<&str>) -> String {
    crate::llm::prompts::get_complexity_prompt(code, &language.to_string(), task_description)
}

/// Format a complexity report for terminal display
pub fn format_complexity_report(raw_response: &str) -> String {
    // The raw LLM response is already formatted as markdown.
    // We just wrap it with a header.
    format!(
        "\n{}\n{}\n{}\n",
        "═".repeat(50),
        raw_response.trim(),
        "═".repeat(50),
    )
}
