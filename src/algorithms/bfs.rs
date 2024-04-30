use super::super::Graph;

pub struct BFS {
    graph: Graph,
}

impl BFS {
    pub fn new(graph: Graph) -> BFS {
        BFS{graph}
    }
}