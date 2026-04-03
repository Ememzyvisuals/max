// ============================================================
//  MAX — Production CLI AI Agent
//  Created by Ememzyvisuals (Emmanuel Ariyo)
//  https://github.com/ememzyvisuals
// ============================================================

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod agent;
mod buddy;
mod cli;
mod config;
mod memory;
mod model;
mod tools;
mod ui;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use tracing_subscriber::{fmt, EnvFilter};

use crate::cli::{Cli, Commands};
use crate::config::MaxConfig;
use crate::ui::splash::print_splash;
use crate::ui::spinner::Spinner;

#[tokio::main]
async fn main() -> Result<()> {
    // Init logging
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    let cli = Cli::parse();

    // Load config
    let config = MaxConfig::load()?;

    // Splash screen (unless --quiet)
    if !cli.quiet {
        print_splash(&config);
    }

    match cli.command {
        Some(Commands::Chat { message, model }) => {
            cli::commands::chat::run(message, model, &config).await?;
        }
        Some(Commands::Buddy { action }) => {
            buddy::run(action, &config).await?;
        }
        Some(Commands::Plan { task, agents }) => {
            agent::orchestrator::ultraplan(task, agents, &config).await?;
        }
        Some(Commands::Code { prompt, lang, output }) => {
            cli::commands::code::run(prompt, lang, output, &config).await?;
        }
        Some(Commands::Run { file }) => {
            tools::sandbox::execute_file(&file, &config).await?;
        }
        Some(Commands::Memory { action }) => {
            memory::commands::run(action, &config).await?;
        }
        Some(Commands::Config { action }) => {
            config::commands::run(action, &config).await?;
        }
        Some(Commands::Models { action }) => {
            model::commands::run(action, &config).await?;
        }
        None => {
            // Interactive REPL mode
            cli::repl::start(&config).await?;
        }
    }

    Ok(())
}
