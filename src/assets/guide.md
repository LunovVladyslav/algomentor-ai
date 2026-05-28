# 🚀 AlgoMentor Guide

Welcome to **AlgoMentor**, your AI-powered companion for mastering algorithms and acing technical interviews! 

AlgoMentor is designed to help you solve algorithmic challenges locally, running code in sandboxes, and providing intelligent hints powered by state-of-the-art LLMs and a built-in RAG (Retrieval-Augmented Generation) knowledge base.

---

## 📚 Game Modes

### 1. Task Mode (Default)
`algomentor chat <task_name>`
Use this mode to solve specific algorithmic problems (e.g., LeetCode).
- The mentor restricts answers to algorithmic concepts.
- The **RAG Knowledge Base** provides deep insights into common patterns like DFS, Two Pointers, and Dynamic Programming.
- **Compiler Support**: The mentor can automatically compile and run your Python, Node, Java, or Rust code to find bugs!

### 2. Project Mode (MCP)
`algomentor project`
Use this mode for general software engineering and project architecture.
- Connects to external **MCP Servers** (like Context7).
- Automatically fetches up-to-date documentation for libraries like React, Next.js, and Rust crates to prevent AI hallucinations.

---

## 🛠️ Chat Commands

While in the chat interface, you can use the following commands:
- `/help` - Show all commands.
- `/clear` - Clear the terminal screen.
- `/quit` or `/exit` - Exit the chat session.

*Tip: You can use the `Up` and `Down` arrow keys to navigate your prompt history!*

---

## 🧠 Knowledge Base (RAG)

AlgoMentor ships with a curated knowledge base covering 14+ essential computer science topics. During your first run, AlgoMentor automatically ingested these topics into its vector database:
1. Arrays & Two Pointers
2. Hashing & Frequency Patterns
3. Sorting & Binary Search
4. Strings & Tries
5. Linked Lists
6. Stacks & Queues
7. Trees & Graphs (DFS/BFS)
8. Heaps & Priority Queues
9. Dynamic Programming & Backtracking

When you ask a question like *"How do I implement a Trie in Rust?"*, the mentor automatically queries this database to give you the most accurate and structured explanation.

---

Happy coding! 💻
