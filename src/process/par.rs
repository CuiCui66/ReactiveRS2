use node::*;
use super::*;

/// A process implementation that put two processes in parallel.
///
/// It wait that both processes have finished before continuing.
/// It takes a pair of value (a,b) a returns (P(a),Q(b)).
pub struct Par<P, Q>(pub(crate) P, pub(crate) Q);

impl<'a, P, Q, InP: Val<'a>, InQ: Val<'a>, OutP: Val<'a>, OutQ: Val<'a>> IntProcess<'a, (InP, InQ)>
    for Par<P, Q>
where
    P: Process<'a, InP, Out = OutP>,
    Q: Process<'a, InQ, Out = OutQ>,
{
    type Out = (OutP, OutQ);
    type MarkOnce = <And<P::MarkOnce, Q::MarkOnce> as GiveOnce>::Once;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let (begp, endp) = self.0.printDot(curNum);
        let (begq, endq) = self.1.printDot(curNum);
        let numbeg = *curNum;
        let numend = numbeg + 1;
        *curNum += 2;
        println!("{} [shape = triangle, label = \"\"]", numbeg);
        println!(
            "{}:sw -> {}:n [label = \"{}\"]",
            numbeg,
            begp,
            tname::<InP>()
        );
        println!(
            "{}:se -> {}:n [label = \"{}\"]",
            numbeg,
            begq,
            tname::<InQ>()
        );
        println!("{} [shape= invtriangle, label = \"\"]", numend);
        println!(
            "{}:s -> {}:nw [label = \"{}\"]",
            endp,
            numend,
            tname::<OutP>()
        );
        println!(
            "{}:s -> {}:ne [label = \"{}\"]",
            endq,
            numend,
            tname::<OutQ>()
        );
        (numbeg, numend)
    }
}

// NI - NI
implNI!{
    (InP,InQ),
    impl<'a, InP: Val<'a>, InQ: Val<'a>, OutP: Val<'a>, OutQ: Val<'a>,
         MarkOnceP, MarkOnceQ, PNI, PNO, QNI, QNO>
        for Par<ProcessNotIm<'a, InP, OutP, MarkOnceP, PNI, PNO>,
                ProcessNotIm<'a, InQ, OutQ, MarkOnceQ, QNI, QNO>>
        where
        MarkOnceP: Once,
        MarkOnceQ: Once,
        PNI: Node<'a, InP, Out = ()>,
        PNO: Node<'a, (), Out = OutP>,
        QNI: Node<'a, InQ, Out = ()>,
        QNO: Node<'a, (), Out = OutQ>,

    trait IntProcessNotIm<'a, (InP,InQ)>
    {
        type NI = NSeq<NPar<PNI, QNI>, Ignore>;
        type NO = NMerge<OutP, OutQ>;
        fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let s = *self;
            let Par(p, q) = s;
            let (pni, pind, pno) = p.compile(g);
            let (qni, qind, qno) = q.compile(g);
            let out_ind = g.reserve();
            let rc1 = Rcjp::new();
            let rc2 = rc1.clone();
            let rcout = rc1.clone();
            g.set(pind, box node!(pno >> set1(rc1, out_ind)));
            g.set(qind, box node!(qno >> set2(rc2, out_ind)));
            (nodei!(pni || qni), out_ind, merge(rcout))
        }

    }
}

// Im - NI
implNI!{
    (InP,InQ),
    impl<'a, InP: Val<'a>, InQ: Val<'a>, OutP: Val<'a>, OutQ: Val<'a>,
         MarkOnceP, MarkOnceQ, PNIO, QNI, QNO>
        for Par<ProcessIm<'a, InP, OutP, MarkOnceP, PNIO>,
            ProcessNotIm<'a, InQ, OutQ, MarkOnceQ, QNI, QNO>>
        where
        MarkOnceP: Once,
        MarkOnceQ: Once,
        PNIO: Node<'a, InP, Out = OutP>,
        QNI: Node<'a, InQ, Out = ()>,
        QNO: Node<'a, (), Out = OutQ>,

    trait IntProcessNotIm<'a, (InP,InQ)>
    {
        type NI = NSeq<NPar<NSeq<PNIO, NStore<OutP>>, QNI>, Ignore>;
        type NO = NSeq<GenP, NPar<NLoad<OutP>, QNO>>;
        fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let s = *self;
            let Par(p, q) = s;
            let pnio = p.compileIm(g);
            let (qni, qind, qno) = q.compile(g);
            let rcin = RCell::new();
            let rcout = rcin.clone();
            (
                nodei!((pnio >> store(rcin)) || qni),
                qind,
                nodep!(load(rcout) || qno),
            )

        }

    }
}

