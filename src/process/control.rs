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
impl<'a, PT, PF, InT: 'a, InF: 'a, Out: 'a> Process<'a, ChoiceData<InT, InF>>
    for PChoice<MarkedProcess<PT, NotIm>, MarkedProcess<PF, NotIm>>
where
    PT: Process<'a, InT, Out = Out>,
    PF: Process<'a, InF, Out = Out>,
{
    type Out = Out;
    type NI = NChoice<PT::NI, PF::NI>;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (ptni, ptind, ptno) = self.pt.p.compile(g);
        let (pfni, pfind, pfno) = self.pf.p.compile(g);
        let rct = new_rcell();
        let rcf = rct.clone();
        let rcout = rct.clone();
        let out = g.reserve();
        g.set(ptind, box node!(ptno >> store(rct) >> jump(out)));
        g.set(pfind, box node!(pfno >> store(rcf) >> jump(out)));
        (node!(choice ptni pfni), out, load(rcout))
    }
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (begt,endt) = self.pt.p.printDot(curNum);
        let (begf,endf) = self.pf.p.printDot(curNum);
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

impl<'a, PT, PF, InT: 'a, InF: 'a, Out: 'a> Process<'a, ChoiceData<InT, InF>>
    for PChoice<MarkedProcess<PT, IsIm>, MarkedProcess<PF, NotIm>>
where
    PT: Process<'a, InT, Out = Out>,
    PF: Process<'a, InF, Out = Out>,
{
    type Out = Out;
    type NI = NChoice<NSeq<PT::NIO, NSeq<RcStore<Out>, NJump>>, PF::NI>;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let ptnio = self.pt.p.compileIm(g);
        let (pfni, pfind, pfno) = self.pf.p.compile(g);
        let rct = new_rcell();
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
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (begt,endt) = self.pt.p.printDot(curNum);
        let (begf,endf) = self.pf.p.printDot(curNum);
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


impl<'a, PT, PF, InT: 'a, InF: 'a, Out: 'a> Process<'a, ChoiceData<InT, InF>>
    for PChoice<MarkedProcess<PT, NotIm>, MarkedProcess<PF, IsIm>>
where
    PT: Process<'a, InT, Out = Out>,
    PF: Process<'a, InF, Out = Out>,
{
    type Out = Out;
    type NI = NChoice<PT::NI, NSeq<PF::NIO, NSeq<RcStore<Out>, NJump>>>;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (ptni, ptind, ptno) = self.pt.p.compile(g);
        let pfnio = self.pf.p.compileIm(g);
        let rct = new_rcell();
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
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (begt,endt) = self.pt.p.printDot(curNum);
        let (begf,endf) = self.pf.p.printDot(curNum);
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

impl<'a, PT, PF, InT: 'a, InF: 'a, Out: 'a> Process<'a, ChoiceData<InT, InF>>
    for PChoice<MarkedProcess<PT, IsIm>, MarkedProcess<PF, IsIm>>
where
    PT: Process<'a, InT, Out = Out>,
    PF: Process<'a, InF, Out = Out>,
{
    type Out = Out;
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = NChoice<PT::NIO, PF::NIO>;
    type Mark = IsIm;
    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        let ptnio = self.pt.p.compileIm(g);
        let pfnio = self.pf.p.compileIm(g);
        node!(choice ptnio pfnio)
    }
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let (begt,endt) = self.pt.p.printDot(curNum);
        let (begf,endf) = self.pf.p.printDot(curNum);
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
