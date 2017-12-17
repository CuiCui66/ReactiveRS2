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
pub struct NGetD {}

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




//     _                _ _
//    / \__      ____ _(_) |_
//   / _ \ \ /\ / / _` | | __|
//  / ___ \ V  V / (_| | | |_
// /_/   \_\_/\_/ \__,_|_|\__|


#[derive(Clone, Copy)]
pub struct NAwaitD(pub usize);

impl<'a, SV: 'a> Node<'a, SignalRuntimeRef<SV>> for NAwaitD
where
    SV: SignalValue,
{
    type Out = SignalRuntimeRef<SV>;

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, sr: SignalRuntimeRef<SV>) -> Self::Out {
        sr.await(&mut sub_runtime.tasks, self.0);
        sr
    }
}


//     _                _ _   ___                              _ _       _
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|

#[derive(Clone, Copy)]
pub struct NAwaitImmediateD(pub usize);

impl<'a, SV: 'a> Node<'a, SignalRuntimeRef<SV>> for NAwaitImmediateD
    where
        SV: SignalValue,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, sr: SignalRuntimeRef<SV>) -> Self::Out {
        sr.await_immediate(&mut sub_runtime.tasks, self.0);
    }
}


//  ____                           _
// |  _ \ _ __ ___  ___  ___ _ __ | |_
// | |_) | '__/ _ \/ __|/ _ \ '_ \| __|
// |  __/| | |  __/\__ \  __/ | | | |_
// |_|   |_|  \___||___/\___|_| |_|\__|

#[derive(Clone, Copy)]
pub struct NPresentD {
    pub node_true: usize,
    pub node_false: usize,
}

impl<'a, SV: 'a> Node<'a, SignalRuntimeRef<SV>> for NPresentD
where
    SV: SignalValue
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, sr: SignalRuntimeRef<SV>) -> Self::Out {
        sr.present(sub_runtime, self.node_true, self.node_false);
    }
}