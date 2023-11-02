// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate lazy_static;

use crate::menu::menu_init;
use tauri::{Manager, WindowBuilder};

mod types;
mod computation;
mod computation_results;
mod menu;
mod session;
mod tauri_commands;
mod model;



fn main() {
    let menu = menu_init();
    tauri::Builder::default()
        .setup(|app| {
            WindowBuilder::new(
                app,
                "model-window-main".to_string(),
                tauri::WindowUrl::App("index.html".into()),
            )
            .menu(menu)
            .title("Aeon/BIODIVINE - model editor")
            .inner_size(1000f64, 700f64)
            .build()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tauri_commands::model_commands::check_update_function,
            tauri_commands::model_commands::sbml_to_aeon,
            tauri_commands::model_commands::aeon_to_sbml,
            tauri_commands::model_commands::aeon_to_sbml_instantiated,
            tauri_commands::session_commands::add_session,
            tauri_commands::session_commands::remove_session,
            tauri_commands::session_commands::has_running_computation,
            tauri_commands::computation_commands::start_computation,
            tauri_commands::computation_commands::cancel_computation,
            tauri_commands::computation_commands::get_results,
            tauri_commands::computation_commands::get_computation_process_info,
            tauri_commands::window_commands::open_model_window,
            tauri_commands::window_commands::open_computation_window,
            tauri_commands::window_commands::open_explorer_window,
            tauri_commands::window_commands::open_tree_explorer_window,
            tauri_commands::window_commands::open_help_window,
            tauri_commands::window_commands::open_manual_window,
            tauri_commands::computation_results_commands::get_witness,
            tauri_commands::computation_results_commands::get_tree_witness,
            tauri_commands::computation_results_commands::get_stability_witness,
            tauri_commands::computation_results_commands::get_attractors,
            tauri_commands::computation_results_commands::get_tree_attractors,
            tauri_commands::computation_results_commands::get_stability_attractors,
            tauri_commands::computation_results_commands::get_bifurcation_tree,
            tauri_commands::tree_explorer_commands::auto_expand,
            tauri_commands::tree_explorer_commands::get_attributes,
            tauri_commands::tree_explorer_commands::apply_tree_precision,
            tauri_commands::tree_explorer_commands::get_tree_precision,
            tauri_commands::tree_explorer_commands::apply_attribute,
            tauri_commands::tree_explorer_commands::revert_decision,
            tauri_commands::tree_explorer_commands::get_stability_data,
            tauri_commands::version_command::get_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
