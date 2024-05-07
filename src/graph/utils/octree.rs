use std::fmt::Debug;

use crate::graph::{Graph, Vertex};

pub struct Octree {
    node_pool: Vec<OctreeNode>,
    node_pool_index: usize,
    pub max_size: f64,
    pub root: OctreeNode,
}

pub struct OctreeNode {
    pub cube: [f64; 3],
    pub children: Vec<OctreeNode>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub number: usize,
    pub size: f64,
    pub node: usize,
}

impl Octree {
    pub fn new() -> Octree {
        Octree {
            node_pool: Vec::new(),
            node_pool_index: 0,
            max_size: 0.,
            root: OctreeNode {
                cube: [0., 0., 0.],
                children: Vec::new(),
                x: 0.,
                y: 0.,
                z: 0.,
                number: 0,
                size: 0.,
                node: 0,
            },
        }
    }

    pub fn build(&mut self, graph: &Graph) {
        let mut root = self.new_node(self.max_size, [0., 0., 0.]);
        root.size = self.max_size;
        for vertex in &graph.vertexes {
            self.set_into_tree(graph, vertex, &mut root);
        }
        self.calc_gravity_center(&mut root);
        self.root = root;
    }

    fn set_into_tree(&mut self, graph: &Graph, vertex: &Vertex, tree: &mut OctreeNode) {
        if tree.number == 0 {
            // empty octree
            tree.node = vertex.index;
        } else if tree.number == 1 {
            // make tree children
            let next_size = tree.size / 2.0;
            // octree cube size
            let tmp = [
                tree.cube[0] + next_size,
                tree.cube[0] - next_size,
                tree.cube[1] + next_size,
                tree.cube[1] - next_size,
                tree.cube[2] + next_size,
                tree.cube[2] - next_size,
            ];
            tree.children = vec![
                self.new_node(next_size, [tmp[0], tmp[2], tmp[4]]),
                self.new_node(next_size, [tmp[0], tmp[2], tmp[5]]),
                self.new_node(next_size, [tmp[0], tmp[3], tmp[4]]),
                self.new_node(next_size, [tmp[0], tmp[3], tmp[5]]),
                self.new_node(next_size, [tmp[1], tmp[2], tmp[4]]),
                self.new_node(next_size, [tmp[1], tmp[2], tmp[5]]),
                self.new_node(next_size, [tmp[1], tmp[3], tmp[4]]),
                self.new_node(next_size, [tmp[1], tmp[3], tmp[5]]),
            ];
            if next_size < 0.00001 {
                // size too small
                tree.number += 1;
                return;
            }
            let node = &graph.vertexes[tree.node];
            // set old node to children
            let old_index = self.check_node_position(&node.x, &node.y, &node.z, &tree.cube);
            self.set_into_tree(graph, node, &mut tree.children[old_index]);
            tree.node = usize::MAX;
            // set now node to children
            let new_index = self.check_node_position(&vertex.x, &vertex.y, &vertex.z, &tree.cube);
            self.set_into_tree(graph, vertex, &mut tree.children[new_index]);
        } else {
            let index = self.check_node_position(&vertex.x, &vertex.y, &vertex.z, &tree.cube);
            self.set_into_tree(graph, vertex, &mut tree.children[index]);
        }

        tree.number += 1;
        tree.x += vertex.x as f64;
        tree.y += vertex.y as f64;
        tree.z += vertex.z as f64;
    }

    fn calc_gravity_center(&mut self, now: &mut OctreeNode) {
        if now.number > 0 {
            now.x /= now.number as f64;
            now.y /= now.number as f64;
            now.z /= now.number as f64;
        }
        let mut i = 0;
        let mut len = now.children.len();
        while i < len {
            if now.children[i].number > 0 {
                self.calc_gravity_center(&mut now.children[i]);
                i += 1;
            } else {
                let item = now.children.pop().unwrap();
                len -= 1;

                if i >= len {
                    break;
                }
                let _ = std::mem::replace(&mut now.children[i], item);
            }
        }
    }

    fn check_node_position(&self, x: &f64, y: &f64, z: &f64, cube: &[f64; 3]) -> usize {
        if *x > cube[0] {
            // 8
            if *y > cube[1] {
                // 4
                if *z > cube[2] {
                    // 2
                    return 0;
                } else {
                    return 1;
                }
            } else {
                if *z > cube[2] {
                    return 2;
                } else {
                    return 3;
                }
            }
        } else {
            if *y > cube[1] {
                // 2
                if *z > cube[2] {
                    return 4;
                } else {
                    return 5;
                }
            } else {
                if *z > cube[2] {
                    return 6;
                } else {
                    return 7;
                }
            }
        }
    }
    fn new_node(&mut self, size: f64, cube: [f64; 3]) -> OctreeNode {
        OctreeNode {
            cube,
            children: Vec::new(),
            x: 0.,
            y: 0.,
            z: 0.,
            number: 0,
            size: size,
            node: 0,
        }
    }
}

impl Debug for OctreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OctreeNode")
            .field("cube", &self.cube)
            .field("children", &self.children)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("number", &self.number)
            .field("size", &self.size)
            .field("node", &self.node)
            .finish()
    }
}
