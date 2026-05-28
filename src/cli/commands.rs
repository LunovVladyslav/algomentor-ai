use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::path::Path;

use algomentor::analyzer::{code_reader, complexity, highlighter::Highlighter, Language};
use algomentor::chat::session::ChatSession;
use algomentor::config::settings::{AppConfig, CONFIG_DIR};
use algomentor::dashboard::app::DashboardApp;
use algomentor::llm::embedding;
use algomentor::llm::provider::{create_provider, CompletionOptions, Message};
use algomentor::memory::database::Database;
use algomentor::memory::history::ChatHistory;
use algomentor::memory::rag::RagSystem;
use algomentor::task::{discovery, parser};
use algomentor::watcher::handler::FileWatcher;

use super::{ConfigAction, KnowledgeAction};

/// Initialize AlgoMentor in the project directory
pub async fn run_init(project_dir: &Path) -> Result<()> {
    if AppConfig::is_initialized(project_dir) {
        println!("{} AlgoMentor is already initialized in this directory.", "ℹ️".yellow());
        println!("  Config: {}", project_dir.join(CONFIG_DIR).display().to_string().dimmed());
        return Ok(());
    }

    println!("{}", "🧠 Initializing AlgoMentor...".cyan().bold());

    let mut config = AppConfig::default();

    // Interactive Onboarding
    let theme = ColorfulTheme::default();
    
    let langs = &["ru (Russian)", "en (English)", "auto"];
    let lang_sel = Select::with_theme(&theme)
        .with_prompt("What language should the mentor speak?")
        .default(0)
        .items(&langs[..])
        .interact()?;
    config.general.language = langs[lang_sel].split_whitespace().next().unwrap().to_string();

    let prog_lang: String = Input::with_theme(&theme)
        .with_prompt("What programming language will you practice? (e.g., python, rust, cpp)")
        .default("python".into())
        .interact_text()?;
    config.general.programming_language = prog_lang;

    let providers = &["openai", "anthropic", "openrouter", "ollama", "lmstudio"];
    let prov_sel = Select::with_theme(&theme)
        .with_prompt("Which LLM provider do you want to use?")
        .default(0)
        .items(&providers[..])
        .interact()?;
    config.provider.active = providers[prov_sel].to_string();

    if ["openai", "anthropic", "openrouter"].contains(&providers[prov_sel]) {
        let api_key: String = Input::with_theme(&theme)
            .with_prompt(&format!("Enter your API key for {} (or press enter to skip and set later)", providers[prov_sel]))
            .allow_empty(true)
            .interact_text()?;
            
        if !api_key.is_empty() {
            AppConfig::save_api_key_global(providers[prov_sel], &api_key)?;
            println!("{} API key saved globally.", "✅".green());
        }
    }

    config.save(project_dir)?;
    println!("  {} Created {}/{}", "✅".green(), CONFIG_DIR, "config.toml");

    let db_path = AppConfig::db_path(project_dir);
    let _db = Database::open(&db_path)?;
    println!("  {} Created {}/{}", "✅".green(), CONFIG_DIR, "algomentor.db");

    println!();
    println!("{}", "Setup complete! Important Next Steps:".green().bold());
    println!("{}", "═".repeat(50).cyan());
    
    if config.provider.active == "ollama" {
        println!("  {} You chose Ollama! You need an embedding model for the Knowledge Base.", "⚠️".yellow());
        println!("     Please run this in a separate terminal:");
        println!("     {} ollama pull nomic-embed-text", "$".dimmed());
        println!();
    }

    println!("  1. Load the default knowledge base (Highly Recommended):");
    println!("     {} algomentor knowledge ingest assets/knowledge_base", "$".dimmed());
    println!();
    println!("  2. Add your first algorithm task:");
    println!("     {} algomentor add two-sum --category arrays", "$".dimmed());
    println!();
    println!("  3. Start chatting and coding:");
    println!("     {} algomentor chat two-sum", "$".dimmed());
    println!("{}", "═".repeat(50).cyan());
    println!("Type {} to see a full tutorial at any time.", "algomentor guide".cyan().bold());

    Ok(())
}

/// Show the interactive guide and help
pub async fn run_guide() -> Result<()> {
    use algomentor::assets::Assets;
    
    if let Some(guide_file) = Assets::get("guide.md") {
        let content = std::str::from_utf8(guide_file.data.as_ref())?;
        println!("{}", algomentor::utils::markdown::render_markdown(content));
    } else {
        println!("Guide not found!");
    }
    
    Ok(())
}

