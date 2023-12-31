use crate::computation_results::{
    get_witness_attractors, get_witness_network, try_get_class_params,
};
use crate::session::{get_locked_computation, get_locked_tree};
use crate::types::ErrorMessage;
use biodivine_aeon_desktop::algorithms::scc::algo_stability_analysis::{
    StabilityVector, VariableStability,
};
use biodivine_aeon_desktop::algorithms::scc::{Behaviour, Class};
use biodivine_aeon_desktop::bdt::BdtNodeId;
use biodivine_aeon_desktop::util::functional::Functional;
use biodivine_lib_param_bn::biodivine_std::traits::Set;
use serde_json::{from_str, Value};

/// Get witness of behaviour class.
#[tauri::command]
pub async fn get_witness(class_str: String, session_key: &str) -> Result<String, ErrorMessage> {
    let mut class = Class::new_empty();
    for char in class_str.chars() {
        match char {
            'D' => class.extend(Behaviour::Disorder),
            'O' => class.extend(Behaviour::Oscillation),
            'S' => class.extend(Behaviour::Stability),
            _ => {
                return Err(String::from("Invalid class."));
            }
        }
    }
    {
        let locked_computation = get_locked_computation(session_key);
        let read_computation = locked_computation.read().unwrap();
        if let Some(computation) = read_computation.as_ref() {
            if let Some(classifier) = &computation.classifier {
                if let Some(has_class) = try_get_class_params(classifier, &class) {
                    if let Some(class) = has_class {
                        get_witness_network(&class, session_key)
                    } else {
                        Err(String::from("Specified class has no witness."))
                    }
                } else {
                    Err(String::from(
                        "Classification in progress. Cannot extract witness right now.",
                    ))
                }
            } else {
                Err(String::from("No results available."))
            }
        } else {
            Err(String::from("No results available."))
        }
    }
}

/// Get witness of behaviour class from tree node.
#[tauri::command]
pub async fn get_tree_witness(node_id: String, session_key: &str) -> Result<String, ErrorMessage> {
    let locked_tree = get_locked_tree(session_key);
    let read_tree = locked_tree.read().unwrap();
    return if let Some(tree) = read_tree.as_ref() {
        let node = BdtNodeId::try_from(node_id.parse::<usize>().unwrap(), tree);
        let node = if let Some(node) = node {
            node
        } else {
            return Err(format!("Invalid node id {}.", node_id));
        };

        if let Some(params) = tree.params_for_leaf(node) {
            get_witness_network(params, session_key)
        } else {
            Err(String::from("Given node is not an unprocessed node."))
        }
    } else {
        Err(String::from("No tree present. Run computation first."))
    };
}

/// Get witness of behaviour class from stability analysis.
#[tauri::command]
pub async fn get_stability_witness(
    node_id: String,
    behaviour_str: String,
    variable_str: String,
    vector_str: String,
    session_key: &str,
) -> Result<String, ErrorMessage> {
    let behaviour = Behaviour::try_from(behaviour_str.as_str());
    let behaviour = match behaviour {
        Ok(behaviour) => Some(behaviour),
        Err(error) => {
            if behaviour_str == "total" {
                None
            } else {
                return Err(error);
            }
        }
    };
    let vector = StabilityVector::try_from(vector_str.as_str());
    let vector = match vector {
        Ok(vector) => vector,
        Err(error) => {
            return Err(error);
        }
    };
    // First, extract all colors in that tree node.
    let node_params = {
        let locked_tree = get_locked_tree(session_key);
        let read_tree = locked_tree.read().unwrap();
        if let Some(tree) = read_tree.as_ref() {
            let node = BdtNodeId::try_from(node_id.parse::<usize>().unwrap(), tree);
            let node = if let Some(n) = node {
                n
            } else {
                return Err(format!("Invalid node id {}.", node_id));
            };
            tree.all_node_params(node)
        } else {
            return Err(String::from("No bifurcation tree found."));
        }
    };
    // Then find all attractors of the graph
    let locked_computation = get_locked_computation(session_key);
    let read_computation = locked_computation.read().unwrap();
    if let Some(computation) = read_computation.as_ref() {
        let components = if let Some(classifier) = &computation.classifier {
            if let Some(behaviour) = behaviour {
                classifier.export_components_with_class(behaviour)
            } else {
                classifier
                    .export_components()
                    .into_iter()
                    .map(|(c, _)| c)
                    .collect()
            }
        } else {
            return Err(String::from("No attractor data found."));
        };
        if let Some(graph) = &computation.graph {
            let variable = graph
                .as_network()
                .as_graph()
                .find_variable(variable_str.as_str());
            let variable = if let Some(variable) = variable {
                variable
            } else {
                return Err(format!("Unknown graph variable `{}`.", variable_str));
            };

            // Now compute which attractors are actually relevant for the node colors
            let components = components
                .into_iter()
                .filter_map(|attractor| {
                    attractor
                        .intersect_colors(&node_params)
                        .take_if(|it| !it.is_empty())
                })
                .collect::<Vec<_>>();

            let variable_stability =
                VariableStability::for_attractors(graph, &components, variable);
            if let Some(colors) = &variable_stability[vector] {
                get_witness_network(colors, session_key)
            } else {
                Err(format!("No witness available for vector `{}`.", vector_str))
            }
        } else {
            Err(String::from("No attractor data found."))
        }
    } else {
        Err(String::from("No attractor data found."))
    }
}

