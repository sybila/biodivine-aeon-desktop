#[tauri::command]
pub fn get_version() -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    return VERSION.to_string();
}
