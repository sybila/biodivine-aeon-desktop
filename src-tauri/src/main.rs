// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate lazy_static;

use crate::multi_window::menu_init;
use tauri::WindowBuilder;

mod common;
mod computation;
mod computation_commands;
mod model_commands;
mod multi_window;
mod session;

fn main() {
    let menu = menu_init();
    tauri::Builder::default()
        .setup(|app| {
            WindowBuilder::new(
                app,
                "main-window".to_string(),
                tauri::WindowUrl::App("index.html".into()),
            )
            .menu(menu)
            .title("Aeon/BIODIVINE")
            .inner_size(1000f64, 700f64)
            .build()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            model_commands::check_update_function,
            model_commands::sbml_to_aeon,
            model_commands::aeon_to_sbml,
            model_commands::aeon_to_sbml_instantiated,
            session::add_window_session,
            session::remove_window_session,
            session::has_running_computation,
            computation_commands::start_computation,
            computation_commands::cancel_computation,
            computation_commands::get_results,
            computation_commands::get_computation_process_info,
            multi_window::open_model_window,
            multi_window::open_computation_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
