// ui/animations.rs — Terminal animation helpers
// Created by Ememzyvisuals (Emmanuel Ariyo)

use colored::Colorize;
use std::io::Write;
use std::thread;
use std::time::Duration;

/// Print the compact MAX banner (used after /clear)
pub fn print_mini_banner() {
    println!();
    println!(
        "  {} {}",
        "MAX".bright_cyan().bold(),
        "— Production CLI AI Agent".bright_white()
    );
    println!(
        "  {}",
        "by Ememzyvisuals (Emmanuel Ariyo)".bright_yellow()
    );
    println!("{}", "  ─────────────────────────────────".bright_black());
    println!();
}

/// Typewriter-style text output
pub fn typewrite(text: &str, delay_ms: u64) {
    for ch in text.chars() {
        print!("{}", ch);
        std::io::stdout().flush().ok();
        thread::sleep(Duration::from_millis(delay_ms));
    }
    println!();
}

/// Animated "booting" sequence
pub fn boot_animation() {
    let steps = vec![
        ("Loading model router", 120),
        ("Connecting memory store", 90),
        ("Initializing buddy system", 80),
        ("Registering agent pool", 100),
        ("MAX is ready", 60),
    ];

    for (msg, delay) in steps {
        print!("  {} {}...", "›".bright_cyan(), msg.bright_white());
        std::io::stdout().flush().ok();
        thread::sleep(Duration::from_millis(delay));
        println!(" {}", "✓".bright_green());
    }
    println!();
}

/// Progress bar for long tasks
pub fn progress_bar(label: &str, current: usize, total: usize, width: usize) {
    let pct = if total == 0 { 0 } else { current * 100 / total };
    let fill = current * width / total.max(1);
    let bar = format!(
        "[{}{}] {}%",
        "█".repeat(fill).bright_cyan(),
        "░".repeat(width.saturating_sub(fill)).bright_black(),
        pct
    );
    print!("\r  {} {} ", label.bright_white(), bar);
    std::io::stdout().flush().ok();
    if current >= total {
        println!();
    }
}

/// Print a section divider
pub fn divider(title: &str) {
    let line = "─".repeat(40);
    println!();
    println!(
        "  {} {} {}",
        line[..4].bright_black(),
        title.bright_yellow().bold(),
        line[4..].bright_black()
    );
    println!();
}

/// Success / error banners
pub fn success(msg: &str) {
    println!("  {} {}", "✓".bright_green().bold(), msg.bright_white());
}

pub fn error(msg: &str) {
    eprintln!("  {} {}", "✗".bright_red().bold(), msg.bright_white());
}

pub fn warn(msg: &str) {
    println!("  {} {}", "!".bright_yellow().bold(), msg.bright_white());
}

pub fn info(msg: &str) {
    println!("  {} {}", "·".bright_cyan(), msg.bright_white());
}
