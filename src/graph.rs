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
        let file = File::open(path).expect("Could not open file"); // opens file
        let buf_reader = std::io::BufReader::new(file).lines(); // creates reader for file
        for line in buf_reader { // reads the file line by line using the reader
            let line_str = line.expect("Error reading"); // converts to string

            // Splits the line by spaces and then stores the result in the list of edges
            let v: Vec<&str> = line_str.trim().split(' ').collect();
            let x = v[0].parse::<usize>().unwrap();
            let y = v[1].parse::<usize>().unwrap();
            result.push((x, y));
        }
        return Graph::create(&result);
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

    pub fn create(edges:&ListOfEdges) -> Graph {
        let mut g = Graph{n:0, outedges:HashMap::new()}; // initialize with 0 for n because we need to get n from the size of the hashmap
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g.n = g.outedges.len();
        g                                        
    }

}
