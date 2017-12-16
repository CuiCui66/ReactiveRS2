use node::*;
use engine::*;
use std::marker::PhantomData;
use std::rc::Rc;
use std::cell::*;
use signal::*;

pub trait Is {
    type Value;
}

impl<T> Is for T {
    type Value = T;
}

pub struct NotIm {}
pub struct IsIm {}
pub trait Im: Sized {}
impl Im for NotIm {}
impl Im for IsIm {}


pub trait Process<'a, In: 'a>: 'a + Sized {
    type Out: 'a;
    type NI: Node<'a, In, Out = ()> + Sized;
    type NO: Node<'a, (), Out = Self::Out> + Sized;
    /// If mark is set to IsIm, compile panics, if it is NotIm, compileIm panics
    type Mark: Im;
    fn compile(self, _: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        unreachable!();
    }

    type NIO: Node<'a, In, Out = Self::Out>;
    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        unreachable!();
    }

    fn seq<P>(self, p: P) -> Seq<MarkedProcess<Self, Self::Mark>, MarkedProcess<P, P::Mark>>
    where
        P: Process<'a, Self::Out>,
    {
        Seq {
            p: mp(self),
            q: mp(p),
        }
    }

    fn choice<PF, InF: 'a>(
        self,
        p: PF,
    ) -> PChoice<MarkedProcess<Self, Self::Mark>, MarkedProcess<PF, PF::Mark>>
    where
        PF: Process<'a, InF, Out = Self::Out>,
    {
        PChoice {
            pt: mp(self),
            pf: mp(p),
        }

    }

    fn ploop(self) -> PLoop<MarkedProcess<Self, Self::Mark>> {
        PLoop { p: mp(self) }
    }

    fn join<InQ: 'a, Q>(
        self,
        q: Q,
    ) -> Par<MarkedProcess<Self, Self::Mark>, MarkedProcess<Q, Q::Mark>>
    where
        Q: Process<'a, InQ> + Sized,
        Self: Sized,
    {
        Par {
            p: mp(self),
            q: mp(q),
        }
    }
}

//  _____         _           _           _
// |_   _|__  ___| |__  _ __ (_) ___ __ _| |
//   | |/ _ \/ __| '_ \| '_ \| |/ __/ _` | |
//   | |  __/ (__| | | | | | | | (_| (_| | |
//   |_|\___|\___|_| |_|_| |_|_|\___\__,_|_|


pub trait Graphfiller<'a> {
    fn fill_graph(self, g: &mut Graph<'a>) -> usize;
}

pub struct MarkedProcess<P, Mark: Im> {
    pub p: P,
    pd: PhantomData<Mark>,
}

impl<'a, P> Graphfiller<'a> for MarkedProcess<P, NotIm>
where
    P: Process<'a, (), Out = ()>,
{
    fn fill_graph(self, g: &mut Graph<'a>) -> usize {
        let (pni, pind, pno) = self.p.compile(g);
        g.set(pind, box pno);
        g.add(box pni)
    }
}

impl<'a, P> Graphfiller<'a> for MarkedProcess<P, IsIm>
where
    P: Process<'a, (), Out = ()>,
{
    fn fill_graph(self, g: &mut Graph<'a>) -> usize {
        let pnio = self.p.compileIm(g);
        g.add(box pnio)
    }
}

pub fn mp<'a, In: 'a, P>(p: P) -> MarkedProcess<P, P::Mark>
where
    P: Process<'a, In>,
{
    MarkedProcess {
        p: p,
        pd: PhantomData,
    }
}

//  _   _       _   _     _
// | \ | | ___ | |_| |__ (_)_ __   __ _
// |  \| |/ _ \| __| '_ \| | '_ \ / _` |
// | |\  | (_) | |_| | | | | | | | (_| |
// |_| \_|\___/ \__|_| |_|_|_| |_|\__, |
//                                |___/

pub struct PNothing {}

impl<'a> Process<'a, ()> for PNothing {
    type Out = ();
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type NIO = Nothing;
    fn compileIm(self, _: &mut Graph) -> Self::NIO {
        Nothing {}
    }
    type Mark = IsIm;
}

//  _____      __  __       _
// |  ___| __ |  \/  |_   _| |_
// | |_ | '_ \| |\/| | | | | __|
// |  _|| | | | |  | | |_| | |_
// |_|  |_| |_|_|  |_|\__,_|\__|