/// Add a new task directory and template
pub async fn run_add(
    project_dir: &Path,
    name: &str,
    category: Option<&str>,
    difficulty: Option<&str>,
) -> Result<()> {
    ensure_initialized(project_dir)?;
    let config = AppConfig::load(project_dir)?;

    let mut target_dir = project_dir.to_path_buf();
    if let Some(cat) = category {
        target_dir.push(cat);
    }
    target_dir.push(name);

    if target_dir.exists() {
        anyhow::bail!("Directory '{}' already exists.", target_dir.display());
    }

    std::fs::create_dir_all(&target_dir)?;

    // Create task.md
    let task_md_path = target_dir.join("task.md");
    let diff = difficulty.unwrap_or("Medium");
    let cat = category.unwrap_or("");
    
    let template = format!(
        "---\ntitle: {}\ndifficulty: {}\ncategory: {}\n---\n\nWrite the problem description here...\n",
        name, diff, cat
    );
    std::fs::write(&task_md_path, template)?;

    // Create empty solution file
    let ext = match config.general.programming_language.as_str() {
        "python" => "py",
        "rust" => "rs",
        "typescript" | "ts" => "ts",
        "javascript" | "js" => "js",
        "cpp" | "c++" => "cpp",
        "java" => "java",
        "go" => "go",
        _ => "txt", // fallback
    };
    let sol_file_path = target_dir.join(format!("solution.{}", ext));
    std::fs::write(&sol_file_path, "")?;

    println!("{} Created task '{}'", "✅".green(), name.cyan().bold());
    println!("  📁 Directory: {}", target_dir.display().to_string().dimmed());
    println!("  📄 Files: task.md, solution.{}", ext);
    println!();
    println!("To start practicing:");
    println!("  {} algomentor chat {}", "$".dimmed(), name);
    
    Ok(())
}

/// Start chat mode
pub async fn run_chat(project_dir: &Path, task_name: Option<&str>) -> Result<()> {
    ensure_initialized(project_dir)?;

    let config = AppConfig::load(project_dir)?;
    let db = Database::open(&AppConfig::db_path(project_dir))?;
    let provider = create_provider(&config)
        .context("Failed to create LLM provider. Run `algomentor config provider <name>` and set your API key.")?;

    let task_dir = if let Some(name) = task_name {
        match discovery::resolve_task_dir(project_dir, name)? {
            Some(dir) => Some(dir),
            None => {
                eprintln!("{} Task '{}' not found.", "❌".red(), name);
                eprintln!("  Available tasks:");
                list_available_tasks(project_dir)?;
                return Ok(());
            }
        }
    } else if project_dir.join("task.md").exists() {
        Some(project_dir.to_path_buf())
    } else {
        None
    };

    if let Some(ref dir) = task_dir {
        register_task(&db, dir)?;
    }

    let mut session = ChatSession::new(provider, config, db, task_dir)?;
    session.start().await
}

/// Start a project session with MCP support
pub async fn run_project(project_dir: &Path) -> Result<()> {
    ensure_initialized(project_dir)?;
    
    let config_dir = project_dir.join(CONFIG_DIR);
    let config = AppConfig::load(&config_dir.join("config.toml")).unwrap_or_default();
    
    println!("{}", "🚀 Starting Project Mode...".green().bold());

    let mut mcp_clients = Vec::new();
    let mut all_tools = Vec::new();

    // Start MCP servers
    for (name, srv_config) in &config.mcp {
        println!("{} Starting MCP server: {}", "🔌".cyan(), name);
        match algomentor::mcp::client::McpClient::start(&srv_config.command, &srv_config.args, &srv_config.env).await {
            Ok(mut client) => {
                match client.list_tools().await {
                    Ok(tools) => {
                        println!("  {} Loaded {} tools", "✓".green(), tools.len());
                        all_tools.extend(tools);
                        mcp_clients.push((name.clone(), client));
                    }
                    Err(e) => {
                        eprintln!("  {} Failed to list tools: {}", "❌".red(), e);
                    }
                }
            }
            Err(e) => {
                eprintln!("  {} Failed to start {}: {}", "❌".red(), name, e);
            }
        }
    }

    if all_tools.is_empty() {
        println!("No MCP tools loaded. Check your config.toml [mcp] settings.");
    }

    let db = Database::open(&config_dir.join("algomentor.db"))?;
    let provider = create_provider(&config)?;

    let mut session = ChatSession::new(provider, config, db, Some(project_dir.to_path_buf()))?;
    
    // Pass tools and clients to session
    session.set_mcp_tools(all_tools);
    session.set_mcp_clients(mcp_clients);

    session.start().await
}


