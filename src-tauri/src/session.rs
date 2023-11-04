use crate::computation::{ArcBdt, ArcComputation};
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
pub fn is_there_session(session_key: &str) -> bool {
    let sessions = SESSIONS.read().unwrap();
    sessions.contains_key(session_key)
}

/// Get Arc pointer of locked Computation.
pub fn get_locked_computation(session_key: &str) -> ArcComputation {
    let sessions = SESSIONS.read().unwrap();
    let locked_computation = &sessions.get(session_key).unwrap().computation;
    locked_computation.clone()
}

/// Get Arc pointer of locked Tree.
pub fn get_locked_tree(session_key: &str) -> ArcBdt {
    let sessions = SESSIONS.read().unwrap();
    let locked_tree = &sessions.get(session_key).unwrap().tree;
    locked_tree.clone()
}
