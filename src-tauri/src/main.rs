// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use config::AppConfig;

mod config;
mod game;

struct AppState {
    config: Mutex<AppConfig>,
}

impl AppState {
    pub fn new() -> Self {
        match AppConfig::read() {
            Ok(config) => {
                Self { config: Mutex::new(config) }
            },
            Err(error) => todo!("{error}"),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![game::get_game_version])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