/// Get attractors of behaviour class.
#[tauri::command]
pub async fn get_attractors(class_str: String, session_key: &str) -> Result<Value, ErrorMessage> {
    let mut class = Class::new_empty();
    for char in class_str.chars() {
        match char {
            'D' => class.extend(Behaviour::Disorder),
            'O' => class.extend(Behaviour::Oscillation),
            'S' => class.extend(Behaviour::Stability),
            _ => return Err(String::from("Invalid class.")),
        }
    }
    {
        let locked_computation = get_locked_computation(session_key);
        let read_computation = locked_computation.read().unwrap();
        if let Some(computation) = read_computation.as_ref() {
            if let Some(classifier) = &computation.classifier {
                if let Some(has_class) = try_get_class_params(classifier, &class) {
                    if let Some(class) = has_class {
                        get_witness_attractors(&class, session_key)
                    } else {
                        Err(String::from("Specified class has no witness."))
                    }
                } else {
                    Err(String::from(
                        "Classification still in progress. Cannot explore attractors now.",
                    ))
                }
            } else {
                Err(String::from("No results available."))
            }
        } else {
            Err(String::from("No results available."))
        }
    }
}

/// Get attractors of behaviour class from tree node.
#[tauri::command]
pub async fn get_tree_attractors(
    node_id: String,
    session_key: &str,
) -> Result<Value, ErrorMessage> {
    let locked_tree = get_locked_tree(session_key);
    let read_tree = locked_tree.read().unwrap();
    return if let Some(tree) = read_tree.as_ref() {
        let node = BdtNodeId::try_from(node_id.parse::<usize>().unwrap(), tree);
        let node = if let Some(value) = node {
            value
        } else {
            return Err(format!("Invalid node id {}.", node_id));
        };

        if let Some(params) = tree.params_for_leaf(node) {
            get_witness_attractors(params, session_key)
        } else {
            Err(String::from("Given node is not an unprocessed node."))
        }
    } else {
        Err(String::from("No tree present. Run computation first."))
    };
}

/// Get attractors of behaviour class from stability analysis.
#[tauri::command]
pub async fn get_stability_attractors(
    node_id: String,
    behaviour_str: String,
    variable_str: String,
    vector_str: String,
    session_key: &str,
) -> Result<Value, ErrorMessage> {
    let behaviour = Behaviour::try_from(behaviour_str.as_str());
    let behaviour = match behaviour {
        Ok(behaviour) => Some(behaviour),
        Err(error) => {
            if behaviour_str == "total" {
                None
            } else {
                return Err(error);
            }
        }
    };
    let vector = StabilityVector::try_from(vector_str.as_str());
    let vector = match vector {
        Ok(vector) => vector,
        Err(error) => {
            return Err(error);
        }
    };
    // First, extract all colors in that tree node.
    let node_params = {
        let locked_tree = get_locked_tree(session_key);
        let read_tree = locked_tree.read().unwrap();
        if let Some(tree) = read_tree.as_ref() {
            let node = BdtNodeId::try_from(node_id.parse::<usize>().unwrap(), tree);
            let node = if let Some(n) = node {
                n
            } else {
                return Err(format!("Invalid node id {}.", node_id));
            };
            tree.all_node_params(node)
        } else {
            return Err(String::from("No bifurcation tree found."));
        }
    };
    // Then find all attractors of the graph
    let locked_computation = get_locked_computation(session_key);
    let read_computation = locked_computation.read().unwrap();
    if let Some(computation) = read_computation.as_ref() {
        let components = if let Some(classifier) = &computation.classifier {
            if let Some(behaviour) = behaviour {
                classifier.export_components_with_class(behaviour)
            } else {
                classifier
                    .export_components()
                    .into_iter()
                    .map(|(c, _)| c)
                    .collect()
            }
        } else {
            return Err(String::from("No attractor data found."));
        };
        if let Some(graph) = &computation.graph {
            let variable = graph
                .as_network()
                .as_graph()
                .find_variable(variable_str.as_str());
            let variable = if let Some(variable) = variable {
                variable
            } else {
                return Err(format!("Unknown graph variable `{}`.", variable_str));
            };

            // Now compute which attractors are actually relevant for the node colors
            let components = components
                .into_iter()
                .filter_map(|attractor| {
                    attractor
                        .intersect_colors(&node_params)
                        .take_if(|it| !it.is_empty())
                })
                .collect::<Vec<_>>();

            let variable_stability =
                VariableStability::for_attractors(graph, &components, variable);
            if let Some(colors) = &variable_stability[vector] {
                get_witness_attractors(colors, session_key)
            } else {
                Err(format!("No witness available for vector `{}`.", vector_str))
            }
        } else {
            Err(String::from("No attractor data found."))
        }
    } else {
        Err(String::from("No attractor data found."))
    }
}

/// Get bifurcation tree as a result of computation.
#[tauri::command]
pub async fn get_bifurcation_tree(session_key: &str) -> Result<Value, ErrorMessage> {
    let locked_tree = get_locked_tree(session_key);
    let read_tree = locked_tree.read().unwrap();
    if let Some(tree) = read_tree.as_ref() {
        // Convert JsonValue to serde_json::Value
        Ok(from_str::<Value>(tree.to_json().to_string().as_ref()).unwrap())
    } else {
        Err(String::from("No tree present. Run computation first."))
    }
}
