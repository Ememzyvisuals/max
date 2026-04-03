// ui/spinner.rs — Animated terminal spinner
// Created by Ememzyvisuals (Emmanuel Ariyo)

#![allow(dead_code)]

use colored::Colorize;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Spinner {
    message: String,
    running: Arc<Mutex<bool>>,
}

impl Spinner {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self) {
        let msg = self.message.clone();
        let running = Arc::clone(&self.running);
        *running.lock().unwrap() = true;

        thread::spawn(move || {
            let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let mut i = 0usize;
            while *running.lock().unwrap() {
                let frame = frames[i % frames.len()].bright_cyan().bold();
                print!("\r  {} {}   ", frame, msg.bright_white());
                std::io::stdout().flush().ok();
                thread::sleep(Duration::from_millis(80));
                i += 1;
            }
            print!("\r{}\r", " ".repeat(60));
            std::io::stdout().flush().ok();
        });
    }

    pub fn stop(self) {
        *self.running.lock().unwrap() = false;
        thread::sleep(Duration::from_millis(100));
    }

    pub fn stop_with_message(self, message: &str) {
        *self.running.lock().unwrap() = false;
        thread::sleep(Duration::from_millis(100));
        println!("  {} {}", "✓".bright_green().bold(), message.bright_white());
    }
}

pub struct SimpleSpinner {
    message: String,
}

impl SimpleSpinner {
    pub fn new(message: &str) -> Self {
        Self { message: message.to_string() }
    }

    pub fn start(&self) {
        print!("  {} {}   ", "⠋".bright_cyan(), self.message.bright_white());
        std::io::stdout().flush().ok();
    }

    pub fn stop(&self) {
        print!("\r{}\r", " ".repeat(60));
        std::io::stdout().flush().ok();
    }
}
