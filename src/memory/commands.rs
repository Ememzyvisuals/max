// memory/commands.rs
use anyhow::Result;
use colored::Colorize;
use crate::config::MaxConfig;
use crate::memory::MemoryStore;

pub async fn run(action: String, config: &MaxConfig) -> Result<()> {
    let store = MemoryStore::new(config)?;

    match action.as_str() {
        "list" => {
            let count = store.count()?;
            println!();
            println!("  {} {} memories stored", "Memory:".bright_cyan().bold(), count.to_string().bright_yellow());
            println!();
            let recent = store.recent(10)?;
            for (i, mem) in recent.iter().enumerate() {
                let preview: String = mem.chars().take(70).collect();
                println!("  {} {}{}", format!("[{}]", i + 1).bright_black(), preview, if mem.len() > 70 { "..." } else { "" });
            }
            if recent.is_empty() {
                println!("  {}", "No memories yet. Start chatting!".dimmed());
            }
            println!();
        }
        "clear" => {
            let mut store2 = MemoryStore::new(config)?;
            let deleted = store2.clear()?;
            println!("  {} {} memories cleared.", "✓".bright_green(), deleted.to_string().bright_yellow());
        }
        _ if action.starts_with("search ") => {
            let query = &action[7..];
            let results = store.search(query)?;
            println!();
            println!("  {} '{}' → {} results", "Search:".bright_cyan(), query.bright_white(), results.len().to_string().bright_yellow());
            println!();
            for m in &results {
                println!("  {} {}", m.timestamp.bright_black(), m.user_input.bright_white());
                let resp_preview: String = m.ai_response.chars().take(80).collect();
                println!("    {}{}", resp_preview.dimmed(), if m.ai_response.len() > 80 { "..." } else { "" });
                println!();
            }
        }
        _ => {
            println!("{}", "  Usage: max memory [list|clear|search <query>]".bright_cyan());
        }
    }
    Ok(())
}
