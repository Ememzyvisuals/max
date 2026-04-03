// config/commands.rs — Config CLI actions
use anyhow::Result;
use colored::Colorize;
use crate::config::MaxConfig;

pub async fn run(action: String, config: &MaxConfig) -> Result<()> {
    match action.as_str() {
        "show" | "list" => show_config(config),
        "reset" => {
            let default = MaxConfig::default();
            default.save()?;
            println!("{}", "  Config reset to defaults.".bright_green());
        }
        _ => {
            println!("{} Unknown config action: {}", "  [WARN]".bright_yellow(), action);
        }
    }
    Ok(())
}

fn show_config(config: &MaxConfig) {
    println!();
    println!("{}", "  MAX CONFIGURATION".bright_yellow().bold());
    println!("{}", "  ──────────────────────────────────────".bright_black());
    println!("  {} {}", "Active Model:".bright_cyan(), config.model.active.bright_white());
    println!("  {} {}", "Temperature: ".bright_cyan(), config.model.temperature.to_string().bright_white());
    println!("  {} {}", "Max Tokens:  ".bright_cyan(), config.model.max_tokens.to_string().bright_white());
    println!("  {} {}", "Theme:       ".bright_cyan(), config.ui.color_theme.bright_white());
    println!("  {} {}", "Buddy:       ".bright_cyan(), if config.buddy.enabled { config.buddy.name.bright_green() } else { "disabled".bright_red() });
    println!("  {} {}", "Memory:      ".bright_cyan(), if config.memory.enabled { "enabled".bright_green() } else { "disabled".bright_red() });
    println!("  {} {}", "Groq API:    ".bright_cyan(), if config.api.groq_key.is_some() { "configured".bright_green() } else { "not set".bright_red() });
    println!("  {} {}", "OpenAI API:  ".bright_cyan(), if config.api.openai_key.is_some() { "configured".bright_green() } else { "not set".bright_red() });
    println!();
    println!("  {} {}", "Config file:".bright_black(), MaxConfig::config_path().display().to_string().dimmed());
    println!();
}
