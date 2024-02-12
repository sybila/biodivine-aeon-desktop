use tauri::{EventTarget, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use biodivine_aeon_desktop::ImportModelEvent;
use crate::menu::menu_init;
use crate::types::ErrorMessage;

// Tauri FS does not allow arbitrary file reads, so we provide this instead.
#[tauri::command]
pub async fn read_text_file(path: &str) -> Result<String, ErrorMessage> {
    std::fs::read_to_string(path).map_err(|e| {
        format!("Cannot read file: {}", e)
    })
}

// Tauri FS does not allow arbitrary file writes, so we provide this instead.
#[tauri::command]
pub async fn write_text_file(path: &str, content: &str) -> Result<(), ErrorMessage> {
    std::fs::write(path, content).map_err(|e| {
        format!("Cannot write file: {}", e)
    })
}

/// Open new window with model editor.
#[tauri::command]
pub fn open_model_window(label: &str, content: Option<String>, handle: tauri::AppHandle) -> tauri::Result<()> {
    let window = WebviewWindowBuilder::new(&handle, label, WebviewUrl::App("index.html".into()))
        .center()
        .menu(menu_init(&handle)?)
        .inner_size(1000f64, 700f64)
        .min_inner_size(600f64, 300f64)
        .title("Aeon/BIODIVINE - model editor")
        .build()
        .expect("Error while creating new model window.");
    if let Some(content) = content {
        let label = label.to_string();
        window.clone().once("ready", move |_e| {
            window.emit_to(
                EventTarget::window(label),
                "import-model",
                ImportModelEvent { model_string: content }
            ).unwrap();
        });
    };
    Ok(())
}

/// Open new window to watch computation status.
#[tauri::command]
pub fn open_computation_window(
    label: &str,
    title: &str,
    handle: tauri::AppHandle,
) -> tauri::Result<()> {
    WebviewWindowBuilder::new(&handle,label,WebviewUrl::App("computation-window.html".into()))
    .center()
    .maximizable(false)
    .menu(menu_init(&handle)?)
    .inner_size(550f64, 600f64)
    .min_inner_size(550f64, 400f64)
    .max_inner_size(550f64, 800f64)
    .title(title)
    .build()
    .expect("Error while creating computation window.");
    Ok(())
}

/// Open new window with attractors explorer.
#[tauri::command]
pub fn open_explorer_window(
    label: &str,
    title: &str,
    handle: tauri::AppHandle,
) -> tauri::Result<()> {
    WebviewWindowBuilder::new(
        &handle,
        label,
        WebviewUrl::App("explorer.html".into()),
    )
    .center()
    .menu(menu_init(&handle)?)
    .inner_size(1000f64, 700f64)
    .min_inner_size(800f64, 300f64)
    .title(title)
    .build()
    .expect("Error while creating new explorer window.");
    Ok(())
}

/// Open new window with witness model.
#[tauri::command]
pub fn open_witness_window(
    label: &str,
    title: &str,
    handle: tauri::AppHandle,
) -> tauri::Result<()> {
    WebviewWindowBuilder::new(
        &handle,
        label,
        WebviewUrl::App("witness-window.html".into()),
    )
    .center()
    .menu(menu_init(&handle)?)
    .inner_size(1000f64, 700f64)
    .min_inner_size(600f64, 300f64)
    .title(title)
    .build()
    .expect("Error while creating new model window.");
    Ok(())
}

/// Open new window with bifurcation decision tree.
#[tauri::command]
pub fn open_tree_explorer_window(
    label: &str,
    title: &str,
    handle: tauri::AppHandle,
) -> tauri::Result<()> {
    WebviewWindowBuilder::new(
        &handle,
        label,
        WebviewUrl::App("tree-explorer.html".into()),
    )
    .center()
    .menu(menu_init(&handle)?)
    .inner_size(1000f64, 700f64)
    .min_inner_size(800f64, 300f64)
    .title(title)
    .build()
    .expect("Error while creating new tree explorer window.");
    Ok(())
}

/// Open manual window.
#[tauri::command]
pub fn open_manual_window(handle: tauri::AppHandle) -> tauri::Result<()> {
    WebviewWindowBuilder::new(
        &handle,
        "manual-window",
        WebviewUrl::App("manual/book/index.html".into()),
    )
    .center()
    .menu(menu_init(&handle)?)
    .inner_size(1300f64, 700f64)
    .title("Manual")
    .build()
    .expect("Error while creating new manual window.");
    Ok(())
}

// Open help window.
#[tauri::command]
pub fn open_help_window(handle: tauri::AppHandle) -> tauri::Result<()> {
    WebviewWindowBuilder::new(
        &handle,
        "help-window",
        WebviewUrl::App("help-window.html".into()),
    )
    .center()
    .menu(menu_init(&handle)?)
    .inner_size(530f64, 700f64)
    .min_inner_size(530f64, 300f64)
    .max_inner_size(530f64, 800f64)
    .maximizable(false)
    .title("Help")
    .build()
    .expect("Error while creating new help window.");
    Ok(())
}
