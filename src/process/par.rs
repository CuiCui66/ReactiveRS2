use node::*;
use super::*;


pub struct Par<P, Q> {
    pub(crate) p: P,
    pub(crate) q: Q,
}

impl<'a, P, Q, InP: 'a, InQ: 'a, OutP: 'a, OutQ: 'a> GProcess<'a, (InP, InQ)>
    for Par<P,Q>
where
    P: GProcess<'a, InP, Out = OutP>,
    Q: GProcess<'a, InQ, Out = OutQ>,
{
    type Out = (OutP, OutQ);
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (begp,endp) = self.p.printDot(curNum);
        let (begq,endq) = self.q.printDot(curNum);
        let numbeg = *curNum;
        let numend = numbeg +1;
        *curNum += 2;
        println!("{} [shape = triangle, label = \"\"]",numbeg);
        println!("{}:sw -> {}:n [label = \"{}\"]",numbeg,begp,tname::<InP>());
        println!("{}:se -> {}:n [label = \"{}\"]",numbeg,begq,tname::<InQ>());
        println!("{} [shape= invtriangle, label = \"\"]",numend);
        println!("{}:s -> {}:nw [label = \"{}\"]",endp,numend,tname::<OutP>());
        println!("{}:s -> {}:ne [label = \"{}\"]",endq,numend,tname::<OutQ>());
        (numbeg,numend)
    }
}



impl<'a, P, Q, InP: 'a, InQ: 'a, OutP: 'a, OutQ: 'a> Process<'a, (InP, InQ)>
    for Par<MarkedProcess<P, NotIm>, MarkedProcess<Q, NotIm>>
where
    P: Process<'a, InP, Out = OutP>,
    Q: Process<'a, InQ, Out = OutQ>,
{
    type NI = NSeq<NPar<P::NI, Q::NI>, Ignore>;
    type NO = NMerge<P::Out, Q::Out>;
    type NIO = DummyN<Self::Out>;
    type Mark = NotIm;
    type MarkOnce = And<P::MarkOnce, Q::MarkOnce>;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (pni, pind, pno) = self.p.p.compile(g);
        let (qni, qind, qno) = self.q.p.compile(g);
        let out_ind = g.reserve();
        let rc1 = new_rcjp();
        let rc2 = rc1.clone();
        let rcout = rc1.clone();
        g.set(pind, box node!(pno >> set1(rc1, out_ind)));
        g.set(qind, box node!(qno >> set2(rc2, out_ind)));
        (nodei!(pni || qni), out_ind, merge(rcout))
    }
}

impl<'a, P, Q, InP: 'a, InQ: 'a, OutP: 'a, OutQ: 'a> Process<'a, (InP, InQ)>
    for Par<MarkedProcess<P, IsIm>, MarkedProcess<Q, NotIm>>
where
    P: Process<'a, InP, Out = OutP>,
    Q: Process<'a, InQ, Out = OutQ>,
{
    type NI = NSeq<NPar<NSeq<P::NIO, RcStore<OutP>>, Q::NI>, Ignore>;
    type NO = NSeq<GenP, NPar<RcLoad<OutP>, Q::NO>>;
    type NIO = DummyN<Self::Out>;
    type Mark = NotIm;
    type MarkOnce = And<P::MarkOnce, Q::MarkOnce>;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let pnio = self.p.p.compileIm(g);
        let (qni, qind, qno) = self.q.p.compile(g);
        let rcin = new_rcell();
        let rcout = rcin.clone();
        (
            nodei!((pnio >> store(rcin)) || qni),
            qind,
            nodep!(load(rcout) || qno),
        )

    }
}

impl<'a, P, Q, InP: 'a, InQ: 'a, OutP: 'a, OutQ: 'a> Process<'a, (InP, InQ)>
    for Par<MarkedProcess<P, NotIm>, MarkedProcess<Q, IsIm>>
where
    P: Process<'a, InP, Out = OutP>,
    Q: Process<'a, InQ, Out = OutQ>,
{
    type NI = NSeq<NPar<P::NI, NSeq<Q::NIO, RcStore<OutQ>>>, Ignore>;
    type NO = NSeq<GenP, NPar<P::NO, RcLoad<OutQ>>>;
    type NIO = DummyN<Self::Out>;
    type Mark = NotIm;
    type MarkOnce = And<P::MarkOnce, Q::MarkOnce>;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (pni, pind, pno) = self.p.p.compile(g);
        let qnio = self.q.p.compileIm(g);
        let rcin = new_rcell();
        let rcout = rcin.clone();
        (
            nodei!(pni || (qnio >> store(rcin))),
            pind,
            nodep!(pno || load(rcout)),
        )

    }
}

impl<'a, P, Q, InP: 'a, InQ: 'a, OutP: 'a, OutQ: 'a> Process<'a, (InP, InQ)>
    for Par<MarkedProcess<P, IsIm>, MarkedProcess<Q, IsIm>>
where
    P: Process<'a, InP, Out = OutP>,
    Q: Process<'a, InQ, Out = OutQ>,
{
    type NI = DummyN<()>;
    type NO = DummyN<Self::Out>;
    type NIO = NPar<P::NIO, Q::NIO>;
    type Mark = IsIm;
    type MarkOnce = And<P::MarkOnce, Q::MarkOnce>;
    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        let pnio = self.p.p.compileIm(g);
        let qnio = self.q.p.compileIm(g);
        node!(pnio || qnio)
    }
}

//  ____  _
// | __ )(_) __ _
// |  _ \| |/ _` |
// | |_) | | (_| |
// |____/|_|\__, |
//          |___/


pub struct BigPar<P> {
    pub(crate) vp: Vec<P>,
}

impl<'a, P, In: 'a> GProcess<'a, In> for BigPar<P>
where
    P: GProcess<'a, In, Out = ()>,
    In: Copy
{
    type Out = ();
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"BigPar\"];",num);
        (num,num)
    }
}

impl<'a, P, In: 'a> Process<'a, In> for BigPar<MarkedProcess<P, NotIm>>
where
    P: Process<'a, In, Out = ()>,
    In: Copy
{
    type NI = NSeq<RcStore<In>,NBigPar>;
    type NO = Nothing;
    type NIO = DummyN<Self::Out>;
    type Mark = NotIm;
    type MarkOnce = P::MarkOnce;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let mut dests: Vec<usize> = vec![];
        let end_point = g.reserve();
        let rcbjp = new_rcbjp(self.vp.len(),end_point);
        let rcin = new_rcell();
        for p in self.vp{
            let (pni, pind, pno) = p.p.compile(g);
            g.set(pind, box node!(pno >> big_merge(rcbjp.clone())));
            dests.push(g.add(box node!(load_copy(rcin.clone()) >> pni)));
        };
        (node!(store(rcin) >> NBigPar{dests}),end_point,Nothing)
    }
}

