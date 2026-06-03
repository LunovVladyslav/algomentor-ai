# AlgoMentor AI

AlgoMentor is an AI-powered desktop mentor designed to help developers master algorithms and prepare for technical interviews. Instead of just giving you the answers, AlgoMentor acts like a true pair-programming partner: it analyzes your code in real-time, compiles and runs it locally in a sandbox, provides contextual hints, and evaluates Big O time/space complexity.

It has been rebuilt from a CLI into a modern, native **Desktop Application** using Tauri v2, Rust, and Monaco Editor.

## Features

- **Built-in IDE:** A compact, fast Monaco-powered code editor with syntax highlighting, auto-complete, and a floating task description panel.
- **Mentor Watch Mode:** Monitors your source files and streams AI feedback as a non-intrusive tooltip whenever you pause typing.
- **Native Execution:** Automatically compiles and runs your code (Python, Java, Node.js, Rust, C++) locally, showing output instantly.
- **RAG Knowledge Base:** Comes pre-loaded with over 14 essential algorithmic topics (DFS/BFS, Dynamic Programming, Two Pointers, Tries, etc.).
- **Always-on-top Mode:** Pin the mentor over your main browser window or IDE to keep it accessible at all times.

## Installation

### Prerequisites
You need [Rust and Cargo](https://rustup.rs/) installed on your machine.
For the Tauri frontend, ensure you have system webview dependencies (WebView2 on Windows, WebKit on macOS).

### Build for macOS / Windows / Linux

```bash
# Clone the repository
git clone https://github.com/LunovVladyslav/algomentor-ai.git
cd algomentor-ai

# Build the Desktop App
cargo build -p algomentor-gui --release

# Run the Desktop App directly
cargo run -p algomentor-gui
```

*(Note: On macOS, the compiled `.app` bundle will be located in `target/release/bundle/macos/` if you use `cargo tauri build`.)*

## Quick Start

1. Open the application.
2. Click **Open Workspace** to select a directory where all your algorithmic tasks will be saved.
3. Configure your API key (OpenAI, Anthropic, or OpenRouter) by clicking the Settings gear ⚙️.
4. Click **New Task** to create a fresh algorithmic problem.
5. Start coding in the built-in editor, or ask the Mentor for a hint!

## Configuration

AlgoMentor stores its configuration and SQLite history database in your workspace folder (`.algomentor/` hidden directory).
This makes the app completely portable — you can move your workspace folder anywhere and keep your history intact!

## License

This project is licensed under the MIT License.
