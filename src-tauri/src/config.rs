use std::{fs, path::PathBuf};

use confique::{Config, Error};
use platform_dirs::AppDirs;
use serde::Serialize;

use crate::game::ReleaseType;

#[derive(Config, Serialize)]
pub struct AppConfig {
    #[config(nested)]
    game_paths: GamePaths,
}

#[derive(Config, Serialize)]
struct GamePaths {
    retail: Option<String>,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            game_paths: GamePaths { retail: None },
        }
    }

    pub fn read() -> Result<AppConfig, Error> {
        AppConfig::builder()
            .file(get_or_create_config_dir())
            .load()
    }

    pub fn save(&self) {
        match toml::to_string_pretty(self) {
            Ok(string) => {
                fs::write(get_or_create_config_dir(), string).expect("Failed to write config to file");
            },
            Err(error) => todo!("{error}"),
        }
    }

    pub fn get_game_path(&self, release_type: ReleaseType) -> &Option<String> {
        match release_type {
            ReleaseType::Retail => {
                &self.game_paths.retail
            }
        }
    }
}

fn get_or_create_config_dir() -> PathBuf {
    let app_dirs = AppDirs::new(Some("wam1"), true);
    match app_dirs {
        Some(app_dirs) => {
            let config_path = app_dirs.config_dir.join("config.toml");
            
            if !config_path.exists() {
                fs::create_dir_all(&app_dirs.config_dir).expect("Failed to create config directory");
                fs::write(&config_path, "# nothing yet").expect("Failed to create initial config file");
                AppConfig::new().save();
            }
            config_path
        },
        None => todo!()
    }
}
