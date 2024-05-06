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
        let mut visited: HashMap<Vertex, bool> = HashMap::new(); // Stores vertexes already visited to avoid going backwards
        let mut queue = VecDeque::new(); // Queues previously visited vertexes to begin the traversal from next
        let mut distances = HashMap::new(); // Stores the distances a given vertex and the start node

        // Initialize all with the start vertex
        queue.push_back(start);
        visited.insert(start, true);
        distances.insert(start, 0);

        while let Some(vert) = queue.pop_front() {  // For each vertex in the previously visited vertexes queue that changes throughout
            if let Some(neighbors) = self.graph.outedges.get(&vert) { // For the neighbors that are around the vertex that we are on
                for &neighbor in neighbors {
                    if visited.contains_key(&neighbor) == false {
                        queue.push_back(neighbor);
                        visited.insert(neighbor, true);
                        let distance = distances.get(&vert).unwrap() + 1; // Adds one to the distance and stores it in the hashmap for vertex from the start
                        distances.insert(neighbor, distance);
                    }
                }
            }
        }
        return distances;
    }

    pub fn average_distance(&self, sample_size: usize) -> f64 {
        let verts: Vec<_> = self.graph.outedges.keys().copied().collect(); // Array of the keys in the hashmap
        let total_pairs = sample_size * (self.graph.outedges.len() - 1); // The denominator of the average distance total
        let avg_distances: Vec<f64> = (0..sample_size)
            .into_par_iter() // Performs multithreading for 0 to sample size iterations
            .map(|_| {
                let mut rng = thread_rng(); // Gets the random vertex from the vector
                let &start = verts.choose(&mut rng).unwrap();
                let dists = self.dist_from_vertex(start);   // Gets the distances from the random start vertex
                let total: usize = dists.values().sum(); // Sums the distances into a total
                total as f64 / total_pairs as f64 // Returns the average distance for this iteration
            })
            .collect::<Vec<f64>>(); // Puts the values into the vector of average distances
        let avg_distance = avg_distances.iter().sum::<f64>(); // Gets the sum of the average distances to get the true average distance

        return avg_distance;
    }
}