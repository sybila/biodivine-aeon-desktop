use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Window;
use lazy_static::lazy_static;
use std::sync::{RwLock, Arc};
use biodivine_aeon_desktop::GraphTaskContext;
use biodivine_aeon_desktop::scc::{Behaviour, Class, Classifier};
use biodivine_lib_param_bn::symbolic_async_graph::{GraphColors, SymbolicAsyncGraph};
use std::thread::JoinHandle;
use std::collections::HashMap;


// start_computation
use biodivine_aeon_desktop::scc::algo_interleaved_transition_guided_reduction::interleaved_transition_guided_reduction;
use biodivine_aeon_desktop::scc::algo_xie_beerel::xie_beerel_attractors;
use biodivine_lib_param_bn::BooleanNetwork;
use json::object;
use biodivine_aeon_desktop::bdt::Bdt;
use crate::common::{ErrResponse, OkResponse};


/// Locked type of Computation
type ArcComputation = Arc<RwLock<Option<Computation>>>;


/// Computation keeps all information
struct Computation {
    timestamp: SystemTime,
    input_model: String,    // .aeon string representation of the model
    task: GraphTaskContext, // A task context which keeps track of progress and cancellation.
    graph: Option<Arc<SymbolicAsyncGraph>>, // Model graph - used to create witnesses
    classifier: Option<Arc<Classifier>>, // Classifier used to store the results of the computation
    thread: Option<JoinHandle<()>>, // A thread that is actually doing the computation (so that we can check if it is still running). If none, the computation is done.
    error: Option<String>,          // A string error from the computation
    finished_timestamp: Option<SystemTime>, // A timestamp when the computation was completed (if done)
}

impl Computation {
    pub fn start_timestamp(&self) -> u128 {
        self.timestamp
            .duration_since(UNIX_EPOCH)
            .expect("Time error")
            .as_millis()
    }

    pub fn end_timestamp(&self) -> Option<u128> {
        self.finished_timestamp.map(|t| {
            t.duration_since(UNIX_EPOCH)
                .expect("Time error")
                .as_millis()
        })
    }
}

/// Hashmap with all sessions: key = window label, value = ArcComputation
lazy_static! {
    static ref SESSIONS: Arc<RwLock<HashMap<String, ArcComputation>>> = Arc::new(RwLock::new(HashMap::new()));
}


/// Creates new session when Tauri window is created
#[tauri::command]
pub fn add_window_session(window_label: &str) {
    println!("Window with label: {} was created", window_label);

    let mut sessions = SESSIONS.write().unwrap();
    let empty_computation: ArcComputation = Arc::new(RwLock::new(None));
    sessions.insert(window_label.to_string(), empty_computation);

    println!("Number of sessions: {}", sessions.len());
    println!();
}


/// Removes a window session from the collection of sessions when the window is destroyed
#[tauri::command]
pub fn remove_window_session(window_label: &str) {
    println!("Window with label: {} will be removed", window_label);

    // First, found out if there is running computation.
    // If yes, cancel the computation.
    let locked_computation = get_locked_computation(window_label);
    {
        let cmp = locked_computation.read().unwrap();
        if let Some(computation) = &*cmp {
            if computation.thread.is_some() {
                cancel_computation(window_label);
            }
        }
    }

    let mut sessions = SESSIONS.write().unwrap();
    sessions.remove(window_label);

    println!("Number of sessions: {}", sessions.len());
    println!();
}


/// Gets Arc pointer of locked Computation.
fn get_locked_computation(window_label: &str) -> Arc<RwLock<Option<Computation>>> {
    let sessions = SESSIONS.read().unwrap();
    let locked_session = sessions.get(window_label).unwrap();
    locked_session.clone()
}


