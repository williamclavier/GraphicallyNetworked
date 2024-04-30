use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex,Vertex)>;
pub type AdjacencyMap = HashMap<Vertex, Vec<Vertex>>;

#[derive(Debug)]
pub struct Graph {
    pub n: usize, // vertex labels in {0,...,n-1}
    pub outedges: AdjacencyMap,
}

impl Graph {
    pub fn read(path: &str) -> Graph {
        // Creates a new graph based on the data in a file
        let mut result: ListOfEdges = Vec::new();
        let file = File::open(path).expect("Could not open file");
        let buf_reader = std::io::BufReader::new(file).lines();
        let mut n = 0;
        for line in buf_reader {
            let line_str = line.expect("Error reading");
            let v: Vec<&str> = line_str.trim().split(' ').collect();
            let x = v[0].parse::<usize>().unwrap();
            let y = v[1].parse::<usize>().unwrap();
            result.push((x, y));
            n += 1;
        }
        return Graph::create(n, &result);
    }

    fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            if self.outedges.contains_key(u) {
                // vertex already in hashmap so we need to append the vertex to the corresponding vector
                let mut vertexes: Vec<Vertex> = self.outedges.get(u).expect("Could not find vertex").to_vec();
                vertexes.push(*v);
                self.outedges.insert(*u, vertexes);
            } else {
                self.outedges.insert(*u, vec![*v]);
            }
        }
    }

    fn sort_graph_lists(&mut self) {
        for (_, lst) in self.outedges.iter_mut() {
            lst.sort();
        }
    }

    pub fn create(n:usize,edges:&ListOfEdges) -> Graph {
        let mut g = Graph{n, outedges:HashMap::new()};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }

    // fn random_vertex(&self) -> Vertex {
    //     rand::thread_rng().gen_range(0..(self.n))
    // }

    // pub fn random_steps(&self, v: Vertex, steps: usize) -> Vertex {
    //     // Performs random steps starting from a given vertex
    //     let mut current_vertex: Vertex = v;
    //     for _ in 0..steps { // steps random steps from v
    //         let rand_prob = rand::thread_rng().gen_range(0..10);
    //         if self.outedges[current_vertex].len() == 0 || rand_prob == 9 {
    //             current_vertex = self.random_vertex();
    //         } else {
    //             let rand_index = rand::thread_rng().gen_range(0..self.outedges[current_vertex].len()); // randomly selects index from neighbors range  
    //             current_vertex = self.outedges[current_vertex][rand_index]; // uses random index
    //         }
    //     }
    //     return current_vertex;
    // }

    // pub fn page_rank(&self) -> Vec<usize> {
    //     // Calculates and displays the approximate page rank
    //     // Returns the raw counts of terminations for the random walks
    //     let mut counts = vec![0_usize; self.n];
    //     for i in 0..(self.n) { // iterate over each vertex to do the 100 random 100 step walks from
    //         for _ in 0..100 { // 100 walks from i
    //             let current_vertex = self.random_steps(i, 100);
    //             counts[current_vertex] += 1; // adds 1 to the random walk score for this Vertex
    //         }
    //     }

    //     // Display the top 5 results
    //     let mut indexed_counts: Vec<(usize, &usize)> = counts.iter().enumerate().collect();
    //     indexed_counts.sort_by_key(|&(_, v)| std::cmp::Reverse(v));
    //     let length = std::cmp::min(5, counts.len());
    //     for (index, value) in indexed_counts.iter().take(length) {
    //         let approx_rank: f32 = (*(*value) as f32) / (100.0 * (self.n as f32));
    //         println!("vertex {}: approximate PageRank {}", index, approx_rank);
    //     }

    //     return counts;
    // }
}
