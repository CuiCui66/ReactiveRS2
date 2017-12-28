use node::*;
use super::*;

pub struct Seq<P, Q>(pub(crate) P, pub(crate) Q);

impl<'a, P, Q, In: Val<'a>, Mid: Val<'a>, Out: Val<'a>> IntProcess<'a, In> for Seq<P,Q>
    where
    P: Process<'a, In, Out = Mid>,
    Q: Process<'a, Mid, Out = Out>,
{
    type Out = Out;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (beg,midup) = self.0.printDot(curNum);
        let (middown,end) = self.1.printDot(curNum);
        println!("{} -> {} [label = \"{}\"];",midup,middown,tname::<Mid>());
        (beg,end)
    }
}

// NI - NI
implNI!{
    In,
    impl<'a, In: Val<'a>, Mid: Val<'a>, Out: Val<'a>, PNI, PNO, QNI, QNO>
        for Seq<ProcessNotIm<'a, In, Mid, PNI, PNO>, ProcessNotIm<'a, Mid, Out, QNI, QNO>>
        where
        PNI: Node<'a, In, Out = ()>,
        PNO: Node<'a, (), Out = Mid>,
        QNI: Node<'a, Mid, Out = ()>,
        QNO: Node<'a, (), Out = Out>,

    trait IntProcessNotIm<'a, In>
    {
        type NI = PNI;
        type NO = QNO;
        fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let s = *self;
            let Seq(p, q) = s;
            let (pni, pind, pno) = p.compile(g);
            let (qni, qind, qno) = q.compile(g);
            g.set(pind, box node!(pno >> qni));
            (pni, qind, qno)
        }
    }
}

// Im - NI
implNI!{
    In,
    impl<'a, In: Val<'a>, Mid: Val<'a>, Out: Val<'a>, PNIO, QNI, QNO>
        for Seq<ProcessIm<'a, In, Mid, PNIO>, ProcessNotIm<'a, Mid, Out, QNI, QNO>>
        where
        PNIO: Node<'a, In, Out = Mid>,
        QNI: Node<'a, Mid, Out = ()>,
        QNO: Node<'a, (), Out = Out>,
    trait IntProcessNotIm<'a, In>
    {
        type NI = NSeq<PNIO, QNI>;
        type NO = QNO;
        fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let s = *self;
            let Seq(p, q) = s;
            let pnio = p.compileIm(g);
            let (qni, qind, qno) = q.compile(g);
            (node!(pnio >> qni), qind, qno)
        }

    }
}

// NI - Im
implNI!{
    In,
    impl<'a, In: Val<'a>, Mid: Val<'a>, Out: Val<'a>, PNI, PNO, QNIO>
        for Seq<ProcessNotIm<'a, In, Mid, PNI, PNO>, ProcessIm<'a, Mid, Out, QNIO>>
        where
        PNI: Node<'a, In, Out = ()>,
        PNO: Node<'a, (), Out = Mid>,
        QNIO: Node<'a, Mid, Out = Out>,
    trait IntProcessNotIm<'a, In>
    {
        type NI = PNI;
        type NO = NSeq<PNO, QNIO>;
        fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let s = *self;
            let Seq(p, q) = s;
            let (pni, pind, pno) = p.compile(g);
            let qnio = q.compileIm(g);
            (pni, pind, node!(pno >> qnio))
        }
    }
}

// Im - Im
implIm!{
    In,
    impl<'a, In: Val<'a>, Mid: Val<'a>, Out: Val<'a>, PNIO, QNIO>
        for Seq<ProcessIm<'a, In, Mid, PNIO>, ProcessIm<'a, Mid, Out, QNIO>>
        where
        PNIO: Node<'a, In, Out = Mid>,
        QNIO: Node<'a, Mid, Out = Out>,
    trait IntProcessIm<'a, In>
    {
        type NIO = NSeq<PNIO, QNIO>;
        fn compileIm(self: Box<Self>, g: &mut Graph<'a>) -> Self::NIO {
            let s = *self;
            let Seq(p, q) = s;
            let pnio = p.compileIm(g);
            let qnio = q.compileIm(g);
            node!(pnio >> qnio)
        }
    }
}
