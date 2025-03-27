mod config;
mod models;
mod simulator;
mod utils;

use colored::Colorize;
use crate::config::get_user_choices;
use crate::simulator::run_simulation;
use crate::utils::{clear_screen, sleep};

const ASCII_LOGO: &str = r#"
     █████╗ ██████╗  ██████╗██╗  ██╗████████╗██████╗ ██╗   ██╗
    ██╔══██╗██╔══██╗██╔════╝██║  ██║╚══██╔══╝██╔══██╗╚██╗ ██╔╝
    ███████║██████╔╝██║     ███████║   ██║   ██████╔╝ ╚████╔╝ 
    ██╔══██║██╔══██╗██║     ██╔══██║   ██║   ██╔══██╗  ╚██╔╝  
    ██║  ██║██║  ██║╚██████╗██║  ██║   ██║   ██║  ██║   ██║   
    ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝   ╚═╝   
"#;

fn main() {
    clear_screen();
    println!("{}", ASCII_LOGO.bright_green());
    println!("{}", "Loading ArchTry...".bright_blue());
    sleep(3);
    let user_choices = get_user_choices();
    run_simulation(&user_choices);
}