/// Start watch mode
pub async fn run_watch(project_dir: &Path, task_name: Option<&str>) -> Result<()> {
    ensure_initialized(project_dir)?;

    let config = AppConfig::load(project_dir)?;
    let db = Database::open(&AppConfig::db_path(project_dir))?;
    let provider = create_provider(&config)?;

    let task_dir = if let Some(name) = task_name {
        discovery::resolve_task_dir(project_dir, name)?
            .ok_or_else(|| anyhow::anyhow!("Task '{}' not found", name))?
    } else if project_dir.join("task.md").exists() {
        project_dir.to_path_buf()
    } else {
        anyhow::bail!("Specify a task name or run from within a task directory.\nUsage: algomentor watch <task-name>");
    };

    let watcher = FileWatcher::new(task_dir, config);
    watcher.start(provider, db).await
}

/// Analyze a specific file
pub async fn run_analyze(project_dir: &Path, file: &Path) -> Result<()> {
    ensure_initialized(project_dir)?;

    let config = AppConfig::load(project_dir)?;
    let provider = create_provider(&config)?;

    let file_path = if file.is_absolute() {
        file.to_path_buf()
    } else {
        project_dir.join(file)
    };

    if !file_path.exists() {
        anyhow::bail!("File not found: {}", file_path.display());
    }

    let code_file = code_reader::read_file(&file_path)?;
    let highlighter = Highlighter::new();

    println!("{}", "📊 AlgoMentor — Code Analysis".cyan().bold());
    println!("{}", "═".repeat(50).cyan());

    println!("\n{} {} ({}, {} lines)",
        "📄".blue(),
        file_path.display().to_string().blue().bold(),
        code_file.language.display_name(),
        code_file.line_count
    );
    println!("{}", "─".repeat(50).dimmed());
    let highlighted = highlighter.highlight(&code_file.content, &code_file.language);
    print!("{}", highlighted);
    println!("{}", "─".repeat(50).dimmed());

    let task_dir = file_path.parent().unwrap_or(project_dir);
    let task_desc_path = task_dir.join("task.md");
    let task_body = if task_desc_path.exists() {
        parser::parse_task_file(&task_desc_path).ok().map(|d| d.body)
    } else {
        None
    };

    println!("\n{}", "Analyzing complexity...".yellow());
    let prompt = complexity::build_complexity_prompt(
        &code_file.content,
        &code_file.language,
        task_body.as_deref(),
    );

    let options = CompletionOptions {
        model: config.active_model().to_string(),
        temperature: 0.3,
        max_tokens: Some(2048),
        tools: None,
    };

    let messages = vec![
        Message::system("You are an expert algorithm analyst. Provide precise Big O complexity analysis."),
        Message::user(&prompt),
    ];

    match provider.complete(&messages, &options).await {
        Ok(response) => {
            println!("\n{}", "📊 Complexity Analysis".cyan().bold());
            println!("{}", "═".repeat(50).cyan());
            println!("{}", response.content);
            println!("{}", "═".repeat(50).cyan());
        }
        Err(e) => {
            eprintln!("{} Analysis failed: {}", "❌".red(), e);
        }
    }

    Ok(())
}

/// Show the TUI dashboard
pub async fn run_dashboard(project_dir: &Path) -> Result<()> {
    ensure_initialized(project_dir)?;

    let db = Database::open(&AppConfig::db_path(project_dir))?;
    let mut app = DashboardApp::new(&db, project_dir)?;
    app.run()
}

/// List all tasks
pub async fn run_tasks(project_dir: &Path) -> Result<()> {
    ensure_initialized(project_dir)?;

    let tasks = discovery::discover_tasks(project_dir, 3)?;

    if tasks.is_empty() {
        println!("{}", "No tasks found.".yellow());
        println!("Create a directory with a task.md file to get started.");
        return Ok(());
    }

    println!("\n{}", "📋 Tasks".cyan().bold());
    println!("{}", "═".repeat(60).cyan());

    for task in &tasks {
        let title = task
            .description
            .as_ref()
            .map(|d| d.title.as_str())
            .unwrap_or(&task.name);

        let difficulty = task
            .description
            .as_ref()
            .and_then(|d| d.difficulty.as_ref())
            .map(|d| format!(" {} {}", d.emoji(), d))
            .unwrap_or_default();

        let category = task
            .description
            .as_ref()
            .and_then(|d| d.category.as_ref())
            .map(|c| format!(" ({})", c))
            .unwrap_or_default();

        let solution_info = if task.solution_files.is_empty() {
            "  📝 No solution yet".yellow().to_string()
        } else {
            let langs: Vec<String> = task
                .solution_files
                .iter()
                .map(|f| Language::from_path(f).display_name().to_string())
                .collect();
            format!("  🔄 Solutions: {}", langs.join(", ")).green().to_string()
        };

        println!("  {} {}{}{}", "▸".cyan(), title.white().bold(), difficulty, category);
        println!("    {}", task.directory.display().to_string().dimmed());
        println!("    {}", solution_info);
        println!();
    }

    println!("{}", "═".repeat(60).cyan());
    println!("  Total: {} tasks", tasks.len().to_string().cyan().bold());
    println!();

    Ok(())
}

