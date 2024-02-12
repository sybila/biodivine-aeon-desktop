#[macro_use]
extern crate json;

use crate::algorithms::scc::ProgressTracker;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

/// Contains all non-trivial long-running symbolic algorithms that are used within AEON.
pub mod algorithms;

pub mod bdt;
/// Some utility methods which we can later move to std-lib
pub mod util;

mod _impl_graph_task_context;

/// A context object which aggregates all necessary information about a running task working with
/// a symbolic graph.
///
/// We use this to avoid passing each context variable as a (mutable) reference. It is also easier
/// to implement some utility methods this way.
#[derive(Debug)]
pub struct GraphTaskContext {
    is_cancelled: AtomicBool,
    progress: ProgressTracker,
}

/// Event that can be sent to the model editor to reload the current model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct ImportModelEvent {
    #[serde(rename = "modelString")]
    pub model_string: String,
}


/// Event that is sent by the backend when application menu item is pressed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct AeonMenuEvent {
    #[serde(rename = "menuId")]
    pub menu_id: String,
}

/// Used to aggregate starting arguments / `open with...` files.
#[derive(Default)]
pub struct OpenedUrls(Mutex<Option<Vec<url::Url>>>);

impl OpenedUrls {
    pub fn get(&self) -> Option<Vec<url::Url>> {
        self.0.lock().unwrap().clone()
    }

    pub fn set(&self, value: Vec<url::Url>) {
        *self.0.lock().unwrap() = Some(value);
    }

}