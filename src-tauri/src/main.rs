#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod runner;

use algomentor::{
    analyzer::{code_reader, complexity},
    config::settings::AppConfig,
    llm::{
        prompts,
        provider::{create_provider, CompletionOptions, Message},
    },
    memory::{database::Database, history::ChatHistory},
    task::{discovery, parser},
};
use futures_util::StreamExt;
use runner::PidStore;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::{Path, PathBuf}, sync::Arc};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, State,
};
use tauri_plugin_dialog::{DialogExt, FilePath};
use tokio::sync::Mutex;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════
//  Shared data types
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskInfo {
    pub name: String,
    pub title: String,
    pub category: Option<String>,
    pub difficulty: Option<String>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfigInfo {
    pub provider: String,
    pub model: String,
    pub level: String,
    pub language: String,
    pub programming_language: String,
    pub has_api_key: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct HistoryMsg {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub extension: Option<String>,
    pub monaco_language: String,
}

// ═══════════════════════════════════════════════════════════════
//  App state
// ═══════════════════════════════════════════════════════════════

pub struct AppState {
    pub workspace:    Mutex<Option<PathBuf>>,
    pub current_task: Mutex<Option<PathBuf>>,
    pub conversation: Mutex<HashMap<String, Vec<StoredMessage>>>,
    pub session_id:   String,
    /// PID of the currently running code process (None = idle)
    pub runner_pid:   PidStore,
}

impl AppState {
    fn new() -> Self {
        Self {
            workspace:    Mutex::new(None),
            current_task: Mutex::new(None),
            conversation: Mutex::new(HashMap::new()),
            session_id:   Uuid::new_v4().to_string(),
            runner_pid:   Arc::new(std::sync::Mutex::new(None)),
        }
    }
}

// ═══════════════════════════════════════════════════════════════
//  Helpers
// ═══════════════════════════════════════════════════════════════

fn to_lib_message(m: &StoredMessage) -> Message {
    match m.role.as_str() {
        "user"      => Message::user(&m.content),
        "assistant" => Message::assistant(&m.content),
        _           => Message::system(&m.content),
    }
}

async fn require_workspace(state: &AppState) -> Result<PathBuf, String> {
    state.workspace.lock().await
        .clone()
        .ok_or_else(|| "No workspace selected.".to_string())
}

fn last_workspace_file() -> Option<PathBuf> {
    dirs::data_local_dir().map(|d| d.join("algomentor-gui").join("last_workspace.txt"))
}

fn load_last_workspace() -> Option<PathBuf> {
    let p = last_workspace_file()?;
    let s = std::fs::read_to_string(p).ok()?;
    let pb = PathBuf::from(s.trim());
    if pb.exists() { Some(pb) } else { None }
}

fn save_last_workspace(path: &Path) {
    if let Some(f) = last_workspace_file() {
        if let Some(dir) = f.parent() { let _ = std::fs::create_dir_all(dir); }
        let _ = std::fs::write(f, path.to_string_lossy().as_bytes());
    }
}

fn task_key_from_dir(dir: &Option<PathBuf>) -> String {
    dir.as_ref()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("general")
        .to_string()
}

// ── Find the "main" solution file in a task directory ────────
fn find_solution_file(task_dir: &Path) -> Option<PathBuf> {
    let candidates = ["solution.py","solution.js","solution.ts","solution.rs",
                      "solution.cpp","Solution.java","solution.go","solution.c",
                      "main.py","main.js","main.ts","main.rs","main.go"];
    for c in &candidates {
        let p = task_dir.join(c);
        if p.exists() { return Some(p); }
    }
    // fallback: first non-hidden, non-.md, non-.completed file
    if let Ok(rd) = std::fs::read_dir(task_dir) {
        for entry in rd.flatten() {
            let path = entry.path();
            if path.is_file() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !name.starts_with('.') && !name.ends_with(".md") {
                    return Some(path);
                }
            }
        }
    }
    None
}

// ── Streaming LLM helper ──────────────────────────────────────
async fn stream_prompt(
    app:           &AppHandle,
    state:         &AppState,
    prompt:        String,
    history_label: &str,
) -> Result<(), String> {
    let workspace = require_workspace(state).await?;
    let config    = AppConfig::load(&workspace).unwrap_or_default();

    let system   = prompts::get_mentor_system_prompt(&config.general.level, &config.general.language);
    let task_ctx = {
        let t = state.current_task.lock().await;
        t.as_ref().and_then(|dir| {
            parser::parse_task_file(&dir.join("task.md")).ok()
                .map(|d| format!(
                    "User is working on: \"{}\"\n\nProblem:\n{}",
                    d.title, d.body
                ))
        })
    };

    let mut messages = vec![Message::system(&system)];
    if let Some(ctx) = task_ctx { messages.push(Message::system(&ctx)); }

    let key = task_key_from_dir(&*state.current_task.lock().await);
    {
        let conv = state.conversation.lock().await;
        if let Some(history) = conv.get(&key) {
            let start = history.len().saturating_sub(30);
            for m in &history[start..] { messages.push(to_lib_message(m)); }
        }
    }
    messages.push(Message::user(&prompt));

    let provider = create_provider(&config).map_err(|e| format!("Provider error: {e}"))?;
    let options  = CompletionOptions {
        model: config.active_model().to_string(),
        temperature: config.mentor.temperature,
        max_tokens: Some(4096),
        tools: None,
    };

    app.emit("mentor-start", ()).ok();

    let mut stream = provider.stream(&messages, &options).await
        .map_err(|e| format!("Stream error: {e}"))?;

    let mut full = String::new();
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(t)  => { app.emit("mentor-chunk", &t).ok(); full.push_str(&t); }
            Err(e) => { app.emit("mentor-error", e.to_string()).ok(); break; }
        }
    }
    app.emit("mentor-done", &full).ok();

    {
        let mut conv = state.conversation.lock().await;
        let v = conv.entry(key.clone()).or_default();
        v.push(StoredMessage { role: "user".into(),      content: history_label.to_string() });
        v.push(StoredMessage { role: "assistant".into(), content: full.clone() });
    }

    let db_path = AppConfig::db_path(&workspace);
    if let Ok(db) = Database::open(&db_path) {
        let hist = ChatHistory::new(&db);
        let t_id = if key == "general" { None } else { Some(key.as_str()) };
        let _ = hist.save_message(t_id, &state.session_id, "user",      history_label);
        let _ = hist.save_message(t_id, &state.session_id, "assistant", &full);
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════
//  Commands — Workspace / Tasks
// ═══════════════════════════════════════════════════════════════

#[tauri::command]
async fn pick_directory(app: AppHandle) -> Result<Option<String>, String> {
    let (tx, rx) = tokio::sync::oneshot::channel::<Option<FilePath>>();
    app.dialog().file().set_title("Select AlgoMentor Workspace")
        .pick_folder(move |p| { let _ = tx.send(p); });
    let result = rx.await.map_err(|e| e.to_string())?;
    Ok(result.and_then(|fp| {
        if let FilePath::Path(p) = fp { Some(p.to_string_lossy().to_string()) } else { None }
    }))
}

#[tauri::command]
async fn set_workspace(path: String, state: State<'_, AppState>) -> Result<(), String> {
    let pb = PathBuf::from(&path);
    if !AppConfig::is_initialized(&pb) {
        AppConfig::default().save(&pb).map_err(|e| e.to_string())?;
        Database::open(&AppConfig::db_path(&pb)).map_err(|e| e.to_string())?;
    }
    save_last_workspace(&pb);
    *state.workspace.lock().await    = Some(pb);
    *state.current_task.lock().await = None;
    state.conversation.lock().await.clear();
    Ok(())
}

#[tauri::command]
async fn get_workspace(state: State<'_, AppState>) -> Result<Option<String>, String> {
    Ok(state.workspace.lock().await.as_ref().map(|p| p.to_string_lossy().to_string()))
}

#[tauri::command]
async fn get_last_workspace() -> Result<Option<String>, String> {
    Ok(load_last_workspace().map(|p| p.to_string_lossy().to_string()))
}

#[tauri::command]
async fn get_tasks(state: State<'_, AppState>) -> Result<Vec<TaskInfo>, String> {
    let ws    = require_workspace(&state).await?;
    let tasks = discovery::discover_tasks(&ws, 3).map_err(|e| e.to_string())?;
    Ok(tasks.into_iter().map(|t| {
        let title      = t.description.as_ref().map(|d| d.title.clone()).unwrap_or_else(|| t.name.clone());
        let category   = t.description.as_ref().and_then(|d| d.category.clone());
        let difficulty = t.description.as_ref().and_then(|d| d.difficulty.as_ref()).map(|d| d.to_string());
        let completed  = t.directory.join(".completed").exists();
        TaskInfo { name: t.name, title, category, difficulty, completed }
    }).collect())
}

#[tauri::command]
async fn open_task(task_name: String, state: State<'_, AppState>) -> Result<Option<String>, String> {
    let ws  = require_workspace(&state).await?;
    let dir = discovery::resolve_task_dir(&ws, &task_name)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Task '{}' not found", task_name))?;

    let sol = find_solution_file(&dir).map(|p| p.to_string_lossy().to_string());
    *state.current_task.lock().await = Some(dir);
    Ok(sol) // returns path to solution file or None
}

#[tauri::command]
async fn clear_task(state: State<'_, AppState>) -> Result<(), String> {
    *state.current_task.lock().await = None; Ok(())
}

#[tauri::command]
async fn get_current_task(state: State<'_, AppState>) -> Result<Option<String>, String> {
    let key = task_key_from_dir(&*state.current_task.lock().await);
    if key == "general" { Ok(None) } else { Ok(Some(key)) }
}

#[tauri::command]
async fn add_task(
    name: String, category: Option<String>, difficulty: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let ws     = require_workspace(&state).await?;
    let config = AppConfig::load(&ws).unwrap_or_default();
    let mut target = ws.clone();
    if let Some(ref c) = category { target.push(c); }
    target.push(&name);
    if target.exists() { return Err(format!("Task '{}' already exists", name)); }
    std::fs::create_dir_all(&target).map_err(|e| e.to_string())?;
    let diff = difficulty.as_deref().unwrap_or("Medium");
    let cat  = category.as_deref().unwrap_or("");
    std::fs::write(target.join("task.md"),
        format!("---\ntitle: {name}\ndifficulty: {diff}\ncategory: {cat}\n---\n\n# {name}\n\nDescribe the problem here...\n\n## Examples\n\n## Constraints\n")
    ).map_err(|e| e.to_string())?;
    let ext = match config.general.programming_language.as_str() {
        "python" => "py", "rust" => "rs", "typescript"|"ts" => "ts",
        "javascript"|"js" => "js", "cpp"|"c++" => "cpp",
        "java" => "java", "go" => "go", _ => "py",
    };
    let sol_path = target.join(format!("solution.{ext}"));
    std::fs::write(&sol_path, "").map_err(|e| e.to_string())?;
    Ok(sol_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn import_leetcode(
    url: String, category: Option<String>, state: State<'_, AppState>,
) -> Result<String, String> {
    let url_no_query = url.split('?').next().unwrap_or(&url);
    let title_slug = url_no_query.trim_end_matches('/')
        .split('/')
        .last()
        .ok_or("Invalid LeetCode URL")?;
        
    let problem = algomentor::tools::leetcode::fetch_problem(title_slug).await?;
    
    let ws = require_workspace(&state).await?;
    let config = AppConfig::load(&ws).unwrap_or_default();
    
    let mut target = ws.clone();
    if let Some(ref c) = category {
        if !c.is_empty() { target.push(c); }
    }
    target.push(title_slug);
    if target.exists() { return Err(format!("Task '{}' already exists", title_slug)); }
    std::fs::create_dir_all(&target).map_err(|e| e.to_string())?;
    
    let clean_content = problem.content
        .replace("<p>", "").replace("</p>", "\n\n")
        .replace("<strong>", "**").replace("</strong>", "**")
        .replace("<em>", "*").replace("</em>", "*")
        .replace("<ul>", "").replace("</ul>", "")
        .replace("<li>", "- ").replace("</li>", "\n")
        .replace("<pre>", "\n```\n").replace("</pre>", "\n```\n")
        .replace("<code>", "`").replace("</code>", "`")
        .replace("&nbsp;", " ")
        .replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&")
        .replace("<sup>", "^").replace("</sup>", "");
        
    let diff = problem.difficulty;
    let title = problem.title;
    let cat = category.as_deref().unwrap_or("");
    
    let task_md = format!("---\ntitle: {title}\ndifficulty: {diff}\ncategory: {cat}\n---\n\n# {title}\n\n{clean_content}");
    std::fs::write(target.join("task.md"), task_md).map_err(|e| e.to_string())?;
    
    let ext = match config.general.programming_language.as_str() {
        "python" => "py", "rust" => "rs", "typescript"|"ts" => "ts",
        "javascript"|"js" => "js", "cpp"|"c++" => "cpp",
        "java" => "java", "go" => "go", _ => "py",
    };
    
    let lc_lang = match ext {
        "py" => "python3", "rs" => "rust", "ts" => "typescript",
        "js" => "javascript", "cpp" => "cpp", "java" => "java",
        "go" => "golang", _ => "python3"
    };
    
    let starter_code = problem.code_snippets
        .and_then(|snips| snips.into_iter().find(|s| s.lang_slug == lc_lang))
        .map(|s| s.code)
        .unwrap_or_default();
        
    let sol_path = target.join(format!("solution.{ext}"));
    std::fs::write(&sol_path, starter_code).map_err(|e| e.to_string())?;
    Ok(sol_path.to_string_lossy().to_string())
}

// ═══════════════════════════════════════════════════════════════
//  Commands — File I/O
// ═══════════════════════════════════════════════════════════════

#[tauri::command]
async fn read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| format!("Cannot read '{path}': {e}"))
}

#[tauri::command]
async fn write_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| format!("Cannot write '{path}': {e}"))
}

