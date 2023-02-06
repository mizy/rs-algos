use rs_algos::graph::{
    algos::{apsp::apsp, cnc::cnc, radial_force::RadialForce},
    Graph,
};

fn make_test_graph() -> Graph {
    let mut graph = Graph::new();
    let vertexs = ['a', 'b', 'c'];
    for vertex in vertexs.iter() {
        graph.add_vertex(vertex.to_string(), 0., 0., 0.);
    }

    let edges = [('a', 'b'), ('a', 'c')];
    for edge in edges.iter() {
        graph.add_edge(edge.0.to_string(), edge.1.to_string(), 1.);
    }
    graph
}
#[test]
fn test_graph_init() {
    println!("test init graph... ");
    let mut graph = make_test_graph();
    graph.calc_degree();
    graph.set_directed(true);
}
#[test]
fn test_apsp() {
    println!("test apsp... ");
    let mut graph = make_test_graph();
    let ptr = apsp(&mut graph);
    // a->c = 2
    let a_c = unsafe { *ptr.add(2) };
    assert_eq!(a_c, 1.)
}

#[test]
fn test_cnc() {
    println!("test cnc... ");
    let mut graph = make_test_graph();
    let ptr = cnc(&mut graph);
    // c =0
    let c = unsafe { *ptr.add(2) };
    assert_eq!(c, 0.)
}

#[test]
fn test_radial_force() {
    println!("test radial_force... ");
    let mut graph = make_test_graph();
    graph.set_directed(true);
    let mut radial_force = RadialForce::new(&mut graph, "a".to_string(), 200.);
    for i in 0..1 {
        radial_force.radial_layout(&mut graph, (100 - i) as f32 / 150.);
    }
    graph.vertexs_position.iter().for_each(|x| {
        println!("{:?} ", x);
        if x.is_nan() {
            panic!("radial_force calculate result has nan value");
        }
    });
}
