use node::*;
use super::*;

//  _____           _ _   ____
// | ____|_ __ ___ (_) |_|  _ \
// |  _| | '_ ` _ \| | __| | | |
// | |___| | | | | | | |_| |_| |
// |_____|_| |_| |_|_|\__|____/


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

impl<'a, In: 'a, E: 'a, SV: 'a> Process<'a, (Vec<(SignalRuntimeRef<SV>, E)>,In)> for EmitD
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

impl<'a, E: 'a, SV: 'a> Process<'a, Vec<(SignalRuntimeRef<SV>, E)>> for EmitD
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

//  _____           _ _   ____
// | ____|_ __ ___ (_) |_/ ___|
// |  _| | '_ ` _ \| | __\___ \
// | |___| | | | | | | |_ ___) |
// |_____|_| |_| |_|_|\__|____/

#[derive(Clone)]
pub struct EmitS<SV, E>(pub SignalRuntimeRef<SV>, pub PhantomData<E>);

pub fn emit<SV>(sr: SignalRuntimeRef<SV>) -> EmitS<SV, SV::E>
where
    SV:SignalValue
{
    EmitS(sr, PhantomData)
}

impl<'a, E: 'a, SV: 'a> Process<'a, E> for EmitS<SV, E>
where
    SV: SignalValue<E = E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type Mark = IsIm;
    type NIO = NEmitS<SV, E>;
    type Out = ();

    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        NEmitS(self.0, PhantomData)
    }
}


impl<'a, SV: 'a, E: 'a, In: 'a> Process<'a, (E,In)> for EmitS<SV, E>
where
    SV: SignalValue<E = E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Out = In;
    type NIO = NEmitS<SV, E>;
    type Mark = IsIm;

    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        NEmitS(self.0, PhantomData)
    }
}


//  _____           _ _ __     __        ____
// | ____|_ __ ___ (_) |\ \   / /__  ___/ ___|
// |  _| | '_ ` _ \| | __\ \ / / _ \/ __\___ \
// | |___| | | | | | | |_ \ V /  __/ (__ ___) |
// |_____|_| |_| |_|_|\__| \_/ \___|\___|____/


#[derive(Clone)]
pub struct EmitVecS<SV>(pub Vec<SignalRuntimeRef<SV>>);

pub fn emit_vec<SV>(sr: Vec<SignalRuntimeRef<SV>>) -> EmitVecS<SV>
where
    SV:SignalValue
{
    EmitVecS(sr)
}

impl<'a, E: 'a, SV: 'a> Process<'a, Vec<E>> for EmitVecS<SV>
where
    SV: SignalValue<E = E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type Mark = IsIm;
    type NIO = NEmitVecS<SV>;
    type Out = ();

    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        NEmitVecS(self.0)
    }
}


impl<'a, SV: 'a, E: 'a, In: 'a> Process<'a, (Vec<E>,In)> for EmitVecS<SV>
where
    SV: SignalValue<E = E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Out = In;
    type NIO = NEmitVecS<SV>;
    type Mark = IsIm;

    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        NEmitVecS(self.0)
    }
}


//  _____           _ _ __     ______
// | ____|_ __ ___ (_) |\ \   / / ___|
// |  _| | '_ ` _ \| | __\ \ / /\___ \
// | |___| | | | | | | |_ \ V /  ___) |
// |_____|_| |_| |_|_|\__| \_/  |____/


#[derive(Clone)]
pub struct EmitVS<SV, E>(pub SignalRuntimeRef<SV>, pub E);

pub fn emit_value<SV, E>(sr: SignalRuntimeRef<SV>, value: E) -> EmitVS<SV, E>
    where
SV: SignalValue<E = E>,
E: Clone,
{
    EmitVS(sr, value)
}

impl<'a, In: 'a, E: 'a, SV: 'a> Process<'a, In> for EmitVS<SV, E>
where
    SV: SignalValue<E = E>,
    E: Clone,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Mark = IsIm;
    type NIO = NEmitVS<SV, E>;
    type Out = In;

    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        NEmitVS(self.0, self.1)
    }
}


//  _____           _ _ __     ____     __        ____
// | ____|_ __ ___ (_) |\ \   / /\ \   / /__  ___/ ___|
// |  _| | '_ ` _ \| | __\ \ / /  \ \ / / _ \/ __\___ \
// | |___| | | | | | | |_ \ V /    \ V /  __/ (__ ___) |
// |_____|_| |_| |_|_|\__| \_/      \_/ \___|\___|____/


#[derive(Clone)]
pub struct EmitVVecS<SV, E>(pub Vec<(SignalRuntimeRef<SV>,E)>);

