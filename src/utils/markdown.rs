use termimad::MadSkin;

/// Create a polished terminal markdown skin using termimad's re-exported Color
/// (termimad bundles crossterm 0.29; using its re-export avoids version conflicts)
pub fn create_skin() -> MadSkin {
    use termimad::crossterm::style::Color;

    let mut skin = MadSkin::default();

    // Bold — bright white
    skin.bold.set_fg(Color::White);

    // Italic — cyan tint
    skin.italic.set_fg(Color::Cyan);

    // Inline code — yellow, no background
    skin.inline_code.set_fg(Color::Yellow);
    skin.inline_code.set_bg(Color::Reset);

    // Code block — yellow text
    skin.code_block.set_fg(Color::Yellow);

    // Headers h1 / h2 / h3
    skin.headers[0].set_fg(Color::Cyan);   // # H1
    skin.headers[1].set_fg(Color::Cyan);   // ## H2
    skin.headers[2].set_fg(Color::White);  // ### H3

    // Bullet — cyan middle dot
    skin.bullet = termimad::StyledChar::from_fg_char(Color::Cyan, '·');

    // Quoted text — cyan pipe
    skin.quote_mark = termimad::StyledChar::from_fg_char(Color::Cyan, '│');

    skin
}

/// Render markdown text for terminal display
pub fn render_markdown(text: &str) -> String {
    let skin = create_skin();
    skin.text(text, None).to_string()
}
