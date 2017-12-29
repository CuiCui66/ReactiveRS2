//! This module defines the global mechanism of processes.
//!
//! A process represents any reactive calculation that takes an input value
//! and outputs a value. `()` is used to represent a no-value.
//! A process, when entering a Runtime, will be compiled to control-flow graph (CFG) of Nodes.
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




use node::*;
use engine::*;
use signal::*;
pub(crate) use tname::*;
use super::*;

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
use self::seq::*;


/// Contains parallel structures i.e `||` and `BigPar`
mod par;
#[doc(hidden)]
pub use self::par::*;

/// Contains signal related control structures.
mod signal;
#[doc(hidden)]
pub use self::signal::*;






/// Common interface for processes and process implementations.
pub trait IntProcess<'a, In: Val<'a>>: 'a {
    /// The type outputted by the process when In is given.
    type Out: Val<'a>;

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

/// An immediate process.
///
/// This is simply a virtualization of an immediate process implementation.
pub struct ProcessIm<'a, In, Out, NIO>(pub(crate) Box<IntProcessIm<'a, In, Out = Out, NIO = NIO>>);

impl<'a, In: Val<'a>, Out: Val<'a>, NIO> ProcessIm<'a, In, Out, NIO>
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
pub struct ProcessNotIm<'a, In, Out, NI, NO>(
    pub(crate) Box<IntProcessNotIm<'a, In, Out = Out, NI = NI, NO = NO>>
);


impl<'a, In: Val<'a>, Out: Val<'a>, NI, NO> ProcessNotIm<'a, In, Out, NI, NO>
where
    NI: Node<'a, In, Out = ()>,
    NO: Node<'a, (), Out = Out>,
{
    /// Compiles a non-immediate process
    pub(crate) fn compile(self, g: &mut Graph<'a>) -> (NI, usize, NO) {
        self.0.compile(g)
    }
}

//     _         _          ____            _
//    / \  _   _| |_ ___   | __ )  _____  _(_)_ __   __ _
//   / _ \| | | | __/ _ \  |  _ \ / _ \ \/ / | '_ \ / _` |
//  / ___ \ |_| | || (_) | | |_) | (_) >  <| | | | | (_| |
// /_/   \_\__,_|\__\___/  |____/ \___/_/\_\_|_| |_|\__, |
//                                                  |___/

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
    fn choice<PF, InF : Val<'a>>(
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

    fn present<PF: Val<'a>, S: Signal<'a>>(self, process_false: PF) -> <PresentD<Self, PF> as ToBoxedProcess<'a, S>>::Boxed
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

/// puts a lot of processes in parallel.
pub fn big_join<'a, In: Val<'a>, PNI, PNO>(
    vp: Vec<ProcessNotIm<'a, In, (), PNI, PNO>>,
) -> ProcessNotIm<'a, In, (), NSeq<NStore<In>, NBigPar>, Nothing>
where
    PNI: Node<'a, In, Out = ()>,
    PNO: Node<'a, (), Out = ()>,
    In: Copy,
{
    let mut res = vec![];
    for p in vp {
        res.push(p);
    }
    ProcessNotIm(box BigPar(res))
}

pub trait GraphFiller<'a> {
    fn fill_graph(self, g: &mut Graph<'a>) -> usize;
}

impl<'a, T> GraphFiller<'a> for T
where
    T: Process<'a, (), Out = ()>,
{
    default fn fill_graph(self, _: &mut Graph<'a>) -> usize {
        unreachable!()
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<'a, In: Val<'a>, Out: Val<'a>, NIO> IntProcess<'a, In> for ProcessIm<'a, In, Out, NIO>
where
    NIO: Node<'a, In, Out = Out>,
{
    type Out = Out;
    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        self.0.printDot(curNum)
    }
}

impl<'a, In: Val<'a>, Out: Val<'a>, NI, NO> IntProcess<'a, In> for ProcessNotIm<'a, In, Out, NI, NO>
where
    NI: Node<'a, In, Out = ()>,
    NO: Node<'a, (), Out = Out>,
{
    type Out = Out;
    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        self.0.printDot(curNum)
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<'a, In: Val<'a>, Out: Val<'a>, NIO> Process<'a, In> for ProcessIm<'a, In, Out, NIO>
where
    NIO: Node<'a, In, Out = Out>,
{}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<'a, NIO> GraphFiller<'a> for ProcessIm<'a, (), (), NIO>
    where
    NIO: Node<'a, (), Out = ()>,
{
    fn fill_graph(self, g: &mut Graph<'a>) ->usize
    {
        let pnio = self.compileIm(g);
        g.add(box pnio)
    }
}

impl<'a, In: Val<'a>, Out: Val<'a>, NI,NO> Process<'a, In> for ProcessNotIm<'a, In, Out, NI,NO>
    where
    NI: Node<'a, In, Out = ()>,
    NO: Node<'a, (), Out = Out>,
{}

impl<'a, NI, NO> GraphFiller<'a> for ProcessNotIm<'a, (), (), NI, NO>
where
    NI: Node<'a, (), Out = ()>,
    NO: Node<'a, (), Out = ()>,
{
    fn fill_graph(self, g: &mut Graph<'a>) -> usize {
        let (pni, pind, pno) = self.compile(g);
        g.set(pind, box pno);
        g.add(box pni)
    }
}


//  ____       _       _    ____                 _
// |  _ \ _ __(_)_ __ | |_ / ___|_ __ __ _ _ __ | |__
// | |_) | '__| | '_ \| __| |  _| '__/ _` | '_ \| '_ \
// |  __/| |  | | | | | |_| |_| | | | (_| | |_) | | | |
// |_|   |_|  |_|_| |_|\__|\____|_|  \__,_| .__/|_| |_|
//                                        |_|

pub fn print_graph<'a, P>(p: &'a mut P)
where
    P: Process<'a, (), Out = ()>,
{
    let mut val = 0;
    println!("digraph {{");
    p.printDot(&mut val);
    println!("}}");
}
