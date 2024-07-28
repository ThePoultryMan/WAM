#[tauri::command]
pub async fn create_curseforge_window(app_handle: tauri::AppHandle) {
    let curseforge_window = tauri::WindowBuilder::new(
        &app_handle,
        "curseforge_addons",
        tauri::WindowUrl::External("https://www.curseforge.com/wow/search".parse().unwrap()),
    )
    .title("CurseForge Browser (WAM!)")
    .min_inner_size(1287.0, 256.0)
    .build()
    .unwrap();
}
