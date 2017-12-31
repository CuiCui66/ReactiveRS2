//! This module defines the global mechanism of processes.
//!
//! A process represents any reactive calculation that takes an input value
//! and outputs a value. `()` is used to represent a no-value. A process can be seen as
//! a control flow graph (see `print_graph`) whose edge has types and value of those types
//! moving through those edges.
//!
//! A process, when entering a Runtime, will be compiled to control-flow graph (CFG) of Nodes.
//! This graph is of a different kind and explained in `node`.
//! A process may represent a computation that spans on several instant or just a
//! single instant : If the process is compiled to a single Node, we say that it is
//! immediate (it will take only a single instant to compute, and this is known at compile time).
//! If it compiles to multiple nodes, it is not immediate (it may or may not take a single
//! instant to compute).
//!
//! A process is always boxed. The Box can be either ProcessIm or ProcessNotIm that both
//! implement Process and should be the only ones. The virtualized interfaces are traits
//! IntProcessIm and IntProcessNotIm that have in common IntProcess.
//! To be in concordance with trait names, by "process" we will denote the box (ProcessIm)
//! or (ProcessNotIm), and we will use "process implementation" to denote the concrete type
//! behind the virtual interface.
//!
//! The available constructs may be seen quickly in the method of the `Process` trait and with more
//! in the `macro` crate




use node::*;
use signal::*;
use graph::*;
pub(crate) use utility::*;
use super::*;

use std::marker::PhantomData;

/// Contains all basic struct of reactive processes, closure, Pause, ...
mod base;
#[doc(hidden)] // for private doc remove for public doc
pub use self::base::*;

/// Contains standard control structures : `if` and `loop`
mod control;
#[doc(hidden)]
pub use self::control::*;

/// Contains sequencing structure i.e `;`
mod seq;
#[doc(hidden)]
use self::seq::*;


/// Contains parallel structures i.e `||` and `BigPar`
mod par;
#[doc(hidden)]
pub use self::par::*;

/// Contains signal related control structures.
mod signal;
#[doc(hidden)]
pub use self::signal::*;

//   ___
//  / _ \ _ __   ___ ___
// | | | | '_ \ / __/ _ \
// | |_| | | | | (_|  __/
//  \___/|_| |_|\___\___|

/// `Once` is a marker trait for a process being runnable multiple times or not
///
/// Normally a type verifying `Once` can have only two values: `IsOnce` and `NotOnce`
/// whose meanings are trivial
pub trait Once: Copy + 'static {}

/// This trait is implemented by any meta-function whose return value is a type
/// implementing Once
pub trait GiveOnce {
    type Once: Once;
}


/// Marker for a process that can only be executed once.
///
/// In practice, the only way for that to happen is that there is a `FnOnce` somewhere
#[derive(Clone, Copy)]
pub struct IsOnce;
impl Once for IsOnce {}
/// Marker for a process that can only be executed once.
#[derive(Clone, Copy)]
pub struct NotOnce;
impl Once for NotOnce {}


/// Meta-function taking two type implementing Once and giving a result via `GiveOnce`
pub struct And<O1, O2>
where
    O1: Once,
    O2: Once,
{
    o1: PhantomData<O1>,
    o2: PhantomData<O2>,
}

/// In the general case of composition of to process the result is `IsOnce`
default impl<O1, O2> GiveOnce for And<O1, O2>
where
    O1: Once,
    O2: Once,
{
    type Once = IsOnce;
}

/// Except if both process are `NotOnce`
impl GiveOnce for And<NotOnce, NotOnce> {
    type Once = NotOnce;
}


//  ___       _   ____
// |_ _|_ __ | |_|  _ \ _ __ ___   ___ ___  ___ ___
//  | || '_ \| __| |_) | '__/ _ \ / __/ _ \/ __/ __|
//  | || | | | |_|  __/| | | (_) | (_|  __/\__ \__ \
// |___|_| |_|\__|_|   |_|  \___/ \___\___||___/___/


/// Common interface for processes and process implementations.
pub trait IntProcess<'a, In: Val<'a>>: 'a {
    /// The type outputted by the process when In is given.
    type Out: Val<'a>;

    /// Type-check mark for the process being able to be called multiple times
    ///
    /// * If `MarkOnce` is `IsOnce`, then the process can only be called once
    /// * If `MarkOnce` is `NotOnce`, then the process can be called multiple times
    type MarkOnce: Once;

    /// Prints dot code for this process on stdout.
    ///
    /// This function will use node numbers starting from the initial value of curNum.
    /// it will increase curNum when it use a node number.
    /// The return values is the input and output node of the subgraph representing this process.
    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize);
}



