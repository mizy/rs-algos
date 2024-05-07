use crate::graph::Graph;
use std::collections::{HashSet, VecDeque};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(graph: &Graph, root: &str) -> Vec<f32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let root_index = graph.get_vertex_by_id(root);
    match root_index {
        Some(index) => {
            queue.push_back(index);
        }
        None => {
            return vec![];
        }
    }
    while let Some(src) = queue.pop_front() {
        visited.insert(src);
        graph.get_neighbors(src).iter().for_each(|dst| {
            if !visited.contains(dst) {
                queue.push_back(*dst);
            }
        });
    }
    visited.iter().map(|&v| v as f32).collect()
}
