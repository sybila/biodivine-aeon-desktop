use crate::menu::menu_init;

/// Open new window with model editor.
#[tauri::command]
pub async fn open_model_window(label: &str, handle: tauri::AppHandle) -> Result<(), ()> {
    tauri::WindowBuilder::new(&handle, label, tauri::WindowUrl::App("index.html".into()))
        .center()
        .menu(menu_init())
        .inner_size(1000f64, 700f64)
        .min_inner_size(600f64, 300f64)
        .title("Aeon/BIODIVINE - model editor")
        .build()
        .expect("Error while creating new model window.");
    Ok(())
}

/// Open new window to watch computation status.
#[tauri::command]
pub async fn open_computation_window(
    label: &str,
    title: &str,
    handle: tauri::AppHandle,
) -> Result<(), ()> {
    tauri::WindowBuilder::new(
        &handle,
        label,
        tauri::WindowUrl::App("computation-window.html".into()),
    )
    .center()
    .maximizable(false)
    .menu(menu_init())
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
pub async fn open_explorer_window(label: &str, handle: tauri::AppHandle) -> Result<(), ()> {
    tauri::WindowBuilder::new(
        &handle,
        label,
        tauri::WindowUrl::App("explorer.html".into()),
    )
    .center()
    .menu(menu_init())
    .inner_size(1000f64, 700f64)
    .min_inner_size(800f64, 300f64)
    .title("Aeon/BIODIVINE - attractor explorer")
    .build()
    .expect("Error while creating new explorer window.");
    Ok(())
}

/// Open new window with bifurcation decision tree.
#[tauri::command]
pub async fn open_tree_explorer_window(
    label: &str,
    title: &str,
    handle: tauri::AppHandle,
) -> Result<(), ()> {
    tauri::WindowBuilder::new(
        &handle,
        label,
        tauri::WindowUrl::App("tree-explorer.html".into()),
    )
    .center()
    .menu(menu_init())
    .inner_size(1000f64, 700f64)
    .min_inner_size(800f64, 300f64)
    .title(title)
    .build()
    .expect("Error while creating new tree explorer window.");
    Ok(())
}

/// Open manual window.
#[tauri::command]
pub async fn open_manual_window(handle: tauri::AppHandle) -> Result<(), ()> {
    tauri::WindowBuilder::new(
        &handle,
        "manual-window",
        tauri::WindowUrl::App("manual/book/index.html".into()),
    )
    .center()
    .menu(menu_init())
    .inner_size(1300f64, 700f64)
    .title("Manual")
    .build()
    .expect("Error while creating new manual window.");
    Ok(())
}

// Open help window.
#[tauri::command]
pub async fn open_help_window(handle: tauri::AppHandle) -> Result<(), ()> {
    tauri::WindowBuilder::new(
        &handle,
        "help-window",
        tauri::WindowUrl::App("help-window.html".into()),
    )
    .center()
    .menu(menu_init())
    .inner_size(530f64, 700f64)
    .min_inner_size(530f64, 300f64)
    .max_inner_size(530f64, 800f64)
    .maximizable(false)
    .title("Help")
    .build()
    .expect("Error while creating new help window.");
    Ok(())
}