#[tauri::command]
async fn list_task_files(state: State<'_, AppState>) -> Result<Vec<FileEntry>, String> {
    let t = state.current_task.lock().await;
    let dir = t.as_ref().ok_or("No task selected")?;
    let mut entries = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let path  = e.path();
            let name  = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            if name.starts_with('.') { continue; }
            let ext   = path.extension().and_then(|x| x.to_str()).unwrap_or("").to_string();
            let lang  = runner::monaco_language(&ext).to_string();
            let is_dir = path.is_dir();
            entries.push(FileEntry {
                path: path.to_string_lossy().to_string(),
                name, is_dir,
                extension: if ext.is_empty() { None } else { Some(ext) },
                monaco_language: lang,
            });
        }
    }
    entries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(entries)
}

#[tauri::command]
async fn get_task_description(state: State<'_, AppState>) -> Result<String, String> {
    let t = state.current_task.lock().await;
    let dir = t.as_ref().ok_or("No task selected")?;
    std::fs::read_to_string(dir.join("task.md"))
        .map_err(|e| format!("Cannot read task.md: {e}"))
}

#[tauri::command]
async fn save_task_description(content: String, state: State<'_, AppState>) -> Result<(), String> {
    let t = state.current_task.lock().await;
    let dir = t.as_ref().ok_or("No task selected")?;
    std::fs::write(dir.join("task.md"), content)
        .map_err(|e| format!("Cannot write task.md: {e}"))
}

