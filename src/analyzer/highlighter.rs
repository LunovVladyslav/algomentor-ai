use super::language::Language;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

/// Syntax highlighter using syntect
pub struct Highlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl Highlighter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    /// Highlight code and return ANSI-escaped string for terminal
    pub fn highlight(&self, code: &str, language: &Language) -> String {
        let theme = &self.theme_set.themes["base16-ocean.dark"];

        let syntax = self
            .syntax_set
            .find_syntax_by_name(language.syntect_name())
            .or_else(|| self.syntax_set.find_syntax_by_extension(&language.to_string()))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let mut h = HighlightLines::new(syntax, theme);
        let mut output = String::new();

        let lines: Vec<&str> = code.lines().collect();
        let width = lines.len().to_string().len();

        for (i, line) in LinesWithEndings::from(code).enumerate() {
            // Line number (dim gray)
            output.push_str(&format!("\x1b[38;5;240m{:>width$} │\x1b[0m ", i + 1, width = width));

            // Highlighted code
            if let Ok(ranges) = h.highlight_line(line, &self.syntax_set) {
                let escaped = as_24_bit_terminal_escaped(&ranges, false);
                output.push_str(&escaped);
            } else {
                output.push_str(line);
            }

            // Reset and ensure newline
            output.push_str("\x1b[0m");
            if !line.ends_with('\n') {
                output.push('\n');
            }
        }

        output
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}
