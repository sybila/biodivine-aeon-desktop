// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod commands;
mod common;
mod computation;


fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![
        commands::check_update_function,
        commands::sbml_to_aeon,
        commands::aeon_to_sbml,
        commands::aeon_to_sbml_instantiated,
        computation::add_window_session,
        computation::remove_window_session,
        computation::start_computation,
        computation::cancel_computation,
      ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