/// Interface for immediate process implementations.
pub trait IntProcessIm<'a, In: Val<'a>>: IntProcess<'a, In> {
    /// The type of node this implementation compiles to.
    type NIO: Node<'a, In, Out = Self::Out>;

    /// Compiles an immediate process implementation
    ///
    /// It just outputs a node that do the whole computation implemented by the
    /// process implementation in a single instant.
    fn compileIm(self: Box<Self>, g: &mut Graph<'a>) -> Self::NIO;
}



/// Interface for non-immediate process implementations.
pub trait IntProcessNotIm<'a, In: Val<'a>>: IntProcess<'a, In> {
    /// The input node of the CFG of this implementation
    type NI: Node<'a, In, Out = ()>;
    /// The output node of the CFG of this implementation
    type NO: Node<'a, (), Out = Self::Out>;

    /// Compiles a non-immediate process implementation
    ///
    /// The process may be compiled to an arbitrary control-flow graph, which is set directly
    /// in the mutable variable `g` recieved as input.
    /// `compile` only outputs both ends of the process's graph:
    ///
    /// * The input node which must be fed with the input value of the process, it will
    ///   than probably call by id any node in the graph.
    /// * The output node is returned with an id. It will give the output value of the process
    ///   when called during the normal execution of the runtime (i.e after another node has put
    ///   its id in the runtime). This node must be placed (after adding other stuff behind) in
    ///   the id slot given as middle return value so other nodes can reference it.
    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO);
}

//  ____            _
// | __ )  _____  _(_)_ __   __ _
// |  _ \ / _ \ \/ / | '_ \ / _` |
// | |_) | (_) >  <| | | | | (_| |
// |____/ \___/_/\_\_|_| |_|\__, |
//                          |___/

/// An immediate process.
///
/// This is simply a virtualization of an immediate process implementation.
#[cfg_attr(rustfmt, rustfmt_skip)]
pub struct ProcessIm<'a, In, Out, MarkOnce, NIO>(
    pub(crate) Box<IntProcessIm<'a, In, Out = Out, MarkOnce = MarkOnce, NIO = NIO>>
);


impl<'a, In: Val<'a>, Out: Val<'a>, MarkOnce, NIO> ProcessIm<'a, In, Out, MarkOnce, NIO>
where
    NIO: Node<'a, In, Out = Out>,
{
    /// Compiles an immediate process
    pub(crate) fn compileIm(self, g: &mut Graph<'a>) -> NIO {
        self.0.compileIm(g)
    }
}


/// An non-immediate process.
///
/// This is simply a virtualization of an non-immediate process implementation.
#[cfg_attr(rustfmt, rustfmt_skip)]
pub struct ProcessNotIm<'a, In, Out, MarkOnce, NI, NO>(
    pub(crate) Box<IntProcessNotIm<'a, In, Out = Out, MarkOnce = MarkOnce, NI = NI, NO = NO>>
);


impl<'a, In: Val<'a>, Out: Val<'a>, MarkOnce, NI, NO> ProcessNotIm<'a, In, Out, MarkOnce, NI, NO>
where
    NI: Node<'a, In, Out = ()>,
    NO: Node<'a, (), Out = Out>,
{
/// Compiles a non-immediate process
    pub(crate) fn compile(self, g: &mut Graph<'a>) -> (NI, usize, NO) {
        self.0.compile(g)
    }
}

/// I cannot use the fact that a type T implemented IntProcessIm or IntProcessNotIm
/// to dispatch a function on it. so I need a third trait to say whether an implementation
/// should be boxed in ProcessIm or ProcessNotIm.
///
/// This trait should only be useful for types that are not leaves of the computation tree like
/// Seq, Choice, Loop, Par, Present, ...
pub trait ToBoxedProcess<'a, In: Val<'a>>: IntProcess<'a, In> + Sized {
    type Boxed: Process<'a, In, Out = Self::Out>;
    fn tobox(self) -> Self::Boxed;
}


