use node::*;
// use node::rcmanip::*;
// use node::control::*;
use super::*;

//   ____ _           _
//  / ___| |__   ___ (_) ___ ___
// | |   | '_ \ / _ \| |/ __/ _ \
// | |___| | | | (_) | | (_|  __/
//  \____|_| |_|\___/|_|\___\___|


pub struct PChoice<PT, PF> (pub(crate) PT, pub(crate) PF,);
impl<'a, PT, PF, InT: 'a, InF: 'a, Out: 'a> IntProcess<'a, ChoiceData<InT, InF>>
    for PChoice<PT, PF>
    where
    PT: Process<'a, InT, Out = Out>,
    PF: Process<'a, InF, Out = Out>,
{
    type Out = Out;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (begt,endt) = self.0.printDot(curNum);
        let (begf,endf) = self.1.printDot(curNum);
        let numbeg = *curNum;
        let numend = numbeg +1;
        *curNum += 2;
        println!("{} [shape=diamond, label=\"if\"]",numbeg);
        println!("{}:w -> {} [label = \"True:{}\"];",numbeg,begt,tname::<InT>());
        println!("{}:e -> {} [label = \"False:{}\"];",numbeg,begf,tname::<InF>());
        println!("{} [size = 0.1]",numend);
        println!("{} -> {}:w",endt,numend);
        println!("{} -> {}:e",endf,numend);
        (numbeg,numend)
    }
}

// NI - NI
implNI!{
    ChoiceData<InT,InF>,
    impl<'a, InT: 'a, InF: 'a, Out: 'a, PTNI, PTNO, PFNI, PFNO>
        for PChoice<ProcessNotIm<'a, InT, Out, PTNI, PTNO>, ProcessNotIm<'a, InF, Out, PFNI, PFNO>>
        where
        PTNI: Node<'a, InT, Out = ()>,
        PTNO: Node<'a, (), Out = Out>,
        PFNI: Node<'a, InF, Out = ()>,
        PFNO: Node<'a, (), Out = Out>,

    trait IntProcessNotIm<'a, ChoiceData<InT,InF>>
    {
        type NI = NChoice<PTNI, PFNI>;
        type NO = RcLoad<Out>;
        fn compile(self :Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let s = *self;
            let PChoice(pt, pf) = s;
            let (ptni, ptind, ptno) = pt.compile(g);
            let (pfni, pfind, pfno) = pf.compile(g);
            let rct = new_rcell();
            let rcf = rct.clone();
            let rcout = rct.clone();
            let out = g.reserve();
            g.set(ptind, box node!(ptno >> store(rct) >> njump(out)));
            g.set(pfind, box node!(pfno >> store(rcf) >> njump(out)));
            (node!(choice ptni pfni), out, load(rcout))
        }
    }
}

// NI - Im
implNI!{
    ChoiceData<InT,InF>,
    impl<'a, InT: 'a, InF: 'a, Out: 'a, PTNI, PTNO, PFNIO>
        for PChoice<ProcessNotIm<'a, InT, Out, PTNI, PTNO>, ProcessIm<'a, InF, Out, PFNIO>>
        where
        PTNI: Node<'a, InT, Out = ()>,
        PTNO: Node<'a, (), Out = Out>,
        PFNIO: Node<'a, InF, Out = Out>,

    trait IntProcessNotIm<'a, ChoiceData<InT,InF>>
    {
        type NI = NChoice<PTNI, NSeq<PFNIO, NSeq<RcStore<Out>, NJump>>>;
        type NO = RcLoad<Out>;
        fn compile(self :Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let s = *self;
            let PChoice(pt, pf) = s;
            let (ptni, ptind, ptno) = pt.compile(g);
            let pfnio = pf.compileIm(g);
            let rct = new_rcell();
            let rcf = rct.clone();
            let rcout = rct.clone();
            let out = g.reserve();
            g.set(ptind, box node!(ptno >> store(rct) >> njump(out)));
            (
                node!(choice ptni {pfnio >> store(rcf) >> njump(out)}),
                out,
                load(rcout),
            )
        }
    }
}

