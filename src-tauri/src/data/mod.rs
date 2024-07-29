use std::{fs, path::PathBuf};

use config::AppConfig;
use confique::{Config, Error};
use platform_dirs::AppDirs;
use serde::Serialize;

pub mod config;

pub trait LocalSaveData<T: Config + LocalSaveData<T>> {
    fn load() -> Result<T, Error> {
        T::builder()
            .file(get_or_create_data_dir(&T::get_file_name()))
            .load()
    }

    fn save(&self) where Self: Serialize {
        match toml::to_string_pretty(self) {
            Ok(string) => {
                fs::write(get_or_create_data_dir(&T::get_file_name()), string)
                    .expect(&format!("failed to write '{}' to file", T::get_file_name()));
            }
            Err(error) => todo!("{error}"),
        }
    }

    fn get_file_name() -> String;
}

fn get_or_create_data_dir(file: &str) -> PathBuf {
    let app_dirs = AppDirs::new(Some("wam1"), true);
    match app_dirs {
        Some(app_dirs) => {
            let config_path = app_dirs.config_dir.join(file);

            if !config_path.exists() {
                fs::create_dir_all(&app_dirs.config_dir)
                    .expect("Failed to create config directory");
                fs::write(&config_path, "# nothing yet")
                    .expect("Failed to create initial config file");
                AppConfig::new().save();
            }
            config_path
        }
        None => todo!(),
    }
}
