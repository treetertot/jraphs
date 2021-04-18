use crate::node::Node;
use std::ops::Range;
use crate::traversal::{Edges, Graph};
/// A Directed Graph
#[derive(Debug, Clone)]
pub struct Directed {
    edges: Vec<(Node, Node)>,
    nodes: Vec<Node>
}
impl Default for Directed{
    fn default() -> Self {
        Self::new()
    }
}
impl Directed{
    pub fn new() -> Directed{
        Directed{
            edges: Vec::new(),
            nodes: Vec::new()
        }
    }
    fn edge_idx(&self, pair: &(Node, Node)) -> Result<usize, usize> {
        self.edges.binary_search(pair)
    }
    fn neighbor_range(&self, node: Node) -> Range<usize> {
        let min = match self.edge_idx(&(node, Node::MIN)) {
            Ok(n) => n,
            Err(n) => n
        };
        let max = match self.edge_idx(&(node, Node::MAX)) {
            Ok(n) => n,
            Err(n) => n
        };
        min..max
    }
}

impl<'a> Graph<'a> for Directed {
    fn add_node(&mut self, node: Node) {
        if let Err(idx) = self.nodes.binary_search(&node) {
            self.nodes.insert(idx, node);
        }
    }
    fn rm_edge(&mut self, edge: (Node, Node)) {
        if let Ok(idx) = self.edges.binary_search(&edge) {
            self.edges.remove(idx);
        }
    }
    fn rm_node(&mut self, node: Node) {
        if let Ok(idx) = self.nodes.binary_search(&node) {
            self.nodes.remove(idx);
        }
    }
    fn edges(&'a self, node: Node) -> Edges<'a> {
        Edges::new(&self.edges[self.neighbor_range(node)])
    }
    fn nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }
}

impl Extend<(Node, Node)> for Directed {
    // extend and do full sort if new edges > old
    fn extend<T: IntoIterator<Item = (Node, Node)>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        self.edges.reserve(iter.size_hint().0);
        for edge in iter {
            if let Err(idx) = self.edge_idx(&edge) {
                self.edges.insert(idx, edge);
            }
        }
    }
}