use crate::graph::*;
use crate::algorithms::bfs::BFS;

pub mod graph;
pub mod algorithms;

fn main() {
    let graph = Graph::read("twitter_combined.txt");
    let _ = BFS::new(graph);
    // println!("{:?}", graph.outedges);
}
