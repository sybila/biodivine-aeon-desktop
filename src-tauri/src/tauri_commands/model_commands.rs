use crate::types::{Cardinality, ErrorMessage};
use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use biodivine_lib_param_bn::{BooleanNetwork};
use std::cmp::max;
use crate::model::{max_parameter_cardinality, read_layout};


/// Accept a partial model containing only the necessary regulations and one update function.
/// Return cardinality of such model (i.e. the number of instantiations of this update function)
/// or error if the update function (or model) is invalid.
#[tauri::command]
pub fn check_update_function(data: &str) -> Result<Cardinality, ErrorMessage> {
    BooleanNetwork::try_from(data)
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
                Err(String::from("Function too large for on-the-fly analysis."))
            }
        })
        .map(|g| g.unit_colors().approx_cardinality())
}

/// Accept an SBML (XML) file and try to parse it into a `BooleanNetwork`.
/// If everything goes well, return a standard result object with a parsed model, or
/// error if something fails.
#[tauri::command]
pub async fn sbml_to_aeon(sbml_string: &str) -> Result<String, ErrorMessage> {
    match BooleanNetwork::try_from_sbml(sbml_string) {
        Ok((model, layout)) => {
            let mut model_string = format!("{}", model); // convert back to aeon
            model_string += "\n";
            for (var, (x, y)) in layout {
                model_string += format!("#position:{}:{},{}\n", var, x, y).as_str();
            }
            Ok(model_string)
        }
        Err(error) => Err(error),
    }
}

/// Accept an Aeon file, try to parse it into a `BooleanNetwork`
/// which will then be translated into SBML (XML) representation.
/// Preserve layout metadata.
#[tauri::command]
pub async fn aeon_to_sbml(aeon_string: &str) -> Result<String, ErrorMessage> {
    match BooleanNetwork::try_from(aeon_string) {
        Ok(network) => {
            let layout = read_layout(aeon_string);
            let sbml_string = network.to_sbml(Some(&layout));
            Ok(sbml_string)
        }
        Err(error) => Err(error),
    }
}

/// Accept an Aeon file and create an SBML version with all parameters instantiated (a witness model).
/// Note that this can take quite a while for large models since we have to actually build
/// the unit BDD right now (in the future, we might opt to use a SAT solver which might be faster).
#[tauri::command]
pub async fn aeon_to_sbml_instantiated(aeon_string: &str) -> Result<String, ErrorMessage> {
    match BooleanNetwork::try_from(aeon_string).and_then(SymbolicAsyncGraph::new) {
        Ok(graph) => {
            let witness = graph.pick_witness(graph.unit_colors());
            let layout = read_layout(aeon_string);
            Ok(witness.to_sbml(Some(&layout)))
        }
        Err(error) => Err(error),
    }
}
