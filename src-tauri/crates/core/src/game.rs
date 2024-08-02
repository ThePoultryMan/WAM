use std::{fs, path::PathBuf};

use regex::Regex;
use serde::Deserialize;
use tauri::State;

use crate::AppState;

#[derive(Deserialize)]
pub enum ReleaseType {
    Retail,
}

#[tauri::command]
pub fn get_game_version(state: State<AppState>) -> Option<String> {
    let config = state.config.lock();
    match config {
        Ok(config) => {
            let game_path = config.get_game_path(ReleaseType::Retail);
            if let Some(game_path) = game_path {
                let mut build_info_path = PathBuf::from(game_path);
                build_info_path.push(".build.info");
                match fs::read_to_string(build_info_path) {
                    Ok(string) => {
                        let regex = Regex::new(r"text\?\|\|\|(?<version>.*?)\|\|")
                            .expect("An invalid pattern was supplied");
                        match regex.captures(&string) {
                            Some(captures) => return Some(captures["version"].to_owned()),
                            None => (),
                        }
                    }
                    Err(_) => (),
                }
            }
        }
        Err(_) => (),
    }
    Some("Internal Error".to_owned())
}
