use crate::node::Node;
use crate::directed::Directed;
use crate::traversal::*;

#[derive(Debug, Clone)]
pub struct Undirected(Directed);
impl Default for Undirected {
    fn default() -> Self {
        Undirected::new()
    }
}
impl<'a> Undirected {
    pub fn new() -> Self {
        Undirected(Directed::new())
    }
    pub fn components(&'a self) -> Components<'a> {
        Components {
            remaining: self.nodes(),
            graph: self
        }
    }
}

impl<'a> Graph<'a> for Undirected {
    fn add_node(&mut self, node: Node) {
        self.0.add_node(node);
    }
    fn rm_node(&mut self, node: Node) {
        self.0.rm_node(node)
    }
    fn rm_edge(&mut self, edge: (Node, Node)) {
        self.0.rm_edge(edge)
    }
    fn nodes(&self) -> Vec<Node> {
        self.0.nodes()
    }
    fn edges(&'a self, node: Node) -> Edges<'a> {
        self.0.edges(node)
    }
}
impl Extend<(Node, Node)> for Undirected  {
    // extend and do full sort if new edges > old
    fn extend<T: IntoIterator<Item = (Node, Node)>>(&mut self, iter: T) {
        self.0.extend(Flipper {
            iter: iter.into_iter(),
            hold: None
        })
    }
}

pub struct Flipper<I> {
    iter: I,
    hold: Option<(Node, Node)>
}
impl<I> Iterator for Flipper<I> where
I: Iterator<Item=(Node, Node)> {
    type Item = (Node, Node);

    fn next(&mut self) -> Option<Self::Item> {
        match self.hold {
            Some(pair) => {
                self.hold = None;
                Some(pair)
            },
            None => {
                let (a, b) = self.iter.next()?;
                self.hold = Some((b, a));
                Some((a, b))
            },
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let diff = self.hold.is_some() as usize;
        let (a, b) = self.iter.size_hint();
        (a * 2 - diff, b.map(|b| b * 2 - diff))
    }
}
/// Components is a stream (not an Iterator) over component iterators
pub struct Components<'a> {
    remaining: Vec<Node>,
    graph: &'a Undirected
}
impl<'a> Components<'a> {
    pub fn next(&'a mut self) -> Option<Component<'a>> {
        let nxt = self.remaining.pop()?;
        let search = vec![self.graph.edges(nxt)];
        Some(Component{
            remaining: &mut self.remaining,
            graph: self.graph,
            search_stack: search
        })
    }
}

pub struct Component<'a> {
    remaining: &'a mut Vec<Node>,
    graph: &'a Undirected,
    search_stack: Vec<Edges<'a>>
}
impl<'a> Iterator for Component<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let top = self.search_stack.last_mut()?;

        for (_a, b) in top {
            if let Ok(idx) = self.remaining.binary_search(b) {
                self.remaining.remove(idx);
                let edges = self.graph.edges(*b);
                self.search_stack.push(edges);
                return Some(*b)
            }
        }
        self.search_stack.pop();
        self.next()
    }
}