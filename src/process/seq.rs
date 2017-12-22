use node::*;
use super::*;


pub struct Seq<P, Q>
{
    pub(crate) p: P,
    pub(crate) q: Q,
}

impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> GProcess<'a, In>
    for Seq<P,Q>
    where
    P: GProcess<'a, In, Out = Mid>,
    Q: GProcess<'a, Mid, Out = Out>,
{
    type Out = Q::Out;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (beg,midup) = self.p.printDot(curNum);
        let (middown,end) = self.q.printDot(curNum);
        println!("{} -> {} [label = \" {} \"];",midup,middown,tname::<Mid>());
        (beg,end)
    }
}


impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> Process<'a, In>
    for Seq<MarkedProcess<P, NotIm>, MarkedProcess<Q, NotIm>>
where
    P: Process<'a, In, Out = Mid>,
    Q: Process<'a, Mid, Out = Out>,
{
    type NI = P::NI;
    type NO = Q::NO;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<P::MarkOnce, Q::MarkOnce>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (pni, pind, pno) = self.p.p.compile(g);
        let (qni, qind, qno) = self.q.p.compile(g);
        g.set(pind, box node!(pno >> qni));
        (pni, qind, qno)
    }
}

impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> Process<'a, In>
    for Seq<MarkedProcess<P, IsIm>, MarkedProcess<Q, NotIm>>
where
    P: Process<'a, In, Out = Mid>,
    Q: Process<'a, Mid, Out = Out>,
{
    type NI = NSeq<P::NIO, Q::NI>;
    type NO = Q::NO;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<P::MarkOnce, Q::MarkOnce>;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let pnio = self.p.p.compileIm(g);
        let (qni, qind, qno) = self.q.p.compile(g);
        (node!(pnio >> qni), qind, qno)

    }
}

impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> Process<'a, In>
    for Seq<MarkedProcess<P, NotIm>, MarkedProcess<Q, IsIm>>
where
    P: Process<'a, In, Out = Mid>,
    Q: Process<'a, Mid, Out = Out>,
{
    type NI = P::NI;
    type NO = NSeq<P::NO, Q::NIO>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<P::MarkOnce, Q::MarkOnce>;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (pni, pind, pno) = self.p.p.compile(g);
        let qnio = self.q.p.compileIm(g);
        (pni, pind, node!(pno >> qnio))

    }
}

impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> Process<'a, In>
    for Seq<MarkedProcess<P, IsIm>, MarkedProcess<Q, IsIm>>
where
    P: Process<'a, In, Out = Mid>,
    Q: Process<'a, Mid, Out = Out>,
{
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = NSeq<P::NIO, Q::NIO>;
    type Mark = IsIm;
    type MarkOnce = And<P::MarkOnce, Q::MarkOnce>;
    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        let pnio = self.p.p.compileIm(g);
        let qnio = self.q.p.compileIm(g);
        node!(pnio >> qnio)
    }
}
