use tauri::{CustomMenuItem, Menu, Submenu};

/// Initialize system menu used in model window.
pub fn menu_init() -> Menu {
    let help = CustomMenuItem::new("help", "About/Help");

    // Example models submenu items
    let g2a = CustomMenuItem::new("g2a", "G2A - Cell Division");
    let g2b = CustomMenuItem::new("g2b", "G2B - Cell Division");
    let budding_yeast_orlando =
        CustomMenuItem::new("budding_yeast_orlando", "Orlando - Budding Yeast");
    let budding_yeast_irons = CustomMenuItem::new("budding_yeast_irons", "Irons - Budding Yeast");

    // Import submenu items
    let local_storage = CustomMenuItem::new("local_storage", "Local Storage");
    let aeon_import = CustomMenuItem::new("aeon_import", ".AEON");
    let sbml_import = CustomMenuItem::new("sbml_import", ".SBML");
    let examples_submenu = Submenu::new(
        "Example Models",
        Menu::new()
            .add_item(g2a)
            .add_item(g2b)
            .add_item(budding_yeast_orlando)
            .add_item(budding_yeast_irons),
    );

    // Export submenu items
    let aeon_export = CustomMenuItem::new("aeon_export", ".AEON");
    let sbml_export_parametrized =
        CustomMenuItem::new("sbml_export_parametrized", ".SBML (parametrized)");
    let sbml_export_instantiated =
        CustomMenuItem::new("sbml_export_instantiated", ".SBML (instantiated)");

    // Model submenu items
    let import_submenu = Submenu::new(
        "Import",
        Menu::new()
            .add_item(local_storage)
            .add_item(aeon_import)
            .add_item(sbml_import)
            .add_submenu(examples_submenu),
    );
    let export_submenu = Submenu::new(
        "Export",
        Menu::new()
            .add_item(aeon_export)
            .add_item(sbml_export_parametrized)
            .add_item(sbml_export_instantiated),
    );

    let model_submenu = Submenu::new(
        "Model",
        Menu::new()
            .add_submenu(import_submenu)
            .add_submenu(export_submenu),
    );

    Menu::new().add_item(help).add_submenu(model_submenu)
}

/// Open new window with model editor.
#[tauri::command]
pub async fn open_model_window(label: &str, handle: tauri::AppHandle) -> Result<(), ()> {
    tauri::WindowBuilder::new(&handle, label, tauri::WindowUrl::App("index.html".into()))
        .menu(menu_init())
        .inner_size(1000f64, 700f64)
        .title("Aeon/BIODIVINE")
        .build()
        .expect("Error while creating new model window.");
    Ok(())
}

/// Open new window to watch computation status.
#[tauri::command]
pub async fn open_computation_window(label: &str, handle: tauri::AppHandle) -> Result<(), ()> {
    tauri::WindowBuilder::new(
        &handle,
        label,
        tauri::WindowUrl::App("computation-window.html".into()),
    )
    .inner_size(550f64, 600f64)
    .min_inner_size(400f64, 400f64)
    .title("Computation")
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
    .inner_size(1000f64, 700f64)
    .title("Aeon/BIODIVINE")
    .build()
    .expect("Error while creating new model window.");
    Ok(())
}

/// Open new window with bifurcation decision tree.
#[tauri::command]
pub async fn open_tree_explorer_window(label: &str, handle: tauri::AppHandle) -> Result<(), ()> {
    tauri::WindowBuilder::new(
        &handle,
        label,
        tauri::WindowUrl::App("tree_explorer.html".into()),
    )
    .inner_size(1000f64, 700f64)
    .title("Aeon/BIODIVINE")
    .build()
    .expect("Error while creating new model window.");
    Ok(())
}
