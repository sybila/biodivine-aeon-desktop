use crate::common::{ErrResponse, OkResponse};
use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use biodivine_lib_param_bn::{BooleanNetwork, FnUpdate};
use json::object;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;

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
pub fn check_update_function(data: &str) -> Result<OkResponse, ErrResponse> {
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
        })
        .map(|g| g.unit_colors().approx_cardinality());

    // println!(
    //     "Elapsed: {}, result {:?}",
    //     start.elapsed().unwrap().as_millis(),
    //     graph
    // );

    match graph {
        Ok(cardinality) => Ok(OkResponse::new(
            object! {
                "cardinality" => cardinality
            }
            .to_string()
            .as_str(),
        )),
        Err(error) => Err(ErrResponse::new(&error)),
    }
}

/// Accept an SBML (XML) file and try to parse it into a `BooleanNetwork`.
/// If everything goes well, return a standard result object with a parsed model, or
/// error if something fails.
#[tauri::command]
pub fn sbml_to_aeon(data: &str) -> Result<OkResponse, ErrResponse> {
    match BooleanNetwork::try_from_sbml(data) {
        Ok((model, layout)) => {
            let mut model_string = format!("{}", model); // convert back to aeon
            model_string += "\n";
            for (var, (x, y)) in layout {
                model_string += format!("#position:{}:{},{}\n", var, x, y).as_str();
            }
            Ok(OkResponse::new(
                &object! { "model" => model_string }.to_string(),
            ))
        }
        Err(error) => Err(ErrResponse::new(&error)),
    }
}

/// Try to read the model layout metadata from the given aeon file.
fn read_layout(aeon_string: &str) -> HashMap<String, (f64, f64)> {
    let re = Regex::new(r"^\s*#position:(?P<var>[a-zA-Z0-9_]+):(?P<x>.+?),(?P<y>.+?)\s*$").unwrap();
    let mut layout = HashMap::new();
    for line in aeon_string.lines() {
        if let Some(captures) = re.captures(line) {
            let var = captures["var"].to_string();
            let x = captures["x"].parse::<f64>();
            let y = captures["y"].parse::<f64>();
            if let (Ok(x), Ok(y)) = (x, y) {
                layout.insert(var, (x, y));
            }
        }
    }
    layout
}

/// Accept an Aeon file, try to parse it into a `BooleanNetwork`
/// which will then be translated into SBML (XML) representation.
/// Preserve layout metadata.
#[tauri::command]
pub fn aeon_to_sbml(data: &str) -> Result<OkResponse, ErrResponse> {
    match BooleanNetwork::try_from(data) {
        Ok(network) => {
            let layout = read_layout(&data);
            let sbml_string = network.to_sbml(Some(&layout));
            Ok(OkResponse::new(
                &object! { "model" => sbml_string }.to_string(),
            ))
        }
        Err(error) => Err(ErrResponse::new(&error)),
    }
}

/// Accept an Aeon file and create an SBML version with all parameters instantiated (a witness model).
/// Note that this can take quite a while for large models since we have to actually build
/// the unit BDD right now (in the future, we might opt to use a SAT solver which might be faster).
#[tauri::command]
pub fn aeon_to_sbml_instantiated(data: &str) -> Result<OkResponse, ErrResponse> {
    match BooleanNetwork::try_from(data).and_then(SymbolicAsyncGraph::new) {
        Ok(graph) => {
            let witness = graph.pick_witness(graph.unit_colors());
            let layout = read_layout(&data);
            Ok(OkResponse::new(
                &object! { "model" => witness.to_sbml(Some(&layout)) }.to_string(),
            ))
        }
        Err(error) => Err(ErrResponse::new(&error)),
    }
}