// NI - Im
implNI!{
    (InP,InQ),
    impl<'a, InP: Val<'a>, InQ: Val<'a>, OutP: Val<'a>, OutQ: Val<'a>,
         MarkOnceP, MarkOnceQ, PNI, PNO, QNIO>
        for Par<ProcessNotIm<'a, InP, OutP, MarkOnceP, PNI, PNO>,
                ProcessIm<'a, InQ, OutQ, MarkOnceQ, QNIO>>
        where
        MarkOnceP: Once,
        MarkOnceQ: Once,
        PNI: Node<'a, InP, Out = ()>,
        PNO: Node<'a, (), Out = OutP>,
        QNIO: Node<'a, InQ, Out = OutQ>,

    trait IntProcessNotIm<'a, (InP,InQ)>
    {
        type NI = NSeq<NPar<PNI, NSeq<QNIO, NStore<OutQ>>>, Ignore>;
        type NO = NSeq<GenP, NPar<PNO, NLoad<OutQ>>>;
        fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let s = *self;
            let Par(p, q) = s;
            let (pni, pind, pno) = p.compile(g);
            let qnio = q.compileIm(g);
            let rcin = RCell::new();
            let rcout = rcin.clone();
            (
                nodei!(pni || (qnio >> store(rcin))),
                pind,
                nodep!(pno || load(rcout)),
            )

        }

    }
}

// Im - Im
implIm!{
    (InP,InQ),
    impl<'a, InP: Val<'a>, InQ: Val<'a>, OutP: Val<'a>, OutQ: Val<'a>,
         MarkOnceP, MarkOnceQ, PNIO, QNIO>
        for Par<ProcessIm<'a, InP, OutP, MarkOnceP, PNIO>,
                ProcessIm<'a, InQ, OutQ, MarkOnceQ, QNIO>>
        where
        MarkOnceP: Once,
        MarkOnceQ: Once,
        PNIO: Node<'a, InP, Out = OutP>,
        QNIO: Node<'a, InQ, Out = OutQ>,

    trait IntProcessIm<'a, (InP,InQ)>
    {
        type NIO = NPar<PNIO, QNIO>;
        fn compileIm(self : Box<Self>, g: &mut Graph<'a>) -> Self::NIO {
            let s = *self;
            let Par(p, q) = s;
            let pnio = p.compileIm(g);
            let qnio = q.compileIm(g);
            node!(pnio || qnio)
        }
    }
}





//  ____  _
// | __ )(_) __ _
// |  _ \| |/ _` |
// | |_) | | (_| |
// |____/|_|\__, |
//          |___/


/// A process implementation that put many process in parallel.
///
/// It takes Copy value and send it to all the process.
/// It waits that all processes have finished before continuing.
pub struct BigPar<P>(pub(crate) Vec<P>);

impl<'a, P, In: Val<'a>> IntProcess<'a, In> for BigPar<P>
where
    P: Process<'a, In, Out = ()>,
    In: Copy,
{
    type Out = ();
    type MarkOnce = P::MarkOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"BigPar\"];", num);
        (num, num)
    }
}

impl<'a, In: Val<'a>, MarkOnce, PNI, PNO> IntProcessNotIm<'a, In>
    for BigPar<ProcessNotIm<'a, In, (), MarkOnce, PNI, PNO>>
where
    MarkOnce: Once,
    PNI: Node<'a, In, Out = ()>,
    PNO: Node<'a, (), Out = ()>,
    In: Copy,
{
    type NI = NSeq<NStore<In>, NBigPar>;
    type NO = Nothing;
    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let mut dests: Vec<usize> = vec![];
        let end_point = g.reserve();
        let rcbjp = Rcbjp::new(self.0.len(), end_point);
        let rcin = RCell::new();
        for p in self.0 {
            let (pni, pind, pno) = p.compile(g);
            g.set(pind, box node!(pno >> big_merge(rcbjp.clone())));
            dests.push(g.add(box node!(load_copy(rcin.clone()) >> pni)));
        }
        (
            node!(store(rcin) >> NBigPar { dests }),
            end_point,
            Nothing {},
        )
    }
}
