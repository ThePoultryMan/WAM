use std::{fs, path::PathBuf};

use confique::{Config, Error};
use platform_dirs::AppDirs;
use serde::Serialize;

pub mod addons;
pub mod config;

pub trait LocalSaveData {
    type Data: Config + LocalSaveData + Serialize;

    fn load() -> Result<Self::Data, Error> {
        Self::Data::builder()
            .file(get_or_create_data_dir::<Self::Data>(&Self::Data::get_file_name()))
            .load()
    }

    fn save(&self) where Self: Serialize {
        match toml::to_string_pretty(self) {
            Ok(string) => {
                fs::write(get_or_create_data_dir::<Self::Data>(&Self::Data::get_file_name()), string)
                    .expect(&format!("failed to write '{}' to file", Self::Data::get_file_name()));
            }
            Err(error) => todo!("{error}"),
        }
    }

    fn new() -> Self;

    fn get_file_name() -> String;
}

fn get_or_create_data_dir<T: LocalSaveData + Config + Serialize>(file: &str) -> PathBuf {
    let app_dirs = AppDirs::new(Some("wam1"), true);
    match app_dirs {
        Some(app_dirs) => {
            let config_path = app_dirs.config_dir.join(file);

            if !config_path.exists() {
                fs::create_dir_all(&app_dirs.config_dir)
                    .expect("Failed to create config directory");
                fs::write(&config_path, "# nothing yet")
                    .expect("Failed to create initial config file");
                T::new().save();
            }
            config_path
        }
        None => todo!(),
    }
}
