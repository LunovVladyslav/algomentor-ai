use anyhow::Result;
use colored::Colorize;
use futures_util::StreamExt;
use std::path::PathBuf;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use uuid::Uuid;

use crate::analyzer::{code_reader, complexity, highlighter::Highlighter};
use crate::config::settings::AppConfig;
use crate::llm::prompts;
use crate::llm::provider::{CompletionOptions, LlmProvider, Message, Role};
use crate::memory::database::Database;
use crate::memory::history::ChatHistory;
use crate::memory::rag::RagSystem;
use crate::task::models::TaskDescription;
use crate::task::parser;

use super::input::{self, ChatCommand};
use super::renderer;

/// An interactive chat session with the mentor
pub struct ChatSession {
    provider: Box<dyn LlmProvider>,
    config: AppConfig,
    db: Database,
    session_id: String,
    task_dir: Option<PathBuf>,
    task_description: Option<TaskDescription>,
    messages: Vec<Message>,
    highlighter: Highlighter,
    mcp_tools: Vec<crate::llm::provider::Tool>,
    mcp_clients: Vec<(String, crate::mcp::client::McpClient)>,
}

impl ChatSession {
    pub fn new(
        provider: Box<dyn LlmProvider>,
        config: AppConfig,
        db: Database,
        task_dir: Option<PathBuf>,
    ) -> Result<Self> {
        let task_description = task_dir.as_ref().and_then(|dir| {
            let task_md = dir.join("task.md");
            parser::parse_task_file(&task_md).ok()
        });

        let session_id = Uuid::new_v4().to_string();

        // Build initial system prompt
        let system_prompt = prompts::get_mentor_system_prompt(
            &config.general.level,
            &config.general.language,
        );

        let mut messages = vec![Message::system(&system_prompt)];

        // Add task context if available
        if let Some(task_ctx) = Self::build_task_context(&task_description) {
            messages.push(Message::system(&task_ctx));
        }

        Ok(Self {
            provider,
            config,
            db,
            session_id,
            task_dir,
            task_description,
            messages,
            highlighter: Highlighter::new(),
            mcp_tools: Vec::new(),
            mcp_clients: Vec::new(),
        })
    }

    pub fn set_mcp_tools(&mut self, tools: Vec<crate::llm::provider::Tool>) {
        self.mcp_tools = tools;
    }

    pub fn set_mcp_clients(&mut self, clients: Vec<(String, crate::mcp::client::McpClient)>) {
        self.mcp_clients = clients;
    }

    fn build_task_context(task_description: &Option<TaskDescription>) -> Option<String> {
        task_description.as_ref().map(|desc| {
            format!(
                "The user is working on: \"{}\"\n\nProblem description:\n{}",
                desc.title, desc.body
            )
        })
    }

