use node::*;
use engine::*;
//use signal::*;
pub use std::intrinsics::type_name;

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

/*
/// Contains signal related control structures.
mod signal;
#[doc(hidden)]
pub use self::signal::*;
*/

pub trait IntProcess<'a, In: 'a>: 'a {
    type Out: 'a;
    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize);
}

pub trait IntProcessIm<'a, In: 'a>: IntProcess<'a, In> {
    type NIO: Node<'a, In, Out = Self::Out>;
    fn compileIm(self: Box<Self>, g: &mut Graph<'a>) -> Self::NIO;
}

pub trait IntProcessNotIm<'a, In: 'a>: IntProcess<'a, In> {
    type NI: Node<'a, In, Out = ()>;
    type NO: Node<'a, (), Out = Self::Out>;
    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO);
}

pub struct ProcessIm<'a, In, Out, NIO>(pub(crate) Box<IntProcessIm<'a, In, Out = Out, NIO = NIO>>);

impl<'a, In: 'a, Out: 'a, NIO> ProcessIm<'a, In, Out, NIO>
where
    NIO: Node<'a, In, Out = Out>,
{
    pub(crate) fn compileIm(self, g: &mut Graph<'a>) -> NIO {
        self.0.compileIm(g)
    }
}


#[cfg_attr(rustfmt, rustfmt_skip)]
pub struct ProcessNotIm<'a, In, Out, NI, NO>(
    pub(crate) Box<IntProcessNotIm<'a, In, Out = Out, NI = NI, NO = NO>>
);


impl<'a, In: 'a, Out: 'a, NI, NO> ProcessNotIm<'a, In, Out, NI, NO>
where
    NI: Node<'a, In, Out = ()>,
    NO: Node<'a, (), Out = Out>,
{
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

pub trait ToBoxedProcess<'a, In: 'a>: IntProcess<'a, In> + Sized {
    type Boxed: Process<'a, In, Out = Self::Out>;
    fn tobox(self) -> Self::Boxed;
}


//  ____                                _____          _ _
// |  _ \ _ __ ___   ___ ___  ___ ___  |_   _| __ __ _(_) |_
// | |_) | '__/ _ \ / __/ _ \/ __/ __|   | || '__/ _` | | __|
// |  __/| | | (_) | (_|  __/\__ \__ \   | || | | (_| | | |_
// |_|   |_|  \___/ \___\___||___/___/   |_||_|  \__,_|_|\__|

// pub trait Same<T> {}
// impl<T> Same<T> for T {}

pub trait Process<'a, In: 'a>: IntProcess<'a, In> + Sized {
    fn seq<P>(self, p: P) -> <Seq<Self, P> as ToBoxedProcess<'a, In>>::Boxed
    where
        P: Process<'a, Self::Out>,
        Seq<Self, P>: ToBoxedProcess<'a, In>,
    {
        Seq(self, p).tobox()
    }

    fn choice<PF, InF>(
        self,
        p: PF,
    ) -> <PChoice<Self, PF> as ToBoxedProcess<'a, ChoiceData<In, InF>>>::Boxed
    where
        PF: Process<'a, InF, Out = Self::Out>,
        PChoice<Self, PF>: ToBoxedProcess<'a, ChoiceData<In, InF>>,
    {
        PChoice(self, p).tobox()
    }

    fn ploop<Out: 'a>(self) -> <PLoop<Self> as ToBoxedProcess<'a, In>>::Boxed
    where
        Self: Process<'a, In, Out = ChoiceData<In, Out>>,
        PLoop<Self>: ToBoxedProcess<'a, In>,
    {
        PLoop(self).tobox()
    }

    fn join<InQ: 'a, Q>(self, q: Q) -> <Par<Self, Q> as ToBoxedProcess<'a, (In, InQ)>>::Boxed
    where
        Q: Process<'a, InQ>,
        Par<Self, Q>: ToBoxedProcess<'a, (In, InQ)>,
    {
        Par(self, q).tobox()
    }
}

pub fn big_join<'a, In: 'a, PNI, PNO>(
    vp: Vec<ProcessNotIm<'a, In, (), PNI, PNO>>,
) -> ProcessNotIm<'a, In, (), NSeq<RcStore<In>, NBigPar>, Nothing>
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
impl<'a, In: 'a, Out: 'a, NIO> IntProcess<'a, In> for ProcessIm<'a, In, Out, NIO>
where
    NIO: Node<'a, In, Out = Out>,
{
    type Out = Out;
    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        self.0.printDot(curNum)
    }
}

impl<'a, In: 'a, Out: 'a, NI, NO> IntProcess<'a, In> for ProcessNotIm<'a, In, Out, NI, NO>
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
impl<'a, In: 'a, Out: 'a, NIO> Process<'a, In> for ProcessIm<'a, In, Out, NIO>
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

impl<'a, In: 'a, Out: 'a, NI,NO> Process<'a, In> for ProcessNotIm<'a, In, Out, NI,NO>
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


fn tname<T>() -> &'static str {
    unsafe { type_name::<T>() }
}

pub fn print_graph<'a, P>(p: &'a mut P)
where
    P: Process<'a, (), Out = ()>,
{
    let mut val = 0;
    println!("digraph {{");
    p.printDot(&mut val);
    println!("}}");
}
