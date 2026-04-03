// model/mod.rs — ModelRouter: Offline LLM + API Fallback
// Created by Ememzyvisuals (Emmanuel Ariyo)
// Supports: Groq, OpenAI, Anthropic, local Ollama

pub mod commands;

use anyhow::{anyhow, Result};
use colored::Colorize;
use reqwest::Client;
use serde_json::{json, Value};

use crate::config::MaxConfig;

pub struct ModelRouter {
    client: Client,
}

impl ModelRouter {
    pub fn new(_config: &MaxConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    pub async fn complete(&self, prompt: &str, config: &MaxConfig) -> Result<String> {
        let model = &config.model.active;

        // Route based on model prefix
        if model.starts_with("groq/") {
            if let Some(key) = &config.api.groq_key {
                return self.groq_complete(prompt, model, key, config).await;
            }
        }

        if model.starts_with("openai/") || model.starts_with("gpt-") {
            if let Some(key) = &config.api.openai_key {
                return self.openai_complete(prompt, model, key, config).await;
            }
        }

        if model.starts_with("anthropic/") || model.starts_with("claude-") {
            if let Some(key) = &config.api.anthropic_key {
                return self.anthropic_complete(prompt, model, key, config).await;
            }
        }

        if model.starts_with("ollama/") {
            return self.ollama_complete(prompt, model, config).await;
        }

        // Fallback chain
        if config.model.fallback_to_api {
            if let Some(key) = &config.api.groq_key {
                eprintln!("{}", "  [MAX] Falling back to Groq...".bright_black());
                return self.groq_complete(prompt, "groq/llama3-70b-8192", key, config).await;
            }
            if let Some(key) = &config.api.openai_key {
                eprintln!("{}", "  [MAX] Falling back to OpenAI...".bright_black());
                return self.openai_complete(prompt, "openai/gpt-4-turbo", key, config).await;
            }
        }

        Err(anyhow!(
            "No model available. Set GROQ_API_KEY, OPENAI_API_KEY, or configure Ollama."
        ))
    }

    // ── Groq API ────────────────────────────────────────────────────────────
    async fn groq_complete(
        &self,
        prompt: &str,
        model: &str,
        api_key: &str,
        config: &MaxConfig,
    ) -> Result<String> {
        let model_name = model.trim_start_matches("groq/");

        let body = json!({
            "model": model_name,
            "messages": [
                { "role": "user", "content": prompt }
            ],
            "temperature": config.model.temperature,
            "max_tokens": config.model.max_tokens,
        });

        let response = self
            .client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let err_text = response.text().await?;
            return Err(anyhow!("Groq API error: {}", err_text));
        }

        let json: Value = response.json().await?;
        let text = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Unexpected Groq response format"))?
            .to_string();

        Ok(text)
    }

    // ── OpenAI API ──────────────────────────────────────────────────────────
    async fn openai_complete(
        &self,
        prompt: &str,
        model: &str,
        api_key: &str,
        config: &MaxConfig,
    ) -> Result<String> {
        let model_name = model
            .trim_start_matches("openai/")
            .replace("gpt-4-turbo", "gpt-4-turbo-preview");

        let body = json!({
            "model": model_name,
            "messages": [
                {
                    "role": "system",
                    "content": "You are MAX, a production AI agent created by Ememzyvisuals (Emmanuel Ariyo)."
                },
                { "role": "user", "content": prompt }
            ],
            "temperature": config.model.temperature,
            "max_tokens": config.model.max_tokens,
        });

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let err_text = response.text().await?;
            return Err(anyhow!("OpenAI API error: {}", err_text));
        }

        let json: Value = response.json().await?;
        let text = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Unexpected OpenAI response format"))?
            .to_string();

        Ok(text)
    }

    // ── Anthropic API ────────────────────────────────────────────────────────
    async fn anthropic_complete(
        &self,
        prompt: &str,
        _model: &str,
        api_key: &str,
        config: &MaxConfig,
    ) -> Result<String> {
        let body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": config.model.max_tokens,
            "system": "You are MAX, a production AI agent created by Ememzyvisuals (Emmanuel Ariyo).",
            "messages": [
                { "role": "user", "content": prompt }
            ]
        });

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let err_text = response.text().await?;
            return Err(anyhow!("Anthropic API error: {}", err_text));
        }

        let json: Value = response.json().await?;
        let text = json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow!("Unexpected Anthropic response format"))?
            .to_string();

        Ok(text)
    }

    // ── Ollama (local) ───────────────────────────────────────────────────────
    async fn ollama_complete(
        &self,
        prompt: &str,
        model: &str,
        config: &MaxConfig,
    ) -> Result<String> {
        let model_name = model.trim_start_matches("ollama/");

        let body = json!({
            "model": model_name,
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": config.model.temperature,
                "num_predict": config.model.max_tokens,
            }
        });

        let response = self
            .client
            .post("http://localhost:11434/api/generate")
            .json(&body)
            .send()
            .await
            .map_err(|_| anyhow!("Ollama not running. Start with: ollama serve"))?;

        let json: Value = response.json().await?;
        let text = json["response"]
            .as_str()
            .ok_or_else(|| anyhow!("Unexpected Ollama response"))?
            .to_string();

        Ok(text)
    }
}
