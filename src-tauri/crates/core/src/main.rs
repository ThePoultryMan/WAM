// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use data::{addons::Addons, config::AppConfig, LocalSaveData};
use tauri::Window;

mod curseforge_window;
mod data;
mod game;

struct AppState {
    config: Mutex<AppConfig>,
    addons: Mutex<Addons>,
    curseforge_window: Arc<Mutex<Option<Window>>>,
}

impl AppState {
    pub fn new() -> Self {
        match (AppConfig::load(), Addons::load()) {
            (Ok(config), Ok(addons)) => Self {
                config: Mutex::new(config),
                addons: Mutex::new(addons),
                curseforge_window: Arc::new(Mutex::new(None)),
            },
            _ => todo!(),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            curseforge_window::create_curseforge_window,
            data::config::save_config,
            game::get_game_version,
            game::set_game_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
