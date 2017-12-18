use node::*;
use engine::*;
use std::marker::PhantomData;
use signal::*;

mod base;
pub use self::base::*;
mod control;
pub use self::control::*;
mod seq;
pub use self::seq::*;
mod par;
pub use self::par::*;
mod signal;
pub use self::signal::*;

pub trait Is {
    type Value;
}

impl<T> Is for T {
    type Value = T;
}

pub struct NotIm {}
pub struct IsIm {}
pub trait Im: Sized {}
impl Im for NotIm {}
impl Im for IsIm {}


pub trait Process<'a, In: 'a>: 'a + Sized {
    type Out: 'a;
    type NI: Node<'a, In, Out = ()> + Sized;
    type NO: Node<'a, (), Out = Self::Out> + Sized;
    /// If mark is set to IsIm, compile panics, if it is NotIm, compileIm panics
    type Mark: Im;
    fn compile(self, _: &mut Graph<'a>) -> (Self::NI, usize, Self::NO)
    {
        unreachable!();
    }

    type NIO: Node<'a, In, Out = Self::Out>;
    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO
    {
        unreachable!();
    }

    fn seq<P>(self, p: P) -> Seq<MarkedProcess<Self, Self::Mark>, MarkedProcess<P, P::Mark>>
    where
        P: Process<'a, Self::Out>,
    {
        Seq {
            p: mp(self),
            q: mp(p),
        }
    }

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

    fn ploop(self) -> PLoop<MarkedProcess<Self, Self::Mark>>
    {
        PLoop { p: mp(self) }
    }

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

    fn pbox(self) -> Pbox<'a,In,Self::Out,Self::NI,Self::NO,Self::NIO,Self::Mark>{
        Pbox{p:box self}
    }
}

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


pub trait Graphfiller<'a> {
    fn fill_graph(self, g: &mut Graph<'a>) -> usize;
}

pub struct MarkedProcess<P, Mark: Im> {
    pub p: P,
    pd: PhantomData<Mark>,
}

impl<'a, P> Graphfiller<'a> for MarkedProcess<P, NotIm>
where
    P: Process<'a, (), Out = ()>,
{
    fn fill_graph(self, g: &mut Graph<'a>) -> usize {
        let (pni, pind, pno) = self.p.compile(g);
        g.set(pind, box pno);
        g.add(box pni)
    }
}

impl<'a, P> Graphfiller<'a> for MarkedProcess<P, IsIm>
where
    P: Process<'a, (), Out = ()>,
{
    fn fill_graph(self, g: &mut Graph<'a>) -> usize {
        let pnio = self.p.compileIm(g);
        g.add(box pnio)
    }
}

pub fn mp<'a, In: 'a, P>(p: P) -> MarkedProcess<P, P::Mark>
where
    P: Process<'a, In>,
{
    MarkedProcess {
        p: p,
        pd: PhantomData,
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
    fn compile_box(self: Box<Self>, _: &mut Graph<'a>) -> (Self::NI, usize, Self::NO);

    type NIO: Node<'a, In, Out = Self::Out>;
    fn compileIm_box(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO;
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
    fn compile_box(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO){
        (*self).compile(g)
    }
    fn compileIm_box(self: Box<Self>, g: &mut Graph<'a>) -> Self::NIO{
        (*self).compileIm(g)
    }
}

pub struct Pbox<'a, In, Out, NI, NO, NIO, Mark> {
    p: Box<ProcessBox<'a, In, Out = Out, NI = NI, NO = NO, NIO = NIO, Mark = Mark>>,
}

impl<'a, In: 'a, Out: 'a, NI, NO, NIO, Mark : Im + 'a> Process<'a, In>
    for Pbox<'a, In, Out, NI, NO, NIO, Mark> where
    NI : Node<'a,In,Out=()>,
    NO : Node<'a,(),Out=Out>,
    NIO : Node<'a,In,Out=Out>
{
    type Out = Out;
    type NI = NI;
    type NO = NO;
    type NIO = NIO;
    type Mark = Mark;
    fn compile(self, g: &mut Graph<'a>) -> (NI,usize,NO) {
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

pub struct ForceType<In,Out> {
    a : PhantomData<In>,
    b : PhantomData<Out>,
}

pub fn force_type<In,Out>() -> ForceType<In,Out>{
    ForceType{a:PhantomData, b :PhantomData}
}

impl<In,Out> ForceType<In,Out>{
    pub fn force<'a,P>(p :P) -> P
        where
        P: Process<'a,In,Out = Out>,
        In :'a,
        Out :'a
    {
        p
    }
}


