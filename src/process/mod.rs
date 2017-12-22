use node::*;
use engine::*;
use std::marker::PhantomData;
use signal::*;
use std::intrinsics::type_name;

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

/// Contains signal related control structures.
mod signal;
#[doc(hidden)]
pub use self::signal::*;

// trait Is {
//     type Value;
// }

// impl<T> Is for T {
//     type Value = T;
// }



//  ___
// |_ _|_ __ ___
//  | || '_ ` _ \
//  | || | | | | |
// |___|_| |_| |_|




///[Im]: trait.Im.html
///[iI]: struct.IsIm.html
///[nI]: struct.NotIm.html

/// Mark trait : it marks if the a process is immediate or not.
///
/// Only [`IsIm`][iI] and [`NotIm`][nI] should implement this trait.
pub trait Im: Sized + 'static {}

/// Marker associated to [`Im`](trait.Im.html) : marks the process as non-immediate.
pub struct NotIm {}
impl Im for NotIm {}

/// Marker associated to [`Im`](trait.Im.html) : marks the process as immediate.
pub struct IsIm {}
impl Im for IsIm {}





//   ___
//  / _ \ _ __   ___ ___
// | | | | '_ \ / __/ _ \
// | |_| | | | | (_|  __/
//  \___/|_| |_|\___\___|



/// MarkOnce trait : it marks if the process can be executed multiple times or not.
///
/// A process marked by Once but not by NotOnce can only be executed one time.
/// A process marked by NotOnce can be executed multiple time.
pub trait Once: Sized + 'static {}
pub trait NotOnce: Once {}

/// ZST implementing Once. Used to mark process which can only be executed one time.
pub struct SOnce;
impl Once for SOnce {}

/// ZST implementing NotOnce.  Used to mark processes which can be executed multiple times.
pub struct SNotOnce;
impl Once for SNotOnce {}
impl NotOnce for SNotOnce {}

/// And represent the concatenation of two MarkOnce trait.
/// It is used to mark processes.
/// A process being marked by And can be executed multiple times only if both parts of
/// the And structures are marked by NotOnce.
pub struct And<O1, O2>(pub O1, pub O2);
impl<O1: Once, O2: Once> Once for And<O1, O2> {}
impl<O1: NotOnce, O2: NotOnce> NotOnce for And<O1, O2> {}




//  ____
// |  _ \ _ __ ___   ___ ___  ___ ___
// | |_) | '__/ _ \ / __/ _ \/ __/ __|
// |  __/| | | (_) | (_|  __/\__ \__ \
// |_|   |_|  \___/ \___\___||___/___/

pub trait GProcess<'a, In: 'a> : 'a {
    type Out :'a ;
    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize);
}



/// General trait for representing a reactive process.
///
/// [Im]:  trait.Im.html
/// [iI]:  struct.IsIm.html
/// [noI]: struct.NotIm.html
/// [NI]:  trait.Process.html#associatedtype.NI
/// [NO]:  trait.Process.html#associatedtype.NO
/// [NIO]: trait.Process.html#associatedtype.NIO
/// [c]:   trait.Process.html#method.compile
/// [cI]:  trait.Process.html#method.compileIm
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
/// As result the `Process` trait is a combination of two traits in one:
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
pub trait Process<'a, In: 'a>: Sized + GProcess<'a, In> {
    /// The output type of the process.
    //type Out: 'a;
    /// The input node when compiling in non-immediate mode
    type NI: Node<'a, In, Out = ()> + Sized;
    /// The output node when compiling in non-immediate mode
    type NO: Node<'a, (), Out = Self::Out> + Sized;

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
    type NIO: Node<'a, In, Out = Self::Out>;

    /// Compile an immediate process
    ///
    /// It just outputs a node that do the whole computation represented by the
    /// process in a single instant.
    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        unreachable!();
    }

    /// Print this process as a control flow graph in dot language on stdout.
    ///
    /// The input ref is the first unused node id.
    /// The two return value are the input and output of the graph representing the process.
    //fn printDot(&mut self,curNum : &mut usize) -> (usize,usize);
    /// Build the sequence of two process.
    fn seq<P>(self, p: P) -> Seq<MarkedProcess<Self, Self::Mark>, MarkedProcess<P, P::Mark>>
    where
        P: Process<'a, Self::Out>,
    {
        Seq {
            p: mp(self),
            q: mp(p),
        }
    }

    /// Builds a process taking a ChoiceData value and choosing the right process to call with it.
    fn choice<PF, InF: 'a>(
        self,
        p: PF,
    ) -> PChoice<MarkedProcess<Self, Self::Mark>, MarkedProcess<PF, PF::Mark>>
    where
        PF: Process<'a, InF, Out = Self::Out>,
    {
        PChoice {
            pt: mp(self),
            pf: mp(p),
        }

    }

    /// Builds the present construct which takes a signal, see PresentD.
    fn present<PF>(
        self,
        p: PF,
    ) -> PresentD<MarkedProcess<Self, Self::Mark>, MarkedProcess<PF, PF::Mark>>
    where
        PF: Process<'a, (), Out = Self::Out>,
    {
        PresentD {
            pt: mp(self),
            pf: mp(p),
        }
    }

    /// The process must returns a ChoiceData Value,
    fn ploop<ROut: 'a>(self) -> PLoop<MarkedProcess<Self, Self::Mark>>
    where
        Self: Process<'a, In, Out = ChoiceData<In, ROut>>,
    {
        PLoop { p: mp(self) }
    }

    /// Put two processes in parallel
    fn join<InQ: 'a, Q>(
        self,
        q: Q,
    ) -> Par<MarkedProcess<Self, Self::Mark>, MarkedProcess<Q, Q::Mark>>
    where
        Q: Process<'a, InQ> + Sized,
    {
        Par {
            p: mp(self),
            q: mp(q),
        }
    }

    /// Boxes a process to improve compile-time (rust compilation) performance.
    fn pbox(
        self,
    ) -> Pbox<'a, In, Self::Out, Self::NI, Self::NO, Self::NIO, Self::Mark, Self::MarkOnce> {
        Pbox { p: box self }
    }
}

