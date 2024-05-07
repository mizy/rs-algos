use rs_algos::graph::Graph;

use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub fn get_graph() -> Graph {
    let mut graph = Graph::new();
    graph.add_vertex("a", 1., 0., 0.);
    graph.add_vertex("b", 0., 0., 0.);
    graph.add_edge("a", "b", -1.0);
    graph
}

#[cfg(test)]
#[wasm_bindgen_test]
fn test_apsp() {
    let mut graph = get_graph();
    graph.set_directed(false);
    let result = graph.run_algo("apsp", "");
    println!("----------apsp:{:?}", result);
    // a->b = 1
}