impl<'a, F: 'a, In: 'a, Out: 'a> Process<'a, In> for F
where
    F: FnMut(In) -> Out,
{
    type Out = Out;
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = FnMutN<F>;



    fn compileIm(self, _: &mut Graph) -> Self::NIO {
        FnMutN(self)
    }
    type Mark = IsIm;
}



//  ____
// / ___|  ___  __ _
// \___ \ / _ \/ _` |
//  ___) |  __/ (_| |
// |____/ \___|\__, |
//                |_|

// P and Q should be marked processes
pub struct Seq<P, Q> {
    p: P,
    q: Q,
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
}


//  ____
// |  _ \ __ _ _   _ ___  ___
// | |_) / _` | | | / __|/ _ \
// |  __/ (_| | |_| \__ \  __/
// |_|   \__,_|\__,_|___/\___|

#[derive(Copy, Clone, Debug)]
pub struct Pause {}

#[allow(non_upper_case_globals)]
pub static Pause: Pause = Pause {};

impl<'a, In: 'a> Process<'a, In> for Pause {
    type Out = In;
    type NI = NSeq<RcStore<In>, NPause>;
    type NO = RcLoad<In>;
    type NIO = DummyN<In>;
    type Mark = NotIm;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rcin = new_rcell();
        let rcout = rcin.clone();
        let out = g.reserve();
        (node!(store(rcin) >> pause(out)), out, load(rcout))
    }
}

//   ____ _           _
//  / ___| |__   ___ (_) ___ ___
// | |   | '_ \ / _ \| |/ __/ _ \
// | |___| | | | (_) | | (_|  __/
//  \____|_| |_|\___/|_|\___\___|


pub struct PChoice<PT, PF> {
    pt: PT,
    pf: PF,
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
}



//  _
// | |    ___   ___  _ __
// | |   / _ \ / _ \| '_ \
// | |__| (_) | (_) | |_) |
// |_____\___/ \___/| .__/
//                  |_|

pub struct PLoop<P> {
    p: P,
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
}



//  ____
// |  _ \ __ _ _ __
// | |_) / _` | '__|
// |  __/ (_| | |
// |_|   \__,_|_|

// P and Q should be marked processes
pub struct Par<P, Q> {
    p: P,
    q: Q,
}

impl<'a, P, Q, InP: 'a, InQ: 'a, OutP: 'a, OutQ: 'a> Process<'a, (InP, InQ)>
    for Par<MarkedProcess<P, NotIm>, MarkedProcess<Q, NotIm>>
where
    P: Process<'a, InP, Out = OutP>,
    Q: Process<'a, InQ, Out = OutQ>,
{
    type Out = (OutP, OutQ);
    type NI = NSeq<NPar<P::NI, Q::NI>, Ignore>;
    type NO = NMerge<P::Out, Q::Out>;
    type NIO = DummyN<Self::Out>;
    type Mark = NotIm;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let (pni, pind, pno) = self.p.p.compile(g);
        let (qni, qind, qno) = self.q.p.compile(g);
        let out_ind = g.reserve();
        let rc1 = new_rcjp();
        let rc2 = rc1.clone();
        let rcout = rc1.clone();
        g.set(pind, box node!(pno >> set1(rc1,out_ind)));
        g.set(qind, box node!(qno >> set2(rc2,out_ind)));
        (nodei!(pni || qni), out_ind, merge(rcout))
    }
}

impl<'a, P, Q, InP: 'a, InQ: 'a, OutP: 'a, OutQ: 'a> Process<'a, (InP, InQ)>
    for Par<MarkedProcess<P, IsIm>, MarkedProcess<Q, NotIm>>
where
    P: Process<'a, InP, Out = OutP>,
    Q: Process<'a, InQ, Out = OutQ>,
{
    type Out = (OutP, OutQ);
    type NI = NSeq<NPar<NSeq<P::NIO, RcStore<OutP>>, Q::NI>, Ignore>;
    type NO = NSeq<GenP, NPar<RcLoad<OutP>, Q::NO>>;
    type NIO = DummyN<Self::Out>;
    type Mark = NotIm;
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
    type Out = (OutP, OutQ);
    type NI = NSeq<NPar<P::NI, NSeq<Q::NIO, RcStore<OutQ>>>, Ignore>;
    type NO = NSeq<GenP, NPar<P::NO, RcLoad<OutQ>>>;
    type NIO = DummyN<Self::Out>;
    type Mark = NotIm;
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
    type Out = (OutP, OutQ);
    type NI = DummyN<()>;
    type NO = DummyN<Self::Out>;
    type NIO = NPar<P::NIO, Q::NIO>;
    type Mark = IsIm;
    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        let pnio = self.p.p.compileIm(g);
        let qnio = self.q.p.compileIm(g);
        node!(pnio || qnio)
    }
}




