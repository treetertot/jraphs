use crate::node::Node;
use crate::directed::Graph as DGraph;

#[derive(Debug, Clone)]
pub struct Graph<T>(DGraph<T>);
impl<T> Default for Graph<T> {
    fn default() -> Self {
        Graph::new()
    }
}
impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph(DGraph::new())
    }
    pub fn insert_node(&mut self, node: Node, data: T) -> Option<T> {
        self.0.insert_node(node, data)
    }
    pub fn rm_node(&mut self, node: Node) -> Option<T> {
        self.0.rm_node(node)
    }
    pub fn insert_edge(&mut self, a: Node, b: Node) {
        self.0.insert_edge(a, b);
        self.0.insert_edge(b, a);
    }
    pub fn rm_edge(&mut self, a: Node, b: Node) {
        self.0.rm_edge(a, b);
        self.0.rm_edge(b, a);
    }
}