#[tauri::command]
async fn get_monaco_language(path: String) -> Result<String, String> {
    let ext = PathBuf::from(&path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    Ok(runner::monaco_language(&ext).to_string())
}

// ═══════════════════════════════════════════════════════════════
//  Commands — Code execution
// ═══════════════════════════════════════════════════════════════

#[tauri::command]
async fn run_code(
    file:     String,
    task_dir: String,
    app:      AppHandle,
    state:    State<'_, AppState>,
) -> Result<(), String> {
    // Kill any previous process first
    let pid = Arc::clone(&state.runner_pid);
    runner::kill_process(Arc::clone(&pid)).await;

    let file_path = PathBuf::from(&file);
    let dir_path  = PathBuf::from(&task_dir);

    // Spawn in background — returns immediately, events carry output
    tokio::spawn(async move {
        if let Err(e) = runner::run_file(app.clone(), pid, file_path, dir_path).await {
            app.emit("code-error", e).ok();
            app.emit("code-done", serde_json::json!({
                "exitCode": -1, "durationMs": 0, "success": false
            })).ok();
        }
    });

    Ok(())
}

#[tauri::command]
async fn stop_code(state: State<'_, AppState>) -> Result<(), String> {
    runner::kill_process(Arc::clone(&state.runner_pid)).await;
    Ok(())
}

// ═══════════════════════════════════════════════════════════════
//  Commands — Config
// ═══════════════════════════════════════════════════════════════

#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<ConfigInfo, String> {
    let ws   = require_workspace(&state).await?;
    let cfg  = AppConfig::load(&ws).unwrap_or_default();
    let s    = cfg.active_provider_settings();
    Ok(ConfigInfo {
        provider:             cfg.provider.active.clone(),
        model:                cfg.active_model().to_string(),
        level:                cfg.general.level.clone(),
        language:             cfg.general.language.clone(),
        programming_language: cfg.general.programming_language.clone(),
        has_api_key: !s.api_key.is_empty() || matches!(cfg.provider.active.as_str(), "ollama"|"lmstudio"),
    })
}

