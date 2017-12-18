use engine::*;
use signal::*;
use super::*;


//  _____           _ _   ____
// | ____|_ __ ___ (_) |_|  _ \
// |  _| | '_ ` _ \| | __| | | |
// | |___| | | | | | | |_| |_| |
// |_____|_| |_| |_|_|\__|____/

#[derive(Clone, Copy)]
pub struct NEmitD {}


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


impl<'a, SV: 'a, E: 'a> Node<'a, Vec<(SignalRuntimeRef<SV>, E)>> for NEmitD
    where
        SV: SignalValue<E = E>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, vec: Vec<(SignalRuntimeRef<SV>, E)>) -> () {
        for (sr, emit_value) in vec {
            sr.emit(emit_value, sub_runtime);
        }
        ()
    }
}


impl<'a, SV: 'a, E: 'a, In: 'a> Node<'a, (Vec<(SignalRuntimeRef<SV>, E)>, In)> for NEmitD
    where
        SV: SignalValue<E=E>,
{
    type Out = In;

    fn call(
        &mut self,
        sub_runtime: &mut SubRuntime<'a>,
        (vec,val): (Vec<(SignalRuntimeRef<SV>, E)>, In)
    ) -> Self::Out {
        for (sr,emit_value) in vec {
            sr.emit(emit_value, sub_runtime);
        }
        val
    }
}

//  _____           _ _   ____
// | ____|_ __ ___ (_) |_/ ___|
// |  _| | '_ ` _ \| | __\___ \
// | |___| | | | | | | |_ ___) |
// |_____|_| |_| |_|_|\__|____/


#[derive(Clone)]
pub struct NEmitS<SV, E>(pub SignalRuntimeRef<SV>, pub PhantomData<E>);

impl<'a, SV: 'a, E: 'a> Node<'a, E> for NEmitS<SV, E>
where
    SV: SignalValue<E = E>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, emit_value: E) -> () {
        self.0.emit(emit_value, sub_runtime);
        ()
    }
}


impl<'a, In: 'a, SV: 'a, E: 'a> Node<'a, (E, In)> for NEmitS<SV, E>
where
    SV: SignalValue<E = E>,
{
    type Out = In;

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, (emit_value, val): (E, In)) -> In {
        self.0.emit(emit_value, sub_runtime);
        val
    }
}

//  _____           _ _ __     __        ____
// | ____|_ __ ___ (_) |\ \   / /__  ___/ ___|
// |  _| | '_ ` _ \| | __\ \ / / _ \/ __\___ \
// | |___| | | | | | | |_ \ V /  __/ (__ ___) |
// |_____|_| |_| |_|_|\__| \_/ \___|\___|____/


#[derive(Clone)]
pub struct NEmitVecS<SV>(pub Vec<SignalRuntimeRef<SV>>);

impl<'a, SV: 'a, E: 'a> Node<'a, Vec<E>> for NEmitVecS<SV>
where
    SV: SignalValue<E = E>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, emit_values: Vec<E>) -> () {
        if emit_values.len() != self.0.len() {
            panic!("The vector given to the EmitVecS process should have the same size as the signal vector.")
        }

        for (sr,emit_value) in self.0.iter().zip(emit_values.into_iter()) {
            sr.emit(emit_value, sub_runtime);
        }
        ()
    }
}

impl<'a, SV: 'a, E: 'a, In: 'a> Node<'a, (Vec<E>, In)> for NEmitVecS<SV>
where
    SV: SignalValue<E = E>,
{
    type Out = In;

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, (emit_values, val): (Vec<E>, In)) -> In {
        if emit_values.len() != self.0.len() {
            panic!("The vector given to the EmitVecS process should have the same size as the signal vector.")
        }
        for (sr,emit_value) in self.0.iter().zip(emit_values.into_iter()) {
            sr.emit(emit_value, sub_runtime);
        }
        val
    }
}

