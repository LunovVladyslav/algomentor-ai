use std::fmt;
use std::path::Path;

/// Supported programming languages
#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Java,
    Cpp,
    C,
    Go,
    Rust,
    Ruby,
    CSharp,
    Unknown,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Python => write!(f, "python"),
            Language::JavaScript => write!(f, "javascript"),
            Language::TypeScript => write!(f, "typescript"),
            Language::Java => write!(f, "java"),
            Language::Cpp => write!(f, "cpp"),
            Language::C => write!(f, "c"),
            Language::Go => write!(f, "go"),
            Language::Rust => write!(f, "rust"),
            Language::Ruby => write!(f, "ruby"),
            Language::CSharp => write!(f, "csharp"),
            Language::Unknown => write!(f, "text"),
        }
    }
}

impl Language {
    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "py" => Language::Python,
            "js" | "mjs" | "cjs" => Language::JavaScript,
            "ts" | "tsx" => Language::TypeScript,
            "java" => Language::Java,
            "cpp" | "cc" | "cxx" | "hpp" => Language::Cpp,
            "c" | "h" => Language::C,
            "go" => Language::Go,
            "rs" => Language::Rust,
            "rb" => Language::Ruby,
            "cs" => Language::CSharp,
            _ => Language::Unknown,
        }
    }

    /// Detect language from file path
    pub fn from_path(path: &Path) -> Self {
        path.extension()
            .and_then(|e| e.to_str())
            .map(Self::from_extension)
            .unwrap_or(Language::Unknown)
    }

    /// Human-readable name
    pub fn display_name(&self) -> &str {
        match self {
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Java => "Java",
            Language::Cpp => "C++",
            Language::C => "C",
            Language::Go => "Go",
            Language::Rust => "Rust",
            Language::Ruby => "Ruby",
            Language::CSharp => "C#",
            Language::Unknown => "Unknown",
        }
    }

    /// Syntect syntax name for highlighting
    pub fn syntect_name(&self) -> &str {
        match self {
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Java => "Java",
            Language::Cpp => "C++",
            Language::C => "C",
            Language::Go => "Go",
            Language::Rust => "Rust",
            Language::Ruby => "Ruby",
            Language::CSharp => "C#",
            Language::Unknown => "Plain Text",
        }
    }
}