//  ____                                _____          _ _
// |  _ \ _ __ ___   ___ ___  ___ ___  |_   _| __ __ _(_) |_
// | |_) | '__/ _ \ / __/ _ \/ __/ __|   | || '__/ _` | | __|
// |  __/| | | (_) | (_|  __/\__ \__ \   | || | | (_| | | |_
// |_|   |_|  \___/ \___\___||___/___/   |_||_|  \__,_|_|\__|


/// General trait for a process.
///
/// An end-user should only care about this trait.
/// This trait should only be implemented by ProcessIm and ProcessNotIm.
pub trait Process<'a, In: Val<'a>>: IntProcess<'a, In> + Sized {
    /// a.seq(b) execute a then b in correspond to pro!{a;b}
    ///
    /// The output value of a is given as input to b.
    fn seq<P>(self, p: P) -> <Seq<Self, P> as ToBoxedProcess<'a, In>>::Boxed
    where
        P: Process<'a, Self::Out>,
        Seq<Self, P>: ToBoxedProcess<'a, In>,
    {
        Seq(self, p).tobox()
    }


    /// a.choice(b) execute a or b depending on the input value.
    /// If we input True(x), a will run with input x.
    /// If we input False(x), b will run with input x.
    /// a and b must return the same type that will be return by the choice construct.
    /// a.choice(b) is equivalent to pro!{choice{a}{b}}
    fn choice<PF, InF: Val<'a>>(
        self,
        p: PF,
    ) -> <PChoice<Self, PF> as ToBoxedProcess<'a, ChoiceData<In, InF>>>::Boxed
    where
        PF: Process<'a, InF, Out = Self::Out>,
        PChoice<Self, PF>: ToBoxedProcess<'a, ChoiceData<In, InF>>,
    {
        PChoice(self, p).tobox()
    }

    /// a.ploop() needs a to return a ChoiceDate type.
    /// If a returns True(x), a is run again with x.
    /// else, the whole construct returns the value in False.
    /// a.ploop() is equivalent to pro!{loop{a}}
    fn ploop<Out: Val<'a>>(self) -> <PLoop<Self> as ToBoxedProcess<'a, In>>::Boxed
    where
        Self: Process<'a, In, Out = ChoiceData<In, Out>>,
        PLoop<Self>: ToBoxedProcess<'a, In>,
    {
        PLoop(self).tobox()
    }

    /// a.join(b) execute a and b in parallel on different input (the two members of the
    /// input pair), and returns the pair of results when both are finished.
    /// a.join(b) is equivalent to pro!{a || b}
    fn join<InQ: Val<'a>, Q>(self, q: Q) -> <Par<Self, Q> as ToBoxedProcess<'a, (In, InQ)>>::Boxed
    where
        Q: Process<'a, InQ>,
        Par<Self, Q>: ToBoxedProcess<'a, (In, InQ)>,
    {
        Par(self, q).tobox()
    }

    /// a.present(b) build a present (`PresentD`) construct with a and b.
    /// this is equivalent to pro!{present{a}{b}}
    fn present<PF, S: Signal<'a>>(
        self,
        process_false: PF,
    ) -> <PresentD<Self, PF> as ToBoxedProcess<'a, S>>::Boxed
    where
        PF: Process<'a, ()>,
        Self: Process<'a, ()>,
        PresentD<Self, PF>: ToBoxedProcess<'a, S>,
    {
        (PresentD {
             pt: self,
             pf: process_false,
         }).tobox()
    }
}

/// Puts a lot of processes in parallel, they can take a copy `In` value and must return ().
pub fn big_join<'a, In: Val<'a>, MarkOnce, PNI, PNO>(
    vp: Vec<ProcessNotIm<'a, In, (), MarkOnce, PNI, PNO>>,
) -> ProcessNotIm<'a, In, (), MarkOnce, NSeq<NStore<In>, NBigPar>, Nothing>
where
    PNI: Node<'a, In, Out = ()>,
    PNO: Node<'a, (), Out = ()>,
    In: Copy,
    MarkOnce: Once,
{
    let mut res = vec![];
    for p in vp {
        res.push(p);
    }
    ProcessNotIm(box BigPar(res))
}


