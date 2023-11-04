use crate::computation::{prepare_computation_thread, Computation};
use crate::session::get_locked_computation;
use crate::types::{ErrorMessage, Timestamp};
use biodivine_lib_param_bn::BooleanNetwork;
use json::{object, JsonValue};
use serde_json::{from_str, Value};
use std::sync::{Arc, RwLock};

/// Accept an Aeon model, parse it and start a new computation (if there is no computation running).
#[tauri::command]
pub fn start_computation(session_key: &str, aeon_string: &str) -> Result<Timestamp, ErrorMessage> {
    match BooleanNetwork::try_from(aeon_string) {
        Ok(network) => {
            let locked_computation = get_locked_computation(session_key);

            {
                // First, just try to read the computation, if there is something
                // there, we just want to quit fast...
                let read_computation = locked_computation.read().unwrap();
                if let Some(computation) = read_computation.as_ref() {
                    if computation.is_running() {
                        return Err(String::from("Previous computation is still running. Cancel it before starting a new one."));
                    }
                }
            }

            {
                // Now actually get the write lock, but check again because race conditions...
                let mut write_computation = locked_computation.write().unwrap();
                if let Some(computation) = write_computation.as_ref() {
                    if computation.is_running() {
                        return Err(String::from("Previous computation is still running. Cancel it before starting a new one."));
                    }
                }

                let mut new_computation = Computation::new_computation(aeon_string.to_string());

                // Change to String, so the thread can use it while running
                let session_key = session_key.to_string();

                // Prepare thread - note that we have computation locked, so the thread
                // will have to wait for us to end before writing down the graph and other
                // stuff.
                let computation_thread = prepare_computation_thread(session_key, network);
                new_computation.thread = Some(computation_thread);

                let start = new_computation.start_timestamp();

                // Now write the new computation to the global state...
                *write_computation = Some(new_computation);
                Ok(start as u64)
            }
        }
        Err(error) => Err(error),
    }
}

/// Cancel running computation.
#[tauri::command]
pub fn cancel_computation(session_key: &str) -> Result<String, String> {
    let locked_computation: Arc<RwLock<Option<Computation>>> = get_locked_computation(session_key);
    let read_computation = locked_computation.read().unwrap();

    if let Some(computation) = read_computation.as_ref() {
        match computation.cancel() {
            Ok(ok_message) => Ok(ok_message.to_string()),
            Err(error_message) => Err(error_message.to_string()),
        }
    } else {
        Err("No computation found.".to_string())
    }
}

/// Get result of computation.
#[tauri::command]
pub async fn get_results(session_key: &str) -> Result<Value, ErrorMessage> {
    let locked_computation: Arc<RwLock<Option<Computation>>> = get_locked_computation(session_key);
    let read_computation = locked_computation.read().unwrap();

    let Some(computation) = read_computation.as_ref() else {
        return Err(String::from("No computation found."));
    };

    match computation.get_results() {
        Err(error_message) => Err(error_message.to_string()),
        Ok(results) => {
            // Format result data to json object
            let classification_map: Vec<JsonValue> = results
                .classification_map
                .iter()
                .map(|(c, p)| {
                    object! {
                        sat_count: p.approx_cardinality(),
                        phenotype: c.to_json()
                    }
                })
                .collect();

            println!("Result {:?}", classification_map);

            // Truncate the elapsed time to u64 as u128 is not supported in json right now.
            let elapsed = results.elapsed_ms.unwrap_or(0);
            let elapsed = u64::try_from(elapsed).unwrap_or(u64::MAX);

            let json = object! {
                isPartial: results.is_partial,
                isCancelled: results.is_cancelled,
                elapsed: elapsed,
                data: JsonValue::Array(classification_map)
            };

            // Convert JsonValue to serde_json::Value
            Ok(from_str::<Value>(json.to_string().as_ref()).unwrap())
        }
    }
}

/// Get info about computation process.
#[tauri::command]
pub fn get_computation_process_info(session_key: &str) -> Result<Value, ErrorMessage> {
    let locked_computation: Arc<RwLock<Option<Computation>>> = get_locked_computation(session_key);
    let read_computation = locked_computation.read().unwrap();

    if let Some(computation) = read_computation.as_ref() {
        Ok(computation.get_info())
    } else {
        Err(String::from("No computation found."))
    }
}
