use node::*;
use super::*;

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


//     _                _ _   ___                              _ _       _
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|


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
