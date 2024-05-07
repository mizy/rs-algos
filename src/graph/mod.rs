pub mod algos;
pub mod layouts;

#[cfg(test)]
mod tests;
pub mod utils;
use std::{collections::HashMap, fmt::Display};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;

pub struct Vertex {
    pub id: String,
    pub index: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    pub degree: usize,
}
impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(id: {}, index: {}, x: {}, y: {}, z: {}, vx: {}, vy: {}, vz: {}, degree: {})",
            self.id, self.index, self.x, self.y, self.z, self.vx, self.vy, self.vz, self.degree
        )
    }
}
pub struct Edge {
    source: usize,
    target: usize,
    pub weight: f32,
}
impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(source: {}, target: {}, weight: {})",
            self.source, self.target, self.weight
        )
    }
}

#[wasm_bindgen]
pub struct Graph {
    vertexes: Vec<Vertex>,
    edges: Vec<Edge>,
    vertexs_map: HashMap<String, usize>, // convert vertex id to index
    neighbors_map: Vec<HashMap<usize, f32>>,
    is_directed: bool,
    vertexs_position: Vec<f32>,
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Graph {
        Graph {
            is_directed: false,
            vertexes: vec![],
            edges: Vec::new(),
            neighbors_map: vec![],
            vertexs_map: HashMap::new(),
            vertexs_position: vec![],
        }
    }

    pub fn set_directed(&mut self, is_directed: bool) {
        self.is_directed = is_directed;
    }

    pub fn add_vertex(&mut self, id: &str, x: f32, y: f32, mut z: f32) {
        if z.is_nan() || z.is_infinite() {
            z = 0.;
        }
        let key = id.to_string();
        let index = self.vertexes.len();
        let vertex = Vertex {
            id: id.to_string(),
            index,
            x: x as f64,
            y: y as f64,
            z: z as f64,
            vx: 0.,
            vy: 0.,
            vz: 0.,
            degree: 0,
        };
        self.vertexes.push(vertex);
        self.vertexs_map.insert(key, index);
        self.neighbors_map.push(HashMap::new());
        self.vertexs_position.push(x);
        self.vertexs_position.push(y);
        self.vertexs_position.push(z);
    }

    pub fn add_edge(&mut self, source: &str, target: &str, mut weight: f32) {
        if weight.is_infinite() || weight.is_nan() || weight == 0.0 {
            weight = 1.;
        }
        // not clone but use reference maybe good
        //undirected graph
        let source_index = self.vertexs_map[source];
        let target_index = self.vertexs_map[target];
        self.neighbors_map[source_index].insert(target_index, weight);
        if !self.is_directed {
            self.neighbors_map[target_index].insert(source_index, weight);
        }
        let edge = Edge {
            source: source_index,
            target: target_index,
            weight,
        };
        self.edges.push(edge);
    }

    pub fn get_vertexes_len(&self) -> usize {
        self.vertexes.len()
    }

    pub fn get_vertex_id(&self, index: usize) -> String {
        self.vertexes[index].id.clone()
    }

    pub fn get_vertex_by_id(&self, id: &str) -> Option<usize> {
        match self.vertexs_map.get(id) {
            Some(index) => Some(*index),
            None => None,
        }
    }

    pub fn calc_degree(&mut self) {
        for edge in &self.edges {
            self.vertexes[edge.source].degree += 1;
            if edge.source == edge.target {
                continue;
            }
            self.vertexes[edge.target].degree += 1;
        }
    }

    pub fn set_vertex_position(&mut self, index: usize, position: Vec<f32>) {
        self.vertexs_position[(index * 3)] = position[0];
        self.vertexs_position[(index * 3 + 1)] = position[1];
        self.vertexs_position[(index * 3 + 2)] = position[2];
    }

    pub fn get_vertex_position(&self, index: usize) -> Vec<f32> {
        vec![
            self.vertexs_position[(index * 3)],
            self.vertexs_position[(index * 3 + 1)],
            self.vertexs_position[(index * 3 + 2)],
        ]
    }

    pub fn get_neighbors(&self, index: usize) -> Vec<usize> {
        let mut neighbors = vec![];
        for (neighbor, _) in &self.neighbors_map[index] {
            neighbors.push(*neighbor);
        }
        neighbors
    }

    pub fn clear(&mut self) {
        self.vertexes.clear();
        self.edges.clear();
        self.vertexs_map.clear();
        self.neighbors_map.clear();
        self.vertexs_position.clear();
    }

    pub fn get_nodes_position_ptr(&mut self) -> *const f32 {
        self.vertexs_position.as_ptr()
    }

    pub fn get_nodes_position(&mut self) -> Vec<f32> {
        self.vertexs_position.clone()
    }

    /// return the pointer of the position of the nodes
    pub fn run_algo(&mut self, name: &str, config: &str) -> Vec<f32> {
        match name {
            "apsp" => algos::apsp::run(self),
            "cnc" => algos::cnc::run(self),
            "bfs" => algos::bfs::run(self, config),
            _ => vec![0.],
        }
    }
}
