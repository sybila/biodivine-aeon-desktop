// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod commands;
mod common;

use tauri::{Manager, Window};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_process(window: Window, message: &str) {
    println!("Received message from frontend");
    println!("This is the message: {}", message);
    println!("This is the window: {}", window.label());
    window.emit("my-event", Payload { message: message.parse().unwrap() }).unwrap();
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![
        commands::check_update_function,
        commands::sbml_to_aeon,
        commands::aeon_to_sbml,
          init_process
      ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
