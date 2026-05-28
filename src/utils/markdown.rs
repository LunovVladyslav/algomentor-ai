use termimad::MadSkin;

/// Create a custom terminal markdown skin with colors
pub fn create_skin() -> MadSkin {
    MadSkin::default()
}

/// Render markdown text for terminal display
pub fn render_markdown(text: &str) -> String {
    let skin = create_skin();
    skin.text(text, None).to_string()
}
