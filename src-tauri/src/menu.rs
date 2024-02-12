use tauri::{AppHandle, EventTarget, Manager, Runtime, Window};
use tauri::menu::{Menu, MenuBuilder, MenuEvent, MenuItemBuilder, SubmenuBuilder};
use biodivine_aeon_desktop::AeonMenuEvent;

const MENU_QUIT: &str = "quit";
const MENU_HELP: &str = "help";
const MENU_MANUAL: &str = "manual";
const MENU_NEW: &str = "new_model_editor";

const MENU_EXAMPLE_G2A: &str = "g2a";
const MENU_EXAMPLE_G2B: &str = "g2b";
const MENU_EXAMPLE_ORLANDO: &str = "budding_yeast_orlando";
const MENU_EXAMPLE_IRONS: &str = "budding_yeast_irons";

const MENU_IMPORT_STORAGE: &str = "local_storage";
const MENU_IMPORT_AEON: &str = "aeon_import";
const MENU_IMPORT_SBML: &str = "sbml_import";
const MENU_IMPORT_BNET: &str = "bnet_import";

const MENU_EXPORT_AEON: &str = "aeon_export";
const MENU_EXPORT_SBML_PARAM: &str = "sbml_export_parametrized";
const MENU_EXPORT_SBML_INST: &str = "sbml_export_instantiated";
const MENU_EXPORT_BNET: &str = "bnet_export";


pub fn handle_menu_event(w: &Window, m: MenuEvent) -> tauri::Result<()> {
    w.emit_to(
        EventTarget::window(w.label()),
        "aeon-menu-event",
        AeonMenuEvent {
            menu_id: m.id().0.clone(),
        }
    )
}

/// Initialize system menu used in model window.
pub fn menu_init<R: Runtime>(handle: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    /*
        For now, this is basically just the macOS app-wide menu.
     */

    let help = MenuItemBuilder::with_id(MENU_HELP, "Help").build(handle)?;
    let manual = MenuItemBuilder::with_id(MENU_MANUAL, "Manual").build(handle)?;

    // Example models submenu items
    let g2a = MenuItemBuilder::with_id(MENU_EXAMPLE_G2A, "G2A - Cell Division").build(handle)?;
    let g2b = MenuItemBuilder::with_id(MENU_EXAMPLE_G2B, "G2B - Cell Division").build(handle)?;
    let orlando = MenuItemBuilder::with_id(MENU_EXAMPLE_ORLANDO, "Orlando - Budding Yeast").build(handle)?;
    let irons = MenuItemBuilder::with_id(MENU_EXAMPLE_IRONS, "Irons - Budding Yeast").build(handle)?;

    // Import submenu items
    let local_storage = MenuItemBuilder::with_id(MENU_IMPORT_STORAGE, "Last model").build(handle)?;
    let aeon_import = MenuItemBuilder::with_id(MENU_IMPORT_AEON, ".aeon").build(handle)?;
    let sbml_import = MenuItemBuilder::with_id(MENU_IMPORT_SBML, ".sbml").build(handle)?;
    let bnet_import = MenuItemBuilder::with_id(MENU_IMPORT_BNET, ".bnet").build(handle)?;

    // Export submenu items
    let aeon_export = MenuItemBuilder::with_id(MENU_EXPORT_AEON, ".aeon").build(handle)?;
    let sbml_param_export = MenuItemBuilder::with_id(MENU_EXPORT_SBML_PARAM, ".sbml (partially specified)").build(handle)?;
    let sbml_inst_export = MenuItemBuilder::with_id(MENU_EXPORT_SBML_INST, ".sbml (instantiated)").build(handle)?;
    let bnet_export = MenuItemBuilder::with_id(MENU_EXPORT_BNET, ".bnet").build(handle)?;

    let examples = SubmenuBuilder::new(handle, "Example models")
        .item(&g2a)
        .item(&g2b)
        .item(&orlando)
        .item(&irons)
        .build()?;

    let import = SubmenuBuilder::new(handle, "Import")
        .item(&local_storage)
        .item(&aeon_import)
        .item(&sbml_import)
        .item(&bnet_import)
        .item(&examples)
        .build()?;

    let export = SubmenuBuilder::new(handle, "Export")
        .item(&aeon_export)
        .item(&sbml_param_export)
        .item(&sbml_inst_export)
        .item(&bnet_export)
        .build()?;

    let new = MenuItemBuilder::with_id(MENU_NEW, "New model").build(handle)?;
    let quit = MenuItemBuilder::with_id(MENU_QUIT, "Quit").build(handle)?;

    let app = SubmenuBuilder::new(handle, "Biodivine AEON")
        .item(&quit)
        .build()?;

    let model = SubmenuBuilder::new(handle, "Model")
        .item(&new)
        .item(&import)
        .item(&export)
        .build()?;

    let help_menu = SubmenuBuilder::new(handle, "Help")
        .item(&help)
        .item(&manual)
        .build()?;

    MenuBuilder::new(handle)
        .item(&app)
        .item(&model)
        .item(&help_menu)
        .build()
}
