mod config;
mod models;
mod simulator;
mod utils;

use crate::config::get_user_choices;
use crate::simulator::run_simulation;
use crate::utils::{clear_screen, sleep};
use clap::Parser;
use colored::Colorize;

const ASCII_LOGO: &str = r#"
     █████╗ ██████╗  ██████╗██╗  ██╗████████╗██████╗ ██╗   ██╗
    ██╔══██╗██╔══██╗██╔════╝██║  ██║╚══██╔══╝██╔══██╗╚██╗ ██╔╝
    ███████║██████╔╝██║     ███████║   ██║   ██████╔╝ ╚████╔╝
    ██╔══██║██╔══██╗██║     ██╔══██║   ██║   ██╔══██╗  ╚██╔╝
    ██║  ██║██║  ██║╚██████╗██║  ██║   ██║   ██║  ██║   ██║
    ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝   ╚═╝
"#;

#[derive(Parser)]
#[command(name = "archtry")]
#[command(about = "Arch Linux Installation Simulator")]
#[command(version)]
struct Args {
    /// Disable command hints during simulation
    #[arg(long)]
    no_hints: bool,
}

fn main() {
    let args = Args::parse();

    clear_screen();
    println!("{}", ASCII_LOGO.bright_green());
    println!("{}", "Loading ArchTry...".bright_blue());
    sleep(3);
    let user_choices = get_user_choices();
    run_simulation(&user_choices, args.no_hints);
}
