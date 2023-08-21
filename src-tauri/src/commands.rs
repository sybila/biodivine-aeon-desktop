use std::cmp::max;
use std::time::SystemTime;
use std::sync::{Arc, RwLock};
use biodivine_lib_param_bn::{BooleanNetwork, FnUpdate};
use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use json::object;
use lazy_static::lazy_static;
use crate::common::BackendResponse;



lazy_static! {
    static ref CHECK_UPDATE_FUNCTION_LOCK: Arc<RwLock<bool>> = Arc::new(RwLock::new(true));
}

fn max_parameter_cardinality(function: &FnUpdate) -> usize {
    match function {
        FnUpdate::Const(_) | FnUpdate::Var(_) => 0,
        FnUpdate::Param(_, args) => args.len(),
        FnUpdate::Not(inner) => max_parameter_cardinality(inner),
        FnUpdate::Binary(_, left, right) => max(
            max_parameter_cardinality(left),
            max_parameter_cardinality(right),
        ),
    }
}


/// Accept a partial model containing only the necessary regulations and one update function.
/// Return cardinality of such model (i.e. the number of instantiations of this update function)
/// or error if the update function (or model) is invalid.
#[tauri::command]
pub fn check_update_function(data: &str) -> Result<BackendResponse, BackendResponse> {
    let lock = CHECK_UPDATE_FUNCTION_LOCK.clone();
    let mut lock = lock.write().unwrap();
    let start = SystemTime::now();
    let graph = BooleanNetwork::try_from(data)
        .and_then(|model| {
            let mut max_size = 0;
            for v in model.variables() {
                if let Some(update_function) = model.get_update_function(v) {
                    max_size = max(max_size, max_parameter_cardinality(update_function));
                } else {
                    max_size = max(max_size, model.regulators(v).len())
                }
            }
            if max_size <= 5 {
                // println!(
                //     "Start partial function analysis. {} variables and complexity {}.",
                //     model.num_vars(),
                //     max_size
                // );
                SymbolicAsyncGraph::new(model)
            } else {
                Err("Function too large for on-the-fly analysis.".to_string())
            }
        }).map(|g| g.unit_colors().approx_cardinality());

    // println!(
    //     "Elapsed: {}, result {:?}",
    //     start.elapsed().unwrap().as_millis(),
    //     graph
    // );

    (*lock) = !(*lock);

    match graph {
        Ok(cardinality) => {
            Ok(BackendResponse::ok(object! {
                "cardinality" => cardinality
            }.to_string().as_str()))
        },
        Err(error) => {
            Err(BackendResponse::err(&error))
        }
    }
}



/// Accept an SBML (XML) file and try to parse it into a `BooleanNetwork`.
/// If everything goes well, return a standard result object with a parsed model, or
/// error if something fails.
#[tauri::command]
pub fn sbml_to_aeon(data: &str) -> Result<BackendResponse, BackendResponse> {
    match BooleanNetwork::try_from_sbml(data) {
        Ok((model, layout)) => {
            let mut model_string = format!("{}", model); // convert back to aeon
            model_string += "\n";
            for (var, (x, y)) in layout {
                model_string += format!("#position:{}:{},{}\n", var, x, y).as_str();
            }
            Ok(BackendResponse::ok(&object! { "model" => model_string }.to_string()))
        }
        Err(error) => Err(BackendResponse::err(&error)),
    }
}