// Im - NI
implNI!{
    ChoiceData<InT,InF>,
    impl<'a, InT: 'a, InF: 'a, Out: 'a, PTNIO, PFNI, PFNO>
        for PChoice<ProcessIm<'a, InT, Out, PTNIO>, ProcessNotIm<'a, InF, Out, PFNI, PFNO>>
        where
        PTNIO: Node<'a, InT, Out = Out>,
        PFNI: Node<'a, InF, Out = ()>,
        PFNO: Node<'a, (), Out = Out>,

    trait IntProcessNotIm<'a, ChoiceData<InT,InF>>
    {
        type NI = NChoice<NSeq<PTNIO, NSeq<RcStore<Out>, NJump>>, PFNI>;
        type NO = RcLoad<Out>;
        fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let s = *self;
            let PChoice(pt, pf) = s;
            let ptnio = pt.compileIm(g);
            let (pfni, pfind, pfno) = pf.compile(g);
            let rct = new_rcell();
            let rcf = rct.clone();
            let rcout = rct.clone();
            let out = g.reserve();
            g.set(pfind, box node!(pfno >> store(rcf) >> njump(out)));
            (
                node!(choice {ptnio >> store(rct) >> njump(out)} pfni),
                out,
                load(rcout),
            )

        }
    }
}

// Im - Im
implIm!{
    ChoiceData<InT,InF>,
    impl<'a, InT: 'a, InF: 'a, Out: 'a, PTNIO, PFNIO>
        for PChoice<ProcessIm<'a, InT, Out, PTNIO>, ProcessIm<'a, InF, Out, PFNIO>>
        where
        PTNIO: Node<'a, InT, Out = Out>,
        PFNIO: Node<'a, InF, Out = Out>,

    trait IntProcessIm<'a, ChoiceData<InT,InF>>
    {
        type NIO = NChoice<PTNIO, PFNIO>;
        fn compileIm(self :Box<Self>, g: &mut Graph<'a>) -> Self::NIO {
            let s = *self;
            let PChoice(pt, pf) = s;
            let ptnio = pt.compileIm(g);
            let pfnio = pf.compileIm(g);
            node!(choice ptnio pfnio)
        }

    }
}

/*

//  _
// | |    ___   ___  _ __
// | |   / _ \ / _ \| '_ \
// | |__| (_) | (_) | |_) |
// |_____\___/ \___/| .__/
//                  |_|

pub struct PLoop<P> {
    pub(crate) p: P,
}

impl<'a, P, In: 'a, Out: 'a> Process<'a, In>
    for PLoop<MarkedProcess<P,NotIm>>
    where
    P: Process<'a, In, Out = ChoiceData<In,Out>>,
{
    type Out = Out;
    type NI = NSeq<RcStore<In>,NJump>;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI,usize,Self::NO){
        trace!("");
        let (pni, pind, pno) = self.p.p.compile(g);
        let rcextin = new_rcell();
        let rcbegin = rcextin.clone();
        let rcendin = rcextin.clone();
        let rcendout = new_rcell();
        let rcextout = rcendout.clone();
        let in_id = g.add(box node!(load(rcbegin) >> pni));
        let out_id = g.reserve();
        g.set(pind,box node!(
            pno >> choice {
                store(rcendin) >> jump(in_id)
            }{
                store(rcendout) >> jump(out_id)
            }));
        (
            node!(store(rcextin) >> jump(in_id)),
            out_id,
            node!(load(rcextout))
        )
    }
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (beg,end) = self.p.p.printDot(curNum);
        let numbeg = *curNum;
        let numend = numbeg +1;
        *curNum += 2;
        println!("{} [label = \"loop\"]",numbeg);
        println!("{}:s -> {} [label = \"{}\"]",numbeg,beg,tname::<In>());
        println!("{} [shape=diamond]",numend);
        println!("{} -> {}:n [label = \"{}\"]",end,numend,tname::<ChoiceData<In,Out>>());
        println!("{}:w -> {}:w [label = \"Continue: {}\"];",numend,numbeg,tname::<In>());
        (numbeg,numend)
    }
}

impl<'a, P, In: 'a, Out: 'a> Process<'a, In>
    for PLoop<MarkedProcess<P,IsIm>>
    where
    P: Process<'a, In, Out = ChoiceData<In,Out>>,
{
    type Out = Out;
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = LoopIm<P::NIO>;
    type Mark = IsIm;
    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO{

        trace!("");
        let pnio = self.p.p.compileIm(g);
        LoopIm(pnio)
    }
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (beg,end) = self.p.p.printDot(curNum);
        let numbeg = *curNum;
        let numend = numbeg +1;
        *curNum += 2;
        println!("{} [label = \"loop\"]",numbeg);
        println!("{}:s -> {} [label = \"{}\"]",numbeg,beg,tname::<In>());
        println!("{} [shape=diamond]",numend);
        println!("{} -> {}:n [label = \"{}\"]",end,numend,tname::<ChoiceData<In,Out>>());
        println!("{}:w -> {}:w [label = \"Continue: {}\"];",numend,numbeg,tname::<In>());
        (numbeg,numend)
    }
}

*/