/// this trait is implemented by something that be compiled into a full
/// Control Flow Graph.
pub trait GraphFiller<'a> : 'a {
    /// Compile Self to a `Graph` and return the index of starting `Node`
    fn compile_to_graph(self) -> (Graph<'a>, usize);
}

default impl<'a, T> GraphFiller<'a> for T
where
    T: Process<'a, (), Out = ()>,
{
    fn compile_to_graph(self) -> (Graph<'a>, usize){
        // only ProcessIm and ProcessNotIm implement Process
        unreachable!()
    }
}

//  ___ __  __ ____  _       ____              ___
// |_ _|  \/  |  _ \| |     |  _ \ _ __ ___   |_ _|_ __ ___
//  | || |\/| | |_) | |     | |_) | '__/ _ \   | || '_ ` _ \
//  | || |  | |  __/| |___  |  __/| | | (_) |  | || | | | | |
// |___|_|  |_|_|   |_____| |_|   |_|  \___/  |___|_| |_| |_|


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<'a, In: Val<'a>, Out: Val<'a>, MarkOnce, NIO> IntProcess<'a, In>
    for ProcessIm<'a, In, Out, MarkOnce, NIO>
where
    NIO: Node<'a, In, Out = Out>,
    MarkOnce: Once,
{
    type Out = Out;
    type MarkOnce = MarkOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        self.0.printDot(curNum)
    }
}


#[cfg_attr(rustfmt, rustfmt_skip)]
impl<'a, In: Val<'a>, Out: Val<'a>, MarkOnce, NIO> Process<'a, In>
    for ProcessIm<'a, In, Out, MarkOnce, NIO>
where
    NIO: Node<'a, In, Out = Out>,
    MarkOnce: Once,
{}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<'a, MarkOnce, NIO> GraphFiller<'a> for ProcessIm<'a, (), (), MarkOnce, NIO>
    where
    MarkOnce: Once,
    NIO: Node<'a, (), Out = ()>,
{
    fn compile_to_graph(self) -> (Graph<'a>, usize){
        let mut g = Graph::new();
        let pnio = self.compileIm(&mut g);
        let start = g.add(box node!(pnio >> NEnd{}));
        (g,start)
    }
}


//  ___ __  __ ____  _       ____              _   _ ___
// |_ _|  \/  |  _ \| |     |  _ \ _ __ ___   | \ | |_ _|
//  | || |\/| | |_) | |     | |_) | '__/ _ \  |  \| || |
//  | || |  | |  __/| |___  |  __/| | | (_) | | |\  || |
// |___|_|  |_|_|   |_____| |_|   |_|  \___/  |_| \_|___|

impl<'a, In: Val<'a>, Out: Val<'a>, MarkOnce, NI, NO> IntProcess<'a, In>
    for ProcessNotIm<'a, In, Out, MarkOnce, NI, NO>
    where
    NI: Node<'a, In, Out = ()>,
    NO: Node<'a, (), Out = Out>,
    MarkOnce: Once,
{
    type Out = Out;
    type MarkOnce = MarkOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        self.0.printDot(curNum)
    }
}


impl<'a, In: Val<'a>, Out: Val<'a>, MarkOnce, NI, NO> Process<'a, In>
    for ProcessNotIm<'a, In, Out, MarkOnce, NI, NO>
where
    MarkOnce: Once,
    NI: Node<'a, In, Out = ()>,
    NO: Node<'a, (), Out = Out>,
{
}

impl<'a, MarkOnce, NI, NO> GraphFiller<'a> for ProcessNotIm<'a, (), (), MarkOnce, NI, NO>
where
    NI: Node<'a, (), Out = ()>,
    NO: Node<'a, (), Out = ()>,
    MarkOnce: Once,
{
    fn compile_to_graph(self) -> (Graph<'a>, usize){
        let mut g = Graph::new();
        let (pni, pind, pno) = self.compile(&mut g);
        g.set(pind, box node!(pno >> NEnd{}));
        let start = g.add(box pni);
        (g,start)
    }
}


//  ____       _       _    ____                 _
// |  _ \ _ __(_)_ __ | |_ / ___|_ __ __ _ _ __ | |__
// | |_) | '__| | '_ \| __| |  _| '__/ _` | '_ \| '_ \
// |  __/| |  | | | | | |_| |_| | | | (_| | |_) | | | |
// |_|   |_|  |_|_| |_|\__|\____|_|  \__,_| .__/|_| |_|
//                                        |_|

/// Print a dot code of a nice graph representing the control flow at the process level.
pub fn print_graph<'a, P>(p: &mut P)
where
    P: Process<'a, (), Out = ()>,
{
    let mut val = 0;
    println!("digraph {{");
    p.printDot(&mut val);
    println!("}}");
}
