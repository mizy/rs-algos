use crate::{
    graph::{
        algos::{apsp, cnc},
        Graph,
    },
    log,
};
use std::vec;
use wasm_bindgen::prelude::*;

//https://link.springer.com/content/pdf/10.1007/978-3-642-11805-0_12.pdf

#[wasm_bindgen]
pub struct RadialForce {
    d: Vec<Vec<f32>>,
    w: Vec<Vec<f32>>,
    r: Vec<f32>, // radius vector
    pub line_length: f32,
}
#[wasm_bindgen]
impl RadialForce {
    #[wasm_bindgen(constructor)]
    pub fn new(graph: &mut Graph, node_id: &str, mut line_length: f32) -> RadialForce {
        graph.calc_degree();
        if line_length == 0. {
            line_length = 200.;
        }
        let node_index;
        if node_id == "" {
            let sort_nodes = Self::sort_nodes_by_degree(&graph);
            node_index = sort_nodes[0];
        } else {
            if !graph.vertexs_map.contains_key(node_id) {
                log(&format!("node {} not found", node_id));
                panic!("node not found");
            }
            node_index = graph.vertexs_map[node_id];
        }
        // set 0,0 to default position
        graph.set_vertex_position(node_index as usize, vec![0., 0., 0.]);
        // calculate the position of  node's neighbors
        let mut d = apsp::make_floyd_warshall(graph);
        let max_distance = Self::get_max_distance(&d);
        // get cnc value
        let c = cnc::make_cnc(&d);
        Self::handle_infinite_loop(max_distance + 1., &mut d, node_index as usize);
        //make radius
        let r = {
            let mut c_max = 0.;
            let mut c_min = std::f32::MAX;
            for i in 0..c.len() {
                if c[i] > c_max {
                    c_max = c[i];
                }
                if c[i] < c_min {
                    c_min = c[i];
                }
            }
            let c_dis = c_max - c_min;
            let mut r = vec![0.0; d.len()];
            for next_index in 0..d.len() {
                if c_dis == 0. {
                    r[next_index] = (max_distance + 1.) * line_length;
                } else {
                    r[next_index] =
                        (1. - (c[next_index] - c_min) / c_dis) * (max_distance + 1.) * line_length;
                }
            }
            r
        };
        //repair_distance
        Self::repair_distance(&mut d, line_length);
        // make w
        let w = Self::make_weight_matrix(&d);
        RadialForce {
            d,
            w,
            r,
            line_length,
        }
    }

    // i don't want to clone the graph instance or change lifetime , so you need to pass a mutable reference
    pub fn radial_layout(&mut self, graph: &mut Graph, t: f32) {
        let d = &mut self.d;
        let w = &mut self.w;
        let r = &mut self.r;
        let len = d.len();

        for i in 0..len {
            let mut x_top = 0.;
            let mut y_top = 0.;
            let mut bottom = 0.;
            let nodei = graph.get_vertex_position(i);
            for j in 0..len {
                if i == j {
                    continue;
                }
                let w_i_j = w[i][j];
                let d_i_j = d[i][j];
                let nodej = graph.get_vertex_position(j);
                let mut ai = 1. / Self::get_scalar_distance(nodei[0], nodei[1], nodej[0], nodej[1]);
                if ai.is_infinite() || ai.is_nan() {
                    ai = 0.;
                }
                x_top += w_i_j * (nodej[0] + d_i_j * (nodei[0] - nodej[0]) * ai);
                y_top += w_i_j * (nodej[1] + d_i_j * (nodei[1] - nodej[1]) * ai);
                bottom += w_i_j;
            }
            let r_i = {
                if r[i] < 0.00001 {
                    0.
                } else {
                    1. / r[i]
                }
            };
            let r_i_2 = r_i.powi(2);
            bottom *= 1. - t;
            bottom += t * r_i_2;
            let mut ai = 1. / Self::get_scalar_distance(nodei[0], nodei[1], 0., 0.);
            if ai.is_infinite() {
                ai = 0.;
            }
            //x val
            let mut x = ((1. - t) * x_top + t * r_i * nodei[0] * ai) / bottom;
            //y val
            let mut y = ((1. - t) * y_top + t * r_i * nodei[1] * ai) / bottom;
            if x == 0. && y == 0. {
                x = rand::random::<f32>();
                y = rand::random::<f32>();
            }
            graph.set_vertex_position(i, vec![x, y, 0.])
        }
    }

    fn get_scalar_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }

    fn make_weight_matrix(d: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        let mut w = vec![vec![0.0; d.len()]; d.len()];
        let len = d.len();
        for i in 0..len {
            for j in 0..len {
                let val_j = d[i][j];
                let mut val = 0.;
                if val_j > 0.000001 {
                    val = 1. / (val_j * val_j)
                }
                w[i][j] = val;
            }
        }
        w
    }

    fn get_max_distance(d: &Vec<Vec<f32>>) -> f32 {
        let mut max_distance = 0.;
        for i in 0..d.len() {
            for j in 0..d.len() {
                let val = d[i][j];
                if val != f32::INFINITY {
                    if val > max_distance {
                        max_distance = val;
                    }
                }
            }
        }
        max_distance
    }

    fn sort_nodes_by_degree(graph: &Graph) -> Vec<usize> {
        let mut nodes: Vec<usize> = (0..graph.vertexes.len()).collect();
        nodes.sort_by(|a, b| {
            let a_degree = &graph.vertexes[*a].degree;
            let b_degree = &graph.vertexes[*b].degree;
            a_degree.cmp(&b_degree)
        });
        nodes
    }

    fn handle_infinite_loop(max_distance: f32, d: &mut Vec<Vec<f32>>, node_index: usize) {
        let len = d.len();
        for i in 0..len {
            if d[i][node_index] == f32::INFINITY {
                let val = max_distance + 2.; //a litter far for wcc
                d[i][node_index] = val;
                d[node_index][i] = val;
                for j in 0..len {
                    if d[i][j] != f32::INFINITY && d[j][node_index] == f32::INFINITY {
                        let next_val = d[i][j] + val;
                        d[node_index][j] = next_val;
                        d[j][node_index] = next_val;
                    }
                }
            }
        }
        for i in 0..len {
            if i == node_index {
                continue;
            }
            for j in 0..len {
                if d[i][j] == f32::INFINITY {
                    let minus =
                        (d[node_index][i] - max_distance + d[node_index][j] - max_distance).abs();
                    d[i][j] = {
                        if minus == 0. {
                            1.
                        } else {
                            minus
                        }
                    };
                }
            }
        }
    }

    fn repair_distance(d: &mut Vec<Vec<f32>>, line_length: f32) {
        let len = d.len();
        for i in 0..len {
            for j in 0..len {
                let v = d[i][j];
                if i == j {
                    d[i][j] = 0.;
                } else {
                    d[i][j] = v * line_length;
                }
            }
        }
    }
    pub fn default() -> Self {
        Self {
            d: vec![],
            w: vec![],
            r: vec![],
            line_length: 200.,
        }
    }
}
