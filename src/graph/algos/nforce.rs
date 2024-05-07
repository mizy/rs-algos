use crate::graph::{utils::quadtree::QuadTreeNode, Graph};

pub struct NForce {
    pub alpha: f32,
    pub link_distance: f32,
    pub link_strength: f32,
    pub vertex_strength: f32,
}

impl NForce {
    pub fn new(graph: &Graph) -> NForce {
        NForce {
            alpha: 1.0,
            link_distance: 30.0,
            link_strength: 1.0,
            vertex_strength: -30.0,
        }
    }

    pub fn update(&mut self, alpha: f32) {
        self.alpha = alpha;
    }

    pub fn make_quadtree(&mut self) {}

    pub fn apply_force(&mut self) {}

    pub fn apply_vertex_force(&mut self) {}

    pub fn apply_link_force(&mut self) {}
}
