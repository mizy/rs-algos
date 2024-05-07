use wasm_bindgen::prelude::*;

use crate::graph::{
    utils::octree::{Octree, OctreeNode},
    Graph, Vertex,
};

#[wasm_bindgen]
pub struct NForce {
    pub alpha: f64,
    pub link_distance: f64,
    pub link_strength: f64,
    pub theta: f64,
    octree: Octree,
    center_strength: f64,
    pub strength: f64,
    size: f64,
    bias: Vec<f64>,
}
#[wasm_bindgen]
impl NForce {
    #[wasm_bindgen(constructor)]
    pub fn new() -> NForce {
        NForce {
            alpha: 1.0,
            link_distance: 100.,
            link_strength: 0.1,
            theta: 1.,
            center_strength: 0.01,
            strength: 40.,
            octree: Octree::new(),
            size: 0.,
            bias: Vec::new(),
        }
    }

    /// will calc graph degree and then calc link bias
    pub fn calc_link_bias(&mut self, graph: &mut Graph) -> Vec<f64> {
        graph.calc_degree();
        let vertexes = &graph.vertexes;
        self.bias.resize(graph.edges.len(), 0.);
        for (i, link) in graph.edges.iter().enumerate() {
            let source = &vertexes[link.source];
            let target = &vertexes[link.target];
            self.bias[i] = source.degree as f64 / (source.degree as f64 + target.degree as f64);
        }
        self.bias.clone()
    }

    pub fn update(&mut self, graph: &mut Graph, alpha: f64) {
        self.alpha = alpha;
        self.calc_max_size(graph);
        self.octree.build(graph);
        self.apply_force(graph);
        self.apply_link_force(graph);
        self.set_positions(graph);
    }

    fn apply_force(&self, graph: &mut Graph) {
        let mut queue: [*const OctreeNode; 512] = [&self.octree.root as *const OctreeNode; 512];
        let vertexes = &mut graph.vertexes;
        for vertex in vertexes {
            let mut start_index = 0;
            let mut end_index = 1;
            while end_index > start_index {
                let octree_node = unsafe { &*queue[start_index] };
                start_index += 1;
                if octree_node.node == vertex.index {
                    continue;
                }
                let cube_vec3 = [
                    vertex.x - octree_node.cube[0],
                    vertex.y - octree_node.cube[1],
                    vertex.z - octree_node.cube[2],
                ];
                let distance = cube_vec3[0] * cube_vec3[0]
                    + cube_vec3[1] * cube_vec3[1]
                    + cube_vec3[2] * cube_vec3[2];
                if distance < (octree_node.size * octree_node.size * self.theta) {
                    if octree_node.number == 1 {
                        self.add_octree_force(vertex, octree_node);
                    } else if octree_node.number > 1 {
                        for child in &octree_node.children {
                            queue[end_index % 512] = child as *const OctreeNode;
                            end_index += 1;
                        }
                    }
                } else {
                    self.add_octree_force(vertex, octree_node);
                }
            }
            self.apply_center_force(vertex);
        }
    }

    fn add_octree_force(&self, vertex: &mut Vertex, octree_node: &OctreeNode) {
        let vec3 = [
            octree_node.x - vertex.x,
            octree_node.y - vertex.y,
            octree_node.z - vertex.z,
        ];
        let l = vec3[0] * vec3[0] + vec3[1] * vec3[1] + vec3[2] * vec3[2];
        let scale = -(octree_node.number as f64 * self.alpha * self.strength) / l;
        vertex.vx += vec3[0] * scale;
        vertex.vy += vec3[1] * scale;
        vertex.vz += vec3[2] * scale;
    }

    fn apply_center_force(&self, vertex: &mut Vertex) {
        let tree = &self.octree.root;
        let degree = vertex.degree;
        let center_scale = self.center_strength * self.alpha * (degree + 1) as f64;
        vertex.x -= tree.x;
        vertex.y -= tree.y;
        vertex.z -= tree.z;
        let (x_dir, y_dir, z_dir) = (
            if vertex.x > 0.0 { -1.0 } else { 1.0 },
            if vertex.y > 0.0 { -1.0 } else { 1.0 },
            if vertex.z > 0.0 { -1.0 } else { 1.0 },
        );
        vertex.vx += center_scale * x_dir;
        vertex.vy += center_scale * y_dir;
        vertex.vz += center_scale * z_dir;
    }

    fn apply_link_force(&self, graph: &mut Graph) {
        let links = &graph.edges;
        for (i, link) in links.iter().enumerate() {
            if link.source == link.target {
                continue;
            }
            let source = &graph.vertexes[link.source];
            let target = &graph.vertexes[link.target];
            let dx = source.x + source.vx - target.x - target.vx;
            let dy = source.y + source.vy - target.y - target.vy;
            let dz = source.z + source.vz - target.z - target.vz;
            let l = f64::sqrt(dx * dx + dy * dy + dz * dz);
            let scale = (l - self.link_distance) / l * self.link_strength * self.alpha;
            let bias = (1.0 - self.bias[i]) * scale;
            {
                let source = &mut graph.vertexes[link.source];
                source.vx -= dx * bias;
                source.vy -= dy * bias;
                source.vz -= dz * bias;
            }
            {
                let bias2 = self.bias[i] * scale;
                let target = &mut graph.vertexes[link.target];
                target.vx += dx * bias2;
                target.vy += dy * bias2;
                target.vz += dz * bias2;
            }
        }
    }

    fn calc_max_size(&mut self, graph: &Graph) {
        let mut max_size = 0.;
        for pos in &graph.vertexs_position {
            let abs = f32::abs(pos.clone());
            if abs > max_size {
                max_size = abs;
            }
        }
        self.octree.max_size = max_size as f64;
        self.size = max_size as f64 * 2.;
    }

    fn set_positions(&self, graph: &mut Graph) {
        let len = graph.get_vertexes_len();
        for i in 0..len {
            // for two mut ref can't exist in one scope
            let (index, x, y, z) = {
                let vertex = &mut graph.vertexes[i];
                vertex.x += vertex.vx;
                vertex.y += vertex.vy;
                vertex.z += vertex.vz;
                vertex.vx = 0.;
                vertex.vy = 0.;
                vertex.vz = 0.;
                (vertex.index, vertex.x, vertex.y, vertex.z)
            };
            graph.set_vertex_position(index, vec![x as f32, y as f32, z as f32])
        }
    }

    pub fn get_bias(&self) -> Vec<f64> {
        self.bias.clone()
    }
}