    fn task_id(&self) -> Option<String> {
        self.task_dir
            .as_ref()
            .and_then(|d| d.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
    }

    fn task_name(&self) -> Option<&str> {
        self.task_description
            .as_ref()
            .map(|d| d.title.as_str())
            .or_else(|| {
                self.task_dir
                    .as_ref()
                    .and_then(|d| d.file_name())
                    .and_then(|n| n.to_str())
            })
    }

    fn completion_options(&self) -> CompletionOptions {
        let mut opts = CompletionOptions {
            model: self.config.active_model().to_string(),
            temperature: self.config.mentor.temperature,
            max_tokens: Some(4096),
            tools: Some(vec![
                crate::llm::provider::Tool {
                    type_: "function".to_string(),
                    function: crate::llm::provider::FunctionDefinition {
                        name: crate::tools::compiler::RunCodeTool::name().to_string(),
                        description: crate::tools::compiler::RunCodeTool::description().to_string(),
                        parameters: crate::tools::compiler::RunCodeTool::parameters(),
                    },
                }
            ]),
        };

        if !self.mcp_tools.is_empty() {
            let mut combined_tools = opts.tools.unwrap_or_default();
            combined_tools.extend(self.mcp_tools.clone());
            opts.tools = Some(combined_tools);
        }
        
        opts
    }

    /// Run the interactive chat loop
    pub async fn start(&mut self) -> Result<()> {
        renderer::render_welcome(self.task_name(), self.provider.name());
        
        // Init Knowledge Base
        if let Ok(provider) = crate::llm::embedding::create_embedding_provider(&self.config) {
            let rag = crate::memory::rag::RagSystem::new(&self.db, provider);
            if let Err(e) = rag.init_knowledge_base().await {
                renderer::render_system_message(&format!("Failed to auto-init knowledge base: {}", e));
            }
        }

        let mut rl = DefaultEditor::new().map_err(|e| anyhow::anyhow!("Failed to initialize rustyline: {}", e))?;
        let history_path = dirs::home_dir().unwrap_or_default().join(".algomentor_history");
        let _ = rl.load_history(&history_path);

        loop {
            let readline = rl.readline(&format!("{} ", "▶".green().bold()));
            let input = match readline {
                Ok(line) => {
                    let trimmed = line.trim().to_string();
                    if !trimmed.is_empty() {
                        let _ = rl.add_history_entry(trimmed.as_str());
                    }
                    trimmed
                },
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    break;
                }
                Err(_) => {
                    break;
                }
            };

            if input.is_empty() {
                continue;
            }

            let command = input::parse_command(&input);

            match command {
                ChatCommand::Quit => {
                    let _ = rl.save_history(&history_path);
                    renderer::render_system_message("Goodbye! Keep coding! 🚀");
                    break;
                }
                ChatCommand::Help => {
                    println!("{}", input::help_text());
                }
                ChatCommand::Clear => {
                    renderer::clear_screen();
                }
                ChatCommand::Message(msg) => {
                    if msg.is_empty() {
                        continue;
                    }
                    self.handle_message(&msg).await?;
                }
                ChatCommand::Hint => self.handle_hint().await?,
                ChatCommand::Complexity => self.handle_complexity().await?,
                ChatCommand::ShowCode => self.handle_show_code(),
                ChatCommand::ShowTask => self.handle_show_task(),
                ChatCommand::Solution => self.handle_solution().await?,
                ChatCommand::History => self.handle_history(),
                ChatCommand::Done => self.handle_done().await?,
                ChatCommand::Ping => {
                    renderer::render_system_message("PONG 🏓");
                }
                ChatCommand::Model(name) => {
                    self.config.set_model(&name);
                    let mut saved = false;
                    if let Some(dir) = &self.task_dir {
                        if self.config.save(dir).is_ok() {
                            saved = true;
                        }
                    }
                    if !saved {
                        let home = dirs::home_dir().unwrap_or_default();
                        let _ = self.config.save(&home.join(".algomentor"));
                    }
                    
                    // Needs to update provider if they switched openrouter model vs openai model? 
                    // Wait, the provider instance is already created in commands.rs.
                    // But changing the model name in config doesn't automatically tell the provider the new model, unless the provider gets it from options on each request.
                    // Let's check completion_options: it reads `self.config.active_model()`. So we are good!
                    
                    renderer::render_system_message(&format!("Model successfully changed to: {}", name));
                }
                ChatCommand::Unknown(cmd) => {
                    renderer::render_error(&format!("Unknown command: {}. Type /help for available commands.", cmd));
                }
            }
        }

        Ok(())
    }

    async fn handle_message(&mut self, msg: &str) -> Result<()> {
        renderer::render_user_message(msg);

        // Save user message
        self.save_to_history("user", msg)?;

        // RAG context retrieval
        if let Ok(provider) = crate::llm::embedding::create_embedding_provider(&self.config) {
            let rag = RagSystem::new(&self.db, provider);
            
            // Combine task context (if any) with the user message for a better search query
            let query = if let Some(ref desc) = self.task_description {
                format!("{} {}", desc.title, msg)
            } else {
                msg.to_string()
            };

            if let Ok(results) = rag.search(&query, 2).await {
                let context_chunks: Vec<String> = results.into_iter()
                    .filter(|r| r.score > 0.5) // Minimum relevance threshold
                    .map(|r| r.chunk.content)
                    .collect();

                if !context_chunks.is_empty() {
                    let rag_context = format!(
                        "Here is some relevant knowledge to help you answer (use it if applicable, but don't explicitly mention 'the knowledge base'):\n{}",
                        context_chunks.join("\n\n---\n\n")
                    );
                    self.messages.push(Message::system(&rag_context));
                }
            }
        }

        // Add to context
        self.messages.push(Message::user(msg));
        self.trim_context();

        // Stream response
        let response = self.stream_response().await?;

        // Save assistant response
        self.save_to_history("assistant", &response)?;

        // Add to context
        self.messages.push(Message::assistant(&response));

        Ok(())
    }

