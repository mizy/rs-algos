use std::{f32::NAN, rc::Rc};

use crate::graph::Graph;

pub fn get_graph() -> Graph {
    let mut graph = Graph::new();
    graph.add_vertex("a", 1., 0., 0.1);
    graph.add_vertex("b", 0., 0., 0.2);
    graph.add_vertex("c", 0.1, 0., 0.5);
    graph.add_edge("a", "b", 1.0);
    graph.add_edge("a", "c", 1.0);
    graph
}

#[test]
fn test_graph() {
    let mut graph = get_graph();
    graph.set_directed(false);
    graph.calc_degree();
    let a = graph.get_vertex_position(0);
    assert_eq!(a[0], 1.);
}

#[test]
fn test_apsp() {
    let mut graph = get_graph();
    graph.set_directed(false);
    let result = graph.run_algo("apsp", "");
    println!("----------apsp:{:?}", result);
    // a->b = 1
    assert_eq!(result[1], 1.);
}

#[test]
fn test_cnc() {
    let mut graph = get_graph();
    graph.set_directed(false);
    let result = graph.run_algo("cnc", "");
    println!("----------cnc:{:?}", result);
    // cnc(a) = 1
    assert_eq!(result[0], 1.);
}

#[test]
fn test_nforce_3d() {
    let mut graph = get_graph();
    graph.set_directed(false);
    let mut nforce = crate::graph::layouts::nforce_3d::NForce::new();
    nforce.calc_link_bias(&mut graph);
    let mut t = 0.1;
    loop {
        nforce.update(&mut graph, t);
        // check if has nan
        for i in 0..graph.vertexs_position.len() {
            assert!(!graph.vertexs_position[i].is_nan());
        }
        if t > 0.9 {
            break;
        }
        t += 0.1
    }
}
