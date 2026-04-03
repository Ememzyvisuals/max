// tools/sandbox.rs — Safe code execution sandbox
// Created by Ememzyvisuals (Emmanuel Ariyo)

use anyhow::{anyhow, Result};
use colored::Colorize;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use crate::config::MaxConfig;

/// Detect language from file extension
fn detect_language(file: &str) -> Option<&'static str> {
    let ext = Path::new(file).extension()?.to_str()?;
    match ext {
        "py" => Some("python"),
        "js" | "mjs" => Some("javascript"),
        "ts" => Some("typescript"),
        "sh" | "bash" => Some("bash"),
        "rs" => Some("rust"),
        "go" => Some("go"),
        "rb" => Some("ruby"),
        "php" => Some("php"),
        _ => None,
    }
}

/// Build the shell command to run a file in the given language
fn build_command(lang: &str, file: &str) -> Result<(String, Vec<String>)> {
    let cmd = match lang {
        "python" => ("python3".to_string(), vec![file.to_string()]),
        "javascript" => ("node".to_string(), vec![file.to_string()]),
        "typescript" => ("npx".to_string(), vec!["ts-node".to_string(), file.to_string()]),
        "bash" => ("bash".to_string(), vec![file.to_string()]),
        "ruby" => ("ruby".to_string(), vec![file.to_string()]),
        "go" => ("go".to_string(), vec!["run".to_string(), file.to_string()]),
        "php" => ("php".to_string(), vec![file.to_string()]),
        "rust" => {
            // For Rust, compile then run
            return Err(anyhow!(
                "Use 'cargo run' for Rust files or compile with 'rustc {}' first.",
                file
            ));
        }
        _ => return Err(anyhow!("Unsupported language: {}", lang)),
    };
    Ok(cmd)
}

/// Execute a file in the sandbox
pub async fn execute_file(file: &str, config: &MaxConfig) -> Result<()> {
    // Check file exists
    if !Path::new(file).exists() {
        return Err(anyhow!("File not found: {}", file));
    }

    let lang = detect_language(file)
        .ok_or_else(|| anyhow!("Cannot detect language for: {}", file))?;

    // Check allowed languages
    if !config.tools.allowed_languages.contains(&lang.to_string()) {
        return Err(anyhow!(
            "Language '{}' is not in the allowed list. Edit ~/.max/config.toml to add it.",
            lang
        ));
    }

    // Check if runtime is installed
    let (cmd, args) = build_command(lang, file)?;
    if which::which(&cmd).is_err() {
        return Err(anyhow!(
            "Runtime '{}' not found. Please install it to run {} files.",
            cmd, lang
        ));
    }

    println!();
    println!(
        "  {} {} {}  {}",
        "RUN".bright_green().bold(),
        "›".bright_white(),
        file.bright_cyan(),
        format!("[{}]", lang).bright_black()
    );
    println!("{}", "  ─────────────────────────────────────────────".bright_black());
    println!();

    let timeout = Duration::from_secs(config.tools.timeout_seconds as u64);
    let start = Instant::now();

    // Execute with timeout
    let mut child = Command::new(&cmd)
        .args(&args)
        .spawn()
        .map_err(|e| anyhow!("Failed to spawn process: {}", e))?;

    // Poll for completion with timeout
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let elapsed = start.elapsed();
                println!();
                println!("{}", "  ─────────────────────────────────────────────".bright_black());
                if status.success() {
                    println!(
                        "  {} Exited successfully in {:.2}s",
                        "✓".bright_green().bold(),
                        elapsed.as_secs_f64()
                    );
                } else {
                    let code = status.code().unwrap_or(-1);
                    println!(
                        "  {} Process exited with code {} in {:.2}s",
                        "✗".bright_red().bold(),
                        code.to_string().bright_red(),
                        elapsed.as_secs_f64()
                    );
                }
                println!();
                return Ok(());
            }
            Ok(None) => {
                // Still running
                if start.elapsed() >= timeout {
                    child.kill().ok();
                    return Err(anyhow!(
                        "Process timed out after {}s",
                        config.tools.timeout_seconds
                    ));
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            Err(e) => {
                return Err(anyhow!("Error waiting on process: {}", e));
            }
        }
    }
}
