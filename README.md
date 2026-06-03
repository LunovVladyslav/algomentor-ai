# 🚀 AlgoMentor AI

AlgoMentor is an AI-powered CLI mentor designed to help developers master algorithms and prepare for technical interviews. Instead of just giving you the answers, AlgoMentor acts like a true pair-programming partner: it analyzes your code in real-time, compiles and runs it locally in a sandbox, provides contextual hints, and evaluates Big O time/space complexity.

## ✨ Features

- **🧠 RAG Knowledge Base:** Comes pre-loaded with over 14 essential algorithmic topics (DFS/BFS, Dynamic Programming, Two Pointers, Tries, etc.).
- **⚙️ Native Compilation:** Automatically runs your code (Python, Java, Node.js, Rust) locally to catch bugs and verify test cases.
- **🔌 MCP (Model Context Protocol):** Extend the mentor's capabilities by connecting it to tools like Context7 to fetch up-to-date framework documentation for general software engineering.
- **👀 Watch Mode:** Monitors your source files and streams AI feedback straight to your terminal whenever you save your code in your IDE.
- **💬 Polished CLI Chat:** Features an interactive terminal UI with command history (`rustyline`), rich markdown rendering (`termimad`), and animated loaders.

## 📦 Installation

To install and build AlgoMentor from source, you need [Rust and Cargo](https://rustup.rs/) installed on your machine.

```bash
# Clone the repository
git clone https://github.com/LunovVladyslav/algomentor-ai.git
cd algomentor-ai

# Build and run the project
cargo build --release
```

## 🚀 Quick Start

Initialize your workspace. This will configure your API keys (OpenAI, Anthropic, or Ollama) and auto-index the knowledge base.
```bash
algomentor init
```

Create a new algorithmic task:
```bash
algomentor add two-sum --category arrays
```

Start the interactive chat mentor for your task:
```bash
algomentor chat two-sum
```

For a comprehensive guide, run:
```bash
algomentor guide
```

## ⚙️ Configuration

AlgoMentor stores its configuration and SQLite database in your home directory: `~/.algomentor/`.
You can manage your configuration using the CLI:

```bash
# Set your active provider (ollama, openai, anthropic)
algomentor config set provider openai

# Set your API key
algomentor config api-key sk-your-key-here
```

## 📄 License

This project is licensed under the MIT License.
