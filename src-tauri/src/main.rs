// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod commands;
mod common;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![commands::check_update_function, commands::sbml_to_aeon])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
