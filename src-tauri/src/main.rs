// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate lazy_static;

use std::env;
use std::ops::Deref;
use biodivine_lib_param_bn::BooleanNetwork;
use tauri::{EventTarget, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri::async_runtime::handle;
use tauri_plugin_dialog::DialogExt;
use crate::menu::{handle_menu_event, menu_init};
use biodivine_aeon_desktop::{ImportModelEvent, OpenedUrls};
use crate::tauri_commands::model_commands::_sbml_to_aeon;
use crate::types::ErrorMessage;

mod computation;
mod computation_results;
mod menu;
mod model;
mod session;
mod tauri_commands;
mod types;

/// Import the model file at the specified path, or return an error message explaining why
/// the import failed.
///
/// This differs from `BooleanNetwork::try_from_file` in that it can also
/// read model layout information.
fn import_initial_model(path: &str) -> Result<String, ErrorMessage> {
    if path.ends_with(".sbml") {
        // For SBML, we have a special procedure to parse the model layout.
        let Ok(content) = std::fs::read_to_string(path) else {
            return Err(format!("Cannot read file `{}`.", path))
        };
        _sbml_to_aeon(content.as_str())
    } else if path.ends_with(".aeon") {
        // Aeon is just "copied" into the editor.
        std::fs::read_to_string(path)
            .map_err(|_e| format!("Cannot read file `{}`.", path))
    } else {
        // For bnet and other formats in the future that don't provide a layout.
        BooleanNetwork::try_from_file(path)
            .map(|it| it.to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(OpenedUrls::default())
        .menu(|app| {
            menu_init(app)
        })
        .setup(|app| {
            #[cfg(any(windows, target_os = "linux"))]
            {
                // NOTICE: `args` may include URL protocol (`your-app-protocol://`) or
                // arguments (`--`) if app supports them.
                let mut urls = Vec::new();
                for arg in env::args().skip(1) {
                    if let Ok(url) = url::Url::parse(&arg) {
                        urls.push(url);
                    }
                }

                if !urls.is_empty() {
                    app.state::<OpenedUrls>().inner().set(urls);
                }
            }

            let opened_urls = app.state::<OpenedUrls>().inner().get();

            let opened_urls = if let Some(x)  = opened_urls { Some(x) } else {
                let mut urls = Vec::new();
                for arg in env::args().skip(1) {
                    let arg = format!("file://{}", arg);
                    if let Ok(url) = url::Url::parse(&arg) {
                        urls.push(url);
                    }
                }
                Some(urls)
            };

            let menu = menu_init(app.handle())?;
            let window = WebviewWindowBuilder::new(
                app,
                "model-window-main".to_string(),
                WebviewUrl::App("index.html".into()),
            )
                .center()
                .menu(menu.clone())
                .on_menu_event(|w, m| {
                    handle_menu_event(w, m).unwrap();
                })
                .title("Aeon/BIODIVINE - model editor")
                .inner_size(1000f64, 700f64)
                .min_inner_size(600f64, 300f64)
                .build()?;

            if let Some(mut opened) = opened_urls {
                if let Some(url) = opened.pop() {
                    let path = urlencoding::decode(url.path()).unwrap();
                    let model = match import_initial_model(path.deref()) {
                        Ok(model) => model,
                        Err(issue) => {
                            app.dialog().message(issue)
                                .show(|_e| {});
                            return Ok(());
                        }
                    };
                    let window_copy = window.clone();
                    window.once("ready", move |_e| {
                        window_copy.emit_to(
                            EventTarget::window(window_copy.label()),
                            "import-model",
                            ImportModelEvent { model_string: model }
                        ).unwrap();
                    });
                }
                while let Some(url) = opened.pop() {
                    let window = WebviewWindowBuilder::new(
                        app,
                        "model-window-main".to_string(),
                        WebviewUrl::App("index.html".into()),
                    )
                        .center()
                        .menu(menu.clone())
                        .title("Aeon/BIODIVINE - model editor")
                        .inner_size(1000f64, 700f64)
                        .min_inner_size(600f64, 300f64)
                        .build()?;

                    let path = urlencoding::decode(url.path()).unwrap();
                    let model = match import_initial_model(path.deref()) {
                        Ok(model) => model,
                        Err(issue) => {
                            app.dialog().message(issue)
                                .show(|_e| {});
                            return Ok(());
                        }
                    };
                    println!("Opened model {}.", url.path());
                    let window_copy = window.clone();
                    window.once("ready", move |_e| {
                        window_copy.emit("import-model", ImportModelEvent {
                            model_string: model,
                        }).unwrap();
                    });
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tauri_commands::model_commands::check_update_function,
            tauri_commands::model_commands::sbml_to_aeon,
            tauri_commands::model_commands::bnet_to_aeon,
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
            tauri_commands::window_commands::open_witness_window,
            tauri_commands::window_commands::open_tree_explorer_window,
            tauri_commands::window_commands::open_help_window,
            tauri_commands::window_commands::open_manual_window,
            tauri_commands::window_commands::read_text_file,
            tauri_commands::window_commands::write_text_file,
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
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            /*
                This handles the "open with..." feature on macOS, since macOS sends the files
                as a callback, not as an argument list.
             */
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Opened { urls } = event {
                app.state::<OpenedUrls>().inner().set(urls);
            }
        });
}
