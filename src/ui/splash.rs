// ui/splash.rs — MAX Splash Screen
// Created by Ememzyvisuals (Emmanuel Ariyo)

use colored::Colorize;
use crate::config::MaxConfig;

pub fn print_splash(config: &MaxConfig) {
    // Clear line and print
    println!();
    println!("{}", r"  ███╗   ███╗ █████╗ ██╗  ██╗".bright_cyan().bold());
    println!("{}", r"  ████╗ ████║██╔══██╗╚██╗██╔╝".bright_cyan().bold());
    println!("{}", r"  ██╔████╔██║███████║ ╚███╔╝ ".bright_cyan().bold());
    println!("{}", r"  ██║╚██╔╝██║██╔══██║ ██╔██╗ ".bright_cyan().bold());
    println!("{}", r"  ██║ ╚═╝ ██║██║  ██║██╔╝ ██╗".bright_cyan().bold());
    println!("{}", r"  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝".bright_cyan().bold());
    println!();
    println!(
        "  {}  {}",
        "Production CLI AI Agent".bright_white().bold(),
        format!("v{}", config.meta.version).bright_black()
    );
    println!(
        "  {} {}",
        "by".bright_black(),
        "Ememzyvisuals (Emmanuel Ariyo)".bright_yellow().bold()
    );
    println!();
    println!(
        "  {} {}  {} {}  {} {}",
        "GitHub:".bright_black(),
        "github.com/ememzyvisuals".bright_cyan(),
        "·  X:".bright_black(),
        "@ememzyvisuals".bright_cyan(),
        "·  Kaggle:".bright_black(),
        "kaggle.com/ememzyvisuals".bright_cyan()
    );
    println!();

    // Model info
    println!(
        "  {} {}  {} {}",
        "Model:".bright_black(),
        config.model.active.bright_green(),
        "Buddy:".bright_black(),
        if config.buddy.enabled {
            config.buddy.name.bright_magenta()
        } else {
            "off".bright_black()
        }
    );
    println!();
    println!("{}", "  ─────────────────────────────────────────────".bright_black());
    println!();
}
