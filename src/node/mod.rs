//! This module defines the node trait and lots of implementors
//!
//! A node is a piece of reactive code that can be called. It has an input value
//! and an output value. A node is always instantaneous and sequential.
//! A non-immediate Process is thus compiled to multiple nodes.
//!
//! Contrary to processes, node composition is fully static and thus rustc can
//! fully inline node calls. However it seams that compile time is exponential in
//! the depth of generic types and thus in the length of a static Node sequence.
//! See `PJump` and `NJump` about this.
//!
//! Node that are an item of a process' final CFG have signature () -> () and will be called
//! "main Nodes". Other nodes may be called "internal nodes".
//!
//! One important property on the CFG guaranteed by the Process construction is that a node may
//! never appear twice in the Runtime list of node to execute i.e it is never possible to
//! data-race a node when running such a graph in parallel. (See funsafe implementation in
//! `engine` module). Therefore a given arrow of the graph may also never by passed twice
//! through in different thread at the same time.

use std::marker::PhantomData;

use engine::*;
use utility::{take,tname};
use std::collections::HashMap;
use super::*;

mod mem_manip;
#[doc(hidden)] // for private doc remove for public doc
pub use self::mem_manip::*;
mod control;
#[doc(hidden)] // for private doc remove for public doc
pub use self::control::*;
mod par;
#[doc(hidden)] // for private doc remove for public doc
pub use self::par::*;
mod signal;
#[doc(hidden)] // for private doc remove for public doc
pub use self::signal::*;
pub mod sig_control;
#[doc(hidden)]
pub use self::sig_control::*;

/// Trait for a Node In -> Out
///
/// A Node<'a,In> must live 'a, and be Send when run in parallel
pub trait Node<'a, In: Val<'a>>: Val<'a> {
    /// The type outputted by the node when In is given.
    type Out: Val<'a>;
    /// Calls a node.
    ///
    /// sub_runtime is given to allow the node to perform actions with the reactive runtime.
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: In) -> Self::Out;

    /// Print part of a "record" dot label to represent the node
    fn printDot(&mut self, _: &mut CFGDrawer) {
        print!("{}",tname::<Self>())
    }

    /// Return a node that will run self and then n2
    fn nseq<N2>(self, n2: N2) -> NSeq<Self, N2>
    where
        N2: Node<'a, Self::Out> + Sized,
        Self: Sized,
    {
        NSeq { n1: self, n2: n2 }
    }

    /// Returns a node that will run `self` of `nf` depending on `ChoiceData` input
    fn alter<NF, In2: Val<'a>>(self, nf: NF) -> NChoice<Self, NF>
    where
        NF: Node<'a, In2, Out = Self::Out> + Sized,
        Self: Sized,
    {
        NChoice { nt: self, nf: nf }
    }

    /// Returns a node that will take an input (a,b), run `self` on `a`,
    /// then `n2` on `b` and then return the pair of results
    fn njoin<In2: Val<'a>, N2>(self, n2: N2) -> NPar<Self, N2>
    where
        N2: Node<'a, In2> + Sized,
        Self: Sized,
    {
        NPar { n1: self, n2: n2 }
    }
}

//  _   _       _   _     _
// | \ | | ___ | |_| |__ (_)_ __   __ _
// |  \| |/ _ \| __| '_ \| | '_ \ / _` |
// | |\  | (_) | |_| | | | | | | | (_| |
// |_| \_|\___/ \__|_| |_|_|_| |_|\__, |
//                                |___/

/// Node that does nothing but forcing its In/Out types to be ()
///
/// Signature : `() -> ()`
#[derive(Clone, Copy)]
pub struct Nothing {}

impl<'a> Node<'a, ()> for Nothing {
    type Out = ();
    fn call(&mut self, _: &mut SubRuntime<'a>, _val: ()) -> Self::Out {}
}

//  ___    _            _   _ _
// |_ _|__| | ___ _ __ | |_(_) |_ _   _
//  | |/ _` |/ _ \ '_ \| __| | __| | | |
//  | | (_| |  __/ | | | |_| | |_| |_| |
// |___\__,_|\___|_| |_|\__|_|\__|\__, |
//                                |___/


/// Node that just passes its input value intact.
///
/// Signature : `I -> I`
#[derive(Clone, Copy)]
pub struct NIdentity {}

impl<'a, In: Val<'a>> Node<'a, In> for NIdentity {
    type Out = In;

    fn call(&mut self, _: &mut SubRuntime<'a>, val: In) -> Self::Out {
        val
    }
}

// __     __    _
// \ \   / /_ _| |_   _  ___
//  \ \ / / _` | | | | |/ _ \
//   \ V / (_| | | |_| |  __/
//    \_/ \__,_|_|\__,_|\___|

/// Node that returns a constant value of type V that must be Clone.
///
/// Signature : `() -> V`
pub struct NValue<V>(pub V);

impl<'a, V: Val<'a>> Node<'a, ()> for NValue<V>
where
    V: Clone,
{
    type Out = V;

    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> V {
        self.0.clone()
    }
}


//  _____      __  __       _
// |  ___| __ |  \/  |_   _| |_
// | |_ | '_ \| |\/| | | | | __|
// |  _|| | | | |  | | |_| | |_
// |_|  |_| |_|_|  |_|\__,_|\__|

/// Node that calls a FnMut
///
/// Signature : `I -> O` when `F : FnMut(I) -> O`
pub struct NFnMut<F>(pub F);

