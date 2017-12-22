use std::vec::Vec;


use node::*;
use process::*;
use process_par::*;
use take::take;
use engine::{SubRuntime,Tasks,EndOfInstant};

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
pub struct GraphPar<'a>(Vec<Option<Box<Node<'a, (), Out = ()> + Send + Sync>>>);

impl<'a> GraphPar<'a> {

    /// Creates an empty graph.
    pub(crate) fn new() -> Self {
        GraphPar(vec![])
    }

    /// Reserves a fresh id and returns it
    pub(crate) fn reserve(&mut self) -> usize {
        let &mut GraphPar(ref mut v) = self;
        v.push(None);
        v.len() - 1
    }

    /// Sets a Node at a given position.
    ///
    /// Sets a Node at a position reserved by [`reserve`](struct.Graph.html#method.reserve).
    /// If the position is not valid (it was never reserved or it has already been set), it panics.
    pub(crate) fn set(&mut self, pos: usize, val: Box<Node<'a, (), Out = ()> + Send + Sync>) {
        let &mut GraphPar(ref mut v) = self;
        if let Some(_) = v[pos] {
            panic!("v[pos] != None in Graph::set")
        }
        v[pos] = Some(val);
    }

    /// Adds a new node to the graph
    ///
    /// It's the same than calling reserve then add.
    /// Returns the id of the added node.
    pub(crate) fn add(&mut self, val: Box<Node<'a, (), Out = ()> + Send + Sync>) -> usize {
        let &mut GraphPar(ref mut v) = self;
        v.push(Some(val));
        v.len() - 1
    }


    /// Return the underlying data structure
    pub(crate) fn get(self) -> Vec<Option<Box<Node<'a, (), Out = ()> + Send + Sync>>> {
        let GraphPar(v) = self;
        v
    }
}



/// Runtime for running reactive graph.
///
/// It contains all the information needed to execute of a reactive process.
pub struct RuntimePar<'a> {
    /// The reactive control-flow graph in non-optional version. See [`Graph`](struct.Graph.html).
    nodes: Vec<Box<Node<'a, (), Out = ()> + Send + Sync>>,

    // The SubRuntime containing all runtime info.
    sub_runtime: SubRuntime<'a>,
}

impl<'a> RuntimePar<'a> {
    /// Creates a new empty runtime.
    fn newempty() -> Self {
        RuntimePar::<'a> {
            nodes: vec![],
            sub_runtime: SubRuntime {
                current_instant: 2,
                tasks: Tasks {
                    current: vec![],
                    next: vec![],
                },
                eoi: EndOfInstant {
                    pending: vec![],
                },
            }
        }
    }

    /// Creates a runtime from a Graph.
    ///
    /// The graph must be complete i.e any reserved id must not be empty.
    /// If the graph is not complete, it panics.
    /// This function does not setup a start point:
    fn fromgraph(g: GraphPar<'a>) -> Self {
        let mut r = Self::newempty();
        for n in g.get() {
            match n {
                Some(b) => {
                    r.nodes.push(b);
                }
                None => unreachable!(),
            }
        }
        r
    }

    /// [gf]: ../process/trait.GraphFiller.html
    /// [mp]: ../process/struct.MarkedProcess.html

    /// Creates a Runtime by using a value implementing [`GraphFiller`][gf].
    ///
    /// After this function, the runtime is ready to be used
    /// Normally,types that implement [`GraphFiller`][gf] are [`MarkedProcess`][mp]
    pub fn new<GF>(gf: GF) -> Self
    where
        GF: GraphFillerPar<'a>,
    {
        let mut g = GraphPar::new();
        let start = gf.fill_graph_par(&mut g);
        let mut r = RuntimePar::fromgraph(g);
        r.sub_runtime.tasks.current.push(start);
        r
    }


    /// Executes the whole reactive process until it ends.
    pub fn execute(&mut self) {
        while self.instant() {}
    }

    /// Executes an single instant of the reactive process loaded in the runtime.
    ///
    /// Returns whether the process should continue.
    pub fn instant(&mut self) -> bool {
        while self.sub_runtime.tasks.current.len() > 0 {
            let v = take(&mut self.sub_runtime.tasks.current);
            for i in v {
                self.nodes[i].call(&mut self.sub_runtime, ());
            }
        }
        self.sub_runtime.tasks.current = take(&mut self.sub_runtime.tasks.next);

        let eois = take(&mut self.sub_runtime.eoi.pending);
        for eoi in eois {
            eoi.on_end_of_instant(&mut self.sub_runtime);
        }

        self.sub_runtime.current_instant += 1;

        self.sub_runtime.tasks.current.len() > 0 || self.sub_runtime.eoi.pending.len() > 0
    }
}
