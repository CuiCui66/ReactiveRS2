
use node::*;
/// This type represent a full control-flow graph of a reactive system.
///
/// This is the result of the compilation and will be directly run in the runtime.
/// All the Nodes in the graph are identified by an id (the index in the vector),
/// and they all have a type `() -> ()`. The control-flow edges are encoded in the
/// nodes themselves by interacting with the runtime.
/// A value of `None` marks a reserved id.
/// Reserved values may only be used during the compilation process but not during the runtime
/// (The Runtime type store the same vector but without option).
/// see [Runtime::fromgraph](struct.Runtime.html#method.fromgraph).
pub struct Graph<'a> {
    nodes: Vec<Option<Box<Node<'a, (), Out = ()>>>>,
}

impl<'a> Graph<'a> {

    /// Creates an empty graph.
    pub(crate) fn new() -> Self {
        Graph {
            nodes: vec![],
        }
    }

    /// Reserves a fresh id and returns it
    pub(crate) fn reserve(&mut self) -> usize {
        self.nodes.push(None);
        self.nodes.len() - 1
    }

    /// Sets a Node at a given position.
    ///
    /// Sets a Node at a position reserved by [`reserve`](struct.Graph.html#method.reserve).
    /// If the position is not valid (it was never reserved or it has already been set), it panics.
    pub(crate) fn set(&mut self, pos: usize, val: Box<Node<'a, (), Out = ()>>) {
        if let Some(_) = self.nodes[pos] {
            panic!("v[pos] != None in Graph::set")
        }
        self.nodes[pos] = Some(val);
    }

    /// Adds a new node to the graph
    ///
    /// It's the same than calling reserve then add.
    /// Returns the id of the added node.
    pub(crate) fn add(&mut self, val: Box<Node<'a, (), Out = ()>>) -> usize {
        let pos = self.nodes.len();
        self.nodes.push(Some(val));
        pos
    }

    /// Return the underlying data structure
    pub(crate) fn get(self) -> Vec<Option<Box<Node<'a, (), Out = ()>>>> {
        self.nodes
    }
}
