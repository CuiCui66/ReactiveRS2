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