pub fn emit_value_vec<SV, E>(values: Vec<(SignalRuntimeRef<SV>,E)>) -> EmitVVecS<SV, E>
    where
        SV: SignalValue<E = E>,
        E: Clone,
{
    EmitVVecS(values)
}

impl<'a, In: 'a, E: 'a, SV: 'a> Process<'a, In> for EmitVVecS<SV, E>
    where
        SV: SignalValue<E = E>,
        E: Clone,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Mark = IsIm;
    type NIO = NEmitVVecS<SV, E>;
    type Out = In;

    fn compileIm(self, g: &mut Graph<'a>) -> Self::NIO {
        NEmitVVecS(self.0)
    }
}


//     _                _ _   ____
//    / \__      ____ _(_) |_|  _ \
//   / _ \ \ /\ / / _` | | __| | | |
//  / ___ \ V  V / (_| | | |_| |_| |
// /_/   \_\_/\_/ \__,_|_|\__|____/



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
    type NI = NSeq<NAwaitD, RcStore<SignalRuntimeRef<SV>>>;
    type NO = NSeq<RcLoad<SignalRuntimeRef<SV>>, NGetD>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_rcell();
        let rc2 = rc.clone();

        let ni = node!(NAwaitD(out_id) >> store(rc));
        let no = node!(load(rc2) >> NGetD {});
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
    type NI = NSeq<NPar<NAwaitD,NIdentity>,RcStore<(SignalRuntimeRef<SV>, In)>>;
    type NO = NSeq<RcLoad<(SignalRuntimeRef<SV>, In)>, NGetD>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_rcell();
        let rc2 = rc.clone();

        // Type inference won't work here
        let ni_first = <NAwaitD as Node<'a,SignalRuntimeRef<SV>>>::njoin::<In, NIdentity>(NAwaitD(out_id), NIdentity {});
        let ni = node!(ni_first >> store(rc));
        let no = node!(load(rc2) >> NGetD{});
        (ni, out_id, no)
    }
}

//     _                _ _   ____
//    / \__      ____ _(_) |_/ ___|
//   / _ \ \ /\ / / _` | | __\___ \
//  / ___ \ V  V / (_| | | |_ ___) |
// /_/   \_\_/\_/ \__,_|_|\__|____/

#[derive(Clone)]
pub struct AwaitS<SV>(pub SignalRuntimeRef<SV>);

impl<'a, In: 'a, V: 'a, SV: 'a> Process<'a, In> for AwaitS<SV>
where
    SV: SignalValue<V=V>,
{
    type Out = (V,In);
    type Mark = NotIm;
    type NIO = DummyN<(V,In)>;
    type NI = NSeq<RcStore<In>,NAwaitS<SV>>;
    type NO = NSeq<GenP, NPar<NGetS<SV>, RcLoad<In>>>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_rcell();
        let rc2 = rc.clone();

        let ni = node!(store(rc) >> NAwaitS(self.0.clone(), out_id));
        let no = node!( GenP >> (NGetS(self.0) || load(rc2)));
        (ni, out_id, no)
    }
}


//     _                _ _   ___                              _ _       _       ____
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___|  _ \
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \ | | |
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/ |_| |
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|____/


#[derive(Clone, Copy)]
pub struct AwaitImmediateD {}

#[allow(non_upper_case_globals)]
pub static AwaitImmediateD: AwaitImmediateD = AwaitImmediateD {};

impl<'a, SV: 'a> Process<'a, SignalRuntimeRef<SV>> for AwaitImmediateD
    where
        SV: SignalValue,
{
    type Out = ();
    type Mark = NotIm;
    type NIO = DummyN<()>;
    type NI = NAwaitImmediateD;
    type NO = Nothing;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        (NAwaitImmediateD(out_id), out_id, Nothing {})
    }
}


impl<'a, In: 'a, SV: 'a> Process<'a, (SignalRuntimeRef<SV>, In)> for AwaitImmediateD
    where
        SV: SignalValue,
{
    type Out = In;
    type Mark = NotIm;
    type NIO = DummyN<In>;
    type NI = NSeq<NSeq<NPar<NIdentity,RcStore<In>>, Ignore2>,NAwaitImmediateD>;
    type NO = RcLoad<In>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_rcell();
        let rc2 = rc.clone();

        let ni_first = <NIdentity as Node<'a,SignalRuntimeRef<SV>>>::njoin::<In, RcStore<In>>(NIdentity {}, store(rc));
        let ni_second = <NPar<NIdentity,RcStore<In>> as Node<'a, (SignalRuntimeRef<SV>, In)>>::nseq(ni_first, Ignore2);
        let ni = <NSeq<NPar<NIdentity,RcStore<In>>,Ignore2> as Node<'a, (SignalRuntimeRef<SV>, In)>>::nseq(ni_second, NAwaitImmediateD(out_id));
        let no = load(rc2);
        (ni, out_id, no)
    }
}