    async fn handle_hint(&mut self) -> Result<()> {
        renderer::render_system_message("Analyzing your code for a hint...");

        let (code, lang) = match self.read_current_code() {
            Ok(v) => v,
            Err(_) => {
                renderer::render_error("No solution file found. Create a solution file first.");
                return Ok(());
            }
        };
        let task_body = self.task_description.as_ref().map(|d| d.body.as_str());
        let hint_prompt = prompts::get_hint_prompt(&code, &lang, task_body);

        self.messages.push(Message::user(&hint_prompt));
        self.trim_context();

        let response = self.stream_response().await?;

        self.save_to_history("user", "[Hint requested]")?;
        self.save_to_history("assistant", &response)?;

        self.messages.push(Message::assistant(&response));
        Ok(())
    }

    async fn handle_complexity(&mut self) -> Result<()> {
        renderer::render_system_message("Analyzing Big O complexity...");

        let (_code, _lang) = match self.read_current_code() {
            Ok(v) => v,
            Err(_) => {
                renderer::render_error("No solution file found.");
                return Ok(());
            }
        };

        let task_body = self.task_description.as_ref().map(|d| d.body.as_str());

        if let Some(ref dir) = self.task_dir {
            if let Ok(Some(cf)) = code_reader::read_solution(dir) {
                let prompt = complexity::build_complexity_prompt(&cf.content, &cf.language, task_body);
                self.messages.push(Message::user(&prompt));
                self.trim_context();

                let response = self.stream_response().await?;

                self.save_to_history("user", "[Complexity analysis requested]")?;
                self.save_to_history("assistant", &response)?;

                self.messages.push(Message::assistant(&response));
                return Ok(());
            }
        }

        renderer::render_error("Could not read solution file.");
        Ok(())
    }

    fn handle_show_code(&self) {
        if let Some(ref dir) = self.task_dir {
            if let Ok(Some(cf)) = code_reader::read_solution(dir) {
                let highlighted = self.highlighter.highlight(&cf.content, &cf.language);
                renderer::render_code_block(
                    &highlighted,
                    cf.path.file_name().unwrap_or_default().to_str().unwrap_or("solution"),
                );
                return;
            }
        }
        renderer::render_error("No solution file found. Create a solution file (e.g., solution.py) in the task directory.");
    }

    fn handle_show_task(&self) {
        if let Some(ref desc) = self.task_description {
            renderer::render_task_description(&desc.title, &desc.body);
        } else {
            renderer::render_error("No task.md found.");
        }
    }

    async fn handle_solution(&mut self) -> Result<()> {
        let task_body = self.task_description.as_ref().map(|d| d.body.as_str());
        let prompt = match task_body {
            Some(body) => format!(
                "Explain the general approach/algorithm to solve this problem WITHOUT writing any code:\n\n{}\n\n\
                 Discuss which data structures and patterns would work and why.",
                body
            ),
            None => "Explain the general approach for the problem we're discussing. No code, just concepts.".to_string(),
        };

        self.messages.push(Message::user(&prompt));
        self.trim_context();

        let response = self.stream_response().await?;

        self.save_to_history("user", "[Approach discussion requested]")?;
        self.save_to_history("assistant", &response)?;

        self.messages.push(Message::assistant(&response));
        Ok(())
    }

    fn handle_history(&self) {
        let history = ChatHistory::new(&self.db);
        let messages = history
            .get_messages(self.task_id().as_deref(), &self.session_id, 50)
            .unwrap_or_default();

        if messages.is_empty() {
            renderer::render_system_message("No history yet for this session.");
            return;
        }

        println!("\n{}", "📜 Chat History".blue().bold());
        println!("{}", "─".repeat(60));
        for msg in &messages {
            match msg.role.as_str() {
                "user" => renderer::render_user_message(&msg.content),
                "assistant" | "tool" => renderer::render_mentor_message(&msg.content),
                _ => {}
            }
        }
        println!("{}", "─".repeat(60));
    }

    async fn handle_done(&mut self) -> Result<()> {
        if self.task_dir.is_some() {
            renderer::render_success("🎉 Task marked as completed! Great job!");

            let task_id = self.task_id().unwrap_or_default();
            let now = chrono::Utc::now().to_rfc3339();
            self.db.with_conn(|conn| {
                conn.execute(
                    "UPDATE tasks SET status = 'completed', completed_at = ?1 WHERE id = ?2",
                    rusqlite::params![now, task_id],
                )?;
                Ok(())
            }).ok();
        } else {
            renderer::render_system_message("No task context — nothing to mark as done.");
        }
        Ok(())
    }

    fn read_current_code(&self) -> Result<(String, String)> {
        if let Some(ref dir) = self.task_dir {
            if let Some(code_file) = code_reader::read_solution(dir)? {
                return Ok((code_file.content, code_file.language.to_string()));
            }
        }
        anyhow::bail!("No solution file found")
    }

