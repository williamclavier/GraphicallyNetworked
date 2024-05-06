use project::graph::*;
use project::algorithms::bfs::BFS;

use std::collections::HashMap;
#[test]
fn test_bfs() {
    let mut vec = Vec::new();
    vec.push((0, 1));
    vec.push((1, 2));
    let graph = Graph::create(&vec);
    let bfs = BFS::new(graph);
    let solution = bfs.dist_from_vertex(0);
    let answer = HashMap::from([
        (1, 1),
        (0, 0),
        (2, 2)
    ]);
    let mut flag = true;
    for (key, value) in &solution {
        if let Some(v) = answer.get(&key) {
            if v != value {
                flag = false;
            }
        } else {
            flag = false;
        }
    }
    assert_eq!(flag, true);
}

#[test]
fn test_avg_dist() {
    let mut vec = Vec::new();
    vec.push((0, 1));
    vec.push((1, 2));
    let graph = Graph::create(&vec);
    let bfs = BFS::new(graph);
    let res = (bfs.average_distance(10_000) * 10.0).round() / 10.0;
    assert_eq!(true, approx::abs_diff_eq!(2.0, res));
}