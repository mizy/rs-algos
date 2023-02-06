use wasm_bindgen::prelude::*;

pub mod algos;
pub mod utils;
use std::collections::HashMap;
pub struct Vertex {
    pub id: String,
    pub degree: usize,
    pub index: usize,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct Edge {
    pub source: usize,
    pub target: usize,
    pub weight: f32,
}

pub struct Graph {
    pub vertexes: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub degrees: Vec<usize>,
    pub vertexs_map: HashMap<String, usize>, // convert vertex id to index
    pub neighbors_map: Vec<HashMap<usize, f32>>,
    pub is_directed: bool,
    pub vertexs_position: Vec<f32>,
    pub vertexs_position_3d: Vec<f32>, // 3d
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            is_directed: false,
            vertexes: vec![],
            edges: Vec::new(),
            neighbors_map: vec![],
            degrees: vec![],
            vertexs_map: HashMap::new(),
            vertexs_position: vec![],
            vertexs_position_3d: vec![], // 3d
        }
    }

    pub fn set_directed(&mut self, is_directed: bool) {
        self.is_directed = is_directed;
    }

    pub fn add_vertex(&mut self, id: String, x: f32, y: f32, mut z: f32) {
        if z.is_nan() || z.is_infinite() {
            z = 0.;
        }
        let key = id.clone();
        let index = self.vertexes.len();
        let vertex = Vertex {
            id,
            degree: 0,
            index,
            x,
            y,
            z,
        };
        self.vertexes.push(vertex);
        self.vertexs_map.insert(key, index);
        self.neighbors_map.push(HashMap::new());
        self.vertexs_position.push(x);
        self.vertexs_position.push(y);
        self.vertexs_position_3d.push(x);
        self.vertexs_position_3d.push(y);
        self.vertexs_position_3d.push(z);
    }

    pub fn add_edge(&mut self, source: String, target: String, mut weight: f32) {
        if weight.is_infinite() || weight.is_nan() || weight == 0.0 {
            weight = 1.;
        }
        // not clone but use reference maybe good
        //undirected graph
        let source_index = self.vertexs_map[&source];
        let target_index = self.vertexs_map[&target];
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

    pub fn calc_degree(&mut self) {
        self.degrees = vec![0; self.vertexes.len()];
        for edge in &self.edges {
            self.degrees[edge.source] += 1;
            if edge.source == edge.target {
                continue;
            }
            self.degrees[edge.target] += 1;
        }
    }

    pub fn set_vertex_position(&mut self, index: usize, position: Vec<f32>) {
        self.vertexs_position[(index * 2)] = position[0];
        self.vertexs_position[(index * 2 + 1)] = position[1];
    }

    pub fn get_vertex_position(&self, index: usize) -> Vec<f32> {
        vec![
            self.vertexs_position[(index * 2)],
            self.vertexs_position[(index * 2 + 1)],
        ]
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
}
