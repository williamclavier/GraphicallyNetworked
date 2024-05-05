use crate::graph::*;
use crate::algorithms::bfs::BFS;

pub mod graph;
pub mod algorithms;

fn main() {
    let graph = Graph::read("twitter_combined.txt");
    let bfs = BFS::new(graph);
    println!("{:?}", bfs.average_distance(5000));
}
