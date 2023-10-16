use std::collections::HashSet;
use std::time::Duration;
use biodivine_lib_param_bn::biodivine_std::bitvector::{ArrayBitVector, BitVector};
use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::BooleanNetwork;
use biodivine_lib_param_bn::symbolic_async_graph::{GraphColors, SymbolicAsyncGraph};
use json::object;
use regex::Regex;
use serde_json::Value;
use biodivine_aeon_desktop::scc::{Behaviour, Class, Classifier};
use crate::common::ErrorMessage;
use crate::computation_commands::get_locked_computation;
use crate::model_commands::read_layout;

type EdgeList = Vec<(ArrayBitVector, ArrayBitVector)>;

pub fn read_metadata(aeon_string: &str) -> (Option<String>, Option<String>) {
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

pub fn get_witness_network(colors: &GraphColors, window_session_key: &str) -> Result<String, ErrorMessage> {
    let locked_computation = get_locked_computation(window_session_key);
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
        } else {
            Err(String::from("No results available."))
        }
    } else {
        Err(String::from("No results available."))
    }
}

pub fn try_get_class_params(classifier: &Classifier, class: &Class) -> Option<Option<GraphColors>> {
    for _ in 0..5 {
        if let Some(data) = classifier.try_get_params(class) {
            return Some(data);
        }
        // wait a little - maybe the lock will become free
        std::thread::sleep(Duration::new(1, 0));
    }
    None
}

pub fn get_witness_attractors(f_colors: &GraphColors, window_session_key: &str) -> Result<Value, ErrorMessage> {
    {
        // Variables prefixed with f_ are from the original fully parametrised graph.
        let locked_computation= get_locked_computation(window_session_key);
        let read_computation = locked_computation.read().unwrap();
        if let Some(computation) = read_computation.as_ref() {
            if let Some(f_classifier) = &computation.classifier {
                if let Some(graph) = &computation.graph {
                    let f_witness_colour = f_colors.pick_singleton();
                    let witness_network: BooleanNetwork = graph.pick_witness(&f_witness_colour);
                    let witness_graph = SymbolicAsyncGraph::new(witness_network.clone()).unwrap();
                    let witness_str = witness_network.to_string();
                    let f_witness_attractors = f_classifier.attractors(&f_witness_colour);
                    let variable_name_strings = witness_network
                        .variables()
                        .map(|id| format!("\"{}\"", witness_network.get_variable_name(id)));

                    let mut all_attractors: Vec<(Behaviour, EdgeList, HashSet<usize>)> = Vec::new();

                    // Note that the choice of graph/witness_graph is not arbitrary.
                    // The attractor set is from the original graph, but source_set/target_set
                    // are based on the witness_graph. This means they have different number
                    // of BDD variables inside!
                    let mut has_large_attractors = false;
                    for (f_attractor, behaviour) in f_witness_attractors.iter() {
                        println!(
                            "Attractor {:?} state count: {}",
                            behaviour,
                            f_attractor.approx_cardinality()
                        );
                        let mut attractor_graph: Vec<(ArrayBitVector, ArrayBitVector)> = Vec::new();
                        let mut not_fixed_vars: HashSet<usize> = HashSet::new();
                        if *behaviour == Behaviour::Stability {
                            // This is a sink - no edges
                            assert_eq!(f_attractor.materialize().iter().count(), 1);
                            let sink: ArrayBitVector =
                                f_attractor.materialize().iter().next().unwrap();
                            attractor_graph.push((sink.clone(), sink));
                            for i in 0..witness_network.num_vars() {
                                // In sink, we mark everything as "not-fixed" because we want to just display it normally.
                                not_fixed_vars.insert(i);
                            }
                        } else if f_attractor.approx_cardinality() >= 500.0 {
                            has_large_attractors = true;
                            // For large attractors, only show fixed values.
                            let mut state_0 =
                                ArrayBitVector::from(vec![false; graph.as_network().num_vars()]);
                            let mut state_1 =
                                ArrayBitVector::from(vec![true; graph.as_network().num_vars()]);
                            for var in graph.as_network().variables() {
                                let f_var_true = graph.fix_network_variable(var, true).vertices();
                                let f_var_false = graph.fix_network_variable(var, false).vertices();
                                let f_always_one = f_attractor.intersect(&f_var_false).is_empty();
                                let f_always_zero = f_attractor.intersect(&f_var_true).is_empty();
                                if f_always_one {
                                    state_0.set(var.into(), true);
                                } else if f_always_zero {
                                    state_1.set(var.into(), false);
                                } else {
                                    not_fixed_vars.insert(var.into());
                                }
                            }
                            attractor_graph.push((state_0.clone(), state_1.clone()));
                            attractor_graph.push((state_1, state_0));
                        } else {
                            for source in f_attractor.materialize().iter() {
                                let source_set = witness_graph.vertex(&source);
                                let mut target_set = witness_graph.mk_empty_vertices();
                                for v in witness_graph.as_network().variables() {
                                    let post = witness_graph.var_post(v, &source_set);
                                    if !post.is_empty() {
                                        not_fixed_vars.insert(v.into());
                                        target_set = target_set.union(&post);
                                    }
                                }

                                for target in target_set.vertices().materialize().iter() {
                                    attractor_graph.push((source.clone(), target));
                                }
                            }
                        }

                        all_attractors.push((*behaviour, attractor_graph, not_fixed_vars));
                    }

                    // now the data is stored in `all_attractors`, just convert it to json:
                    let mut json = String::new();

                    for (i, (behavior, graph, not_fixed)) in all_attractors.iter().enumerate() {
                        if i != 0 {
                            json += ",";
                        } // json? no trailing commas for you
                        json += &format!("{{\"class\":\"{:?}\", \"graph\":[", behavior);
                        let mut edge_count = 0;
                        for (j, edge) in graph.iter().enumerate() {
                            fn state_to_binary(
                                state: &ArrayBitVector,
                                not_fixed: &HashSet<usize>,
                            ) -> String {
                                let mut result = String::new();
                                for i in 0..state.len() {
                                    if not_fixed.contains(&i) {
                                        result.push(if state.get(i) { '1' } else { '0' });
                                    } else {
                                        result.push(if state.get(i) { '⊤' } else { '⊥' });
                                    }
                                }
                                result
                            }
                            let from: String = state_to_binary(&edge.0, not_fixed);
                            let to: String = state_to_binary(&edge.1, not_fixed);
                            if j != 0 {
                                json += ","
                            }
                            json += &format!("[\"{}\", \"{}\"]", from, to);
                            edge_count += 1;
                        }
                        json += &format!("], \"edges\":{}}}", edge_count);
                    }
                    json = "{ \"attractors\":[".to_owned() + &json + "], \"variables\":[";
                    for (i, var) in variable_name_strings.enumerate() {
                        if i != 0 {
                            json += ",";
                        }
                        json += var.as_str();
                    }
                    json += &format!(
                        "], \"model\":{}, \"has_large_attractors\": {}",
                        &object! { "model" => witness_str }.to_string(),
                        has_large_attractors
                    );
                    json += "}";
                    Ok(serde_json::from_str(json.as_str()).unwrap())
                } else {
                    Err(String::from("No results available."))
                }
            } else {
                Err(String::from("No results available."))
            }
        } else {
            Err(String::from("No results available."))
        }
    }
}

