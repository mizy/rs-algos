// use wasm_bindgen_test::wasm_bindgen_test_configure;

// wasm_bindgen_test_configure!(run_in_browser);
use rs_algos::graph::layouts::radial_force::RadialForce;
use rs_algos::graph::Graph;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_radial_layout() {
    let mut graph = Graph::new();
    graph.add_vertex("1", 0.0, 0.0, 0.);
    graph.add_vertex("2", 1.0, 0.0, 0.);
    graph.add_vertex("3", 2.0, 0.0, 0.);
    graph.add_vertex("4", 3.0, 0.0, 0.);
    graph.add_edge("1", "3", 1.);
    graph.add_edge("1", "4", 1.);
    let mut radial_force = RadialForce::new(&mut graph, "1", 100.);
    let mut t = 0.1;
    loop {
        radial_force.radial_layout(&mut graph, t);
        let position_ptr = graph.get_nodes_position_ptr();
        let position_slice = unsafe { std::slice::from_raw_parts(position_ptr, 4 * 3) };
        // check if position has nan or infinite
        for i in 0..4 {
            assert!(!position_slice[i].is_nan());
            assert!(!position_slice[i].is_infinite());
        }
        if t > 1. {
            break;
        }
        t += 0.1;
    }
}
#[wasm_bindgen_test]
fn test_radial_layout_no_line() {
    let mut graph = Graph::new();
    graph.add_vertex("1", 0.0, 0.0, 0.);
    graph.add_vertex("2", 1.0, 0.0, 0.);
    let mut radial_force = RadialForce::new(&mut graph, "1", 100.);
    let mut t = 0.1;
    loop {
        radial_force.radial_layout(&mut graph, t);
        let position_ptr = graph.get_nodes_position_ptr();
        let position_slice = unsafe { std::slice::from_raw_parts(position_ptr, 2 * 3) };
        for i in 0..2 {
            assert!(!position_slice[i].is_nan());
            assert!(!position_slice[i].is_infinite());
        }
        if t > 1. {
            break;
        }
        t += 0.1;
    }
}
