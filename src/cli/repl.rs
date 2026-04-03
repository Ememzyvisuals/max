// cli/repl.rs — Interactive REPL for MAX
// Created by Ememzyvisuals (Emmanuel Ariyo)

use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

use crate::agent::orchestrator;
use crate::config::MaxConfig;
use crate::memory::MemoryStore;
use crate::model::ModelRouter;
use crate::ui::animations;

pub async fn start(config: &MaxConfig) -> Result<()> {
    println!();
    println!(
        "{}",
        "  MAX interactive session. Type /help for commands, /exit to quit."
            .bright_cyan()
            .bold()
    );
    println!();

    let mut memory = MemoryStore::new(config)?;
    let router = ModelRouter::new(config);
    let mut history: Vec<(String, String)> = Vec::new();

    loop {
        // Prompt
        print!("{} {} ", "MAX".bright_yellow().bold(), "›".bright_white());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_string();

        if input.is_empty() {
            continue;
        }

        // Slash commands
        match input.as_str() {
            "/exit" | "/quit" | "/q" => {
                println!("{}", "\n  Goodbye. MAX is offline.\n".bright_magenta());
                break;
            }
            "/help" => {
                print_help();
                continue;
            }
            "/clear" => {
                print!("\x1B[2J\x1B[1;1H");
                animations::print_mini_banner();
                continue;
            }
            "/history" => {
                print_history(&history);
                continue;
            }
            "/memory" => {
                let mems = memory.recent(10)?;
                if mems.is_empty() {
                    println!("{}", "  No memories stored yet.".dimmed());
                } else {
                    for (i, m) in mems.iter().enumerate() {
                        println!("  {}  {}", format!("[{}]", i + 1).bright_cyan(), m.dimmed());
                    }
                }
                continue;
            }
            "/buddy" => {
                crate::buddy::run("status".to_string(), config).await?;
                continue;
            }
            _ if input.starts_with("/plan ") => {
                let task = input[6..].trim().to_string();
                orchestrator::ultraplan(task, 3, config).await?;
                continue;
            }
            _ if input.starts_with("/code ") => {
                let prompt = input[6..].trim().to_string();
                crate::cli::commands::code::run(prompt, "python".into(), None, config).await?;
                continue;
            }
            _ => {}
        }

        // Regular AI chat
        let spinner = crate::ui::spinner::Spinner::new("MAX is thinking...");
        spinner.start();

        let context = build_context(&history, &input);
        match router.complete(&context, config).await {
            Ok(response) => {
                spinner.stop();
                print_response(&response);
                history.push((input.clone(), response.clone()));
                memory.store(&input, &response)?;
            }
            Err(e) => {
                spinner.stop();
                eprintln!("{} {}", "  [ERROR]".bright_red().bold(), e);
            }
        }
    }

    Ok(())
}

fn build_context(history: &[(String, String)], current: &str) -> String {
    let mut ctx = String::from(
        "You are MAX, a production-grade AI agent built for developers. \
         You are helpful, precise, and technically excellent. \
         Created by Ememzyvisuals (Emmanuel Ariyo).\n\n",
    );

    for (user, assistant) in history.iter().rev().take(6).collect::<Vec<_>>().iter().rev() {
        ctx.push_str(&format!("User: {}\nMAX: {}\n\n", user, assistant));
    }

    ctx.push_str(&format!("User: {}\nMAX:", current));
    ctx
}

fn print_response(response: &str) {
    println!();
    println!(
        "{} {}",
        "  MAX".bright_yellow().bold(),
        "›".bright_white()
    );
    println!();

    // Syntax-aware printing with line wrap
    for line in response.lines() {
        if line.starts_with("```") {
            println!("  {}", line.bright_cyan());
        } else if line.starts_with('#') {
            println!("  {}", line.bright_white().bold());
        } else if line.starts_with("- ") || line.starts_with("* ") {
            println!("  {}", line.bright_green());
        } else {
            println!("  {}", line);
        }
    }

    println!();
}

fn print_history(history: &[(String, String)]) {
    if history.is_empty() {
        println!("{}", "  No history yet.".dimmed());
        return;
    }
    println!();
    for (i, (user, assistant)) in history.iter().enumerate() {
        println!("  {} {}", format!("[{}] You:", i + 1).bright_cyan(), user);
        let preview: String = assistant.chars().take(80).collect();
        println!(
            "      {} {}{}",
            "MAX:".bright_yellow(),
            preview,
            if assistant.len() > 80 { "..." } else { "" }
        );
        println!();
    }
}

fn print_help() {
    println!();
    println!("{}", "  MAX COMMANDS".bright_yellow().bold());
    println!("{}", "  ─────────────────────────────────────────".bright_black());
    let cmds = vec![
        ("/help", "Show this help menu"),
        ("/exit", "Exit MAX"),
        ("/clear", "Clear terminal"),
        ("/history", "Show conversation history"),
        ("/memory", "Show recent memories"),
        ("/buddy", "Check your /buddy companion"),
        ("/plan <task>", "Run ULTRAPLAN multi-agent on a task"),
        ("/code <prompt>", "Generate code from prompt"),
    ];
    for (cmd, desc) in cmds {
        println!(
            "  {:<20} {}",
            cmd.bright_cyan().bold(),
            desc.bright_white()
        );
    }
    println!();
    println!("{}", "  Or just type naturally to chat with MAX.".dimmed());
    println!();
}
