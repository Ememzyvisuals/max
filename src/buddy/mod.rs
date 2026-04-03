// buddy/mod.rs — MAX /buddy AI Companion System
// Created by Ememzyvisuals (Emmanuel Ariyo)

use anyhow::Result;
use colored::Colorize;
use rand::Rng;

use crate::config::MaxConfig;
use crate::memory::MemoryStore;

#[derive(Debug, Clone)]
pub struct BuddyState {
    pub name: String,
    pub mood: String,
    pub energy: u8,
    pub xp: u32,
    pub stage: u8,
    pub interactions: u32,
    pub last_seen: String,
}

impl BuddyState {
    pub fn stage_name(&self) -> &str {
        match self.stage {
            1 => "Sprite",
            2 => "Companion",
            3 => "Guardian",
            4 => "Sage",
            5 => "Legend",
            _ => "Unknown",
        }
    }

    pub fn avatar(&self) -> &str {
        match self.stage {
            1 => "◉",
            2 => "◈",
            3 => "◆",
            4 => "★",
            5 => "✦",
            _ => "○",
        }
    }

    pub fn color_avatar(&self) -> colored::ColoredString {
        match self.stage {
            1 => self.avatar().bright_cyan().bold(),
            2 => self.avatar().bright_green().bold(),
            3 => self.avatar().bright_yellow().bold(),
            4 => self.avatar().bright_magenta().bold(),
            5 => self.avatar().bright_white().bold(),
            _ => self.avatar().normal(),
        }
    }
}

pub async fn run(action: String, config: &MaxConfig) -> Result<()> {
    if !config.buddy.enabled {
        println!("{}", "  /buddy is disabled. Enable in config.".bright_black());
        return Ok(());
    }

    let mut store = MemoryStore::new(config)?;
    let mut state = load_buddy_state(&store, config)?;

    match action.as_str() {
        "status" | "show" => show_status(&state),
        "feed" => {
            state.energy = (state.energy + 20).min(100);
            state.xp += 10;
            state.mood = "happy".to_string();
            save_buddy_state(&mut store, &state)?;
            animate_feed(&state);
        }
        "play" => {
            if state.energy < 20 {
                println!("  {} is too tired to play. Try feeding first!", state.name.bright_cyan());
                return Ok(());
            }
            state.energy = state.energy.saturating_sub(15);
            state.xp += 25;
            state.interactions += 1;
            state.mood = "excited".to_string();
            save_buddy_state(&mut store, &state)?;
            animate_play(&state);
        }
        "evolve" => {
            if state.xp >= xp_threshold(state.stage) && state.stage < 5 {
                state.stage += 1;
                state.xp = 0;
                save_buddy_state(&mut store, &state)?;
                animate_evolve(&state);
            } else if state.stage >= 5 {
                println!("  {} has reached maximum evolution!", state.name.bright_magenta().bold());
            } else {
                let needed = xp_threshold(state.stage) - state.xp;
                println!(
                    "  Not enough XP. Need {} more XP to evolve.",
                    needed.to_string().bright_yellow()
                );
            }
        }
        "reset" => {
            let default = default_buddy_state(config);
            save_buddy_state(&mut store, &default)?;
            println!("  {} has been reset.", default.name.bright_cyan());
        }
        _ => {
            println!(
                "  {} Unknown buddy action: {}",
                "[WARN]".bright_yellow(),
                action
            );
            println!("  Actions: status | feed | play | evolve | reset");
        }
    }

    Ok(())
}

fn load_buddy_state(store: &MemoryStore, config: &MaxConfig) -> Result<BuddyState> {
    let name = store
        .get_buddy_state("name")?
        .unwrap_or_else(|| config.buddy.name.clone());
    let mood = store
        .get_buddy_state("mood")?
        .unwrap_or_else(|| "curious".to_string());
    let energy: u8 = store
        .get_buddy_state("energy")?
        .and_then(|v| v.parse().ok())
        .unwrap_or(80);
    let xp: u32 = store
        .get_buddy_state("xp")?
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let stage: u8 = store
        .get_buddy_state("stage")?
        .and_then(|v| v.parse().ok())
        .unwrap_or(config.buddy.evolution_stage);
    let interactions: u32 = store
        .get_buddy_state("interactions")?
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    Ok(BuddyState {
        name,
        mood,
        energy,
        xp,
        stage,
        interactions,
        last_seen: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
    })
}

fn save_buddy_state(store: &mut MemoryStore, state: &BuddyState) -> Result<()> {
    store.set_buddy_state("name", &state.name)?;
    store.set_buddy_state("mood", &state.mood)?;
    store.set_buddy_state("energy", &state.energy.to_string())?;
    store.set_buddy_state("xp", &state.xp.to_string())?;
    store.set_buddy_state("stage", &state.stage.to_string())?;
    store.set_buddy_state("interactions", &state.interactions.to_string())?;
    Ok(())
}

