use node::*;
use super::*;

//   ____ _           _
//  / ___| |__   ___ (_) ___ ___
// | |   | '_ \ / _ \| |/ __/ _ \
// | |___| | | | (_) | | (_|  __/
//  \____|_| |_|\___/|_|\___\___|

/// A basic branching struct that takes a `ChoiceData` and call PT or PF depending of
/// its value.
pub struct PChoice<PT, PF>(pub(crate) PT, pub(crate) PF);


impl<'a, PT, PF, InT: Val<'a>, InF: Val<'a>, Out: Val<'a>> IntProcess<'a, ChoiceData<InT, InF>>
    for PChoice<PT, PF>
where
    PT: Process<'a, InT, Out = Out>,
    PF: Process<'a, InF, Out = Out>,
{
    type Out = Out;
    type MarkOnce = <And<PT::MarkOnce, PF::MarkOnce> as GiveOnce>::Once;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let (begt, endt) = self.0.printDot(curNum);
        let (begf, endf) = self.1.printDot(curNum);
        let numbeg = *curNum;
        let numend = numbeg + 1;
        *curNum += 2;
        println!("{} [shape=diamond, label=\"if\"]", numbeg);
        println!(
            "{}:w -> {} [label = \"True:{}\"];",
            numbeg,
            begt,
            tname::<InT>()
        );
        println!(
            "{}:e -> {} [label = \"False:{}\"];",
            numbeg,
            begf,
            tname::<InF>()
        );
        println!("{} [size = 0.1]", numend);
        println!("{} -> {}:w", endt, numend);
        println!("{} -> {}:e", endf, numend);
        (numbeg, numend)
    }
}

