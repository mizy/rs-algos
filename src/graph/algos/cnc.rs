use crate::graph::{algos::apsp::make_floyd_warshall, Graph};

pub fn run(graph: &mut Graph) -> Vec<f32> {
    let dist = make_floyd_warshall(graph);
    make_cnc(&dist)
}

pub fn make_cnc(d: &Vec<Vec<f32>>) -> Vec<f32> {
    let mut cnc = vec![0.0; d.len()];
    for i in 0..d.len() {
        let mut sum = 0.;
        let mut num = 0.;
        for j in 0..d.len() {
            if d[i][j] != f32::INFINITY && i != j {
                sum += d[i][j];
                num += 1.;
            }
        }
        if sum != 0. {
            cnc[i] = num / sum;
        } else {
            cnc[i] = 0.;
        }
    }
    cnc
}