#[tauri::command]
async fn save_config(
    provider: String, model: String, level: String,
    language: String, programming_language: String,
    api_key: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let ws = require_workspace(&state).await?;
    let mut cfg = AppConfig::load(&ws).unwrap_or_default();
    cfg.provider.active = provider.clone();
    cfg.set_model(&model);
    cfg.general.level                = level;
    cfg.general.language             = language;
    cfg.general.programming_language = programming_language;
    cfg.save(&ws).map_err(|e| e.to_string())?;
    if let Some(k) = api_key { if !k.is_empty() {
        AppConfig::save_api_key_global(&provider, &k).map_err(|e| e.to_string())?;
    }}
    Ok(())
}

// ═══════════════════════════════════════════════════════════════
//  Commands — LLM mentor actions
// ═══════════════════════════════════════════════════════════════

#[tauri::command]
async fn send_chat(msg: String, app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    stream_prompt(&app, &state, msg.clone(), &msg).await
}

#[tauri::command]
async fn run_hint(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let (code, lang, body) = {
        let t   = state.current_task.lock().await;
        let dir = t.as_ref().ok_or("No task selected")?;
        let cf  = code_reader::read_solution(dir).map_err(|e| e.to_string())?
                      .ok_or("No solution file")?;
        let b   = parser::parse_task_file(&dir.join("task.md")).ok().map(|d| d.body);
        (cf.content, cf.language.to_string(), b)
    };
    let prompt = prompts::get_hint_prompt(&code, &lang, body.as_deref());
    stream_prompt(&app, &state, prompt, "[Hint requested]").await
}