/// Accept an Aeon model, parse it and start a new computation (if there is no computation running).
#[tauri::command]
pub fn start_computation(window_label: &str, aeon_string: &str) -> Result<OkResponse, ErrResponse> {
    match BooleanNetwork::try_from(aeon_string) {
        Ok(network) => {

            let locked_computation = get_locked_computation(window_label);
            // Now we can try to start the computation...
            {
                // First, just try to read the computation, if there is something
                // there, we just want to quit fast...
                let cmp = locked_computation.read().unwrap();
                if let Some(computation) = &*cmp {
                    if computation.thread.is_some() {
                        return Err(ErrResponse::new("Previous computation is still running. Cancel it before starting a new one."));
                    }
                }
            }
            {
                // Now actually get the write lock, but check again because race conditions...
                let mut cmp = locked_computation.write().unwrap();
                if let Some(computation) = &*cmp {
                    if computation.thread.is_some() {
                        return Err(ErrResponse::new("Previous computation is still running. Cancel it before starting a new one."));
                    }
                }
                let mut new_cmp = Computation {
                    timestamp: SystemTime::now(),
                    task: GraphTaskContext::new(),
                    input_model: aeon_string.to_string(),
                    graph: None,
                    classifier: None,
                    thread: None,
                    error: None,
                    finished_timestamp: None,
                };

                // Create copy of window_label for thread
                let cloned_window_label = window_label.to_string();

                // Prepare thread - note that we have computation locked, so the thread
                // will have to wait for us to end before writing down the graph and other
                // stuff.
                let cmp_thread = std::thread::spawn(move || {
                    let cmp: Arc<RwLock<Option<Computation>>> = get_locked_computation(cloned_window_label.as_str());
                    match SymbolicAsyncGraph::new(network) {
                        Ok(graph) => {
                            // Now that we have graph, we can create classifier and progress
                            // and save them into the computation.
                            let classifier = Arc::new(Classifier::new(&graph));
                            let graph = Arc::new(graph);
                            {
                                if let Some(cmp) = cmp.write().unwrap().as_mut() {
                                    cmp.graph = Some(graph.clone());
                                    cmp.classifier = Some(classifier.clone());
                                } else {
                                    panic!("Cannot save graph. No computation found.")
                                }
                            }

                            if let Some(cmp) = cmp.read().unwrap().as_ref() {
                                // TODO: Note that this holds the read-lock on computation
                                // for the  whole time, which is mostly ok because it can be
                                // cancelled without write-lock, but we should find a
                                // way to avoid this!
                                let task_context = &cmp.task;
                                task_context.restart(&graph);

                                // Now we can actually start the computation...

                                // First, perform ITGR reduction.
                                let (universe, active_variables) =
                                    interleaved_transition_guided_reduction(
                                        task_context,
                                        &graph,
                                        graph.mk_unit_colored_vertices(),
                                    );

                                // Then run Xie-Beerel to actually detect the components.
                                xie_beerel_attractors(
                                    task_context,
                                    &graph,
                                    &universe,
                                    &active_variables,
                                    |component| {
                                        println!(
                                            "Component {}",
                                            component.approx_cardinality()
                                        );
                                        classifier.add_component(component, &graph);
                                    },
                                );
                            }

                            {
                                if let Some(cmp) = cmp.write().unwrap().as_mut() {
                                    cmp.finished_timestamp = Some(SystemTime::now());
                                } else {
                                    panic!(
                                        "Cannot finish computation. No computation found."
                                    )
                                }
                            }

                            {
                                let result = classifier.export_result();
                                // let tree = TREE.clone();
                                // let mut tree = tree.write().unwrap();
                                // *tree = Some(Bdt::new_from_graph(result, &graph));
                                // println!("Saved decision tree");
                            }

                            println!("Component search done...");
                        }
                        Err(error) => {
                            if let Some(cmp) = cmp.write().unwrap().as_mut() {
                                cmp.error = Some(error);
                            } else {
                                panic!(
                                    "Cannot save computation error. No computation found."
                                )
                            }
                        }
                    }
                    {
                        // Remove reference to thread, since we are done now...
                        if let Some(cmp) = cmp.write().unwrap().as_mut() {
                            cmp.thread = None;
                        } else {
                            panic!("Cannot finalize thread. No computation found.");
                        };
                    }
                });
                new_cmp.thread = Some(cmp_thread);

                let start = new_cmp.start_timestamp();
                // Now write the new computation to the global state...
                *cmp = Some(new_cmp);

                Ok(OkResponse::new(&object! { "timestamp" => start as u64 }.to_string()))
                // status of the computation can be obtained via ping...
            }
        }
        Err(error) => Err(ErrResponse::new(&error)),
    }
}


#[tauri::command]
pub fn cancel_computation(window_label: &str) -> Result<OkResponse, ErrResponse> {
    let locked_computation = get_locked_computation(window_label);
    {
        // first just check there is something to cancel
        let cmp = locked_computation.read().unwrap();
        if let Some(cmp) = &*cmp {
            if cmp.thread.is_none() {
                return Err(ErrResponse::new("Nothing to cancel. Computation already done."));
            }
            if cmp.task.is_cancelled() {
                return Err(ErrResponse::new("Computation already cancelled."));
            }
        } else {
            return Err(ErrResponse::new("No computation to cancel."));
        }
    }
    let cmp = locked_computation.read().unwrap();
    if let Some(cmp) = &*cmp {
        if cmp.thread.is_none() {
            return Err(ErrResponse::new("Nothing to cancel. Computation already done."));
        }
        if cmp.task.cancel() {
            Ok(OkResponse::new(&"\"ok\"".to_string()))
        } else {
            Err(ErrResponse::new("Computation already cancelled."))
        }
    } else {
        Err(ErrResponse::new("No computation to cancel."))
    }
}
