use crate::graph::{Graph, Vertex};
use std::collections::VecDeque;
use std::collections::HashMap;

pub struct BFS {
    graph: Graph,
}

impl BFS {
    pub fn new(graph: Graph) -> BFS {
        BFS{graph}
    }
    pub fn dist_from_vertex(&self, start: Vertex) -> HashMap<usize, usize> {
        let mut visited: HashMap<Vertex, bool> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();

        queue.push_back(start);
        visited.insert(start, true);
        distances.insert(start, 0);

        while let Some(vert) = queue.pop_front() {
            if let Some(neighbors) = self.graph.outedges.get(&vert) {
                for &neighbor in neighbors {
                    if visited.contains_key(&neighbor) == false {
                        queue.push_back(neighbor);
                        visited.insert(neighbor, true);
                        let distance = distances.get(&vert).unwrap() + 1;
                        distances.insert(neighbor, distance);
                    }
                }
            }
        }
        return distances;
    }
}