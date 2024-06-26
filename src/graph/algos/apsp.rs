use crate::graph::Graph;

/// every algo need return the result's ownership to js, so we need to return a Vec<f32>
pub fn run(graph: &mut Graph) -> Vec<f32> {
    let dist = make_floyd_warshall(graph);
    let mut arr = vec![0.; dist.len() * dist.len()];
    let mut i = 0;
    for row in dist {
        for col in row {
            arr[i] = col;
            i += 1;
        }
    }
    arr
}

pub fn make_floyd_warshall(graph: &mut Graph) -> Vec<Vec<f32>> {
    let len = graph.vertexes.len();
    let mut dist = vec![vec![f32::INFINITY; len]; len];
    for i in 0..len {
        dist[i] = vec![f32::INFINITY; len];
        let neighbors = graph.neighbors_map.get_mut(i).unwrap();
        for j in 0..len {
            if i == j {
                dist[i][j] = 0.;
            } else if neighbors.contains_key(&j) {
                dist[i][j] = neighbors[&j];
            } else {
                dist[i][j] = f32::INFINITY;
            }
        }
    }
    let len = graph.vertexes.len();
    for k in 0..len {
        for i in 0..len {
            for j in 0..len {
                if i == j {
                    // make sure the self path is 0, because some weight is negative value
                    continue;
                }
                if dist[i][k] + dist[k][j] < dist[i][j] {
                    let new_val = dist[i][k] + dist[k][j];
                    dist[i][j] = new_val;
                }
            }
        }
    }
    dist
}
