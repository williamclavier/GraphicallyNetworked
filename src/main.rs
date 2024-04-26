mod graph;
use crate::graph::*;

fn main() {
    let graph = Graph::read("twitter_combined.txt");
    println!("{:?}", graph.outedges);
}
