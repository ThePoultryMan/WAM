use regex::Regex;
use tauri::State;

use crate::AppState;

#[tauri::command]
pub async fn create_curseforge_window(
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let regex_pattern = Regex::new(
        r"https:\/\/www\.curseforge\.com\/wow\/addons\/[\S]+\/download\/(?<file_id>[0-9]+)",
    )
    .expect("fatal internal error occurred at 'curseforge_window.rs:13'");

    let global_curseforge_window = state.curseforge_window.clone();
    let curseforge_window = Some(
        tauri::WindowBuilder::new(
            &app_handle,
            "curseforge_addons",
            tauri::WindowUrl::External("https://www.curseforge.com/wow/search".parse().unwrap()),
        )
        .title("CurseForge Browser (WAM!)")
        .min_inner_size(1287.0, 256.0)
        .on_navigation(move |url| {
            match global_curseforge_window.lock() {
                Ok(curseforge_window_guard) => {
                    if let Some(curseforge_window) = &*curseforge_window_guard {
                        curseforge_window
                            .eval(include_str!("./assets/main-cleaner.js"))
                            .unwrap();
                    }
                }
                Err(error) => todo!("{error}"),
            }

            true
        })
        .build()
        .unwrap(),
    );

    *state.curseforge_window.lock().unwrap() = curseforge_window;

    Ok(())
}
