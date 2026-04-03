// cli/mod.rs — CLI command definitions for MAX
// Created by Ememzyvisuals (Emmanuel Ariyo)

pub mod commands;
pub mod repl;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "max",
    version = "1.0.0",
    author = "Ememzyvisuals (Emmanuel Ariyo) <Ememzyvisuals@gmail.com>",
    about = "MAX — Production CLI AI Agent\nCreated by Ememzyvisuals (Emmanuel Ariyo)\nGitHub: https://github.com/ememzyvisuals",
    long_about = None,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Suppress splash screen
    #[arg(long, short = 'q', global = true)]
    pub quiet: bool,

    /// Override model for this session
    #[arg(long, short = 'm', global = true)]
    pub model: Option<String>,

    /// Enable verbose output
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start a chat session with MAX
    Chat {
        /// Initial message (optional; enters REPL if omitted)
        message: Option<String>,
        /// Model override
        #[arg(long)]
        model: Option<String>,
    },

    /// Interact with your /buddy AI companion
    Buddy {
        /// Action: feed | play | status | evolve | reset
        #[arg(default_value = "status")]
        action: String,
    },

    /// Run ULTRAPLAN multi-agent orchestration
    Plan {
        /// Task description for multi-agent planning
        task: String,
        /// Number of parallel agents (default: 3)
        #[arg(long, short = 'a', default_value = "3")]
        agents: u8,
    },

    /// Generate code with AI assistance
    Code {
        /// Natural language prompt for code generation
        prompt: String,
        /// Target language (rust, python, js, go, etc.)
        #[arg(long, short = 'l', default_value = "python")]
        lang: String,
        /// Output file path
        #[arg(long, short = 'o')]
        output: Option<String>,
    },

    /// Execute a file in the MAX sandbox
    Run {
        /// File to execute
        file: String,
    },

    /// Manage persistent memory
    Memory {
        /// Action: list | search | clear | export
        #[arg(default_value = "list")]
        action: String,
    },

    /// Configure MAX settings
    Config {
        /// Action: show | set | reset
        #[arg(default_value = "show")]
        action: String,
    },

    /// Manage offline and API models
    Models {
        /// Action: list | download | set | info
        #[arg(default_value = "list")]
        action: String,
    },
}
