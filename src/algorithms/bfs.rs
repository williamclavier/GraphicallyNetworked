use crate::graph::{Graph, Vertex};
use std::collections::VecDeque;
use std::collections::HashMap;
use rand::prelude::SliceRandom;
use rayon::prelude::*;
use rand::thread_rng;

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

    pub fn average_distance(&self, sample_size: usize) -> f64 {
        let verts: Vec<_> = self.graph.outedges.keys().copied().collect();
        let total_pairs = sample_size * (self.graph.outedges.len() - 1);
        let avg_distance: f64 = (0..sample_size)
            .into_par_iter()
            .map(|_| {
                let mut rng = thread_rng();
                let &start = verts.choose(&mut rng).unwrap();
                let dists = self.dist_from_vertex(start);
                let total: usize = dists.values().sum();
                total as f64 / total_pairs as f64
            })
            .collect::<Vec<f64>>()
            .iter()
            .sum::<f64>();

        return avg_distance;
    }
}