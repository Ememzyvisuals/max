// model/commands.rs
use anyhow::Result;
use colored::Colorize;
use crate::config::MaxConfig;

pub async fn run(action: String, config: &MaxConfig) -> Result<()> {
    match action.as_str() {
        "list" => list_models(config),
        "info" => model_info(config),
        "set" => {
            println!("{}", "  Usage: Edit ~/.max/config.toml and set model.active".bright_cyan());
            println!("  Available models:");
            println!("    groq/llama3-70b-8192");
            println!("    groq/llama3-8b-8192");
            println!("    groq/mixtral-8x7b-32768");
            println!("    groq/gemma-7b-it");
            println!("    openai/gpt-4-turbo");
            println!("    openai/gpt-3.5-turbo");
            println!("    anthropic/claude-3-5-sonnet");
            println!("    ollama/mistral");
            println!("    ollama/llama3");
            println!("    ollama/codellama");
            println!("    ollama/starcoder");
        }
        "download" => {
            println!("{}", "  Offline model download via Ollama:".bright_cyan().bold());
            println!("  Install Ollama: https://ollama.ai");
            println!();
            let models = vec![
                ("ollama pull mistral", "Mistral 7B — General purpose"),
                ("ollama pull llama3", "LLaMA 3 8B — General purpose"),
                ("ollama pull codellama", "Code LLaMA — Code generation"),
                ("ollama pull starcoder2", "StarCoder 2 — Code generation"),
                ("ollama pull mixtral", "Mixtral 8x7B — High performance"),
            ];
            for (cmd, desc) in models {
                println!("  {} {}", cmd.bright_yellow(), format!("# {}", desc).bright_black());
            }
            println!();
            println!("  Then set: model.active = \"ollama/<model-name>\" in config");
        }
        _ => {
            eprintln!("{} Unknown action: {}", "  [WARN]".bright_yellow(), action);
        }
    }
    Ok(())
}

fn list_models(config: &MaxConfig) {
    println!();
    println!("{}", "  AVAILABLE MODELS".bright_yellow().bold());
    println!("{}", "  ────────────────────────────────────────────".bright_black());

    let categories = vec![
        ("GROQ (Fast API)", vec![
            ("groq/llama3-70b-8192", "LLaMA 3 70B — Best quality"),
            ("groq/llama3-8b-8192", "LLaMA 3 8B — Fast"),
            ("groq/mixtral-8x7b-32768", "Mixtral — Long context"),
        ]),
        ("OPENAI", vec![
            ("openai/gpt-4-turbo", "GPT-4 Turbo — Premium"),
            ("openai/gpt-3.5-turbo", "GPT-3.5 — Fast & cheap"),
        ]),
        ("ANTHROPIC", vec![
            ("anthropic/claude-3-5-sonnet", "Claude 3.5 Sonnet — Excellent"),
        ]),
        ("OLLAMA (Offline)", vec![
            ("ollama/mistral", "Mistral 7B — Offline"),
            ("ollama/llama3", "LLaMA 3 8B — Offline"),
            ("ollama/codellama", "Code LLaMA — Offline code"),
            ("ollama/starcoder2", "StarCoder 2 — Offline code"),
        ]),
    ];

    for (category, models) in categories {
        println!("  {}", category.bright_cyan().bold());
        for (id, desc) in models {
            let active = if id == config.model.active { " ◄ active".bright_green().bold() } else { "".normal() };
            println!("    {:<35} {}{}", id.bright_white(), desc.bright_black(), active);
        }
        println!();
    }
}

fn model_info(config: &MaxConfig) {
    println!();
    println!("  {} {}", "Active model:".bright_cyan(), config.model.active.bright_white().bold());
    println!("  {} {}", "Temperature: ".bright_cyan(), config.model.temperature.to_string().bright_white());
    println!("  {} {}", "Max tokens:  ".bright_cyan(), config.model.max_tokens.to_string().bright_white());
    println!("  {} {}", "Fallback API:".bright_cyan(), config.model.fallback_to_api.to_string().bright_white());
    println!();
}
