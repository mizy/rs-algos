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
