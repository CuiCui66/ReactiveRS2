use node::*;
use super::*;

//   ____ _           _
//  / ___| |__   ___ (_) ___ ___
// | |   | '_ \ / _ \| |/ __/ _ \
// | |___| | | | (_) | | (_|  __/
//  \____|_| |_|\___/|_|\___\___|

impl<'a, PT, PF, InT: 'a, InF: 'a, Out: 'a> ProcessPar<'a, ChoiceData<InT, InF>>
    for PChoice<MarkedProcessPar<PT, NotIm>, MarkedProcessPar<PF, NotIm>>
where
    PT: ProcessPar<'a, InT, Out = Out>,
    PF: ProcessPar<'a, InF, Out = Out>,
    Out: Send + Sync,
{
    type Out = Out;
    type NI = NChoice<PT::NI, PF::NI>;
    type NO = ArcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (ptni, ptind, ptno) = self.pt.p.compile(g);
        let (pfni, pfind, pfno) = self.pf.p.compile(g);
        let rct = new_amutex();
        let rcf = rct.clone();
        let rcout = rct.clone();
        let out = g.reserve();
        g.set(ptind, box node!(ptno >> store_par(rct) >> jump(out)));
        g.set(pfind, box node!(pfno >> store_par(rcf) >> jump(out)));
        (node!(choice ptni pfni), out, load_par(rcout))
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

impl<'a, PT, PF, InT: 'a, InF: 'a, Out: 'a> ProcessPar<'a, ChoiceData<InT, InF>>
    for PChoice<MarkedProcessPar<PT, IsIm>, MarkedProcessPar<PF, NotIm>>
where
    PT: ProcessPar<'a, InT, Out = Out>,
    PF: ProcessPar<'a, InF, Out = Out>,
    Out: Send + Sync,
{
    type Out = Out;
    type NI = NChoice<NSeq<PT::NIO, NSeq<ArcStore<Out>, NJump>>, PF::NI>;
    type NO = ArcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let ptnio = self.pt.p.compileIm(g);
        let (pfni, pfind, pfno) = self.pf.p.compile(g);
        let rct = new_amutex();
        let rcf = rct.clone();
        let rcout = rct.clone();
        let out = g.reserve();
        g.set(pfind, box node!(pfno >> store_par(rcf) >> jump(out)));
        (
            node!(choice {ptnio >> store_par(rct)>>jump(out)} pfni),
            out,
            load_par(rcout),
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


impl<'a, PT, PF, InT: 'a, InF: 'a, Out: 'a> ProcessPar<'a, ChoiceData<InT, InF>>
    for PChoice<MarkedProcessPar<PT, NotIm>, MarkedProcessPar<PF, IsIm>>
where
    PT: ProcessPar<'a, InT, Out = Out>,
    PF: ProcessPar<'a, InF, Out = Out>,
    Out: Send + Sync,
{
    type Out = Out;
    type NI = NChoice<PT::NI, NSeq<PF::NIO, NSeq<ArcStore<Out>, NJump>>>;
    type NO = ArcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (ptni, ptind, ptno) = self.pt.p.compile(g);
        let pfnio = self.pf.p.compileIm(g);
        let rct = new_amutex();
        let rcf = rct.clone();
        let rcout = rct.clone();
        let out = g.reserve();
        g.set(ptind, box node!(ptno >> store_par(rct) >> jump(out)));
        (
            node!(choice ptni {pfnio >> store_par(rcf) >> jump(out)}),
            out,
            load_par(rcout),
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

impl<'a, PT, PF, InT: 'a, InF: 'a, Out: 'a> ProcessPar<'a, ChoiceData<InT, InF>>
    for PChoice<MarkedProcessPar<PT, IsIm>, MarkedProcessPar<PF, IsIm>>
where
    PT: ProcessPar<'a, InT, Out = Out>,
    PF: ProcessPar<'a, InF, Out = Out>,
    Out: Send + Sync,
{
    type Out = Out;
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

impl<'a, P, In: 'a, Out: 'a, OnceStruct> ProcessPar<'a, In>
    for PLoop<MarkedProcessPar<P,NotIm>>
where
    OnceStruct: NotOnce,
    P: ProcessPar<'a, In, Out = ChoiceData<In,Out>, MarkOnce = OnceStruct>,
    Out: Send + Sync,
    In: Send + Sync,
{
    type Out = Out;
    type NI = NSeq<ArcStore<In>,NJump>;
    type NO = ArcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = SNotOnce;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI,usize,Self::NO){
        trace!("");
        let (pni, pind, pno) = self.p.p.compile(g);
        let rcextin = new_amutex();
        let rcbegin = rcextin.clone();
        let rcendin = rcextin.clone();
        let rcendout = new_amutex();
        let rcextout = rcendout.clone();
        let in_id = g.add(box node!(load_par(rcbegin) >> pni));
        let out_id = g.reserve();
        g.set(pind,box node!(
            pno >> choice {
                store_par(rcendin) >> jump(in_id)
            }{
                store_par(rcendout) >> jump(out_id)
            }));
        (
            node!(store_par(rcextin) >> jump(in_id)),
            out_id,
            node!(load_par(rcextout))
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

impl<'a, P, In: 'a, Out: 'a, OnceStruct> ProcessPar<'a, In>
    for PLoop<MarkedProcessPar<P,IsIm>>
where
    OnceStruct: NotOnce,
    P: ProcessPar<'a, In, Out = ChoiceData<In,Out>, MarkOnce = OnceStruct>,
    Out: Send + Sync,
{
    type Out = Out;
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = LoopIm<P::NIO>;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;

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