impl<'a, F, In: Val<'a>, Out: Val<'a>> Node<'a, In> for NFnMut<F>
where
    F: FnMut(In) -> Out + Val<'a>,
{
    type Out = Out;
    fn call(&mut self, _: &mut SubRuntime<'a>, val: In) -> Out {
        (&mut self.0)(val)
    }
    fn printDot(&mut self, _: &mut CFGDrawer) {
        print!("FnMut : {} -\\> {}", tname::<In>(), tname::<Out>())
    }
}


//  _____       ___
// |  ___| __  / _ \ _ __   ___ ___
// | |_ | '_ \| | | | '_ \ / __/ _ \
// |  _|| | | | |_| | | | | (_|  __/
// |_|  |_| |_|\___/|_| |_|\___\___|

/// Node that calls a FnOnce. Will panic if called twice
///
/// Signature : `I -> O` when `F : FnOnce(I) -> O`
pub struct NFnOnce<F>(pub Option<F>);

impl<'a, F, In: Val<'a>, Out: Val<'a>> Node<'a, In> for NFnOnce<F>
where
    F: FnOnce(In) -> Out + Val<'a>,
{
    type Out = Out;

    fn call(&mut self, _: &mut SubRuntime<'a>, val: In) -> Out {
        let option = take(&mut self.0);
        if let Some(f) = option {
            f(val)
        } else {
            panic!("NFnOnce was called twice!");
        }
    }
    fn printDot(&mut self, _: &mut CFGDrawer) {
        print!("FnOne : {} -> {}", tname::<In>(), tname::<Out>())
    }
}

//  ____
// / ___|  ___  __ _
// \___ \ / _ \/ _` |
//  ___) |  __/ (_| |
// |____/ \___|\__, |
//                |_|

/// Node that call n1 on input value and then n2 on n1's output.
///
/// Signature : `I -> O` when `n1: I -> M` and `n2: M -> O`
pub struct NSeq<N1, N2> {
    n1: N1,
    n2: N2,
}

impl<'a, N1, N2, In: Val<'a>, Mid: Val<'a>, Out: Val<'a>> Node<'a, In> for NSeq<N1, N2>
where
    N1: Node<'a, In, Out = Mid>,
    N2: Node<'a, Mid, Out = Out>,
{
    type Out = Out;
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: In) -> Out {
        let valm = self.n1.call(sub_runtime, val);
        self.n2.call(sub_runtime, valm)
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        print!("{{{{");
        self.n1.printDot(cfgd);
        print!("}}|{{");
        self.n2.printDot(cfgd);
        print!("}}}}");
    }
}

//  _____           _
// | ____|_ __   __| |
// |  _| | '_ \ / _` |
// | |___| | | | (_| |
// |_____|_| |_|\__,_|

/// Node that signal to the runtime that the whole process has ended
///
/// Only used by `compile_to_graph` functions.
///
/// Signature : `() -> ()`
pub struct NEnd{}
impl<'a> Node<'a, ()> for NEnd
{
    type Out = ();
    fn call(&mut self, sub: &mut SubRuntime<'a>, _: ()) -> () {
        sub.end();
    }
}





//   ____                 _
//  / ___|_ __ __ _ _ __ | |__
// | |  _| '__/ _` | '_ \| '_ \
// | |_| | | | (_| | |_) | | | |
//  \____|_|  \__,_| .__/|_| |_|
//                 |_|

/// Structure that stores information to draw Node graphs
pub struct CFGDrawer {
    map: HashMap<usize, usize>,
    ptr_ind: usize,
    node_ind: usize,
    internal_node_ind: usize,
    arrow: Vec<(usize, usize)>,
}


impl CFGDrawer {
    pub fn new() -> CFGDrawer {
        CFGDrawer {
            map: HashMap::new(),
            ptr_ind: 0,
            node_ind: 0,
            internal_node_ind: 0,
            arrow: vec![]
        }
    }
    /// Starts the drawing of a node
    fn start_node(&mut self, node : usize){
        self.node_ind = node;
        print!("{} [shape=record,label=\"",node);
    }
    /// Ends the drawing of a node
    fn stop_node(&mut self){
        println!("\"]");
        for &(s,i) in &self.arrow{
            println!("{}:f{} -> {}",self.node_ind,s,i);
        }
        self.arrow.clear();
    }
    /// Get the next pointer index.
    ///
    /// Pointer like RCell or Rcjp have an index that is printed in the final graph.
    fn get_ind<T>(&mut self, ptr: *const T) -> usize {
        let u = ptr as usize;
        match self.map.get(&u) {
            Some(ind) => {return *ind; }
            None => {}
        }
        let ind = self.ptr_ind;
        self.ptr_ind += 1;
        self.map.insert(u,ind);
        ind
    }

    /// Get the main node index
    fn get_node(&mut self) -> usize{
        self.node_ind
    }
    /// Get the next internal node index (not all internal node will require an index)
    fn get_node_ind(&mut self) -> usize{
        let ind = self.internal_node_ind;
        self.internal_node_ind +=1;
        ind
    }
    /// Add an arrow from the internal node arr.0 of the current main node to the main node arr.1
    fn add_arrow(&mut self, arr: (usize, usize)){
        self.arrow.push(arr)
    }
    /// Print the full text of a node
    pub fn printNode<'a,N : ?Sized>(&mut self, ind: usize, n: &mut N)
        where N : Node<'a,(),Out =()>
    {
        self.start_node(ind);
        n.printDot(self);
        self.stop_node();
    }

}
