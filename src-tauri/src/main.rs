// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use config::AppConfig;
use tauri::Window;

mod config;
mod curseforge_window;
mod game;

struct AppState {
    config: Mutex<AppConfig>,
    curseforge_window: Arc<Mutex<Option<Window>>>,
}

impl AppState {
    pub fn new() -> Self {
        match AppConfig::read() {
            Ok(config) => Self {
                config: Mutex::new(config),
                curseforge_window: Arc::new(Mutex::new(None)),
            },
            Err(error) => todo!("{error}"),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            game::get_game_version,
            curseforge_window::create_curseforge_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
