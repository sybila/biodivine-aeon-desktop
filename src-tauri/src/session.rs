use crate::computation::{ArcBdt, ArcComputation};
use crate::computation_commands::{cancel_computation, get_locked_computation};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Session {
    pub computation: ArcComputation,
    pub tree: ArcBdt,
}

/// Create an empty session with no data.
impl Default for Session {
    fn default() -> Self {
        Session {
            computation: Arc::new(RwLock::default()),
            tree: Arc::new(Default::default()),
        }
    }
}

/// Print when session is destroyed.
impl Drop for Session {
    fn drop(&mut self) {
        println!("Session dropped")
    }
}

lazy_static! {
    /// Hashmap with all sessions: key = window label, value = ArcComputation.
    pub static ref SESSIONS: Arc<RwLock<HashMap<String, Session>>> = Arc::new(RwLock::new(HashMap::new()));
}

/// Check if window has session.
pub fn is_there_session(window_session_key: &str) -> bool {
    let sessions = SESSIONS.read().unwrap();
    sessions.contains_key(window_session_key)
}

/// Check if session has running computation.
#[tauri::command]
pub fn has_running_computation(window_session_key: &str) -> bool {
    let locked_computation = get_locked_computation(window_session_key);
    let read_computation = locked_computation.read().unwrap();

    if let Some(computation) = read_computation.as_ref() {
        return computation.is_running();
    }

    false
}

/// Create new session when Tauri window is created.
#[tauri::command]
pub fn add_window_session(window_session_key: &str) {
    println!(
        "Window session with key: '{}' was created",
        window_session_key
    );

    let mut sessions = SESSIONS.write().unwrap();
    sessions.insert(window_session_key.to_string(), Session::default());

    println!("Number of window sessions: {}", sessions.len());
    println!();
}

/// Remove a window session from the collection of sessions when the window is destroyed.
#[tauri::command]
pub fn remove_window_session(window_session_key: &str) -> Result<String, String> {
    println!(
        "Window session with key: '{}' will be removed",
        window_session_key
    );

    // First, found out if there is running computation.
    let locked_computation = get_locked_computation(window_session_key);
    let read_computation = locked_computation.read().unwrap();
    if let Some(computation) = read_computation.as_ref() {
        if computation.is_running() {
            match cancel_computation(window_session_key) {
                Ok(..) => {
                    // Computation was successfully canceled, continue
                }
                Err(error_message) => return Err(error_message),
            }
        }
    }

    let mut sessions = SESSIONS.write().unwrap();
    sessions.remove(window_session_key);

    println!("Session with key: '{}' was removed", window_session_key);

    Ok("Window session was successfully removed".to_string())
}
