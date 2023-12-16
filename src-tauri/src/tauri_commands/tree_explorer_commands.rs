use crate::session::{get_locked_computation, get_locked_tree};
use crate::types::ErrorMessage;
use biodivine_aeon_desktop::algorithms::scc::algo_stability_analysis::compute_stability;
use biodivine_aeon_desktop::algorithms::scc::Behaviour;
use biodivine_aeon_desktop::bdt::{AttributeId, BdtNodeId};
use biodivine_aeon_desktop::util::functional::Functional;
use biodivine_lib_param_bn::biodivine_std::traits::Set;
use json::{array, object, JsonValue};
use serde_json::{from_str, Value};

/// Expand subtree of Bdt tree to defined depth.
#[tauri::command]
pub async fn auto_expand(
    node_id: String,
    depth: String,
    session_key: &str,
) -> Result<Value, ErrorMessage> {
    let depth: u32 = {
        let parsed = depth.parse::<u32>();
        if let Ok(depth) = parsed {
            depth
        } else {
            return Err(format!("Invalid tree depth: {}", depth));
        }
    };
    if depth > 10 {
        return Err(String::from("Maximum allowed depth is 10."));
    }
    let locked_tree = get_locked_tree(session_key);
    let mut write_tree = locked_tree.write().unwrap();
    if let Some(tree) = write_tree.as_mut() {
        let node_id: BdtNodeId =
            if let Some(node_id) = BdtNodeId::try_from_str(node_id.clone(), tree) {
                node_id
            } else {
                return Err(format!("Invalid node id {}.", node_id));
            };
        let changed = tree.auto_expand(node_id, depth);
        // Convert JsonValue to serde_json::Value
        Ok(from_str::<Value>(tree.to_json_partial(&changed).to_string().as_ref()).unwrap())
    } else {
        Err(String::from("Cannot modify decision tree."))
    }
}

/// Get all decision attributes of Bdt tree node.
#[tauri::command]
pub async fn get_attributes(node_id: String, session_key: &str) -> Result<Value, ErrorMessage> {
    let locked_tree = get_locked_tree(session_key);
    let read_tree = locked_tree.read().unwrap();
    if let Some(tree) = read_tree.as_ref() {
        let node = BdtNodeId::try_from_str(node_id.clone(), tree);
        let node = if let Some(node) = node {
            node
        } else {
            return Err(format!("Invalid node id {}.", node_id));
        };
        // Convert JsonValue to serde_json::Value
        Ok(from_str::<Value>(tree.attribute_gains_json(node).to_string().as_ref()).unwrap())
    } else {
        Err(String::from("No tree present. Run computation first."))
    }
}

/// Apply precision to Bdt tree.
#[tauri::command]
pub async fn apply_tree_precision(
    precision: String,
    session_key: &str,
) -> Result<String, ErrorMessage> {
    if let Ok(precision) = precision.parse::<u32>() {
        let locked_tree = get_locked_tree(session_key);
        let mut write_tree = locked_tree.write().unwrap();
        if let Some(tree) = write_tree.as_mut() {
            tree.set_precision(precision);
            Ok(String::from("\"ok\""))
        } else {
            Err(String::from("Cannot modify decision tree."))
        }
    } else {
        Err(String::from("Given precision is not a number."))
    }
}

/// Get current precision of Bdt tree.
#[tauri::command]
pub async fn get_tree_precision(session_key: &str) -> Result<u32, ErrorMessage> {
    let locked_tree = get_locked_tree(session_key);
    let read_tree = locked_tree.read().unwrap();
    if let Some(tree) = read_tree.as_ref() {
        Ok(tree.get_precision())
    } else {
        Err(String::from("Cannot modify decision tree."))
    }
}

/// Apply decision attribute to node of Bdt tree.
#[tauri::command]
pub async fn apply_attribute(
    node_id: usize,
    attribute_id: usize,
    session_key: &str,
) -> Result<Value, ErrorMessage> {
    let locked_tree = get_locked_tree(session_key);
    let mut write_tree = locked_tree.write().unwrap();
    return if let Some(tree) = write_tree.as_mut() {
        let node = BdtNodeId::try_from(node_id, tree);
        let node = if let Some(node) = node {
            node
        } else {
            return Err(format!("Invalid node id {}.", node_id));
        };
        let attribute = AttributeId::try_from(attribute_id, tree);
        let attribute = if let Some(val) = attribute {
            val
        } else {
            return Err(format!("Invalid attribute id {}.", attribute_id));
        };
        if let Ok((left, right)) = tree.make_decision(node, attribute) {
            let changes = array![
                tree.node_to_json(node),
                tree.node_to_json(left),
                tree.node_to_json(right),
            ];
            // Convert JsonValue to serde_json::Value
            Ok(from_str::<Value>(changes.to_string().as_ref()).unwrap())
        } else {
            Err(String::from("Invalid node or attribute id."))
        }
    } else {
        Err(String::from("No tree present. Run computation first."))
    };
}

/// Revert decision that was applied to Bdt tree node.
#[tauri::command]
pub async fn revert_decision(node_id: String, session_key: &str) -> Result<Value, ErrorMessage> {
    let locked_tree = get_locked_tree(session_key);
    let mut write_tree = locked_tree.write().unwrap();
    return if let Some(tree) = write_tree.as_mut() {
        let node = BdtNodeId::try_from_str(node_id.clone(), tree);
        let node = if let Some(node) = node {
            node
        } else {
            return Err(format!("Invalid node id {}.", node_id));
        };
        let removed = tree.revert_decision(node);
        let removed = removed
            .into_iter()
            .map(|v| v.to_index())
            .collect::<Vec<_>>();
        let response = object! {
                "node": tree.node_to_json(node),
                "removed": JsonValue::from(removed)
        };

        // Convert JsonValue to serde_json::Value
        Ok(from_str::<Value>(response.to_string().as_ref()).unwrap())
    } else {
        Err(String::from("No tree present. Run computation first."))
    };
}

/// Get stability data of Bdt tree node.
#[tauri::command]
pub async fn get_stability_data(
    node_id: String,
    behaviour_str: String,
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
    // First, extract all colors in that tree node.
    let node_params = {
        let locked_tree = get_locked_tree(session_key);
        let read_tree = locked_tree.read().unwrap();
        if let Some(tree) = read_tree.as_ref() {
            let node = BdtNodeId::try_from_str(node_id.clone(), tree);
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
            // Now compute which attractors are actually relevant for the node colors
            let components = components
                .into_iter()
                .filter_map(|attractor| {
                    attractor
                        .intersect_colors(&node_params)
                        .take_if(|it| !it.is_empty())
                })
                .collect::<Vec<_>>();

            if components.is_empty() {
                return Err(String::from("No attractors with this property."));
            }

            let stability_data = compute_stability(graph, &components);
            let mut response = JsonValue::new_array();
            for variable in graph.as_network().variables() {
                response
                    .push(object! {
                        "variable": graph.as_network().get_variable_name(variable).clone(),
                        "data": stability_data[&variable].to_json(),
                    })
                    .unwrap();
            }
            // Convert JsonValue to serde_json::Value
            Ok(from_str::<Value>(response.to_string().as_ref()).unwrap())
        } else {
            Err(String::from("No attractor data found."))
        }
    } else {
        Err(String::from("No attractor data found."))
    }
}