fn default_buddy_state(config: &MaxConfig) -> BuddyState {
    BuddyState {
        name: config.buddy.name.clone(),
        mood: "curious".to_string(),
        energy: 80,
        xp: 0,
        stage: 1,
        interactions: 0,
        last_seen: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
    }
}

fn xp_threshold(stage: u8) -> u32 {
    match stage {
        1 => 100,
        2 => 300,
        3 => 600,
        4 => 1000,
        _ => u32::MAX,
    }
}

fn show_status(state: &BuddyState) {
    let next_xp = xp_threshold(state.stage);
    let xp_bar_fill = if next_xp < u32::MAX {
        (state.xp * 20 / next_xp).min(20) as usize
    } else {
        20
    };

    let xp_bar = format!(
        "[{}{}]",
        "█".repeat(xp_bar_fill).bright_magenta(),
        "░".repeat(20 - xp_bar_fill).bright_black()
    );

    let energy_fill = (state.energy as usize * 20 / 100).min(20);
    let energy_color = if state.energy > 60 {
        "█".repeat(energy_fill).bright_green()
    } else if state.energy > 30 {
        "█".repeat(energy_fill).bright_yellow()
    } else {
        "█".repeat(energy_fill).bright_red()
    };

    let energy_bar = format!("[{}{}]", energy_color, "░".repeat(20 - energy_fill).bright_black());

    println!();
    println!("  {} {} — {}", state.color_avatar(), state.name.bright_cyan().bold(), state.stage_name().bright_white());
    println!("{}", "  ────────────────────────────────────".bright_black());
    println!("  {} {}", "Mood:        ".bright_black(), mood_display(&state.mood));
    println!("  {} {} {}", "Energy:      ".bright_black(), energy_bar, format!("{}%", state.energy).bright_white());
    println!("  {} {} {}/{}", "XP:          ".bright_black(), xp_bar, state.xp.to_string().bright_white(), if next_xp < u32::MAX { next_xp.to_string() } else { "MAX".to_string() });
    println!("  {} {}", "Interactions:".bright_black(), state.interactions.to_string().bright_yellow());
    println!("  {} {}", "Stage:       ".bright_black(), format!("{}/5 — {}", state.stage, state.stage_name()).bright_magenta());
    println!();

    // Random buddy message
    let messages = buddy_messages(&state.mood);
    let mut rng = rand::thread_rng();
    let msg = messages[rng.gen_range(0..messages.len())];
    println!("  {} {}", format!("{}:", state.name).bright_cyan().bold(), msg.bright_white().italic());
    println!();
}

fn mood_display(mood: &str) -> colored::ColoredString {
    match mood {
        "happy" => "😊 Happy".bright_green(),
        "excited" => "⚡ Excited".bright_yellow(),
        "curious" => "🔍 Curious".bright_cyan(),
        "tired" => "😴 Tired".bright_black(),
        "bored" => "😑 Bored".bright_red(),
        _ => mood.bright_white(),
    }
}

fn animate_feed(state: &BuddyState) {
    println!();
    println!("  {} {}", state.color_avatar(), "nom nom nom...".bright_green());
    std::thread::sleep(std::time::Duration::from_millis(400));
    println!("  {} {} {} energy +20, XP +10!", state.color_avatar(), state.name.bright_cyan().bold(), "feels better!".bright_green());
    println!();
}

fn animate_play(state: &BuddyState) {
    let frames = vec!["◉", "◈", "◉", "◆", "◉"];
    for frame in &frames {
        print!("\r  {} {}", frame.bright_yellow().bold(), "wheeeee!   ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
    println!();
    println!("  {} {} had fun! XP +25", state.color_avatar(), state.name.bright_cyan().bold());
    println!();
}

fn animate_evolve(state: &BuddyState) {
    println!();
    for _ in 0..3 {
        print!("\r  {} {} EVOLVING...   ", "✦".bright_magenta().bold(), state.name.bright_white().bold());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
        print!("\r  {} {} evolving...   ", "★".bright_yellow().bold(), state.name.bright_white().bold());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    println!();
    println!();
    println!(
        "  {} {} evolved to {} {}!",
        "✦".bright_magenta().bold(),
        state.name.bright_cyan().bold(),
        state.stage_name().bright_yellow().bold(),
        format!("(Stage {})", state.stage).bright_white()
    );
    println!();
}

fn buddy_messages(mood: &str) -> Vec<&'static str> {
    match mood {
        "happy" => vec![
            "I love when we build things together!",
            "You're on fire today! Keep going!",
            "This is the best day ever.",
        ],
        "excited" => vec![
            "LET'S GOOO! What are we building next?!",
            "My circuits are buzzing with excitement!",
            "I can barely contain my processing power!",
        ],
        "tired" => vec![
            "zzzz... please feed me...",
            "Need... more... energy...",
            "My energy levels are critically low.",
        ],
        _ => vec![
            "What shall we build today?",
            "I'm ready to help you create something amazing.",
            "Feed me a task. I'm hungry for work.",
            "Awaiting instructions, developer.",
        ],
    }
}
