use node::*;
use super::*;


pub struct Seq<P, Q>
{
    pub(crate) p: P,
    pub(crate) q: Q,
}

impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> Process<'a, In>
    for Seq<MarkedProcess<P, NotIm>, MarkedProcess<Q, NotIm>>
where
    P: Process<'a, In, Out = Mid>,
    Q: Process<'a, Mid, Out = Out>,
{
    type Out = Q::Out;
    type NI = P::NI;
    type NO = Q::NO;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (pni, pind, pno) = self.p.p.compile(g);
        let (qni, qind, qno) = self.q.p.compile(g);
        g.set(pind, box node!(pno >> qni));
        (pni, qind, qno)
    }
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (beg,midup) = self.p.p.printDot(curNum);
        let (middown,end) = self.q.p.printDot(curNum);
        println!("{} -> {} [label = \"{}\"];",midup,middown,tname::<Mid>());
        (beg,end)
    }
}

impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> Process<'a, In>
    for Seq<MarkedProcess<P, IsIm>, MarkedProcess<Q, NotIm>>
where
    P: Process<'a, In, Out = Mid>,
    Q: Process<'a, Mid, Out = Out>,
{
    type Out = Q::Out;
    type NI = NSeq<P::NIO, Q::NI>;
    type NO = Q::NO;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let pnio = self.p.p.compileIm(g);
        let (qni, qind, qno) = self.q.p.compile(g);
        (node!(pnio >> qni), qind, qno)

    }
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (beg,midup) = self.p.p.printDot(curNum);
        let (middown,end) = self.q.p.printDot(curNum);
        println!("{} -> {} [label = \"{}\"];",midup,middown,tname::<Mid>());
        (beg,end)
    }
}

impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> Process<'a, In>
    for Seq<MarkedProcess<P, NotIm>, MarkedProcess<Q, IsIm>>
where
    P: Process<'a, In, Out = Mid>,
    Q: Process<'a, Mid, Out = Out>,
{
    type Out = Q::Out;
    type NI = P::NI;
    type NO = NSeq<P::NO, Q::NIO>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (pni, pind, pno) = self.p.p.compile(g);
        let qnio = self.q.p.compileIm(g);
        (pni, pind, node!(pno >> qnio))

    }
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (beg,midup) = self.p.p.printDot(curNum);
        let (middown,end) = self.q.p.printDot(curNum);
        println!("{} -> {} [label = \"{}\"];",midup,middown,tname::<Mid>());
        (beg,end)
    }
}

impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> Process<'a, In>
    for Seq<MarkedProcess<P, IsIm>, MarkedProcess<Q, IsIm>>
where
    P: Process<'a, In, Out = Mid>,
    Q: Process<'a, Mid, Out = Out>,
{
    type Out = Q::Out;
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = NSeq<P::NIO, Q::NIO>;
    type Mark = IsIm;
    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        let pnio = self.p.p.compileIm(g);
        let qnio = self.q.p.compileIm(g);
        node!(pnio >> qnio)
    }
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (beg,midup) = self.p.p.printDot(curNum);
        let (middown,end) = self.q.p.printDot(curNum);
        println!("{} -> {} [label = \"{}\"];",midup,middown,tname::<Mid>());
        (beg,end)
    }
}
