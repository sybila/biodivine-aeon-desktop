use crate::session::{get_locked_computation, Session, SESSIONS};
use crate::tauri_commands::computation_commands::cancel_computation;

/// Check if session has running computation.
#[tauri::command]
pub fn has_running_computation(session_key: &str) -> bool {
    let locked_computation = get_locked_computation(session_key);
    let read_computation = locked_computation.read().unwrap();

    if let Some(computation) = read_computation.as_ref() {
        return computation.is_running();
    }

    false
}

/// Create new session when Tauri window is created.
#[tauri::command]
pub fn add_session(session_key: &str) {
    println!(
        "Window session with key: '{}' was created",
        session_key
    );

    let mut sessions = SESSIONS.write().unwrap();
    sessions.insert(session_key.to_string(), Session::default());

    println!("Number of window sessions: {}", sessions.len());
    println!();
}

/// Remove a window session from the collection of sessions when the window is destroyed.
#[tauri::command]
pub fn remove_session(session_key: &str) -> Result<String, String> {
    println!(
        "Window session with key: '{}' will be removed",
        session_key
    );

    // First, found out if there is running computation.
    let locked_computation = get_locked_computation(session_key);
    let read_computation = locked_computation.read().unwrap();
    if let Some(computation) = read_computation.as_ref() {
        if computation.is_running() {
            match cancel_computation(session_key) {
                Ok(..) => {
                    // Computation was successfully canceled, continue
                }
                Err(error_message) => return Err(error_message),
            }
        }
    }

    let mut sessions = SESSIONS.write().unwrap();
    sessions.remove(session_key);

    println!("Session with key: '{}' was removed", session_key);

    Ok("Window session was successfully removed".to_string())
}