    async fn stream_response(&mut self) -> Result<String> {
        let options = self.completion_options();

        // If tools are available, do a complete call first to check if the model wants to use a tool
        if options.tools.is_some() {
            let response = self.provider.complete(&self.messages, &options).await?;
            
            if let Some(tool_calls) = response.tool_calls {
                for call in tool_calls {
                    if call.function.name == "run_code" {
                        let spinner = renderer::start_spinner("⚙️ Mentor is running your code...");
                        
                        let args: serde_json::Value = serde_json::from_str(&call.function.arguments).unwrap_or_default();
                        if let Some(file_path) = args["file_path"].as_str() {
                            let absolute_path = if let Some(ref dir) = self.task_dir {
                                dir.join(file_path).to_string_lossy().to_string()
                            } else {
                                file_path.to_string()
                            };

                            let result = crate::tools::compiler::RunCodeTool::execute(&absolute_path).await;
                            
                            spinner.finish_and_clear();
                            self.messages.push(Message::assistant_with_tools("", vec![call.clone()]));
                            self.messages.push(Message::tool_result(&call.id, &result));
                        } else {
                            spinner.finish_and_clear();
                        }
                    } else {
                        // Check MCP clients
                        let mut executed = false;
                        for (_, client) in &mut self.mcp_clients {
                            let spinner = renderer::start_spinner(&format!("🔌 Calling MCP Tool: {}...", call.function.name));
                            let args: serde_json::Value = serde_json::from_str(&call.function.arguments).unwrap_or_default();
                            
                            match client.call_tool(&call.function.name, args).await {
                                Ok(result) => {
                                    spinner.finish_and_clear();
                                    self.messages.push(Message::assistant_with_tools("", vec![call.clone()]));
                                    self.messages.push(Message::tool_result(&call.id, &result));
                                    executed = true;
                                    break;
                                }
                                Err(e) => {
                                    spinner.finish_and_clear();
                                    if !e.to_string().contains("not found") {
                                        // Error occurred while executing this tool
                                        self.messages.push(Message::assistant_with_tools("", vec![call.clone()]));
                                        self.messages.push(Message::tool_result(&call.id, &format!("MCP Tool error: {}", e)));
                                        executed = true;
                                        break;
                                    }
                                }
                            }
                        }
                        
                        if !executed {
                            self.messages.push(Message::assistant_with_tools("", vec![call.clone()]));
                            self.messages.push(Message::tool_result(&call.id, "Tool not found"));
                        }
                    }
                }
                
                // Now stream the final response after tools
                renderer::render_streaming_start();
                let mut stream = self.provider.stream(&self.messages, &options).await?;
                let mut full_response = String::new();

                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(text) => {
                            renderer::render_streaming_chunk(&text);
                            full_response.push_str(&text);
                        }
                        Err(e) => {
                            renderer::render_streaming_end(&full_response);
                            renderer::render_error(&format!("Stream error: {}", e));
                            break;
                        }
                    }
                }

                renderer::render_streaming_end(&full_response);
                return Ok(full_response);
            }
            
            // If no tools were called, we already have the full text, but we didn't stream it.
            // For better UX we could yield it block by block or just print it.
            // Since we got it instantly, we just print it.
            if !response.content.is_empty() {
                renderer::render_mentor_message(&response.content);
                return Ok(response.content);
            }
        }

        // Fallback or toolless
        renderer::render_streaming_start();
        let mut stream = self.provider.stream(&self.messages, &options).await?;
        let mut full_response = String::new();

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(text) => {
                    renderer::render_streaming_chunk(&text);
                    full_response.push_str(&text);
                }
                Err(e) => {
                    renderer::render_streaming_end(&full_response);
                    renderer::render_error(&format!("Stream error: {}", e));
                    break;
                }
            }
        }

        renderer::render_streaming_end(&full_response);
        Ok(full_response)
    }

    /// Save a message to chat history
    fn save_to_history(&self, role: &str, content: &str) -> Result<()> {
        let history = ChatHistory::new(&self.db);
        history.save_message(self.task_id().as_deref(), &self.session_id, role, content)
    }

    fn trim_context(&mut self) {
        let max = self.config.mentor.max_context_messages + 2;
        if self.messages.len() > max {
            let system_count = self
                .messages
                .iter()
                .take_while(|m| m.role == Role::System)
                .count();
            let to_remove = self.messages.len() - max;
            if to_remove > 0 && system_count < self.messages.len() {
                let end = (system_count + to_remove).min(self.messages.len());
                self.messages.drain(system_count..end);
            }
        }
    }
}
