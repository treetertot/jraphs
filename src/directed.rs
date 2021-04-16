use crate::node::{Node, NodeData};

#[derive(Debug, Clone)]
pub struct Graph<T> {
    edges: Vec<(Node, Node)>,
    data: NodeData<T>
}
impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            edges: Vec::new(),
            data: NodeData::new()
        }
    }
    fn edge_idx(&self, a: Node, b: Node) -> Result<usize, usize> {
        self.edges.binary_search_by_key(&(a, b), |pair| *pair)
    }
    pub fn insert_node(&mut self, node: Node, data: T) -> Option<T> {
        self.data.insert(node, data)
    }
    pub fn rm_node(&mut self, node: Node) -> Option<T> {
        self.data.remove(node)
    }
    pub fn insert_edge(&mut self, a: Node, b: Node) {
        if let Err(idx) = self.edge_idx(a, b) {
            self.edges.insert(idx, (a, b));
        }
    }
    pub fn rm_edge(&mut self, a: Node, b: Node) {
        if let Ok(idx) = self.edge_idx(a, b) {
            self.edges.remove(idx);
        }
    }
}