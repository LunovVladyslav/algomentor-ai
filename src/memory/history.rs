use anyhow::Result;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};

use super::database::Database;

/// A single chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: i64,
    pub task_id: Option<String>,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

/// Chat history manager
pub struct ChatHistory<'a> {
    db: &'a Database,
}

impl<'a> ChatHistory<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Save a message to the database
    pub fn save_message(
        &self,
        task_id: Option<&str>,
        session_id: &str,
        role: &str,
        content: &str,
    ) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.db.with_conn(|conn| {
            conn.execute(
                "INSERT INTO chat_messages (task_id, session_id, role, content, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![task_id, session_id, role, content, now],
            )?;
            Ok(())
        })
    }

    /// Get messages for a specific session
    pub fn get_messages(
        &self,
        task_id: Option<&str>,
        session_id: &str,
        limit: usize,
    ) -> Result<Vec<ChatMessage>> {
        self.db.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, task_id, session_id, role, content, created_at
                 FROM chat_messages
                 WHERE (?1 IS NULL OR task_id = ?1) AND session_id = ?2
                 ORDER BY id DESC
                 LIMIT ?3"
            )?;

            let messages = stmt.query_map(params![task_id, session_id, limit as i64], |row| {
                Ok(ChatMessage {
                    id: row.get(0)?,
                    task_id: row.get(1)?,
                    session_id: row.get(2)?,
                    role: row.get(3)?,
                    content: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

            // Reverse to get chronological order
            let mut messages = messages;
            messages.reverse();
            Ok(messages)
        })
    }

    /// Get the context window for LLM (last N messages for a task)
    pub fn get_context_window(
        &self,
        task_id: Option<&str>,
        session_id: &str,
        max_messages: usize,
    ) -> Result<Vec<ChatMessage>> {
        self.get_messages(task_id, session_id, max_messages)
    }

    /// Get list of sessions for a task
    pub fn get_sessions(&self, task_id: Option<&str>) -> Result<Vec<String>> {
        self.db.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT DISTINCT session_id FROM chat_messages
                 WHERE (?1 IS NULL OR task_id = ?1)
                 ORDER BY created_at DESC"
            )?;
            let sessions = stmt.query_map(params![task_id], |row| {
                row.get::<_, String>(0)
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(sessions)
        })
    }

    /// Get all messages for a task (across all sessions)
    pub fn get_all_task_messages(&self, task_id: &str) -> Result<Vec<ChatMessage>> {
        self.db.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, task_id, session_id, role, content, created_at
                 FROM chat_messages
                 WHERE task_id = ?1
                 ORDER BY id ASC"
            )?;
            let messages = stmt.query_map(params![task_id], |row| {
                Ok(ChatMessage {
                    id: row.get(0)?,
                    task_id: row.get(1)?,
                    session_id: row.get(2)?,
                    role: row.get(3)?,
                    content: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(messages)
        })
    }

    /// Export a session's chat history to Markdown
    pub fn export_to_markdown(&self, task_id: Option<&str>, session_id: &str) -> Result<String> {
        let messages = self.get_messages(task_id, session_id, 1000)?;
        let mut md = String::from("# Chat History\n\n");

        for msg in &messages {
            let role_label = match msg.role.as_str() {
                "user" => "**You**",
                "assistant" => "**Mentor**",
                "system" => "*System*",
                _ => &msg.role,
            };
            md.push_str(&format!("### {} ({})\n\n{}\n\n---\n\n", role_label, msg.created_at, msg.content));
        }
        Ok(md)
    }
}