#[tauri::command]
async fn run_explain(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let ws = require_workspace(&state).await?;
    let prog_lang = AppConfig::load(&ws).unwrap_or_default().general.programming_language;
    let (body, code_pair) = {
        let t = state.current_task.lock().await;
        if let Some(dir) = t.as_ref() {
            let b = parser::parse_task_file(&dir.join("task.md")).ok().map(|d| d.body);
            let c = code_reader::read_solution(dir).ok().flatten()
                        .map(|cf| (cf.content, cf.language.to_string()));
            (b, c)
        } else { (None, None) }
    };
    let code_ref = code_pair.as_ref().map(|(c, l)| (c.as_str(), l.as_str()));
    let prompt   = prompts::get_explain_prompt(body.as_deref(), code_ref, &prog_lang);
    stream_prompt(&app, &state, prompt, "[Explanation requested]").await
}

#[tauri::command]
async fn run_complexity(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let (code, lang, body) = {
        let t   = state.current_task.lock().await;
        let dir = t.as_ref().ok_or("No task selected")?;
        let cf  = code_reader::read_solution(dir).map_err(|e| e.to_string())?
                      .ok_or("No solution file")?;
        let b   = parser::parse_task_file(&dir.join("task.md")).ok().map(|d| d.body);
        (cf.content, cf.language, b)
    };
    let prompt = complexity::build_complexity_prompt(&code, &lang, body.as_deref());
    stream_prompt(&app, &state, prompt, "[Complexity analysis]").await
}

#[tauri::command]
async fn run_solution(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let body = {
        let t = state.current_task.lock().await;
        t.as_ref().and_then(|dir| {
            parser::parse_task_file(&dir.join("task.md")).ok().map(|d| d.body)
        })
    };
    let prompt = match body {
        Some(b) => format!("Explain the general algorithm approach for this (no code):\n\n{b}"),
        None    => "Explain the general approach for this problem. No code, just concepts.".into(),
    };
    stream_prompt(&app, &state, prompt, "[Approach discussion]").await
}

