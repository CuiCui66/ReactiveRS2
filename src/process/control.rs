use node::*;
// use node::rcmanip::*;
// use node::control::*;
use super::*;

//   ____ _           _
//  / ___| |__   ___ (_) ___ ___
// | |   | '_ \ / _ \| |/ __/ _ \
// | |___| | | | (_) | | (_|  __/
//  \____|_| |_|\___/|_|\___\___|


pub struct PChoice<PT, PF> {
    pub(crate) pt: PT,
    pub(crate) pf: PF,
}

impl<'a, PT, PF, InT: Val<'a>, InF: Val<'a>, Out: Val<'a>> GProcess<'a, ChoiceData<InT, InF>>
    for PChoice<PT,PF>
where
    PT: GProcess<'a, InT, Out = Out>,
    PF: GProcess<'a, InF, Out = Out>,
{
    type Out = Out;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (begt,endt) = self.pt.printDot(curNum);
        let (begf,endf) = self.pf.printDot(curNum);
        let numbeg = *curNum;
        let numend = numbeg +1;
        *curNum += 2;
        println!("{} [shape=diamond, label=\"if\"]",numbeg);
        println!("{}:w -> {} [label = \" True:{} \"];",numbeg,begt,tname::<InT>());
        println!("{}:e -> {} [label = \" False:{} \"];",numbeg,begf,tname::<InF>());
        println!("{} [size = 0.1]",numend);
        println!("{} -> {}:w",endt,numend);
        println!("{} -> {}:e",endf,numend);
        (numbeg,numend)
    }
}


impl<'a, PT, PF, InT: Val<'a>, InF: Val<'a>, Out: Val<'a>> Process<'a, ChoiceData<InT, InF>>
    for PChoice<MarkedProcess<PT, NotIm>, MarkedProcess<PF, NotIm>>
where
    PT: Process<'a, InT, Out = Out>,
    PF: Process<'a, InF, Out = Out>,
{
    type NI = NChoice<PT::NI, PF::NI>;
    type NO = NLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (ptni, ptind, ptno) = self.pt.p.compile(g);
        let (pfni, pfind, pfno) = self.pf.p.compile(g);
        let rct = RCell::new();
        let rcf = rct.clone();
        let rcout = rct.clone();
        let out = g.reserve();
        g.set(ptind, box node!(ptno >> store(rct) >> jump(out)));
        g.set(pfind, box node!(pfno >> store(rcf) >> jump(out)));
        (node!(choice ptni pfni), out, load(rcout))
    }
}

impl<'a, PT, PF, InT: Val<'a>, InF: Val<'a>, Out: Val<'a>> Process<'a, ChoiceData<InT, InF>>
    for PChoice<MarkedProcess<PT, IsIm>, MarkedProcess<PF, NotIm>>
where
    PT: Process<'a, InT, Out = Out>,
    PF: Process<'a, InF, Out = Out>,
{
    type NI = NChoice<NSeq<PT::NIO, NSeq<NStore<Out>, NJump>>, PF::NI>;
    type NO = NLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let ptnio = self.pt.p.compileIm(g);
        let (pfni, pfind, pfno) = self.pf.p.compile(g);
        let rct = RCell::new();
        let rcf = rct.clone();
        let rcout = rct.clone();
        let out = g.reserve();
        g.set(pfind, box node!(pfno >> store(rcf) >> jump(out)));
        (
            node!(choice {ptnio >> store(rct)>>jump(out)} pfni),
            out,
            load(rcout),
        )

    }
}


impl<'a, PT, PF, InT: Val<'a>, InF: Val<'a>, Out: Val<'a>> Process<'a, ChoiceData<InT, InF>>
    for PChoice<MarkedProcess<PT, NotIm>, MarkedProcess<PF, IsIm>>
where
    PT: Process<'a, InT, Out = Out>,
    PF: Process<'a, InF, Out = Out>,
{
    type NI = NChoice<PT::NI, NSeq<PF::NIO, NSeq<NStore<Out>, NJump>>>;
    type NO = NLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (ptni, ptind, ptno) = self.pt.p.compile(g);
        let pfnio = self.pf.p.compileIm(g);
        let rct = RCell::new();
        let rcf = rct.clone();
        let rcout = rct.clone();
        let out = g.reserve();
        g.set(ptind, box node!(ptno >> store(rct) >> jump(out)));
        (
            node!(choice ptni {pfnio >> store(rcf) >> jump(out)}),
            out,
            load(rcout),
        )
    }
}

impl<'a, PT, PF, InT: Val<'a>, InF: Val<'a>, Out: Val<'a>> Process<'a, ChoiceData<InT, InF>>
    for PChoice<MarkedProcess<PT, IsIm>, MarkedProcess<PF, IsIm>>
where
    PT: Process<'a, InT, Out = Out>,
    PF: Process<'a, InF, Out = Out>,
{
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = NChoice<PT::NIO, PF::NIO>;
    type Mark = IsIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        let ptnio = self.pt.p.compileIm(g);
        let pfnio = self.pf.p.compileIm(g);
        node!(choice ptnio pfnio)
    }
}



//  _
// | |    ___   ___  _ __
// | |   / _ \ / _ \| '_ \
// | |__| (_) | (_) | |_) |
// |_____\___/ \___/| .__/
//                  |_|

pub struct PLoop<P> {
    pub(crate) p: P,
}

impl<'a, P, In: Val<'a>, Out: Val<'a>> GProcess<'a, In>
    for PLoop<P>
    where
    P: GProcess<'a, In, Out = ChoiceData<In,Out>>,
{
    type Out = Out;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (beg,end) = self.p.printDot(curNum);
        let numbeg = *curNum;
        let numend = numbeg +1;
        *curNum += 2;
        println!("{} [label = \"loop\"]",numbeg);
        println!("{}:s -> {} [label = \" {} \"]",numbeg,beg,tname::<In>());
        println!("{} [shape=diamond,label=\"\"]",numend);
        println!("{} -> {}:n [label = \" {} \"]",end,numend,tname::<ChoiceData<In,Out>>());
        println!("{}:w -> {}:w [label = \" Continue: {} \"];",numend,numbeg,tname::<In>());
        (numbeg,numend)
    }
}


impl<'a, P, In: Val<'a>, Out: Val<'a>, OnceStruct> Process<'a, In>
    for PLoop<MarkedProcess<P,NotIm>>
    where
    OnceStruct: NotOnce,
    P: Process<'a, In, Out = ChoiceData<In,Out>, MarkOnce = OnceStruct>,
{
    type NI = NSeq<NStore<In>,NJump>;
    type NO = NLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = SNotOnce;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI,usize,Self::NO){
        let (pni, pind, pno) = self.p.p.compile(g);
        let rcextin = RCell::new();
        let rcbegin = rcextin.clone();
        let rcendin = rcextin.clone();
        let rcendout = RCell::new();
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
}

impl<'a, P, In: Val<'a>, Out: Val<'a>, OnceStruct> Process<'a, In>
    for PLoop<MarkedProcess<P,IsIm>>
    where
    OnceStruct: NotOnce,
    P: Process<'a, In, Out = ChoiceData<In,Out>, MarkOnce = OnceStruct>,
{
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = LoopIm<P::NIO>;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;

    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO{
        let pnio = self.p.p.compileIm(g);
        LoopIm(pnio)
    }
}
