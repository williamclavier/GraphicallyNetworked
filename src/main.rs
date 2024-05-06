use crate::graph::*;
use crate::algorithms::bfs::BFS;

pub mod graph;
pub mod algorithms;

fn main() {
    let twitter_graph = Graph::read("twitter_combined.txt");
    let bfs = BFS::new(twitter_graph);

    // Calculate: five, 10,000 sample average distance scores
    let sample_size: usize = 10000;
    println!("For samples of size: {:?}", sample_size);
    for i in 0..5 {
        println!("i={:?}\tAverage node distance: {:?}", i, bfs.average_distance(sample_size));
    }
}
