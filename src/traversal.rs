use crate::node::Node;
use std::slice::Iter;

pub struct Edges<'a>(Iter<'a, (Node, Node)>);
impl<'a> ExactSizeIterator for Edges<'a> {}
impl<'a> Iterator for Edges<'a> {
    type Item = &'a (Node, Node);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<'a> Edges<'a> {
    pub (crate) fn new<I: IntoIterator<IntoIter=Iter<'a, (Node, Node)>>>(iter: I) -> Self {
        Edges(iter.into_iter())
    }
}

pub trait Graph<'a> {
    fn add_node(&mut self, node: Node);
    fn rm_node(&mut self, node: Node);
    fn rm_edge(&mut self, edge: (Node, Node));
    fn nodes(&self) -> Vec<Node>;
    fn edges(&'a self, node: Node) -> Edges<'a>;
}