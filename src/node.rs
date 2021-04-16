use std::cmp::{Ord, PartialOrd, Eq, PartialEq, Ordering};
use std::mem;

pub type Node = usize;

#[derive(Debug, Clone)]
struct NodeStorage<T> {
    node: Node,
    data: T
}
impl<T> PartialEq for NodeStorage<T> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}
impl<T> Eq for NodeStorage<T> {}
impl<T> PartialOrd for NodeStorage<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.node.partial_cmp(&other.node)
    }
}

impl<T> Ord for NodeStorage<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.node.cmp(&other.node)
    }
}
#[derive(Debug, Clone)]
pub struct NodeData<T> (Vec<NodeStorage<T>>);
impl<T> Default for NodeData<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> NodeData<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    fn node_idx(&self, node: Node) -> Result<usize, usize> {
        self.0.binary_search_by_key(&node, |ns| ns.node)
    }
    pub fn insert(&mut self, node: Node, data: T) -> Option<T> {
        match self.node_idx(node) {
            Ok(idx) => Some(mem::replace(&mut self.0[idx].data, data)),
            Err(idx) => {
                self.0.insert(idx, NodeStorage{
                    node,
                    data
                });
                None
            }
        }
    }
    pub fn remove(&mut self, node: Node) -> Option<T> {
        match self.node_idx(node) {
            Ok(idx) => Some(self.0.remove(idx).data),
            Err(_idx) => None
        }
    }
    pub fn get(&self, node: Node) -> Option<&T> {
        match self.node_idx(node) {
            Ok(idx) => Some(&self.0[idx].data),
            Err(_idx) => None
        }
    }
}