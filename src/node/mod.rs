use std::marker::PhantomData;

use engine::*;

mod rcmanip;
pub use self::rcmanip::*;
mod control;
pub use self::control::*;
mod par;
pub use self::par::*;
mod signal;
pub use self::signal::*;

pub trait Node<'a, In: 'a>: 'a {
    type Out: 'a;
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: In) -> Self::Out;
    fn nseq<N2>(self, n2: N2) -> NSeq<Self, N2>
    where
        N2: Node<'a, Self::Out> + Sized,
        Self: Sized,
    {
        NSeq { n1: self, n2: n2 }
    }
    fn nseqd<N2>(self, n2: N2) -> NSeqD<'a,In,Self::Out,N2::Out>
        where
        N2: Node<'a, Self::Out> + Sized,
        Self: Sized,
    {
        NSeqD { n1: box self, n2: box n2 } 
    }

    fn alter<NF, In2: 'a>(self, nf: NF) -> NChoice<Self, NF>
    where
        NF: Node<'a, In2, Out = Self::Out> + Sized,
        Self: Sized,
    {
        NChoice { nt: self, nf: nf }
    }
    fn njoin<In2: 'a, N2>(self, n2: N2) -> NPar<Self, N2>
    where
        N2: Node<'a, In2> + Sized,
        Self: Sized,
    {
        NPar { n1: self, n2: n2 }
    }
}


#[derive(Clone, Copy)]
pub struct Nothing {}

#[allow(non_upper_case_globals)]
pub static Nothing: Nothing = Nothing {};

impl<'a> Node<'a, ()> for Nothing {
    type Out = ();
    fn call(&mut self, _: &mut SubRuntime<'a>, _val: ()) -> Self::Out {}
}


pub struct NIdentity {}

#[allow(non_upper_case_globals)]
pub static NIdentity: NIdentity = NIdentity {};

impl<'a, In: 'a> Node<'a, In> for NIdentity {
    type Out = In;

    fn call(&mut self, _: &mut SubRuntime<'a>, val: In) -> Self::Out {
        val
    }
}

//  _____                 _
// | ____|_ __ ___  _ __ | |_ _   _
// |  _| | '_ ` _ \| '_ \| __| | | |
// | |___| | | | | | |_) | |_| |_| |
// |_____|_| |_| |_| .__/ \__|\__, |
//                 |_|        |___/

pub struct DummyN<Out> {
    dummy: PhantomData<Out>,
}


impl<'a, In: 'a, Out: 'a> Node<'a, In> for DummyN<Out>
where
    Out: 'a,
{
    type Out = Out;
    fn call(&mut self, _: &mut SubRuntime<'a>, _: In) -> Out {
        panic!("Called empty node");
    }
}


//  _____      __  __       _
// |  ___| __ |  \/  |_   _| |_
// | |_ | '_ \| |\/| | | | | __|
// |  _|| | | | |  | | |_| | |_
// |_|  |_| |_|_|  |_|\__,_|\__|

pub struct FnMutN<F>(pub F);

impl<'a, F, In: 'a, Out: 'a> Node<'a, In> for FnMutN<F>
where
    F: FnMut(In) -> Out + 'a,
{
    type Out = Out;
    fn call(&mut self, _: &mut SubRuntime<'a>, val: In) -> Out {
        let &mut FnMutN(ref mut f) = self;
        f(val)
    }
}

//  ____
// / ___|  ___  __ _
// \___ \ / _ \/ _` |
//  ___) |  __/ (_| |
// |____/ \___|\__, |
//                |_|

pub struct NSeq<N1, N2> {
    n1: N1,
    n2: N2,
}

impl<'a, N1, N2, In: 'a, Mid: 'a, Out: 'a> Node<'a, In> for NSeq<N1, N2>
where
    N1: Node<'a, In, Out = Mid>,
    N2: Node<'a, Mid, Out = Out>,
{
    type Out = Out;
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: In) -> Out {
        let valm = self.n1.call(sub_runtime, val);
        self.n2.call(sub_runtime, valm)
    }
}

pub struct NSeqD<'a, In, Mid, Out> {
    n1: Box<Node<'a, In, Out = Mid>>,
    n2: Box<Node<'a, Mid, Out = Out>>,
}

impl<'a, In: 'a, Mid: 'a, Out: 'a> Node<'a, In> for NSeqD<'a,In, Mid, Out>
{
    type Out = Out;
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: In) -> Out {
        let valm = self.n1.call(sub_runtime, val);
        self.n2.call(sub_runtime, valm)
    }
}
