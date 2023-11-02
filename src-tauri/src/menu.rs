use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

/// Initialize system menu used in model window.
pub fn menu_init() -> Menu {
    let help = CustomMenuItem::new("help", "About/Help");
    let manual = CustomMenuItem::new("manual", "Manual");

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
    let new_model_editor = CustomMenuItem::new("new_model_editor", "New model editor");

    // Main submenus in menu bar
    let app_submenu = Submenu::new(
        "AEON/BIODIVINE",
        Menu::new().add_item(help).add_native_item(MenuItem::Quit),
    );

    let model_submenu = Submenu::new(
        "Model",
        Menu::new()
            .add_submenu(import_submenu)
            .add_submenu(export_submenu)
            .add_item(new_model_editor),
    );

    let manual_submenu = Submenu::new("Manual", Menu::new().add_item(manual));

    Menu::new()
        .add_submenu(app_submenu)
        .add_submenu(model_submenu)
        .add_submenu(manual_submenu)
}
