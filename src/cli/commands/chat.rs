// cli/commands/chat.rs — Single-turn chat command
// Created by Ememzyvisuals (Emmanuel Ariyo)

use anyhow::Result;
use colored::Colorize;

use crate::config::MaxConfig;
use crate::model::ModelRouter;
use crate::ui::spinner::Spinner;

pub async fn run(
    message: Option<String>,
    model_override: Option<String>,
    config: &MaxConfig,
) -> Result<()> {
    let msg = match message {
        Some(m) => m,
        None => {
            // Drop into REPL
            return crate::cli::repl::start(config).await;
        }
    };

    let mut cfg = config.clone();
    if let Some(m) = model_override {
        cfg.model.active = m;
    }

    let prompt = format!(
        "You are MAX, a production AI agent by Ememzyvisuals.\nUser: {}\nMAX:",
        msg
    );

    let spinner = Spinner::new("MAX is processing...");
    spinner.start();

    let router = ModelRouter::new(&cfg);
    match router.complete(&prompt, &cfg).await {
        Ok(response) => {
            spinner.stop();
            println!();
            println!("{}", "  ─────────────────────────────".bright_black());
            for line in response.lines() {
                println!("  {}", line);
            }
            println!("{}", "  ─────────────────────────────".bright_black());
            println!();
        }
        Err(e) => {
            spinner.stop();
            eprintln!("{} {}", "[ERROR]".bright_red().bold(), e);
        }
    }

    Ok(())
}