#[tauri::command]
async fn mark_done(state: State<'_, AppState>) -> Result<(), String> {
    let t = state.current_task.lock().await;
    if let Some(dir) = t.as_ref() {
        std::fs::write(dir.join(".completed"), chrono::Utc::now().to_rfc3339())
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn run_watch(code: String, app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let ws = require_workspace(&state).await?;
    let config = AppConfig::load(&ws).unwrap_or_default();
    let prog_lang = config.general.programming_language.clone();
    
    let body = {
        let t = state.current_task.lock().await;
        t.as_ref().and_then(|dir| {
            parser::parse_task_file(&dir.join("task.md")).ok().map(|d| d.body)
        })
    };
    
    let prompt = prompts::get_watch_analysis_prompt(&code, &prog_lang, body.as_deref());
    let system = prompts::get_mentor_system_prompt(&config.general.level, &config.general.language);
    
    let messages = vec![
        Message::system(&system),
        Message::user(&prompt),
    ];
    
    let provider = create_provider(&config).map_err(|e| format!("Provider error: {e}"))?;
    let options = CompletionOptions {
        model: config.active_model().to_string(),
        temperature: config.mentor.temperature,
        max_tokens: Some(500),
        tools: None,
    };
    
    app.emit("watch-start", ()).ok();
    
    let mut stream = provider.stream(&messages, &options).await
        .map_err(|e| format!("Stream error: {e}"))?;
        
    let mut full = String::new();
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(t) => { app.emit("watch-chunk", &t).ok(); full.push_str(&t); }
            Err(e) => { app.emit("watch-error", e.to_string()).ok(); break; }
        }
    }
    app.emit("watch-done", &full).ok();
    
    // Does NOT save to chat history to prevent spam
    Ok(())
}


#[tauri::command]
async fn get_history(state: State<'_, AppState>) -> Result<Vec<HistoryMsg>, String> {
    let ws  = require_workspace(&state).await?;
    let key = task_key_from_dir(&*state.current_task.lock().await);
    let db  = Database::open(&AppConfig::db_path(&ws)).map_err(|e| e.to_string())?;
    let h   = ChatHistory::new(&db);
    let t   = if key == "general" { None } else { Some(key.as_str()) };
    let sessions = h.get_sessions(t).map_err(|e| e.to_string())?;
    let mut out  = Vec::new();
    for sid in sessions.iter().rev().take(1) {
        for m in h.get_messages(t, sid, 100).map_err(|e| e.to_string())? {
            out.push(HistoryMsg { role: m.role, content: m.content });
        }
    }
    Ok(out)
}

#[tauri::command]
async fn set_always_on_top(value: bool, window: tauri::WebviewWindow) -> Result<(), String> {
    window.set_always_on_top(value).map_err(|e| e.to_string())
}

// ═══════════════════════════════════════════════════════════════
//  Main
// ═══════════════════════════════════════════════════════════════

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::new())
        .setup(|app| {
            let show_i = MenuItem::with_id(app, "show", "Show AlgoMentor", true, None::<&str>)?;
            let sep    = PredefinedMenuItem::separator(app)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu   = Menu::with_items(app, &[&show_i, &sep, &quit_i])?;

            let icon_rgba: Vec<u8> = std::iter::repeat([0x58u8, 0xA6, 0xFF, 0xFF])
                .take(16 * 16).flatten().collect();

            TrayIconBuilder::with_id("main-tray")
                .icon(tauri::image::Image::new_owned(icon_rgba, 16, 16))
                .tooltip("AlgoMentor — AI Coding Mentor")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show(); let _ = w.set_focus(); let _ = w.unminimize();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up, ..
                    } = event {
                        if let Some(w) = tray.app_handle().get_webview_window("main") {
                            if w.is_visible().unwrap_or(false) { let _ = w.hide(); }
                            else { let _ = w.show(); let _ = w.set_focus(); }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // workspace
            pick_directory, set_workspace, get_workspace, get_last_workspace,
            // tasks
            get_tasks, open_task, clear_task, get_current_task, add_task, import_leetcode,
            // file i/o
            read_file, write_file, list_task_files,
            get_task_description, save_task_description, get_monaco_language,
            // runner
            run_code, stop_code,
            // config
            get_config, save_config,
            // mentor
            send_chat, run_hint, run_explain, run_complexity, run_solution,
            mark_done, get_history, run_watch,
            // window
            set_always_on_top,
        ])
        .build(tauri::generate_context!())
        .expect("Failed to build AlgoMentor")
        .run(|app, event| {
            if let tauri::RunEvent::WindowEvent {
                label,
                event: tauri::WindowEvent::CloseRequested { api, .. }, ..
            } = event {
                if label == "main" {
                    api.prevent_close();
                    if let Some(w) = app.get_webview_window("main") { let _ = w.hide(); }
                }
            }
        });
}