/// View chat history
pub async fn run_history(project_dir: &Path, task_name: Option<&str>) -> Result<()> {
    ensure_initialized(project_dir)?;

    let db = Database::open(&AppConfig::db_path(project_dir))?;
    let history = ChatHistory::new(&db);

    let task_id = task_name.map(|n| n.to_string());
    let sessions = history.get_sessions(task_id.as_deref())?;

    if sessions.is_empty() {
        println!("{}", "No chat history found.".yellow());
        return Ok(());
    }

    println!("\n{}", "📜 Chat History".cyan().bold());
    println!("{}", "═".repeat(60).cyan());

    for session_id in &sessions {
        let messages = history.get_messages(task_id.as_deref(), session_id, 100)?;
        if messages.is_empty() {
            continue;
        }

        println!("\n{} Session: {}", "📎".blue(), &session_id[..8.min(session_id.len())]);
        println!("{}", "─".repeat(50).dimmed());

        for msg in &messages {
            match msg.role.as_str() {
                "user" => println!("  {} {}", "You:".green().bold(), msg.content),
                "assistant" => {
                    let display = if msg.content.len() > 200 {
                        format!("{}...", &msg.content[..200])
                    } else {
                        msg.content.clone()
                    };
                    println!("  {} {}", "Mentor:".cyan().bold(), display);
                }
                _ => {}
            }
        }
    }

    println!("\n{}", "═".repeat(60).cyan());
    Ok(())
}

/// Manage configuration
pub async fn run_config(project_dir: &Path, action: Option<ConfigAction>) -> Result<()> {
    let action = action.unwrap_or(ConfigAction::Show);

    match action {
        ConfigAction::Show => {
            let config = AppConfig::load(project_dir).unwrap_or_default();
            println!("\n{}", "⚙️  AlgoMentor Configuration".cyan().bold());
            println!("{}", "═".repeat(50).cyan());
            println!("  Provider:    {}", config.provider.active.green().bold());
            println!("  Model:       {}", config.active_model().white());
            println!("  Level:       {}", config.general.level.yellow());
            println!("  Language:    {}", config.general.language.white());
            println!("  Temperature: {}", config.mentor.temperature.to_string().white());
            println!("  Debounce:    {}ms", config.watcher.debounce_ms);
            println!("{}", "═".repeat(50).cyan());

            let settings = config.active_provider_settings();
            let has_key = !settings.api_key.is_empty()
                || config.provider.active == "ollama"
                || config.provider.active == "lmstudio";
            if has_key {
                println!("  API Key:     {}", "✅ Set".green());
            } else {
                println!("  API Key:     {}", "❌ Not set".red());
                println!("  Run: algomentor config api-key YOUR_KEY");
            }
            println!();
        }
        ConfigAction::Set { key, value } => {
            let mut config = AppConfig::load(project_dir).unwrap_or_default();
            match key.as_str() {
                "provider" => config.provider.active = value.clone(),
                "model" => config.provider.model = value.clone(),
                "language" | "lang" => config.general.language = value.clone(),
                "level" => config.general.level = value.clone(),
                "temperature" | "temp" => {
                    config.mentor.temperature = value.parse().context("Invalid temperature")?;
                }
                "debounce" => {
                    config.watcher.debounce_ms = value.parse().context("Invalid debounce value")?;
                }
                _ => {
                    eprintln!("{} Unknown key: {}", "❌".red(), key);
                    println!("Available keys: provider, model, language, level, temperature, debounce");
                    return Ok(());
                }
            }
            config.save(project_dir)?;
            println!("{} Set {} = {}", "✅".green(), key.cyan(), value.white().bold());
        }
        ConfigAction::Provider { name } => {
            let mut config = AppConfig::load(project_dir).unwrap_or_default();
            let valid = ["openai", "anthropic", "openrouter", "ollama", "lmstudio"];
            if !valid.contains(&name.as_str()) {
                eprintln!("{} Invalid provider: {}. Use: {}", "❌".red(), name, valid.join(", "));
                return Ok(());
            }
            config.provider.active = name.clone();
            config.save(project_dir)?;
            println!("{} Provider set to: {}", "✅".green(), name.cyan().bold());
        }
        ConfigAction::Model { name } => {
            let mut config = AppConfig::load(project_dir).unwrap_or_default();
            match config.provider.active.as_str() {
                "anthropic" => config.provider.anthropic.model = name.clone(),
                "openrouter" => config.provider.openrouter.model = name.clone(),
                "ollama" => config.provider.ollama.model = name.clone(),
                "lmstudio" => config.provider.lmstudio.model = name.clone(),
                _ => config.provider.openai.model = name.clone(),
            }
            config.provider.model = name.clone(); // Update fallback too
            config.save(project_dir)?;
            println!("{} Model set to {} for active provider ({})", "✅".green(), name.cyan().bold(), config.provider.active.cyan());
        }
        ConfigAction::ApiKey { key } => {
            let config = AppConfig::load(project_dir).unwrap_or_default();
            let provider = &config.provider.active;
            AppConfig::save_api_key_global(provider, &key)?;
            println!(
                "{} API key saved for {} (stored in global config)",
                "✅".green(),
                provider.cyan().bold()
            );
        }
    }

    Ok(())
}

