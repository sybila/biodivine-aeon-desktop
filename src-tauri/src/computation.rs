use crate::session::{get_locked_computation, get_locked_tree, is_there_session};
use biodivine_aeon_desktop::algorithms::scc::algo_interleaved_transition_guided_reduction::interleaved_transition_guided_reduction;
use biodivine_aeon_desktop::algorithms::scc::algo_xie_beerel::xie_beerel_attractors;
use biodivine_aeon_desktop::algorithms::scc::{Class, Classifier};
use biodivine_aeon_desktop::bdt::Bdt;
use biodivine_aeon_desktop::GraphTaskContext;
use biodivine_lib_param_bn::symbolic_async_graph::{GraphColors, SymbolicAsyncGraph};
use biodivine_lib_param_bn::BooleanNetwork;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Locked type of Bifurcation decision tree.
pub type ArcBdt = Arc<RwLock<Option<Bdt>>>;

/// Locked type of Computation.
pub type ArcComputation = Arc<RwLock<Option<Computation>>>;

/// Computation keeps all information.
pub struct Computation {
    pub timestamp: SystemTime,
    pub input_model: String,    // .aeon string representation of the model
    pub task: GraphTaskContext, // A task context which keeps track of progress and cancellation.
    pub graph: Option<Arc<SymbolicAsyncGraph>>, // Model graph - used to create witnesses
    pub classifier: Option<Arc<Classifier>>, // Classifier used to store the results of the computation
    pub thread: Option<JoinHandle<()>>, // A thread that is actually doing the computation (so that we can check if it is still running). If none, the computation is done.
    pub error: Option<String>,          // A string error from the computation
    pub finished_timestamp: Option<SystemTime>, // A timestamp when the computation was completed (if done)
}

pub struct ComputationResults {
    pub classification_map: HashMap<Class, GraphColors>,
    pub elapsed_ms: Option<u128>,
    pub is_partial: bool,
    pub is_cancelled: bool,
}

impl Drop for Computation {
    fn drop(&mut self) {
        if self.is_running() {
            self.task.cancel();
            println!("Computation cancelled and dropped");
            return;
        }
        println!("Computation dropped")
    }
}

impl Computation {
    pub fn new_computation(aeon_string: String) -> Computation {
        Computation {
            timestamp: SystemTime::now(),
            task: GraphTaskContext::new(),
            input_model: aeon_string,
            graph: None,
            classifier: None,
            thread: None,
            error: None,
            finished_timestamp: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.thread.is_some()
    }

    pub fn is_done(&self) -> bool {
        self.thread.is_none()
    }

    pub fn is_cancelled(&self) -> bool {
        self.task.is_cancelled()
    }

    pub fn get_percentage_progress(&self) -> String {
        self.task.get_percent_string()
    }

    pub fn get_results(&self) -> Result<ComputationResults, &str> {
        let is_partial = self.is_running();
        let is_cancelled = self.is_cancelled();

        if let Some(classes) = &self.classifier {
            let mut result = None;
            for _ in 0..5 {
                if let Some(data) = classes.try_export_result() {
                    result = Some(data);
                    break;
                }
                // wait a little - maybe the lock will become free
                std::thread::sleep(Duration::new(1, 0));
            }
            if let Some(result) = result {
                let elapsed_ms = self.end_timestamp().map(|t| t - self.start_timestamp());
                Ok(ComputationResults {
                    classification_map: result,
                    elapsed_ms,
                    is_partial,
                    is_cancelled,
                })
            } else {
                Err("Classification running. Cannot export components right now.")
            }
        } else {
            Err("Results not available yet.")
        }
    }

    pub fn get_info(&self) -> Value {
        let mut response = json! ({
        "timestamp": null,                // if there is some computation (not necessarily running, this is the time when it started
        "is_cancelled": false,            // true if the computation has been canceled
        "is_running": false,              // true if the computation thread is still alive
        "progress": "unknown",            // arbitrary progress string
        "error": null,                    // arbitrary error string - currently not really used
        "num_classes": null,              // number of discovered classes so far
        });
        response["timestamp"] = (self.start_timestamp() as u64).into();
        response["is_cancelled"] = self.is_cancelled().into();
        response["progress"] = self.get_percentage_progress().into();
        response["is_running"] = self.is_running().into();
        if let Some(error) = &self.error {
            response["error"] = error.clone().into();
        }
        if let Some(classes) = self.classifier.as_ref().map(|c| c.try_get_num_classes()) {
            response["num_classes"] = classes.into();
        }

        response
    }

    pub fn cancel(&self) -> Result<&str, &str> {
        if self.is_done() {
            return Err("Nothing to cancel. Computation already done.");
        }
        if self.task.cancel() {
            Ok("Computation successfully canceled.")
        } else {
            Err("Computation already cancelled.")
        }
    }

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

/// Prepare thread for computation.
pub fn prepare_computation_thread(session_key: String, network: BooleanNetwork) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let cmp: Arc<RwLock<Option<Computation>>> = get_locked_computation(session_key.as_str());
        match SymbolicAsyncGraph::new(&network) {
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
                    let (universe, active_variables) = interleaved_transition_guided_reduction(
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
                            println!("Component {}", component.approx_cardinality());
                            classifier.add_component(component, &graph);
                        },
                    );
                }

                {
                    if let Some(cmp) = cmp.write().unwrap().as_mut() {
                        cmp.finished_timestamp = Some(SystemTime::now());
                    } else {
                        panic!("Cannot finish computation. No computation found.")
                    }
                }

                {
                    // Check if the session still exists because window might be already closed
                    if is_there_session(session_key.as_str()) {
                        let result = classifier.export_result();
                        let tree = get_locked_tree(session_key.as_str());
                        let mut tree = tree.write().unwrap();
                        *tree = Some(Bdt::new_from_graph(result, &graph));
                        println!("Saved decision tree.");
                    } else if let Some(cmp) = cmp.read().unwrap().as_ref() {
                        if cmp.is_cancelled() {
                            return;
                        } else {
                            panic!("Computation lost session but is not cancelled.");
                        }
                    } else {
                        panic!("Cannot save tree. Thread lost its computation")
                    }
                }

                println!("Component search done...");
            }
            Err(error) => {
                if let Some(cmp) = cmp.write().unwrap().as_mut() {
                    cmp.error = Some(error);
                } else {
                    panic!("Cannot save computation error. No computation found.")
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
    })
}
