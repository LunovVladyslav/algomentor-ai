/// Chat command parsed from user input
#[derive(Debug)]
pub enum ChatCommand {
    /// Regular message to send to the mentor
    Message(String),
    /// Request a hint for the current code
    Hint,
    /// Run Big O complexity analysis
    Complexity,
    /// Show current code with syntax highlighting
    ShowCode,
    /// Show the task description
    ShowTask,
    /// Ask mentor to explain the approach (without code)
    Solution,
    /// View chat history
    History,
    /// Clear the screen
    Clear,
    /// Mark the task as completed
    Done,
    /// Show help
    Help,
    /// Quit the chat
    Quit,
    /// Unknown command
    Unknown(String),
}

/// Parse user input into a chat command
pub fn parse_command(input: &str) -> ChatCommand {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return ChatCommand::Message(String::new());
    }

    if !trimmed.starts_with('/') {
        return ChatCommand::Message(trimmed.to_string());
    }

    let cmd = trimmed.to_lowercase();
    match cmd.as_str() {
        "/hint" | "/h" => ChatCommand::Hint,
        "/complexity" | "/bigo" | "/big-o" | "/o" => ChatCommand::Complexity,
        "/code" | "/c" => ChatCommand::ShowCode,
        "/task" | "/t" => ChatCommand::ShowTask,
        "/solution" | "/approach" | "/s" => ChatCommand::Solution,
        "/history" | "/hist" => ChatCommand::History,
        "/clear" | "/cls" => ChatCommand::Clear,
        "/done" | "/complete" | "/solved" => ChatCommand::Done,
        "/help" | "/?" => ChatCommand::Help,
        "/quit" | "/exit" | "/q" => ChatCommand::Quit,
        _ => ChatCommand::Unknown(trimmed.to_string()),
    }
}

/// Display available commands
pub fn help_text() -> &'static str {
    r#"
╔══════════════════════════════════════════════════════╗
║                  Available Commands                   ║
╠══════════════════════════════════════════════════════╣
║  /hint (/h)        — Get a hint for your solution     ║
║  /bigo (/o)        — Analyze Big O complexity          ║
║  /code (/c)        — Show your current code            ║
║  /task (/t)        — Show the problem description      ║
║  /solution (/s)    — Discuss approach (no code!)       ║
║  /history          — View chat history                 ║
║  /clear            — Clear screen                      ║
║  /done             — Mark task as completed            ║
║  /help (/?)        — Show this help                    ║
║  /quit (/q)        — Exit the chat                     ║
║                                                        ║
║  Or just type your message to chat with the mentor!    ║
╚══════════════════════════════════════════════════════╝"#
}
