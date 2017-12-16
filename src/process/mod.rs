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
    fn compile(self, _: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        unreachable!();
    }

    type NIO: Node<'a, In, Out = Self::Out>;
    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
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

    fn ploop(self) -> PLoop<MarkedProcess<Self, Self::Mark>> {
        PLoop { p: mp(self) }
    }

    fn join<InQ: 'a, Q>(
        self,
        q: Q,
    ) -> Par<MarkedProcess<Self, Self::Mark>, MarkedProcess<Q, Q::Mark>>
    where
        Q: Process<'a, InQ> + Sized,
        Self: Sized,
    {
        Par {
            p: mp(self),
            q: mp(q),
        }
    }
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