fn ensure_initialized(project_dir: &Path) -> Result<()> {
    if !AppConfig::is_initialized(project_dir) {
        println!("{}", "Auto-initializing AlgoMentor...".yellow());
        let config = AppConfig::default();
        config.save(project_dir)?;
        let db_path = AppConfig::db_path(project_dir);
        let _db = Database::open(&db_path)?;
        println!(
            "{} Initialized! Configure your provider: algomentor config provider openai",
            "✅".green(),
        );
    }
    Ok(())
}

fn register_task(db: &Database, task_dir: &Path) -> Result<()> {
    let name = task_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let now = chrono::Utc::now().to_rfc3339();

    let task_md = task_dir.join("task.md");
    let desc = parser::parse_task_file(&task_md).ok();

    let difficulty = desc
        .as_ref()
        .and_then(|d| d.difficulty.as_ref())
        .map(|d| d.to_string());
    let category = desc
        .as_ref()
        .and_then(|d| d.category.clone());

    db.with_conn(|conn| {
        conn.execute(
            "INSERT OR IGNORE INTO tasks (id, name, directory, difficulty, category, status, started_at, attempts)
             VALUES (?1, ?2, ?3, ?4, ?5, 'in_progress', ?6, 0)",
            rusqlite::params![&name, &name, task_dir.display().to_string(), difficulty, category, now],
        )?;
        conn.execute(
            "UPDATE tasks SET attempts = attempts + 1 WHERE id = ?1",
            rusqlite::params![&name],
        )?;
        Ok(())
    })
}

fn list_available_tasks(project_dir: &Path) -> Result<()> {
    let tasks = discovery::discover_tasks(project_dir, 3)?;
    if tasks.is_empty() {
        println!("    (none found)");
    } else {
        for task in &tasks {
            println!("    • {}", task.name.cyan());
        }
    }
    Ok(())
}

/// Manage knowledge base
pub async fn run_knowledge(project_dir: &Path, action: KnowledgeAction) -> Result<()> {
    ensure_initialized(project_dir)?;
    let config = AppConfig::load(project_dir)?;
    let db_path = AppConfig::db_path(project_dir);
    let db = Database::open(&db_path)?;

    match action {
        KnowledgeAction::Ingest { dir } => {
            let provider = embedding::create_embedding_provider(&config)
                .context("Failed to create embedding provider. Ensure your provider is supported (ollama/openai).")?;
                
            let rag = RagSystem::new(&db, provider);
            
            let dir_path = Path::new(&dir);
            if !dir_path.exists() || !dir_path.is_dir() {
                anyhow::bail!("Directory '{}' does not exist.", dir);
            }

            println!("{}", "🧠 Ingesting knowledge base...".cyan().bold());
            let mut total_files = 0;
            let mut total_chunks = 0;

            for entry in walkdir::WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "md" || ext == "txt" {
                            let content = std::fs::read_to_string(path)?;
                            let source = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                            
                            println!("  {} Processing {}...", "📄".blue(), source);
                            let chunks = rag.ingest_file(&source, &content).await?;
                            total_chunks += chunks;
                            total_files += 1;
                        }
                    }
                }
            }

            println!("\n{} Knowledge base updated!", "✅".green().bold());
            println!("  Files processed: {}", total_files.to_string().cyan());
            println!("  Chunks stored:   {}", total_chunks.to_string().cyan());
        }
    }

    Ok(())
}
