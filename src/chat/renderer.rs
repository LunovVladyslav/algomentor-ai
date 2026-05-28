use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use std::time::Duration;

/// Render a user message
pub fn render_user_message(msg: &str) {
    println!("\n {} {}", "You:".green().bold(), msg);
}

/// Render a mentor message (full) using termimad
pub fn render_mentor_message(msg: &str) {
    println!("\n {}", "🧠 Mentor:".cyan().bold());
    let rendered = crate::utils::markdown::render_markdown(msg);
    println!("{}", rendered);
}

/// Print the streaming start indicator
pub fn render_streaming_start() {
    println!("\n {}", "🧠 Mentor:".cyan().bold());
}

/// Print a single streaming chunk (no newline)
pub fn render_streaming_chunk(chunk: &str) {
    print!("{}", chunk);
    io::stdout().flush().ok();
}

/// End the streaming output
pub fn render_streaming_end(_full_message: &str) {
    // Clear the streamed text (this is a bit tricky without full TUI, 
    // so we just print a separator or re-render if we want.
    // For now, streaming prints raw markdown.
    println!();
}

/// Create and start a spinner
pub fn start_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg:.yellow}")
            .unwrap(),
    );
    pb.set_message(msg.to_string());
    pb
}

/// Render a system/info message
pub fn render_system_message(msg: &str) {
    println!(" {} {}", "ℹ️ ".yellow(), msg.yellow());
}

/// Render an error message
pub fn render_error(msg: &str) {
    eprintln!("\n {} {}", "❌".red(), msg.red());
}

/// Render a success message
pub fn render_success(msg: &str) {
    println!("\n {} {}", "✅".green(), msg.green());
}

/// Render code with a header
pub fn render_code_block(code: &str, filename: &str) {
    println!("\n {} {}", "📄 File:".blue().bold(), filename.blue());
    println!("{}", "─".repeat(60).dimmed());
    print!("{}", code);
    println!("{}", "─".repeat(60).dimmed());
}

/// Render the welcome message
pub fn render_welcome(task_name: Option<&str>, provider_name: &str) {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║           🧠 AlgoMentor — AI Coding Mentor           ║".cyan());
    println!("{}", "╠══════════════════════════════════════════════════════╣".cyan());

    if let Some(name) = task_name {
        println!(
            "{}  Task: {:<42} {}",
            "║".cyan(),
            name.yellow().bold(),
            "║".cyan()
        );
    } else {
        println!(
            "{}  {}                                          {}",
            "║".cyan(),
            "General Chat Mode".white().bold(),
            "║".cyan()
        );
    }

    println!(
        "{}  Provider: {:<38} {}",
        "║".cyan(),
        provider_name.dimmed(),
        "║".cyan()
    );
    println!("{}", "║                                                        ║".cyan());
    println!(
        "{}  {} {}  {}",
        "║".cyan(),
        "Type".dimmed(),
        "/help".white().bold(),
        "for available commands                ║".cyan()
    );
    println!("{}", "╚══════════════════════════════════════════════════════╝".cyan());
    println!();
}

/// Render the complexity report
pub fn render_complexity_report(report: &str) {
    println!("\n{}", "📊 Big O Complexity Analysis".cyan().bold());
    println!("{}", "═".repeat(50).cyan());
    println!("{}", report);
    println!("{}", "═".repeat(50).cyan());
}

/// Render task description
pub fn render_task_description(title: &str, body: &str) {
    println!("\n{} {}", "📋 Task:".blue().bold(), title.blue().bold());
    println!("{}", "─".repeat(60).dimmed());
    println!("{}", body);
    println!("{}", "─".repeat(60).dimmed());
}

/// Clear the terminal screen
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().ok();
}

/// Read a line of input with a prompt
pub fn read_input_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
