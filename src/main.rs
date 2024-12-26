mod commands;
mod errors;
mod utils;

use clap::Parser;
use commands::{App, AppState, Handle};
use std::process;

fn main() {
    let mut app_state = AppState::new();
    let app = App::parse();

    match app.command.handle(&mut app_state) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            process::exit(e.exit_code());
        }
    }
}