//     _                _ _   ___                              _ _       _       ____
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___/ ___|
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \___ \
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/___) |
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|____/


#[derive(Clone)]
pub struct AwaitImmediateS<SV>(pub SignalRuntimeRef<SV>);


impl<'a, In: 'a, V: 'a, SV: 'a> Process<'a, In> for AwaitImmediateS<SV>
    where
        SV: SignalValue<V=V>,
{
    type Out = In;
    type Mark = NotIm;
    type NIO = DummyN<In>;
    type NI = NSeq<RcStore<In>,NAwaitImmediateS<SV>>;
    type NO = RcLoad<In>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_rcell();
        let rc2 = rc.clone();

        let ni = node!(store(rc) >> NAwaitImmediateS(self.0.clone(), out_id));
        let no = load(rc2);
        (ni, out_id, no)
    }
}

//  ____                           _
// |  _ \ _ __ ___  ___  ___ _ __ | |_
// | |_) | '__/ _ \/ __|/ _ \ '_ \| __|
// |  __/| | |  __/\__ \  __/ | | | |_
// |_|   |_|  \___||___/\___|_| |_|\__|

pub struct PresentD<PT, PF> {
    pub(crate) pt: PT,
    pub(crate) pf: PF,
}

impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, SignalRuntimeRef<SV>>
    for PresentD<MarkedProcess<PT, NotIm>, MarkedProcess<PF, NotIm>>
where
    PT: Process<'a, (), Out=Out>,
    PF: Process<'a, (), Out=Out>,
    SV: SignalValue,
{
    type Out = Out;
    type NI = NPresentD;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_rcell();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let (ptni, ptind, ptno) = self.pt.p.compile(g);
        let (pfni, pfind, pfno) = self.pf.p.compile(g);

        let out_id = g.reserve();
        g.set(ptind, box node!(ptno >> store(rct) >> jump(out_id)));
        g.set(pfind, box node!(pfno >> store(rcf) >> jump(out_id)));
        let nit_id = g.add(box ptni);
        let nif_id = g.add(box pfni);

        let ni = NPresentD {
            node_true: nit_id,
            node_false: nif_id,
        };

        (ni, out_id, load(rc_out))
    }
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, SignalRuntimeRef<SV>>
for PresentD<MarkedProcess<PT, IsIm>, MarkedProcess<PF, NotIm>>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type Out = Out;
    type NI = NPresentD;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_rcell();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let ptnio = self.pt.p.compileIm(g);
        let (pfni, pfind, pfno) = self.pf.p.compile(g);

        let out_id = g.reserve();
        let ptind = g.add(box node!(ptnio >> store(rct) >> jump(out_id)));
        g.set(pfind, box node!(pfno >> store(rcf) >> jump(out_id)));
        let nif_id = g.add(box pfni);

        let ni = NPresentD {
            node_true: ptind,
            node_false: nif_id,
        };

        (ni, out_id, load(rc_out))
    }
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, SignalRuntimeRef<SV>>
for PresentD<MarkedProcess<PT, NotIm>, MarkedProcess<PF, IsIm>>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type Out = Out;
    type NI = NPresentD;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_rcell();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let pfnio = self.pf.p.compileIm(g);
        let (ptni, ptind, ptno) = self.pt.p.compile(g);

        let out_id = g.reserve();
        let pfind = g.add(box node!(pfnio >> store(rcf) >> jump(out_id)));
        g.set(ptind, box node!(ptno >> store(rct) >> jump(out_id)));
        let nit_id = g.add(box ptni);

        let ni = NPresentD {
            node_true: nit_id,
            node_false: pfind,
        };

        (ni, out_id, load(rc_out))
    }
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, SignalRuntimeRef<SV>>
for PresentD<MarkedProcess<PT, IsIm>, MarkedProcess<PF, IsIm>>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type Out = Out;
    type NI = NPresentD;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_rcell();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let pfnio = self.pf.p.compileIm(g);
        let ptnio = self.pt.p.compileIm(g);

        let out_id = g.reserve();
        let pfind = g.add(box node!(pfnio >> store(rcf) >> jump(out_id)));
        let ptind = g.add(box node!(ptnio >> store(rct) >> jump(out_id)));

        let ni = NPresentD {
            node_true: ptind,
            node_false: pfind,
        };

        (ni, out_id, load(rc_out))
    }
}