//  _____           _ _
// | ____|_ __ ___ (_) |_
// |  _| | '_ ` _ \| | __|
// | |___| | | | | | | |_
// |_____|_| |_| |_|_|\__|

#[derive(Copy, Clone)]
pub struct EmitD {}

#[allow(non_upper_case_globals)]
pub static EmitD: EmitD = EmitD {};

impl<'a, In: 'a, E: 'a, SV: 'a> Process<'a, ((SignalRuntimeRef<SV>, E),In)> for EmitD
where
    SV: SignalValue<E=E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Mark = IsIm;
    type NIO = NEmitD;
    type Out = In;

    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        NEmitD {}
    }
}

impl<'a, E: 'a, SV: 'a> Process<'a, (SignalRuntimeRef<SV>, E)> for EmitD
where
    SV: SignalValue<E = E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type Mark = IsIm;
    type NIO = NEmitD;
    type Out = ();

    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        NEmitD {}
    }
}


//     _                _ _
//    / \__      ____ _(_) |_
//   / _ \ \ /\ / / _` | | __|
//  / ___ \ V  V / (_| | | |_
// /_/   \_\_/\_/ \__,_|_|\__|

#[derive(Clone, Copy)]
pub struct AwaitD {}

#[allow(non_upper_case_globals)]
pub static AwaitD: AwaitD = AwaitD {};

impl<'a, V: 'a, SV: 'a> Process<'a, SignalRuntimeRef<SV>> for AwaitD
where
    SV: SignalValue<V = V>,
{
    type Out = V;
    type Mark = NotIm;
    type NIO = DummyN<V>;
    type NI = NSeq<RcStoreClone<SignalRuntimeRef<SV>>, NWaitD>;
    type NO = NSeq<RcLoad<SignalRuntimeRef<SV>>, NGetD>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let on_wait_id = g.reserve();

        g.set(on_wait_id, box pause(out_id));

        let rc = new_rcell();
        let rc2 = rc.clone();

        let ni_store = store_clone(rc);
        let ni_wait = NWaitD(on_wait_id);
        let ni = node!(ni_store >> ni_wait);

        let no_load = load(rc2);
        let no_get = NGetD {};
        let no = node!(no_load >> no_get);

        (ni, out_id, no)
    }
}

impl<'a, In: 'a, V: 'a, SV: 'a> Process<'a, (SignalRuntimeRef<SV>, In)> for AwaitD
where
    SV: SignalValue<V=V>,
{
    type Out = (V,In);
    type Mark = NotIm;
    type NIO = DummyN<(V,In)>;
    type NI = NSeq<RcStoreCloneFirst<(SignalRuntimeRef<SV>, In)>, NWaitD>;
    type NO = NSeq<RcLoad<(SignalRuntimeRef<SV>, In)>, NGetD>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let on_wait_id = g.reserve();

        g.set(on_wait_id, box pause(out_id));

        let rc = new_rcell();
        let rc2 = rc.clone();

        let ni_store = store_clone_first(rc);
        let ni_wait = NWaitD(on_wait_id);
        let ni = node!(ni_store >> ni_wait);

        let no_load = load(rc2);
        let no_get = NGetD {};
        let no = node!(no_load >> no_get);

        (ni, out_id, no)
    }
}



//  ____
// |  _ \ _ __ ___
// | |_) | '__/ _ \
// |  __/| | |  __/
// |_|   |_|  \___|

#[derive(Clone, Copy)]
pub struct PreD {}

#[allow(non_upper_case_globals)]
pub static PreD: PreD = PreD {};

impl<'a, V: 'a, SV: 'a> Process<'a, SignalRuntimeRef<SV>> for PreD
where
    SV: SignalValue<V = V>,
{
    type Out = V;
    type NI = DummyN<()>;
    type NO = DummyN<V>;
    type NIO = NGetD;
    type Mark = IsIm;

    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        NGetD {}
    }
}

impl<'a, In: 'a, V: 'a, SV: 'a> Process<'a, (SignalRuntimeRef<SV>, In)> for PreD
where
    SV: SignalValue<V=V>
{
    type Out = (V, In);
    type NI = DummyN<()>;
    type NO = DummyN<(V, In)>;
    type NIO = NGetD;
    type Mark = IsIm;

    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        NGetD {}
    }
}
