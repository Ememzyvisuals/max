// config/mod.rs — MAX Configuration Manager
// Created by Ememzyvisuals (Emmanuel Ariyo)

pub mod commands;

use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxConfig {
    pub meta: MetaConfig,
    pub model: ModelConfig,
    pub ui: UiConfig,
    pub memory: MemoryConfig,
    pub buddy: BuddyConfig,
    pub api: ApiConfig,
    pub tools: ToolsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaConfig {
    pub version: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub active: String,
    pub offline_path: String,
    pub fallback_to_api: bool,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub animation_speed: String, // slow | normal | fast | off
    pub color_theme: String,     // dark | light | neon | minimal
    pub splash_enabled: bool,
    pub spinner_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub enabled: bool,
    pub depth: u32,
    pub embeddings: bool,
    pub db_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuddyConfig {
    pub enabled: bool,
    pub name: String,
    pub personality: String,
    pub evolution_stage: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub groq_key: Option<String>,
    pub openai_key: Option<String>,
    pub anthropic_key: Option<String>,
    pub preferred_api: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    pub sandbox_enabled: bool,
    pub allowed_languages: Vec<String>,
    pub timeout_seconds: u32,
}

impl Default for MaxConfig {
    fn default() -> Self {
        Self {
            meta: MetaConfig {
                version: "1.0.0".into(),
                author: "Ememzyvisuals (Emmanuel Ariyo)".into(),
            },
            model: ModelConfig {
                active: "groq/llama3-70b-8192".into(),
                offline_path: "~/.max/models/".into(),
                fallback_to_api: true,
                temperature: 0.7,
                max_tokens: 2048,
            },
            ui: UiConfig {
                animation_speed: "normal".into(),
                color_theme: "dark".into(),
                splash_enabled: true,
                spinner_style: "dots".into(),
            },
            memory: MemoryConfig {
                enabled: true,
                depth: 50,
                embeddings: false,
                db_path: "~/.max/memory.db".into(),
            },
            buddy: BuddyConfig {
                enabled: true,
                name: "CHIP".into(),
                personality: "curious".into(),
                evolution_stage: 1,
            },
            api: ApiConfig {
                groq_key: std::env::var("GROQ_API_KEY").ok(),
                openai_key: std::env::var("OPENAI_API_KEY").ok(),
                anthropic_key: std::env::var("ANTHROPIC_API_KEY").ok(),
                preferred_api: "groq".into(),
            },
            tools: ToolsConfig {
                sandbox_enabled: true,
                allowed_languages: vec![
                    "python".into(),
                    "javascript".into(),
                    "bash".into(),
                    "rust".into(),
                ],
                timeout_seconds: 30,
            },
        }
    }
}

impl MaxConfig {
    pub fn config_path() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home.join(".max").join("config.toml")
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path();

        if !path.exists() {
            let config = MaxConfig::default();
            config.save()?;
            return Ok(config);
        }

        let contents = fs::read_to_string(&path)?;
        let config: MaxConfig = toml::from_str(&contents).unwrap_or_default();

        // Always refresh API keys from env
        let mut config = config;
        if let Ok(k) = std::env::var("GROQ_API_KEY") {
            config.api.groq_key = Some(k);
        }
        if let Ok(k) = std::env::var("OPENAI_API_KEY") {
            config.api.openai_key = Some(k);
        }
        if let Ok(k) = std::env::var("ANTHROPIC_API_KEY") {
            config.api.anthropic_key = Some(k);
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let toml_str = toml::to_string_pretty(self)?;
        fs::write(&path, toml_str)?;
        Ok(())
    }
}