// NI - NI
implNI!{
    ChoiceData<InT,InF>,
    impl<'a, InT: Val<'a>, InF: Val<'a>, Out: Val<'a>, PTNI, PTNO, PFNI, PFNO, MarkOnceT, MarkOnceF>
        for PChoice<ProcessNotIm<'a, InT, Out, MarkOnceT, PTNI, PTNO>,
                    ProcessNotIm<'a, InF, Out, MarkOnceF, PFNI, PFNO>>
        where
        MarkOnceT: Once,
        MarkOnceF: Once,
        PTNI: Node<'a, InT, Out = ()>,
        PTNO: Node<'a, (), Out = Out>,
        PFNI: Node<'a, InF, Out = ()>,
        PFNO: Node<'a, (), Out = Out>,

    trait IntProcessNotIm<'a, ChoiceData<InT,InF>>
    {
        type NI = NChoice<PTNI, PFNI>;
        type NO = NLoad<Out>;
        fn compile(self :Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let s = *self;
            let PChoice(pt, pf) = s;
            let (ptni, ptind, ptno) = pt.compile(g);
            let (pfni, pfind, pfno) = pf.compile(g);
            let rct = RCell::new();
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
    impl<'a, InT: Val<'a>, InF: Val<'a>, Out: Val<'a>, MarkOnceT, MarkOnceF, PTNI, PTNO, PFNIO>
        for PChoice<ProcessNotIm<'a, InT, Out, MarkOnceT, PTNI, PTNO>,
                    ProcessIm<'a, InF, Out, MarkOnceF, PFNIO>>
        where
        MarkOnceT: Once,
        MarkOnceF: Once,
        PTNI: Node<'a, InT, Out = ()>,
        PTNO: Node<'a, (), Out = Out>,
        PFNIO: Node<'a, InF, Out = Out>,

    trait IntProcessNotIm<'a, ChoiceData<InT,InF>>
    {
        type NI = NChoice<PTNI, NSeq<PFNIO, NSeq<NStore<Out>, NJump>>>;
        type NO = NLoad<Out>;
        fn compile(self :Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let s = *self;
            let PChoice(pt, pf) = s;
            let (ptni, ptind, ptno) = pt.compile(g);
            let pfnio = pf.compileIm(g);
            let rct = RCell::new();
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
    impl<'a, InT: Val<'a>, InF: Val<'a>, Out: Val<'a>, MarkOnceT, MarkOnceF, PTNIO, PFNI, PFNO>
        for PChoice<ProcessIm<'a, InT, Out, MarkOnceT, PTNIO>,
                    ProcessNotIm<'a, InF, Out, MarkOnceF, PFNI, PFNO>>
        where
        MarkOnceT: Once,
        MarkOnceF: Once,
        PTNIO: Node<'a, InT, Out = Out>,
        PFNI: Node<'a, InF, Out = ()>,
        PFNO: Node<'a, (), Out = Out>,

    trait IntProcessNotIm<'a, ChoiceData<InT,InF>>
    {
        type NI = NChoice<NSeq<PTNIO, NSeq<NStore<Out>, NJump>>, PFNI>;
        type NO = NLoad<Out>;
        fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let s = *self;
            let PChoice(pt, pf) = s;
            let ptnio = pt.compileIm(g);
            let (pfni, pfind, pfno) = pf.compile(g);
            let rct = RCell::new();
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
    impl<'a, InT: Val<'a>, InF: Val<'a>, Out: Val<'a>, MarkOnceT, MarkOnceF, PTNIO, PFNIO>
        for PChoice<ProcessIm<'a, InT, Out, MarkOnceT, PTNIO>,
                    ProcessIm<'a, InF, Out, MarkOnceF, PFNIO>>
        where
        MarkOnceT: Once,
        MarkOnceF: Once,
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



//  _
// | |    ___   ___  _ __
// | |   / _ \ / _ \| '_ \
// | |__| (_) | (_) | |_) |
// |_____\___/ \___/| .__/
//                  |_|

/// A basic looping construct that run a process `NotOnce` returning a `ChoiceData`
/// And relauch it while it return the `True` constructor.
pub struct PLoop<P>(pub(crate) P);

impl<'a, P, In: Val<'a>, Out: Val<'a>> IntProcess<'a, In> for PLoop<P>
where
    P: Process<
        'a,
        In,
        Out = ChoiceData<
            In,
            Out,
        >,
    >,
{
    type Out = Out;
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let (beg, end) = self.0.printDot(curNum);
        let numbeg = *curNum;
        let numend = numbeg + 1;
        *curNum += 2;
        println!("{} [label = \"loop\"]", numbeg);
        println!("{}:s -> {} [label = \"{}\"]", numbeg, beg, tname::<In>());
        println!("{} [shape=diamond]", numend);
        println!(
            "{} -> {}:n [label = \"{}\"]",
            end,
            numend,
            tname::<ChoiceData<In, Out>>()
        );
        println!(
            "{}:w -> {}:w [label = \"Continue: {}\"];",
            numend,
            numbeg,
            tname::<In>()
        );
        (numbeg, numend)
    }
}


implNI!{
    In,
    impl<'a, In: Val<'a>, Out: Val<'a>, MarkOnce, PNI, PNO>
        for PLoop<ProcessNotIm<'a, In, ChoiceData<In,Out>, MarkOnce, PNI, PNO>>
        where
        MarkOnce: Once,
        PNI: Node<'a, In, Out = ()>,
        PNO: Node<'a, (), Out = ChoiceData<In,Out>>,
    trait IntProcessNotIm<'a, In> {
        type NI = NSeq<NStore<In>,NJump>;
        type NO = NLoad<Out>;
        fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI,usize,Self::NO){
            let (pni, pind, pno) = self.0.compile(g);
            let rcextin = RCell::new();
            let rcbegin = rcextin.clone();
            let rcendin = rcextin.clone();
            let rcendout = RCell::new();
            let rcextout = rcendout.clone();
            let in_id = g.add(box node!(load(rcbegin) >> pni));
            let out_id = g.reserve();
            g.set(pind,box node!(
                pno >> choice {
                    store(rcendin) >> njump(in_id)
                }{
                    store(rcendout) >> njump(out_id)
                }));
            (
                node!(store(rcextin) >> njump(in_id)),
                out_id,
                node!(load(rcextout))
            )
        }
    }
}

implIm!{
    In,
    impl<'a, In: Val<'a>, Out: Val<'a>, MarkOnce, PNIO>
        for PLoop<ProcessIm<'a,In,ChoiceData<In,Out>, MarkOnce, PNIO>>
        where
        MarkOnce: Once,
        PNIO: Node<'a, In, Out = ChoiceData<In,Out>>,
    trait IntProcessIm<'a, In> {
        type NIO = LoopIm<PNIO>;
        fn compileIm(self: Box<Self>, g: &mut Graph<'a>) -> Self::NIO{
            LoopIm(self.0.compileIm(g))
        }

    }
}
