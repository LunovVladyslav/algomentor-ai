use anyhow::Result;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};

use super::database::Database;

/// User statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserStats {
    pub total_tasks: i64,
    pub completed_tasks: i64,
    pub in_progress_tasks: i64,
    pub abandoned_tasks: i64,
    pub total_sessions: i64,
    pub total_messages: i64,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub level: String,
    pub preferred_language: String,
    pub communication_language: String,
}

/// User profile manager
pub struct UserProfile<'a> {
    db: &'a Database,
}

impl<'a> UserProfile<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Get a profile value by key
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        self.db.with_conn(|conn| {
            let result = conn.query_row(
                "SELECT value FROM user_profile WHERE key = ?1",
                params![key],
                |row| row.get::<_, String>(0),
            );
            match result {
                Ok(val) => Ok(Some(val)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e.into()),
            }
        })
    }

    /// Set a profile value
    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.db.with_conn(|conn| {
            conn.execute(
                "INSERT INTO user_profile (key, value, updated_at)
                 VALUES (?1, ?2, ?3)
                 ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = ?3",
                params![key, value, now],
            )?;
            Ok(())
        })
    }

    /// Get aggregated user statistics
    pub fn get_stats(&self) -> Result<UserStats> {
        self.db.with_conn(|conn| {
            let total_tasks: i64 = conn
                .query_row("SELECT COUNT(*) FROM tasks", [], |r| r.get(0))
                .unwrap_or(0);
            let completed_tasks: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM tasks WHERE status = 'completed'",
                    [],
                    |r| r.get(0),
                )
                .unwrap_or(0);
            let in_progress_tasks: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM tasks WHERE status = 'in_progress'",
                    [],
                    |r| r.get(0),
                )
                .unwrap_or(0);
            let abandoned_tasks: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM tasks WHERE status = 'abandoned'",
                    [],
                    |r| r.get(0),
                )
                .unwrap_or(0);
            let total_sessions: i64 = conn
                .query_row(
                    "SELECT COUNT(DISTINCT session_id) FROM chat_messages",
                    [],
                    |r| r.get(0),
                )
                .unwrap_or(0);
            let total_messages: i64 = conn
                .query_row("SELECT COUNT(*) FROM chat_messages", [], |r| r.get(0))
                .unwrap_or(0);

            // Get strengths - categories with most completed tasks
            let mut strengths = Vec::new();
            if let Ok(mut stmt) = conn.prepare(
                "SELECT category, COUNT(*) as cnt FROM tasks
                 WHERE status = 'completed' AND category IS NOT NULL AND category != ''
                 GROUP BY category ORDER BY cnt DESC LIMIT 3"
            ) {
                if let Ok(rows) = stmt.query_map([], |row| row.get::<_, String>(0)) {
                    for row in rows.flatten() {
                        strengths.push(row);
                    }
                }
            }

            // Get weaknesses - categories with most abandoned/in-progress
            let mut weaknesses = Vec::new();
            if let Ok(mut stmt) = conn.prepare(
                "SELECT category, COUNT(*) as cnt FROM tasks
                 WHERE status IN ('abandoned', 'in_progress') AND category IS NOT NULL AND category != ''
                 GROUP BY category ORDER BY cnt DESC LIMIT 3"
            ) {
                if let Ok(rows) = stmt.query_map([], |row| row.get::<_, String>(0)) {
                    for row in rows.flatten() {
                        weaknesses.push(row);
                    }
                }
            }

            let level = self.get("level").ok().flatten().unwrap_or_else(|| "intermediate".into());
            let preferred_language = self.get("preferred_language").ok().flatten().unwrap_or_default();
            let communication_language = self.get("communication_language").ok().flatten().unwrap_or_else(|| "auto".into());

            Ok(UserStats {
                total_tasks,
                completed_tasks,
                in_progress_tasks,
                abandoned_tasks,
                total_sessions,
                total_messages,
                strengths,
                weaknesses,
                level,
                preferred_language,
                communication_language,
            })
        })
    }

    /// Record an analytics event
    pub fn record_event(&self, task_id: Option<&str>, event_type: &str, metadata: Option<&str>) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.db.with_conn(|conn| {
            conn.execute(
                "INSERT INTO analytics (task_id, event_type, metadata, created_at)
                 VALUES (?1, ?2, ?3, ?4)",
                params![task_id, event_type, metadata, now],
            )?;
            Ok(())
        })
    }
}
