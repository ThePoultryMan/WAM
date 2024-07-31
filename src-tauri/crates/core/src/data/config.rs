use confique::Config;
use serde::Serialize;
use tauri::State;

use crate::{game::ReleaseType, AppState};

use super::LocalSaveData;

#[derive(Config, Serialize)]
pub struct AppConfig {
    #[config(nested)]
    game_paths: GamePaths,
}

#[derive(Config, Serialize)]
struct GamePaths {
    retail: Option<String>,
}

impl LocalSaveData for AppConfig {
    type Data = AppConfig;

    fn new() -> Self {
        Self {
            game_paths: GamePaths { retail: None },
        }
    }

    fn get_file_name() -> String {
        String::from("config.toml")
    }
}

#[wam_macros::contains_tauri_commands(body_state = "state.config", mutex_behavior = "lock")]
impl AppConfig {
    #[wam_macros::with_tauri_command]
    pub fn get_game_path(&self, release_type: ReleaseType) -> &Option<String> {
        match release_type {
            ReleaseType::Retail => &self.game_paths.retail,
        }
    }

    pub fn set_game_path(&mut self, release_type: ReleaseType, path: String) {
        match release_type {
            ReleaseType::Retail => self.game_paths.retail = Some(path),
        }
    }
}

#[tauri::command]
pub fn save_config(state: State<AppState>) {
    if let Ok(config) = state.config.lock() {
        config.save();
    }
}
