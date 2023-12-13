/// Get app version that is defined in Cargo.toml file.
#[tauri::command]
pub fn get_version() -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    VERSION.to_string()
}
