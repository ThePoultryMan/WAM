use confique::Config;
use serde::Serialize;

use crate::game::ReleaseType;

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

impl AppConfig {
    pub fn get_game_path(&self, release_type: ReleaseType) -> &Option<String> {
        match release_type {
            ReleaseType::Retail => &self.game_paths.retail,
        }
    }
}
