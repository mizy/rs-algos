use crate::graph::Graph;
pub struct QuadTreeNode {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub depth: usize,
    pub max_depth: usize,
    pub max_objects: usize,
    pub objects: Vec<usize>,
    pub nodes: Vec<QuadTreeNode>,
}
pub fn newQuadTreeFromGraph(graph: &Graph) -> QuadTreeNode {
    let mut quadtree = QuadTreeNode {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
        depth: 0,
        max_depth: 4,
        max_objects: 4,
        objects: vec![],
        nodes: vec![],
    };
    quadtree
}

pub fn getMaxSize(graph: &Graph) -> (f32) {
    let mut max_size = 0.;
    for vertex in &graph.vertexes {
        if f32::abs(vertex.x) > max_size {
            max_size = f32::abs(vertex.x);
        }
        if f32::abs(vertex.y) > max_size {
            max_size = f32::abs(vertex.y);
        }
        if f32::abs(vertex.z) > max_size {
            max_size = f32::abs(vertex.z);
        }
    }
    (max_size)
}