/// Join N process in parallel, The input value must be `Copy` and the output value is `()`
pub fn big_join<'a, In: 'a, P>(vp: Vec<P>) -> BigPar<MarkedProcess<P, P::Mark>>
where
    P: Process<'a, In, Out = ()> + Sized,
    In: Copy,
{
    let mut res = vec![];
    for p in vp {
        res.push(mp(p));
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
pub struct MarkedProcess<P, Mark: Im> {
    pub p: P,
    pd: PhantomData<Mark>,
}

/// Marks a process with its [`Im`](trait.Im.html) tag
pub fn mp<'a, In: 'a, P>(p: P) -> MarkedProcess<P, P::Mark>
where
    P: Process<'a, In>,
{
    MarkedProcess {
        p: p,
        pd: PhantomData,
    }
}

impl<'a, In :'a, P, M> GProcess<'a, In> for MarkedProcess<P, M>
where
    P: Process<'a, In>,
    M: Im,
{
    type Out = P::Out;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        self.p.printDot(curNum)
    }
}

/// This is trait of an object that can be compiled to a control-flow [`Graph`](../engine/struct.Graph.html).
pub trait GraphFiller<'a> {
    fn fill_graph(self, g: &mut Graph<'a>) -> usize;
}

impl<'a, P, Mark> GraphFiller<'a> for MarkedProcess<P, Mark>
where
    P: Process<'a, (), Out = ()>,
    Mark: Im,
{
    default fn fill_graph(self, _: &mut Graph<'a>) -> usize {
        unreachable!();
    }
}


impl<'a, P> GraphFiller<'a> for MarkedProcess<P, NotIm>
where
    P: Process<'a, (), Out = ()>,
{
    fn fill_graph(self, g: &mut Graph<'a>) -> usize {
        let (pni, pind, pno) = self.p.compile(g);
        g.set(pind, box pno);
        g.add(box pni)
    }
}

impl<'a, P> GraphFiller<'a> for MarkedProcess<P, IsIm>
where
    P: Process<'a, (), Out = ()>,
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

pub trait ProcessBox<'a, In: 'a>: 'a {
    type Out: 'a;
    type NI: Node<'a, In, Out = ()> + Sized;
    type NO: Node<'a, (), Out = Self::Out> + Sized;
    /// If mark is set to IsIm, compile panics, if it is NotIm, compileIm panics
    type Mark: Im;

    type MarkOnce: Once;
    fn compile_box(self: Box<Self>, _: &mut Graph<'a>) -> (Self::NI, usize, Self::NO);

    type NIO: Node<'a, In, Out = Self::Out>;
    fn compileIm_box(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO;

    // pretty print
    fn printDot_box(self: &mut Self, curNum: &mut usize) -> (usize, usize);
}

impl<'a, In: 'a, P> ProcessBox<'a, In> for P
where
    P: Process<'a, In>,
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
    fn printDot_box(self: &mut Self, curNum: &mut usize) -> (usize, usize) {
        self.printDot(curNum)
    }
}

pub struct Pbox<'a, In, Out, NI, NO, NIO, Mark, MarkOnce> {
    p: Box<
        ProcessBox<
            'a,
            In,
            Out = Out,
            NI = NI,
            NO = NO,
            NIO = NIO,
            Mark = Mark,
            MarkOnce = MarkOnce,
        >,
    >,
}

impl<'a, In: 'a, Out: 'a, NI, NO, NIO, Mark: Im, MarkOnce: Once> GProcess<'a, In>
    for Pbox<'a, In, Out, NI, NO, NIO, Mark, MarkOnce>
    where
    NI: Node<'a, In, Out = ()>,
    NO: Node<'a, (), Out = Out>,
    NIO: Node<'a, In, Out = Out>,
{
    type Out = Out;
    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        self.p.printDot_box(curNum)
    }
}


impl<'a, In: 'a, Out: 'a, NI, NO, NIO, Mark: Im, MarkOnce: Once> Process<'a, In>
    for Pbox<'a, In, Out, NI, NO, NIO, Mark, MarkOnce>
where
    NI: Node<'a, In, Out = ()>,
    NO: Node<'a, (), Out = Out>,
    NIO: Node<'a, In, Out = Out>,
{
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
        P: Process<'a, In, Out = Out>,
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


fn tname<T>() -> String{
    let s = String::from(unsafe { type_name::<T>() });
    s.replace("ReactiveRS2::node::ChoiceData","CD")
}

pub fn print_graph<'a, P>(p: &'a mut P)
where
    P: GProcess<'a, (), Out = ()>,
{
    let mut val = 0;
    println!("digraph {{");
    p.printDot(&mut val);
    println!("}}");
}
