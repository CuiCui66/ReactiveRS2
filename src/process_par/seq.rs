use node::*;
use super::*;

impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> ProcessPar<'a, In>
    for Seq<MarkedProcessPar<P, NotIm>, MarkedProcessPar<Q, NotIm>>
where
    P: ProcessPar<'a, In, Out = Mid>,
    Q: ProcessPar<'a, Mid, Out = Out>,
    Out: Send + Sync,
{
    type NI = P::NI;
    type NO = Q::NO;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<P::MarkOnce, Q::MarkOnce>;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let (pni, pind, pno) = self.p.p.compile_par(g);
        let (qni, qind, qno) = self.q.p.compile_par(g);
        g.set(pind, box node!(pno >> qni));
        (pni, qind, qno)
    }
}

impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> ProcessPar<'a, In>
    for Seq<MarkedProcessPar<P, IsIm>, MarkedProcessPar<Q, NotIm>>
where
    P: ProcessPar<'a, In, Out = Mid>,
    Q: ProcessPar<'a, Mid, Out = Out>,
    Out: Send + Sync,
{
    type NI = NSeq<P::NIO, Q::NI>;
    type NO = Q::NO;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<P::MarkOnce, Q::MarkOnce>;
    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let pnio = self.p.p.compileIm_par(g);
        let (qni, qind, qno) = self.q.p.compile_par(g);
        (node!(pnio >> qni), qind, qno)

    }
}

impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> ProcessPar<'a, In>
    for Seq<MarkedProcessPar<P, NotIm>, MarkedProcessPar<Q, IsIm>>
where
    P: ProcessPar<'a, In, Out = Mid>,
    Q: ProcessPar<'a, Mid, Out = Out>,
    Out: Send + Sync,
{
    type NI = P::NI;
    type NO = NSeq<P::NO, Q::NIO>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<P::MarkOnce, Q::MarkOnce>;
    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let (pni, pind, pno) = self.p.p.compile_par(g);
        let qnio = self.q.p.compileIm_par(g);
        (pni, pind, node!(pno >> qnio))

    }
}

impl<'a, P, Q, In: 'a, Mid: 'a, Out: 'a> ProcessPar<'a, In>
    for Seq<MarkedProcessPar<P, IsIm>, MarkedProcessPar<Q, IsIm>>
where
    P: ProcessPar<'a, In, Out = Mid>,
    Q: ProcessPar<'a, Mid, Out = Out>,
    Out: Send + Sync,
{
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = NSeq<P::NIO, Q::NIO>;
    type Mark = IsIm;
    type MarkOnce = And<P::MarkOnce, Q::MarkOnce>;
    fn compileIm_par(self, g: &mut GraphPar<'a>) -> Self::NIO {
        let pnio = self.p.p.compileIm_par(g);
        let qnio = self.q.p.compileIm_par(g);
        node!(pnio >> qnio)
    }
}
