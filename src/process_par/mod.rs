use node::*;
use engine::*;
use std::marker::PhantomData;
use signal::*;
pub use std::intrinsics::type_name;
pub use process::*;


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
pub use self::seq::*;

/// Contains parallel structures i.e `||` and `BigPar`
mod par;
#[doc(hidden)]
pub use self::par::*;

/*
/// Contains signal related control structures.
mod signal;
#[doc(hidden)]
pub use self::signal::*;*/

/// General trait for representing a reactive process.
///
/// [Im]:  trait.Im.html
/// [iI]:  struct.IsIm.html
/// [noI]: struct.NotIm.html
/// [NI]:  trait.ProcessPar.html#associatedtype.NI
/// [NO]:  trait.ProcessPar.html#associatedtype.NO
/// [NIO]: trait.ProcessPar.html#associatedtype.NIO
/// [c]:   trait.ProcessPar.html#method.compile
/// [cI]:  trait.ProcessPar.html#method.compileIm
/// [Gr]:  ../engine/struct.Graph.html
///
/// A process represent any reactive calculation that takes an input value of type `In`
/// and outputs a value of type `Out`. `()` is used to represent a no-value.
/// A process when entering a Runtime, will be compiled to control-flow graph of Nodes.
/// A process may represent a computation that spans on several instant or just a
/// single instant :
///
/// There are to kinds of process: immediate and not-immediate.
/// Because it is not possible to have the same static interface for both, I should have used two
/// different traits but rust does not support overloading, so I need a single trait.
/// As result the `ProcessPar` trait is a combination of two traits in one:
///
/// * If `Mark` is set to [`IsIm`][iI], the process is immediate, the compilation method is
///   [`compileIm`][cI] which outputs a single Node of type [`NIO`][NIO].
///   The type [`NI`][NI] and [`NO`][NO] have dummy values and [`compile`][c] will crash if called.
///
/// * If `Mark` is set to [`NotIm`][noI], the process is not immediate, the compilation method is
///   [`compile`][c] which outputs an input Node of type [`NI`][NI],
///   and an output Node of type [`NO`][NO]
///   along with its id (see [`Graph`][Gr]).
///   The type [`NIO`][NIO] has a dummy value and [`compileIm`][cI] will crash if called.
///
///Any other method will work in both cases.
pub trait ProcessPar<'a, In: 'a>: 'a + Sized{
    /// The output type of the process.
    type Out: 'a;

    /// The input node when compiling in non-immediate mode
    type NI: Node<'a, In, Out = ()> + Sized + Send + Sync;
    /// The output node when compiling in non-immediate mode
    type NO: Node<'a, (), Out = Self::Out> + Sized + Send + Sync;

    /// Determines the type of the process: immediate or not.
    type Mark: Im;

    /// Determines if the process can only be called one time
    type MarkOnce: Once;

    /// Compile a non-immediate process.
    ///
    /// This method is required if `Mark` = `NotIm`.
    /// The process may be compiled to an arbitrary control-flow graph, which is set directly
    /// in the mutable variable recieved as input.
    /// `compile` only outputs both ends of the process's graph:
    ///
    /// * The input node which must be fed with the input value of the process, it will
    ///   than probably call by id any node in the graph.
    /// * The output node is given with an id. It will give the output value of the process
    ///   when called during the normal execution of the runtime (i.e after another node has put
    ///   its id in the runtime). This node must be placed (after adding other stuff behind) in
    ///   the id slot given as middle value so other node can reference it.
    fn compile(self, _: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        unreachable!();
    }

    ///The Input-Output node when compiling in immediate mode.
    type NIO: Node<'a, In, Out = Self::Out> + Send + Sync;

    /// Compile an immediate process
    ///
    /// It just outputs a node that do the whole computation represented by the process in a single instant.
    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        unreachable!();
    }

    /// Print this process as a control flow graph in dot language on stdout.
    ///
    /// The input ref is the first unused node id.
    /// The two return value are the input and output of the graph representing the process.
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize);


    /// Build the sequence of two process.
    fn seq<P>(self, p: P) -> Seq<MarkedProcessPar<Self, Self::Mark>, MarkedProcessPar<P, P::Mark>>
    where
        P: ProcessPar<'a, Self::Out>,
    {
        Seq {
            p: mp_par(self),
            q: mp_par(p),
        }
    }

    /// Builds a process taking a ChoiceData value and choosing the right process to call with it.
    fn choice<PF, InF: 'a>(
        self,
        p: PF,
    ) -> PChoice<MarkedProcessPar<Self, Self::Mark>, MarkedProcessPar<PF, PF::Mark>>
    where
        PF: ProcessPar<'a, InF, Out = Self::Out>,
    {
        PChoice {
            pt: mp_par(self),
            pf: mp_par(p),
        }

    }

    /*
    /// Builds the present construct which takes a signal, see PresentD.
    fn present<PF>(
        self,
        p: PF,
    ) -> PresentD<MarkedProcessPar<Self, Self::Mark>, MarkedProcessPar<PF, PF::Mark>>
    where
        PF: ProcessPar<'a, (), Out = Self::Out>,
    {
        PresentD {
            pt: mp_par(self),
            pf: mp_par(p),
        }
    }*/

    /// The process must returns a ChoiceData Value,
    fn ploop<ROut>(self) -> PLoop<MarkedProcessPar<Self, Self::Mark>>
        where
        Self: ProcessPar<'a, In,Out = ChoiceData<In,ROut>>
    {
        PLoop { p: mp_par(self) }
    }

    /// Put two processes in parallel
    fn join<InQ: 'a, Q>(
        self,
        q: Q,
    ) -> Par<MarkedProcessPar<Self, Self::Mark>, MarkedProcessPar<Q, Q::Mark>>
    where
        Q: ProcessPar<'a, InQ> + Sized,
    {
        Par {
            p: mp_par(self),
            q: mp_par(q),
        }
    }

    /// Boxes a process to improve compile-time (rust compilation) performance.
    fn pbox_par(self) -> PboxPar<'a, In, Self::Out, Self::NI, Self::NO, Self::NIO, Self::Mark, Self::MarkOnce> {
        PboxPar { p: box self }
    }
}

