use serde::Deserialize;
use reqwest::Client;

#[derive(Deserialize, Debug)]
pub struct LeetCodeSnippet {
    pub lang: String,
    #[serde(rename = "langSlug")]
    pub lang_slug: String,
    pub code: String,
}

#[derive(Deserialize, Debug)]
pub struct LeetCodeQuestion {
    pub title: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    pub content: String,
    pub difficulty: String,
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Option<Vec<LeetCodeSnippet>>,
}

#[derive(Deserialize, Debug)]
struct LeetCodeData {
    question: Option<LeetCodeQuestion>,
}

#[derive(Deserialize, Debug)]
struct LeetCodeResponse {
    data: Option<LeetCodeData>,
}

pub async fn fetch_problem(title_slug: &str) -> Result<LeetCodeQuestion, String> {
    let client = Client::new();
    let query = r#"
        query questionData($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                title
                titleSlug
                content
                difficulty
                codeSnippets {
                    lang
                    langSlug
                    code
                }
            }
        }
    "#;
    
    let res = client.post("https://leetcode.com/graphql")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .header("Content-Type", "application/json")
        .header("Referer", format!("https://leetcode.com/problems/{}/", title_slug))
        .json(&serde_json::json!({
            "query": query,
            "variables": { "titleSlug": title_slug },
            "operationName": "questionData"
        }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
        
    let lc_res: LeetCodeResponse = res.json().await.map_err(|e| format!("Parse error: {}", e))?;
    
    lc_res.data
        .and_then(|d| d.question)
        .ok_or_else(|| format!("Question '{}' not found on LeetCode", title_slug))
}
