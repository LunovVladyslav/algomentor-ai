use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use std::time::Duration;

// ─── Layout constants ────────────────────────────────────────────────────────

const BOX_WIDTH: usize = 58; // inner content width (between the │ borders)

// ─── Box drawing helpers ─────────────────────────────────────────────────────

/// Render a single box row: "║  <content padded to BOX_WIDTH>  ║"
/// Strips ANSI codes when measuring length so colours don't break padding.
fn box_row(content: &str, content_visual_len: usize) -> String {
    let padding = BOX_WIDTH.saturating_sub(content_visual_len);
    format!(
        "{}  {}{}  {}",
        "║".cyan(),
        content,
        " ".repeat(padding),
        "║".cyan()
    )
}

fn box_top() -> String {
    format!("╔{}╗", "═".repeat(BOX_WIDTH + 4)).cyan().to_string()
}

fn box_sep() -> String {
    format!("╠{}╣", "═".repeat(BOX_WIDTH + 4)).cyan().to_string()
}

fn box_bot() -> String {
    format!("╚{}╝", "═".repeat(BOX_WIDTH + 4)).cyan().to_string()
}

fn box_empty() -> String {
    format!("{}{}{}",
        "║".cyan(),
        " ".repeat(BOX_WIDTH + 4),
        "║".cyan()
    )
}

// ─── Section divider ─────────────────────────────────────────────────────────

fn thin_rule() -> String {
    "─".repeat(BOX_WIDTH + 4).dimmed().to_string()
}

// ─── Welcome banner ──────────────────────────────────────────────────────────

pub fn render_welcome(task_name: Option<&str>, provider_name: &str) {
    let model_display = provider_name;

    // Title row
    let title_raw = "🧠  AlgoMentor  ·  AI Coding Mentor";
    let title_colored = format!("{}", title_raw.white().bold());
    let title_vis = strip_visual_len(title_raw);

    // Mode row
    let (mode_icon, mode_label, mode_vis) = match task_name {
        Some(name) => {
            let raw = format!("Task  ›  {}", name);
            let vis = strip_visual_len(&raw);
            let colored = format!("{}  {}  {}", "Task".dimmed(), "›".cyan(), name.yellow().bold());
            (colored, "", vis)
        }
        None => {
            let raw = "General Chat Mode";
            (format!("{}", raw.white().italic()), "", strip_visual_len(raw))
        }
    };
    let _ = mode_label; // suppress warning

    // Provider row
    let provider_raw = format!("Provider  {}  {}", "·", model_display);
    let provider_vis = strip_visual_len(&provider_raw);
    let provider_colored = format!(
        "{}  {}  {}",
        "Provider".dimmed(),
        "·".dimmed(),
        model_display.white()
    );

    // Hint row
    let hint_raw = "Type /help to see all commands";
    let hint_colored = format!("Type {} to see all commands", "/help".cyan().bold());
    let hint_vis = strip_visual_len(hint_raw);

    println!();
    println!("{}", box_top());
    println!("{}", box_row(&title_colored, title_vis));
    println!("{}", box_sep());
    println!("{}", box_row(&mode_icon, mode_vis));
    println!("{}", box_row(&provider_colored, provider_vis));
    println!("{}", box_empty());
    println!("{}", box_row(&hint_colored, hint_vis));
    println!("{}", box_bot());
    println!();
}

// ─── Message rendering ───────────────────────────────────────────────────────

pub fn render_user_message(msg: &str) {
    println!();
    println!(
        "  {} {}",
        "you".green().bold(),
        "›".green().dimmed()
    );
    println!("  {}", msg.white());
}

/// Render a full (non-streaming) mentor message with a header
pub fn render_mentor_message(msg: &str) {
    println!();
    render_mentor_header();
    let rendered = crate::utils::markdown::render_markdown(msg);
    // Indent each line slightly
    for line in rendered.lines() {
        println!("  {}", line);
    }
    println!();
    println!("  {}", thin_rule());
}

/// Print the streaming start indicator (header only)
pub fn render_streaming_start() {
    println!();
    render_mentor_header();
}

/// Print a single streaming chunk (no newline, no indent — streaming is inline)
pub fn render_streaming_chunk(chunk: &str) {
    // Add 2-space indent on new lines within the stream
    let indented = chunk.replace('\n', "\n  ");
    print!("  {}", indented);
    io::stdout().flush().ok();
}

/// End the streaming output — print a trailing rule
pub fn render_streaming_end(_full_message: &str) {
    println!();
    println!();
    println!("  {}", thin_rule());
}

fn render_mentor_header() {
    println!(
        "  {} {}",
        "mentor".cyan().bold(),
        "›".cyan().dimmed()
    );
}

// ─── System messages ─────────────────────────────────────────────────────────

pub fn render_system_message(msg: &str) {
    println!("  {}  {}", "·".dimmed(), msg.dimmed());
}

pub fn render_error(msg: &str) {
    eprintln!();
    eprintln!(
        "  {}  {}",
        "✗".red().bold(),
        msg.red()
    );
}

pub fn render_success(msg: &str) {
    println!();
    println!(
        "  {}  {}",
        "✓".green().bold(),
        msg.green()
    );
}

// ─── Code block ──────────────────────────────────────────────────────────────

pub fn render_code_block(code: &str, filename: &str) {
    println!();
    println!(
        "  {}  {}",
        "file".blue().bold(),
        filename.white().bold()
    );
    println!("  {}", thin_rule());
    for line in code.lines() {
        println!("  {}", line);
    }
    println!("  {}", thin_rule());
}

// ─── Task description ─────────────────────────────────────────────────────────

pub fn render_task_description(title: &str, body: &str) {
    println!();
    println!("  {}  {}", "task".blue().bold(), title.white().bold());
    println!("  {}", thin_rule());
    for line in body.lines() {
        println!("  {}", line);
    }
    println!("  {}", thin_rule());
}

// ─── Complexity report ────────────────────────────────────────────────────────

pub fn render_complexity_report(report: &str) {
    println!();
    println!(
        "  {}  {}",
        "complexity".yellow().bold(),
        "Big O Analysis".white()
    );
    println!("  {}", thin_rule());
    let rendered = crate::utils::markdown::render_markdown(report);
    for line in rendered.lines() {
        println!("  {}", line);
    }
    println!("  {}", thin_rule());
}

// ─── Spinner ─────────────────────────────────────────────────────────────────

pub fn start_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("  {spinner:.cyan} {msg:.dim}")
            .unwrap(),
    );
    pb.set_message(msg.to_string());
    pb
}

// ─── Screen ──────────────────────────────────────────────────────────────────

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().ok();
}

// ─── Input ───────────────────────────────────────────────────────────────────

pub fn read_input_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Approximate visual length of a string (strips common ANSI codes & counts
/// Unicode codepoints rather than bytes, treating emoji as width 2).
fn strip_visual_len(s: &str) -> usize {
    // A quick-and-dirty walk; good enough for our fixed strings.
    let mut len = 0usize;
    let mut in_escape = false;
    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
            continue;
        }
        if in_escape {
            if ch == 'm' { in_escape = false; }
            continue;
        }
        // Emoji and CJK: count as width 2
        let w = if ch as u32 > 0x2E80 { 2 } else { 1 };
        len += w;
    }
    len
}
