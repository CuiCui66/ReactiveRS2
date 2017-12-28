use std::vec::Vec;
use std::boxed::Box;
use core::ops::DerefMut;

use node::*;
use node::sig_control::ControlSignal;
use process::*;
use take::take;
use super::*;







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
    control_signals: Vec<ControlSignal<'a>>,
}

impl<'a> Graph<'a> {

    /// Creates an empty graph.
    pub(crate) fn new() -> Self {
        Graph {
            nodes: vec![],
            control_signals: vec![],
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

        if !self.control_signals.is_empty() {
            let node = ControlNode {
                id: pos,
                node: val,
                control_sig: self.control_signals.clone()
            };
            self.nodes[pos] = Some(box node);
        } else {
            self.nodes[pos] = Some(val);
        }
    }

    /// Adds a new node to the graph
    ///
    /// It's the same than calling reserve then add.
    /// Returns the id of the added node.
    pub(crate) fn add(&mut self, val: Box<Node<'a, (), Out = ()>>) -> usize {
        let pos = self.nodes.len();
        if !self.control_signals.is_empty() {
            let node = ControlNode {
                id: pos,
                node: val,
                control_sig: self.control_signals.clone()
            };
            self.nodes.push(Some(box node));
        } else {
            self.nodes.push(Some(val));
        }
        pos
    }

    /// Return the underlying data structure
    pub(crate) fn get(self) -> Vec<Option<Box<Node<'a, (), Out = ()>>>> {
        self.nodes
    }

    /// Add a control signal
    /// This will wrap nodes with signal checking, as long as the control signal is not pop
    pub(crate) fn push_control_signal(&mut self, control_signal: ControlSignal<'a>) {
        self.control_signals.push(control_signal);
    }

    /// Remove the last pushed control signal
    pub(crate) fn pop_control_signal(&mut self) {
        self.control_signals.pop();
    }
}





/// Contains the remaining node to be executed
pub(crate) struct Tasks {
    /// Contains nodes to be executed on the current instants.
    /// Nodes can add other nodes' id to continue the execution in an other node on the same instant.
    pub(crate) current: Vec<usize>,
    /// Contains nodes to be executed on the next instants.
    /// Nodes can add other nodes' id and stop to pause their execution until the next instant.
    pub(crate) next: Vec<usize>,
}





pub trait EndOfInstantCallback<'a> : Val<'a>{
    fn on_end_of_instant(&self, sub_runtime: &mut SubRuntime<'a>);
}





/// Contains a list of [signal](../signal/index.html)
/// related continuation to be run at the end of the instant.
pub(crate) struct EndOfInstant<'a> {
    pub(crate) pending: Vec<Box<EndOfInstantCallback<'a> + 'a>>,
}





/// The part of the runtime that is passed to Nodes, see
/// [Node::call](../node/trait.Node.html#tymethod.call).
pub struct SubRuntime<'a> {
    /// The tasks lists
    tasks: Tasks,
    /// The end of instant continuations.
    eoi: EndOfInstant<'a>,
    /// The id of the current instant.
    current_instant: usize,
}

impl<'a> SubRuntime<'a>{
    pub fn add_current(&mut self, ind: usize) {
        self.tasks.current.push(ind);
    }
    pub fn add_next(&mut self, ind: usize) {
        self.tasks.next.push(ind);
    }
    pub fn add_eoi(&mut self, box_eoi: Box<EndOfInstantCallback<'a>>) {
        self.eoi.pending.push(box_eoi);
    }
    pub fn get_current_instant(&mut self) -> usize {
        self.current_instant
    }
}





/// Runtime for running reactive graph.
///
/// It contains all the information needed to execute of a reactive process.
pub struct Runtime<'a> {
    /// The reactive control-flow graph in non-optional version. See [`Graph`](struct.Graph.html).
    nodes: Vec<Box<Node<'a, (), Out = ()>>>,

    // The SubRuntime containing all runtime info.
    sub_runtime: SubRuntime<'a>,
}

impl<'a> Runtime<'a> {
    /// Creates a new empty runtime.
    fn newempty() -> Self {
        Runtime::<'a> {
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
    fn fromgraph(g: Graph<'a>) -> Self {
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

    /// [gf]: ../process/trait..html
    /// [mp]: ../process/struct.MarkedProcess.html

    /// Creates a Runtime by using a value implementing [``][gf].
    ///
    /// After this function, the runtime is ready to be used
    /// Normally,types that implement [`GraphFiller`][gf] are [`MarkedProcess`][mp]
    pub fn new<P>(p: P) -> Self
    where
        P: GraphFiller<'a>,
    {
        let mut g = Graph::new();
        let start = p.fill_graph(&mut g);
        let mut r = Runtime::fromgraph(g);
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

    pub fn printDot(&mut self){
        println!("digraph {{");
        let mut cfgd = CFGDrawer::new();
        for (i,node) in self.nodes.iter_mut().enumerate(){
            printNode(i,node.deref_mut(),&mut cfgd);
        }
        println!("}}");
    }
}