/// Join N process in parallel, The input value must be `Copy` and the output value is `()`
pub fn big_join<'a, In: 'a, P>(vp: Vec<P>) -> BigPar<MarkedProcessPar<P, P::Mark>>
where
    P: ProcessPar<'a, In, Out = ()> + Sized,
    In: Copy,
{
    let mut res = vec![];
    for p in vp {
        res.push(mp_par(p));
    }
    BigPar { vp: res }
}



//  _____         _           _           _
// |_   _|__  ___| |__  _ __ (_) ___ __ _| |
//   | |/ _ \/ __| '_ \| '_ \| |/ __/ _` | |
//   | |  __/ (__| | | | | | | | (_| (_| | |
//   |_|\___|\___|_| |_|_| |_|_|\___\__,_|_|


/// This type is to mark a process with [`IsIm`](struct.IsIm.html) or `NotIm`(struct.NotIm.html).
///
/// I've introduced it to do type dispatch on processes:
/// nearly all structures that should take a process, take a marked process instead to know how to use it.
pub struct MarkedProcessPar<P, Mark: Im> {
    pub p: P,
    pd: PhantomData<Mark>,
}

/// Marks a process with its [`Im`](trait.Im.html) tag
pub fn mp_par<'a, In: 'a, P>(p: P) -> MarkedProcessPar<P, P::Mark>
    where
    P: ProcessPar<'a, In>,
{
    MarkedProcessPar {
        p: p,
        pd: PhantomData,
    }
}

impl<'a, P, Mark> GraphFiller<'a> for MarkedProcessPar<P, Mark>
where
    P: ProcessPar<'a, (), Out = ()>,
    Mark: Im,
{
    default fn fill_graph(self, _: &mut Graph<'a>) -> usize {
        unreachable!();
    }
}


impl<'a, P> GraphFiller<'a> for MarkedProcessPar<P, NotIm>
where
    P: ProcessPar<'a, (), Out = ()>,
{
    fn fill_graph(self, g: &mut Graph<'a>) -> usize {
        let (pni, pind, pno) = self.p.compile(g);
        g.set(pind, box pno);
        g.add(box pni)
    }
}

impl<'a, P> GraphFiller<'a> for MarkedProcessPar<P, IsIm>
where
    P: ProcessPar<'a, (), Out = ()>,
{
    fn fill_graph(self, g: &mut Graph<'a>) -> usize {
        let pnio = self.p.compileIm(g);
        g.add(box pnio)
    }
}


//  ____  ____
// |  _ \| __ )  _____  __
// | |_) |  _ \ / _ \ \/ /
// |  __/| |_) | (_) >  <
// |_|   |____/ \___/_/\_\

