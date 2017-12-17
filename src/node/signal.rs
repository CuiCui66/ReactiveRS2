use engine::*;
use signal::*;
use super::*;



//  _____           _ _
// | ____|_ __ ___ (_) |_
// |  _| | '_ ` _ \| | __|
// | |___| | | | | | | |_
// |_____|_| |_| |_|_|\__|

#[derive(Clone, Copy)]
pub struct NEmitD {}

impl<'a, SV: 'a, E: 'a, In: 'a> Node<'a, ((SignalRuntimeRef<SV>, E), In)> for NEmitD
    where
    SV: SignalValue<E=E>,
{
    type Out = In;

    fn call(
        &mut self,
        sub_runtime: &mut SubRuntime<'a>,
        ((sr,e),val): ((SignalRuntimeRef<SV>, E), In)
    ) -> Self::Out {
        sr.emit(e, sub_runtime);
        val
    }
}

impl<'a, SV: 'a, E: 'a> Node<'a, (SignalRuntimeRef<SV>, E)> for NEmitD
where
    SV: SignalValue<E = E>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, (sr, e): (SignalRuntimeRef<SV>, E)) -> () {
        sr.emit(e, sub_runtime);
        ()
    }
}





//   ____      _
//  / ___| ___| |_
// | |  _ / _ \ __|
// | |_| |  __/ |_
//  \____|\___|\__|


#[derive(Clone, Copy)]
pub(crate) struct NGetD {}

impl<'a, SV: 'a, V: 'a, In: 'a> Node<'a, (SignalRuntimeRef<SV>, In)> for NGetD
where
    SV: SignalValue<V = V>,
{
    type Out = (V, In);

    fn call(
        &mut self,
        sub_runtime: &mut SubRuntime<'a>,
        (sr, val): (SignalRuntimeRef<SV>, In),
    ) -> Self::Out {
        (sr.signal_runtime.values.get_pre_value(), val)
    }
}

impl<'a, SV: 'a, V: 'a> Node<'a, SignalRuntimeRef<SV>> for NGetD
where
    SV: SignalValue<V = V>,
{
    type Out = V;

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, sr: SignalRuntimeRef<SV>) -> Self::Out {
        sr.signal_runtime.values.get_pre_value()
    }
}




// __        __    _ _
// \ \      / /_ _(_) |_
//  \ \ /\ / / _` | | __|
//   \ V  V / (_| | | |_
//    \_/\_/ \__,_|_|\__|

#[derive(Clone, Copy)]
pub(crate) struct NWaitD(pub usize);

impl<'a, SV: 'a> Node<'a, SignalRuntimeRef<SV>> for NWaitD
where
    SV: SignalValue,
{
    type Out = SignalRuntimeRef<SV>;

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, sr: SignalRuntimeRef<SV>) -> Self::Out {
        sr.await(&mut sub_runtime.tasks, self.0);
        sr
    }
}
