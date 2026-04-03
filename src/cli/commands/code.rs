// cli/commands/code.rs — AI-powered code generation
// Created by Ememzyvisuals (Emmanuel Ariyo)

use anyhow::Result;
use colored::Colorize;
use std::fs;

use crate::config::MaxConfig;
use crate::model::ModelRouter;
use crate::ui::spinner::Spinner;

pub async fn run(
    prompt: String,
    lang: String,
    output: Option<String>,
    config: &MaxConfig,
) -> Result<()> {
    println!();
    println!(
        "  {} {} {}",
        "CODE".bright_cyan().bold(),
        "›".bright_white(),
        prompt.bright_white()
    );
    println!(
        "  {} {}",
        "Language:".bright_black(),
        lang.bright_yellow()
    );
    println!();

    let system_prompt = format!(
        "You are MAX Code Engine — an expert {} developer.\n\
         Generate clean, production-quality, well-commented {} code.\n\
         Respond ONLY with the code block, no explanation outside the code.\n\
         Use proper formatting and best practices.\n\
         Add a brief comment header with what the code does.\n\
         Task: {}\n\
         Code:",
        lang, lang, prompt
    );

    let spinner = Spinner::new(&format!("Generating {} code...", lang));
    spinner.start();

    let router = ModelRouter::new(config);
    match router.complete(&system_prompt, config).await {
        Ok(code) => {
            spinner.stop();

            // Print syntax-highlighted code block
            println!("{}", format!("  ┌─ {}", lang).bright_cyan().bold());
            for line in code.lines() {
                println!("  │ {}", line.bright_white());
            }
            println!("{}", "  └─────────────────────────────".bright_cyan());
            println!();

            // Save to file if output path specified
            if let Some(path) = output {
                fs::write(&path, &code)?;
                println!(
                    "  {} {}",
                    "Saved to:".bright_green().bold(),
                    path.bright_white()
                );
                println!();
            } else {
                println!(
                    "  {}",
                    "Tip: Use --output <file> to save this code.".dimmed()
                );
            }
        }
        Err(e) => {
            spinner.stop();
            eprintln!("{} {}", "  [ERROR]".bright_red().bold(), e);
        }
    }

    Ok(())
}
