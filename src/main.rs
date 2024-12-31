mod commands;
mod errors;
mod utils;

use std::process;
use std::sync::{Arc, Mutex};

use clap::Parser;
use kryptopass_utils::password_generator::config::Config;

use commands::{App, AppState, Handle};
use kryptopass_utils::password_generator::GenPassword;

fn main() {
    let app_state = Arc::new(Mutex::new(AppState::new()));
    let app = App::parse();

    let app_state_ctrlc = app_state.clone();
    ctrlc::set_handler(move || {
        if let Ok(app_state) = app_state_ctrlc.lock() {
            if let Err(e) = app_state.close() {
                eprintln!("❌ Error during shutdown: {}", e);
            }
        } else {
            eprintln!("❌ Failed to acquire lock during Ctrl-C");
        }

        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    match Config::from_file("./default.toml") {
        Ok(config) => {
            let generator = GenPassword::new(config);
            println!("{:#?}", generator.generate());
        }
        Err(e) => eprintln!("Error loading configuration: {}", e),
    }

    if let Err(e) = app.command.handle(app_state) {
        eprintln!("❌ Error: {} with code {}", e, e.exit_code());
        process::exit(1);
    }
}
