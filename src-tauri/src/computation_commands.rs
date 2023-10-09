use std::time::SystemTime;
use std::sync::{RwLock, Arc};
use biodivine_aeon_desktop::scc::Classifier;
use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use std::thread::JoinHandle;
use biodivine_aeon_desktop::scc::algo_interleaved_transition_guided_reduction::interleaved_transition_guided_reduction;
use biodivine_aeon_desktop::scc::algo_xie_beerel::xie_beerel_attractors;
use biodivine_lib_param_bn::BooleanNetwork;
use json::{object};
use biodivine_aeon_desktop::bdt::Bdt;
use crate::common::{ErrResponse, OkResponse};
use crate::computation::{ArcBdt, ArcComputation, Computation};
use crate::session::{SESSIONS, is_there_session};


/// Get Arc pointer of locked Computation.
pub fn get_locked_computation(window_session_key: &str) -> ArcComputation {
    let sessions = SESSIONS.read().unwrap();
    let locked_computation= &sessions.get(window_session_key).unwrap().computation;
    locked_computation.clone()
}


/// Get Arc pointer of locked Tree.
pub fn get_locked_tree(window_session_key: &str) -> ArcBdt {
    let sessions = SESSIONS.read().unwrap();
    let locked_tree= &sessions.get(window_session_key).unwrap().tree;
    locked_tree.clone()
}


/// Prepare thread for computation.
fn prepare_computation_thread(window_session_key: String, network: BooleanNetwork) -> JoinHandle<()> {
    let cmp_thread = std::thread::spawn(move || {
        let cmp: Arc<RwLock<Option<Computation>>> = get_locked_computation(window_session_key.as_str());
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
                        panic!("Cannot finish computation. No computation found.")
                    }
                }

                {
                    // Check if the session still exists because window might be already closed
                    if is_there_session(window_session_key.as_str()) {
                        let result = classifier.export_result();
                        let tree = get_locked_tree(window_session_key.as_str());
                        let mut tree = tree.write().unwrap();
                        *tree = Some(Bdt::new_from_graph(result, &graph));
                        println!("Saved decision tree.");
                    } else {
                        if let Some(cmp) = cmp.read().unwrap().as_ref() {
                            if cmp.is_cancelled() {
                                return
                            }
                            else {
                                panic!("Computation lost session but is not cancelled.");
                            }
                        } else {
                            panic!("Cannot save tree. Thread lost its computation")
                        }
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
    });
    return cmp_thread
}


/// Accept an Aeon model, parse it and start a new computation (if there is no computation running).
#[tauri::command]
pub fn start_computation(window_session_key: &str, aeon_string: &str) -> Result<OkResponse, ErrResponse> {
    match BooleanNetwork::try_from(aeon_string) {
        Ok(network) => {
            let locked_computation = get_locked_computation(window_session_key);

            {
                // First, just try to read the computation, if there is something
                // there, we just want to quit fast...
                let read_computation = locked_computation.read().unwrap();
                if let Some(computation) = read_computation.as_ref() {
                    if computation.is_running() {
                        return Err(ErrResponse::new("Previous computation is still running. Cancel it before starting a new one."));
                    }
                }
            }

            {
                // Now actually get the write lock, but check again because race conditions...
                let mut write_computation = locked_computation.write().unwrap();
                if let Some(computation) = write_computation.as_ref() {
                    if computation.is_running() {
                        return Err(ErrResponse::new("Previous computation is still running. Cancel it before starting a new one."));
                    }
                }

                let mut new_computation = Computation::new_computation(aeon_string.to_string());

                // Change to String, so the thread can use it while running
                let window_session_key = window_session_key.to_string();

                // Prepare thread - note that we have computation locked, so the thread
                // will have to wait for us to end before writing down the graph and other
                // stuff.
                let computation_thread = prepare_computation_thread(window_session_key, network);
                new_computation.thread = Some(computation_thread);

                let start = new_computation.start_timestamp();

                // Now write the new computation to the global state...
                *write_computation = Some(new_computation);
                Ok(OkResponse::new(&object! { "timestamp" => start as u64 }.to_string()))
            }
        }
        Err(error) => Err(ErrResponse::new(&error)),
    }
}


/// Cancel running computation.
#[tauri::command]
pub fn cancel_computation(window_session_key: &str) -> Result<String, String> {
    let locked_computation: Arc<RwLock<Option<Computation>>> = get_locked_computation(window_session_key);
    let read_computation = locked_computation.read().unwrap();

    if let Some(computation) = read_computation.as_ref() {
        match computation.cancel() {
            Ok(ok_message) => {
                return Ok(ok_message.to_string())
            },
            Err(error_message) => {
                return Err(error_message.to_string())
            }
        }
    } else {
        return Err("No computation found.".to_string())
    }
}


/// Get result of computation.
#[tauri::command]
pub fn get_results(window_session_key: &str) -> Result<OkResponse, ErrResponse> {
    let locked_computation: Arc<RwLock<Option<Computation>>> = get_locked_computation(window_session_key);
    let read_computation = locked_computation.read().unwrap();

    if let Some(computation) = read_computation.as_ref() {
        match computation.get_results() {
            Ok((data, elapsed, is_partial, is_cancelled)) => {

                // Format result data to json object
                let lines: Vec<String> = data
                    .iter()
                    .map(|(c, p)| {
                        format!(
                            "{{\"sat_count\":{},\"phenotype\":{}}}",
                            p.approx_cardinality(),
                            c
                        )
                    })
                    .collect();

                println!("Result {:?}", lines);

                let elapsed = if let Some(e) = elapsed { e } else { 0 };

                let mut json = String::new();
                for line in lines.iter().take(lines.len() - 1) {
                    json += &format!("{},", line);
                }
                json = format!(
                    "{{ \"isPartial\":{}, \"isCancelled\":{}, \"data\":[{}{}], \"elapsed\":{} }}",
                    is_partial,
                    is_cancelled,
                    json,
                    lines.last().unwrap(),
                    elapsed,
                );

                return Ok(OkResponse::new(&json))
            },
            Err(error_message) => {
                return Err(ErrResponse::new(error_message))
            }
        }
    } else {
        Err(ErrResponse::new("No computation found."))
    }
}


/// Get info about computation process.
#[tauri::command]
pub fn get_computation_process_info(window_session_key: &str) -> String {
    let locked_computation: Arc<RwLock<Option<Computation>>> = get_locked_computation(window_session_key);
    let read_computation = locked_computation.read().unwrap();

    if let Some(computation) = read_computation.as_ref() {
        return computation.get_info().to_string()
    } else {
        return String::from("No computation found.")
    }
}