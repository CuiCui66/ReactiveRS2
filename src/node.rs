use std::marker::PhantomData;

use engine::*;

pub trait Node<'a, In>: 'a {
    type Out;
    fn call(&mut self, tasks: &mut Tasks, val: In) -> Self::Out;
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

impl<'a, In, Out> Node<'a, In> for DummyN<Out>
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

impl<'a, F, In, Out> Node<'a, In> for F
where
    F: FnMut(In) -> Out + 'a,
{
    type Out = Out;
    fn call(&mut self, _: &mut Tasks, val: In) -> Out {
        self(val)
    }
}
