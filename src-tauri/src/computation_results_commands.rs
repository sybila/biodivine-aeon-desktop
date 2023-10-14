use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use biodivine_lib_param_bn::biodivine_std::bitvector::{ArrayBitVector, BitVector};
use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::BooleanNetwork;
use biodivine_lib_param_bn::symbolic_async_graph::{GraphColors, SymbolicAsyncGraph};
use json::object;
use regex::Regex;
use biodivine_aeon_desktop::scc::{Behaviour, Class, Classifier};
use crate::common::ErrorMessage;
use crate::computation::Computation;
use crate::computation_commands::get_locked_computation;
use crate::model_commands::read_layout;





fn read_metadata(aeon_string: &str) -> (Option<String>, Option<String>) {
    let mut model_name = None;
    let mut model_description = None;
    let name_regex = Regex::new(r"^\s*#name:(?P<name>.+)$").unwrap();
    let description_regex = Regex::new(r"^\s*#description:(?P<desc>.+)$").unwrap();
    for line in aeon_string.lines() {
        if let Some(captures) = name_regex.captures(line) {
            model_name = Some(captures["name"].to_string());
        }
        if let Some(captures) = description_regex.captures(line) {
            model_description = Some(captures["desc"].to_string());
        }
    }
    (model_name, model_description)
}


fn get_witness_network(colors: &GraphColors, window_session_key: &str) -> Result<String, ErrorMessage> {
    let locked_computation: Arc<RwLock<Option<Computation>>> = get_locked_computation(window_session_key);;
    let read_computation = locked_computation.read().unwrap();
    if let Some(computation) = read_computation.as_ref() {
        if let Some(graph) = &computation.graph {
            let witness = graph.pick_witness(colors);
            let layout = read_layout(computation.input_model.as_str());
            let mut model_string = format!("{}", witness); // convert back to aeon
            model_string += "\n";
            for (var, (x, y)) in layout {
                model_string += format!("#position:{}:{},{}\n", var, x, y).as_str();
            }
            let (name, description) = read_metadata(computation.input_model.as_str());
            if let Some(name) = name {
                model_string += format!("#name:{}\n", name).as_str();
            }
            if let Some(description) = description {
                model_string += format!("#description:{}\n", description).as_str();
            }
            Ok(model_string)
            //BackendResponse::ok(&object! { "model" => model_string }.to_string())
        } else {
            Err(String::from("No results available."))
            //BackendResponse::err("No results available.")
        }
    } else {
        Err(String::from("No results available."))
        //BackendResponse::err("No results available.")
    }
}


fn try_get_class_params(classifier: &Classifier, class: &Class) -> Option<Option<GraphColors>> {
    for _ in 0..5 {
        if let Some(data) = classifier.try_get_params(class) {
            return Some(data);
        }
        // wait a little - maybe the lock will become free
        std::thread::sleep(Duration::new(1, 0));
    }
    None
}


#[tauri::command]
pub fn get_witness(class_str: String, window_session_key: &str) -> Result<String, ErrorMessage> {
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
        let locked_computation: Arc<RwLock<Option<Computation>>> = get_locked_computation(window_session_key);;
        let read_computation = locked_computation.read().unwrap();
        if let Some(computation) = read_computation.as_ref() {
            if let Some(classifier) = &computation.classifier {
                if let Some(has_class) = try_get_class_params(classifier, &class) {
                    if let Some(class) = has_class {
                        get_witness_network(&class, window_session_key)
                    } else {
                        Err(String::from("Specified class has no witness."))
                    }
                } else {
                    Err(String::from("Classification in progress. Cannot extract witness right now."))
                }
            } else {
                Err(String::from("No results available."))
            }
        } else {
            Err(String::from("No results available."))
        }
    }
}


