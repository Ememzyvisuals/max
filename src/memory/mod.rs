// memory/mod.rs — Persistent Memory Store
// Created by Ememzyvisuals (Emmanuel Ariyo)

pub mod commands;

use anyhow::Result;
use colored::Colorize;
use rusqlite::{Connection, params};
use std::path::PathBuf;

use crate::config::MaxConfig;

pub struct MemoryStore {
    conn: Connection,
}

#[derive(Debug)]
pub struct Memory {
    pub id: i64,
    pub user_input: String,
    pub ai_response: String,
    pub timestamp: String,
    pub session_id: String,
}

impl MemoryStore {
    pub fn new(config: &MaxConfig) -> Result<Self> {
        let db_path = resolve_path(&config.memory.db_path);

        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)?;

        conn.execute_batch("
            CREATE TABLE IF NOT EXISTS memories (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                user_input  TEXT NOT NULL,
                ai_response TEXT NOT NULL,
                timestamp   TEXT NOT NULL DEFAULT (datetime('now')),
                session_id  TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS buddy_state (
                key   TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_memories_session ON memories(session_id);
            CREATE INDEX IF NOT EXISTS idx_memories_time    ON memories(timestamp DESC);
        ")?;

        Ok(Self { conn })
    }

    pub fn store(&mut self, user_input: &str, ai_response: &str) -> Result<()> {
        let session_id = std::env::var("MAX_SESSION_ID")
            .unwrap_or_else(|_| "default".to_string());

        self.conn.execute(
            "INSERT INTO memories (user_input, ai_response, session_id) VALUES (?1, ?2, ?3)",
            params![user_input, ai_response, session_id],
        )?;

        Ok(())
    }

    pub fn recent(&self, limit: u32) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT user_input FROM memories ORDER BY timestamp DESC LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![limit], |row| {
            row.get::<_, String>(0)
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }

        Ok(results)
    }

    pub fn search(&self, query: &str) -> Result<Vec<Memory>> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, user_input, ai_response, timestamp, session_id 
             FROM memories 
             WHERE user_input LIKE ?1 OR ai_response LIKE ?1
             ORDER BY timestamp DESC
             LIMIT 20",
        )?;

        let rows = stmt.query_map(params![pattern], |row| {
            Ok(Memory {
                id: row.get(0)?,
                user_input: row.get(1)?,
                ai_response: row.get(2)?,
                timestamp: row.get(3)?,
                session_id: row.get(4)?,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }

        Ok(results)
    }

    pub fn clear(&mut self) -> Result<usize> {
        let deleted = self.conn.execute("DELETE FROM memories", [])?;
        Ok(deleted)
    }

    pub fn count(&self) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM memories",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    pub fn get_buddy_state(&self, key: &str) -> Result<Option<String>> {
        let result = self.conn.query_row(
            "SELECT value FROM buddy_state WHERE key = ?1",
            params![key],
            |row| row.get::<_, String>(0),
        );

        match result {
            Ok(val) => Ok(Some(val)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn set_buddy_state(&mut self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO buddy_state (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }
}

fn resolve_path(path_str: &str) -> PathBuf {
    if path_str.starts_with('~') {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home.join(&path_str[2..])
    } else {
        PathBuf::from(path_str)
    }
}
