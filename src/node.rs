use std::marker::PhantomData;

use engine::*;

pub trait Node<'a, In: 'a>: 'a {
    type Out;
    fn call(&mut self, tasks: &mut Tasks, val: In) -> Self::Out;
    fn nseq<N2>(self, n2: N2) -> NSeq<Self, N2>
    where
        N2: Node<'a, Self::Out> + Sized,
        Self: Sized,
    {
        NSeq { n1: self, n2: n2 }
    }
}


/// Partial nodes during compilation
/// normally there is a In and Out such that
/// NI: Node<In,Out=()>
/// NO: Node<(),Out=Out>
/// NIO: Node<In,Out=Out>
pub enum PNode<NI, NO, NIO> {
    InOut(NIO),
    Halves(NI, usize, NO),
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
    fn call(&mut self, _: &mut Tasks, _: In) -> Out {
        panic!("Called empty node");
    }
}


//  _____      __  __       _
// |  ___| __ |  \/  |_   _| |_
// | |_ | '_ \| |\/| | | | | __|
// |  _|| | | | |  | | |_| | |_
// |_|  |_| |_|_|  |_|\__,_|\__|

impl<'a, F, In:'a, Out:'a> Node<'a, In> for F
where
    F: FnMut(In) -> Out + 'a,
{
    type Out = Out;
    fn call(&mut self, _: &mut Tasks, val: In) -> Out {
        self(val)
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
    fn call(&mut self, t: &mut Tasks, val: In) -> Out {
        let valm = self.n1.call(t, val);
        self.n2.call(t, valm)
    }
}
