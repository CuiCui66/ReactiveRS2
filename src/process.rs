use node::*;
use engine::*;


pub trait Process<'a, In>
where
    Self::NI: Node<'a, In, Out = ()>,
    Self::NO: Node<'a, (), Out = Self::Out>,
    Self::NIO: Node<'a, In, Out = Self::Out>,
{
    type Out;
    type NI;
    type NO;
    type NIO;
    fn compile(self, runtime: &mut Graph) -> PNode<Self::NI, Self::NO, Self::NIO>;
}

//  _____      __  __       _
// |  ___| __ |  \/  |_   _| |_
// | |_ | '_ \| |\/| | | | | __|
// |  _|| | | | |  | | |_| | |_
// |_|  |_| |_|_|  |_|\__,_|\__|

impl<'a, F, In, Out> Process<'a, In> for F
where
    F: FnMut(In) -> Out + 'a,
    Out: 'a,
{
    type Out = Out;
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = F;
    fn compile(self, _: &mut Graph) -> PNode<Self::NI, Self::NO, Self::NIO> {
        PNode::InOut(self)
    }
}
