use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rpassword::read_password;
use std::{thread, time::Duration};

pub fn sleep(secs: u64) {
    thread::sleep(Duration::from_secs(secs));
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

pub fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap_or_default();
    input.trim().to_string()
}

pub fn get_user_input_with_prompt(prompt: &str, hide_input: bool) -> String {
    println!("{}", prompt.bright_blue());
    if hide_input {
        read_password().unwrap_or_default()
    } else {
        get_user_input()
    }
}

pub fn create_progress_bar(length: u64) -> ProgressBar {
    let pb = ProgressBar::new(length);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
        .unwrap()
        .progress_chars("=>-"));
    pb
}

// Display helper functions for consistent UI
pub fn show_header(message: &str) {
    println!("\n{}", message.bright_blue());
    sleep(1);
}

pub fn show_success(message: &str) {
    println!("\n{}\n", message.bright_green());
    sleep(1);
}

pub fn show_warning(message: &str) {
    println!("{}", message.bright_yellow());
}