//  _____           _ _ __     ______
// | ____|_ __ ___ (_) |\ \   / / ___|
// |  _| | '_ ` _ \| | __\ \ / /\___ \
// | |___| | | | | | | |_ \ V /  ___) |
// |_____|_| |_| |_|_|\__| \_/  |____/


#[derive(Clone)]
pub struct NEmitVS<SV, E>(pub SignalRuntimeRef<SV>, pub E);

impl<'a, In: 'a, SV: 'a, E: 'a> Node<'a, In> for NEmitVS<SV, E>
where
    SV: SignalValue<E = E>,
    E: Clone
{
    type Out = In;

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: In) -> In {
        self.0.emit(self.1.clone(), sub_runtime);
        val
    }
}


//  _____           _ _ __     ____     __        ____
// | ____|_ __ ___ (_) |\ \   / /\ \   / /__  ___/ ___|
// |  _| | '_ ` _ \| | __\ \ / /  \ \ / / _ \/ __\___ \
// | |___| | | | | | | |_ \ V /    \ V /  __/ (__ ___) |
// |_____|_| |_| |_|_|\__| \_/      \_/ \___|\___|____/

#[derive(Clone)]
pub struct NEmitVVecS<SV,E>(pub Vec<(SignalRuntimeRef<SV>,E)>);

impl<'a, In: 'a, SV: 'a, E: 'a> Node<'a, In> for NEmitVVecS<SV, E>
    where
        SV: SignalValue<E = E>,
        E: Clone,
{
    type Out = In;

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: In) -> In {
        for &(ref sr,ref emit_value) in &self.0 {
            sr.emit(emit_value.clone(), sub_runtime);
        }
        val
    }
}


//   ____      _   ____
//  / ___| ___| |_|  _ \
// | |  _ / _ \ __| | | |
// | |_| |  __/ |_| |_| |
//  \____|\___|\__|____/


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


//   ____      _   ____
//  / ___| ___| |_/ ___|
// | |  _ / _ \ __\___ \
// | |_| |  __/ |_ ___) |
//  \____|\___|\__|____/


#[derive(Clone)]
pub struct NGetS<SV>(pub SignalRuntimeRef<SV>);

impl<'a, SV: 'a, V: 'a> Node<'a, ()> for NGetS<SV>
where
    SV: SignalValue<V = V>,
{
    type Out = V;

    fn call(
        &mut self,
        sub_runtime: &mut SubRuntime<'a>,
        _: (),
    ) -> Self::Out {
        self.0.signal_runtime.values.get_pre_value()
    }
}


//     _                _ _   ____
//    / \__      ____ _(_) |_|  _ \
//   / _ \ \ /\ / / _` | | __| | | |
//  / ___ \ V  V / (_| | | |_| |_| |
// /_/   \_\_/\_/ \__,_|_|\__|____/


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

//     _                _ _   ____
//    / \__      ____ _(_) |_/ ___|
//   / _ \ \ /\ / / _` | | __\___ \
//  / ___ \ V  V / (_| | | |_ ___) |
// /_/   \_\_/\_/ \__,_|_|\__|____/


#[derive(Clone)]
pub struct NAwaitS<SV>(pub SignalRuntimeRef<SV>, pub usize);

impl<'a, SV: 'a> Node<'a, ()> for NAwaitS<SV>
where
    SV: SignalValue,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        self.0.await(&mut sub_runtime.tasks, self.1);
    }
}


//     _                _ _   ___                              _ _       _       ____
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___|  _ \
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \ | | |
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/ |_| |
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|____/


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

//     _                _ _   ___                              _ _       _       ____
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___/ ___|
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \___ \
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/___) |
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|____/


#[derive(Clone)]
pub struct NAwaitImmediateS<SV>(pub SignalRuntimeRef<SV>, pub usize);

impl<'a, SV: 'a> Node<'a, ()> for NAwaitImmediateS<SV>
where
    SV: SignalValue,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        self.0.await_immediate(&mut sub_runtime.tasks, self.1);
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