pub trait ProcessBoxPar<'a, In: 'a>: 'a{
    type Out: 'a;
    type NI: Node<'a, In, Out = ()> + Sized + Send + Sync;
    type NO: Node<'a, (), Out = Self::Out> + Sized + Send + Sync;
    /// If mark is set to IsIm, compile panics, if it is NotIm, compileIm panics
    type Mark: Im;

    type MarkOnce: Once;
    fn compile_box(self: Box<Self>, _: &mut Graph<'a>) -> (Self::NI, usize, Self::NO);

    type NIO: Node<'a, In, Out = Self::Out>;
    fn compileIm_box(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO;

    // pretty print
    fn printDot_box(self: &mut Self, curNum : &mut usize) -> (usize,usize);

}

impl<'a, In: 'a, P> ProcessBoxPar<'a, In> for P
where
    P: ProcessPar<'a, In>,
{
    type Out = P::Out;
    type NI = P::NI;
    type NO = P::NO;
    type NIO = P::NIO;
    type Mark = P::Mark;
    type MarkOnce = P::MarkOnce;

    fn compile_box(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        (*self).compile(g)
    }
    fn compileIm_box(self: Box<Self>, g: &mut Graph<'a>) -> Self::NIO {
        (*self).compileIm(g)
    }
    fn printDot_box(self: &mut Self, curNum : &mut usize) -> (usize,usize){
        self.printDot(curNum)
    }
}

pub struct PboxPar<'a, In, Out, NI, NO, NIO, Mark, MarkOnce> {
    p: Box<ProcessBoxPar<'a, In, Out = Out, NI = NI, NO = NO, NIO = NIO, Mark = Mark, MarkOnce = MarkOnce>>,
}

impl<'a, In: 'a, Out: 'a, NI, NO, NIO, Mark: Im + 'a, MarkOnce: Once + 'a> ProcessPar<'a, In>
    for PboxPar<'a, In, Out, NI, NO, NIO, Mark, MarkOnce>
where
    NI: Node<'a, In, Out = ()> + Send + Sync,
    NO: Node<'a, (), Out = Out> + Send + Sync,
    NIO: Node<'a, In, Out = Out> + Send + Sync,
{
    type Out = Out;
    type NI = NI;
    type NO = NO;
    type NIO = NIO;
    type Mark = Mark;
    type MarkOnce = MarkOnce;

    fn compile(self, g: &mut Graph<'a>) -> (NI, usize, NO) {
        self.p.compile_box(g)
    }
    fn compileIm(self, g: &mut Graph<'a>) -> NIO {
        self.p.compileIm_box(g)
    }
    fn printDot(&mut self, curNum : &mut usize) -> (usize,usize){
        self.p.printDot_box(curNum)
    }
}



//  _____                 _____
// |  ___|__  _ __ ___ __|_   _|   _ _ __   ___
// | |_ / _ \| '__/ __/ _ \| || | | | '_ \ / _ \
// |  _| (_) | | | (_|  __/| || |_| | |_) |  __/
// |_|  \___/|_|  \___\___||_| \__, | .__/ \___|
//                             |___/|_|

pub struct ForceType<In, Out> {
    a: PhantomData<In>,
    b: PhantomData<Out>,
}

pub fn force_type<In, Out>() -> ForceType<In, Out> {
    ForceType {
        a: PhantomData,
        b: PhantomData,
    }
}

impl<In, Out> ForceType<In, Out> {
    pub fn force<'a, P>(p: P) -> P
    where
        P: ProcessPar<'a, In, Out = Out>,
        In: 'a,
        Out: 'a,
    {
        p
    }
}


//  ____       _       _    ____                 _
// |  _ \ _ __(_)_ __ | |_ / ___|_ __ __ _ _ __ | |__
// | |_) | '__| | '_ \| __| |  _| '__/ _` | '_ \| '_ \
// |  __/| |  | | | | | |_| |_| | | | (_| | |_) | | | |
// |_|   |_|  |_|_| |_|\__|\____|_|  \__,_| .__/|_| |_|
//                                        |_|

fn tname<T>()-> &'static str{
    unsafe{type_name::<T>()}
}

pub fn print_graph_par<'a,P>(p : &'a mut P)
    where
    P : ProcessPar<'a,(),Out =()>
{
    let mut val = 0;
    println!("digraph {{");
    p.printDot(&mut val);
    println!("}}");
}
