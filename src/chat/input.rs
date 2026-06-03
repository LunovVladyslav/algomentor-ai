use colored::Colorize;

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
    /// Deep conceptual explanation of the algorithm — spoken-style, no copy-paste code
    Explain,
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
    /// Change the active model
    Model(String),
    /// Test command Ping
    Ping,
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

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let base_cmd = parts[0].to_lowercase();

    if (base_cmd == "/model" || base_cmd == "/m") && parts.len() > 1 {
        return ChatCommand::Model(parts[1..].join(" "));
    }

    match base_cmd.as_str() {
        "/hint" | "/h"                           => ChatCommand::Hint,
        "/complexity" | "/bigo" | "/big-o" | "/o" => ChatCommand::Complexity,
        "/code" | "/c"                           => ChatCommand::ShowCode,
        "/task" | "/t"                           => ChatCommand::ShowTask,
        "/solution" | "/approach" | "/s"         => ChatCommand::Solution,
        "/explain" | "/ex"                       => ChatCommand::Explain,
        "/history" | "/hist"                     => ChatCommand::History,
        "/clear" | "/cls"                        => ChatCommand::Clear,
        "/done" | "/complete" | "/solved"        => ChatCommand::Done,
        "/ping"                                  => ChatCommand::Ping,
        "/help" | "/?"                           => ChatCommand::Help,
        "/quit" | "/exit" | "/q"                 => ChatCommand::Quit,
        _                                        => ChatCommand::Unknown(trimmed.to_string()),
    }
}

/// Print available commands with colour formatting
pub fn print_help() {
    let w = 54usize;
    let border = |ch: &str| ch.cyan().to_string();

    let top   = format!("  ╔{}╗", "═".repeat(w));
    let sep   = format!("  ╠{}╣", "═".repeat(w));
    let bot   = format!("  ╚{}╝", "═".repeat(w));
    let empty = format!("  ║{}║", " ".repeat(w));
    let _ = border; // used implicitly via .cyan()

    // Aligned row: cmd (14) + short (7) + desc (31) = 52 + 2 spaces around = 56 ✓
    let row = |cmd: &str, short: &str, desc: &str| -> String {
        format!(
            "  ║  {:<14}{:<7}{}{}  ║",
            cmd.cyan().bold(),
            short.dimmed(),
            desc.white(),
            // right-pad desc to fill the column
            " ".repeat(31usize.saturating_sub(desc.len()))
        )
    };

    // Title centered
    let title_text = "  Commands";
    let title_pad  = (w - title_text.trim().len()) / 2;
    let title = format!(
        "  ║{}{}{} ║",
        " ".repeat(title_pad),
        title_text.trim().white().bold(),
        " ".repeat(w - title_pad - title_text.trim().len())
    );

    println!();
    println!("{}", top.cyan());
    println!("{}", title);
    println!("{}", sep.cyan());

    // ── Mentor ──────────────────────────────────────────────
    println!("{}", row("/hint",      "/h",   "Hint for your current solution"));
    println!("{}", row("/explain",   "/ex",  "Conceptual lecture — why & how"));
    println!("{}", row("/solution",  "/s",   "Approach discussion (no code)"));
    println!("{}", row("/bigo",      "/o",   "Big O complexity analysis"));

    println!("{}", sep.cyan());

    // ── View ────────────────────────────────────────────────
    println!("{}", row("/code",      "/c",   "Show your current code"));
    println!("{}", row("/task",      "/t",   "Show the problem description"));
    println!("{}", row("/history",   "",     "View this session's history"));

    println!("{}", sep.cyan());

    // ── Session ─────────────────────────────────────────────
    println!("{}", row("/done",      "",     "Mark task as completed ✓"));
    println!("{}", row("/model",     "/m",   "/model <name>  change AI model"));
    println!("{}", row("/clear",     "",     "Clear the screen"));
    println!("{}", row("/help",      "/?",   "Show this help"));
    println!("{}", row("/quit",      "/q",   "Exit the chat"));

    println!("{}", empty.cyan());
    println!(
        "  ║  {}{}  ║",
        "Just type to chat with your mentor".dimmed(),
        " ".repeat(w - 2 - "Just type to chat with your mentor".len())
    );
    println!("{}", bot.cyan());
    println!();
}
