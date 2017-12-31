use engine::*;
use signal::*;
use super::*;


//  _____           _ _   ____
// | ____|_ __ ___ (_) |_|  _ \
// |  _| | '_ ` _ \| | __| | | |
// | |___| | | | | | | |_| |_| |
// |_____|_| |_| |_|_|\__|____/

/// Node emitting a signal.
/// The signal and the emission value is given as input of the node.
/// Also, a vector of (signal,value) can be given.
#[derive(Clone)]
pub struct NEmitD {}


impl<'a, S, E: Val<'a>> Node<'a, (S, E)> for NEmitD
where
    S: Signal<'a, E = E>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, (sr, e): (S, E)) -> () {
        sr.emit(e, sub_runtime);
        ()
    }
}

impl<'a, S, E: Val<'a>, In: Val<'a>> Node<'a, ((S, E), In)> for NEmitD
where
    S: Signal<'a, E = E>,
{
    type Out = In;

    fn call(
        &mut self,
        sub_runtime: &mut SubRuntime<'a>,
        ((sr, e), val): ((S, E), In),
    ) -> Self::Out {
        sr.emit(e, sub_runtime);
        val
    }
}


impl<'a, S, E: Val<'a>> Node<'a, Vec<(S, E)>> for NEmitD
where
    S: Signal<'a, E = E>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, vec: Vec<(S, E)>) -> () {
        for (sr, emit_value) in vec {
            sr.emit(emit_value, sub_runtime);
        }
        ()
    }
}


impl<'a, S, E: Val<'a>, In: Val<'a>> Node<'a, (Vec<(S, E)>, In)> for NEmitD
where
    S: Signal<'a, E = E>,
{
    type Out = In;

    fn call(
        &mut self,
        sub_runtime: &mut SubRuntime<'a>,
        (vec, val): (Vec<(S, E)>, In),
    ) -> Self::Out {
        for (sr, emit_value) in vec {
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


/// Node emitting a signal.
/// The signal is known at the node creation,
/// and the emission value is given as input of the node.
#[derive(Clone)]
pub struct NEmitS<S, E>(pub S, pub PhantomData<E>);

impl<'a, S, E: Val<'a>> Node<'a, E> for NEmitS<S, E>
where
    S: Signal<'a, E = E>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, emit_value: E) -> () {
        self.0.emit(emit_value, sub_runtime);
        ()
    }
}


impl<'a, In: Val<'a>, S, E: Val<'a>> Node<'a, (E, In)> for NEmitS<S, E>
where
    S: Signal<'a, E = E>,
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


/// Node emitting multiple signals.
/// The signals are known at the creation at the node,
/// and the emission values are given as input of the node.
#[derive(Clone)]
pub struct NEmitVecS<S>(pub Vec<S>);

impl<'a, S, E: Val<'a>> Node<'a, Vec<E>> for NEmitVecS<S>
where
    S: Signal<'a, E = E>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, emit_values: Vec<E>) -> () {
        if emit_values.len() != self.0.len() {
            panic!(
                "The vector given to the EmitVecS process should have the same size as the signal vector."
            )
        }

        for (sr, emit_value) in self.0.iter().zip(emit_values.into_iter()) {
            sr.emit(emit_value, sub_runtime);
        }
    }
}

impl<'a, S, E: Val<'a>, In: Val<'a>> Node<'a, (Vec<E>, In)> for NEmitVecS<S>
where
    S: Signal<'a, E = E>,
{
    type Out = In;

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, (emit_values, val): (Vec<E>, In)) -> In {
        if emit_values.len() != self.0.len() {
            panic!(
                "The vector given to the EmitVecS process should have the same size as the signal vector."
            )
        }
        for (sr, emit_value) in self.0.iter().zip(emit_values.into_iter()) {
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


/// Node emitting a signal.
/// The signal and the emission value is known at the creation of the node.
#[derive(Clone)]
pub struct NEmitVS<S, E>(pub S, pub E);

impl<'a, In: Val<'a>, S, E: Val<'a>> Node<'a, In> for NEmitVS<S, E>
where
    S: Signal<'a, E = E>,
    E: Clone,
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

/// Node emitting multiple signals.
/// The signals and the values are known at the creation of the node.
#[derive(Clone)]
pub struct NEmitVVecS<S, E>(pub Vec<(S, E)>);

impl<'a, In: Val<'a>, S, E: Val<'a>> Node<'a, In> for NEmitVVecS<S, E>
where
    S: Signal<'a, E = E>,
    E: Clone,
{
    type Out = In;

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: In) -> In {
        for &(ref sr, ref emit_value) in &self.0 {
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


/// Node getting the value of the signal at the last instant.
/// The signal is given as the node input.
#[derive(Clone, Copy)]
pub struct NGetD {}

impl<'a, S, V: Val<'a>, In: Val<'a>> Node<'a, (S, In)> for NGetD
where
    S: Signal<'a, V = V>,
{
    type Out = (V, In);

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, (sr, val): (S, In)) -> Self::Out {
        (sr.get_pre_value(sub_runtime.get_current_instant()), val)
    }
}

impl<'a, S, V: Val<'a>> Node<'a, S> for NGetD
where
    S: Signal<'a, V = V>,
{
    type Out = V;

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, sr: S) -> Self::Out {
        sr.get_pre_value(sub_runtime.get_current_instant())
    }
}


//   ____      _   ____
//  / ___| ___| |_/ ___|
// | |  _ / _ \ __\___ \
// | |_| |  __/ |_ ___) |
//  \____|\___|\__|____/


/// Node getting the value of the signal at the last instant.
/// The signal is known at the node creation.
#[derive(Clone)]
pub struct NGetS<S>(pub S);

impl<'a, S, V: Val<'a>> Node<'a, ()> for NGetS<S>
where
    S: Signal<'a, V = V>,
{
    type Out = V;

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        self.0.get_pre_value(sub_runtime.get_current_instant())
    }
}

#[derive(Clone)]
pub struct NGetSIn<S>(pub S);

impl<'a, S, V: Val<'a>, In: Val<'a>> Node<'a, In> for NGetSIn<S>
where
    S: Signal<'a, V = V>,
{
    type Out = (V, In);

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: In) -> Self::Out {
        (self.0.get_pre_value(sub_runtime.get_current_instant()), val)
    }
}


//     _                _ _   ____
//    / \__      ____ _(_) |_|  _ \
//   / _ \ \ /\ / / _` | | __| | | |
//  / ___ \ V  V / (_| | | |_| |_| |
// /_/   \_\_/\_/ \__,_|_|\__|____/

/// Node awaiting a signal to be emitted,
/// and jumping to the next node at the instant following the emission.
/// The signal is given as the node input.
#[derive(Clone, Copy)]
pub struct NAwaitD(pub usize);

impl<'a, S> Node<'a, S> for NAwaitD
where
    S: Signal<'a>,
{
    type Out = S;

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, sr: S) -> Self::Out {
        sr.await(sub_runtime, self.0);
        sr
    }
}


//     _                _ _   ____
//    / \__      ____ _(_) |_/ ___|
//   / _ \ \ /\ / / _` | | __\___ \
//  / ___ \ V  V / (_| | | |_ ___) |
// /_/   \_\_/\_/ \__,_|_|\__|____/

/// Node awaiting a signal to be emitted,
/// and jumping to the next node at the next instant following the emission.
/// The signal is fixed by the node.
#[derive(Clone)]
pub struct NAwaitS<S>(pub S, pub usize);

impl<'a, S> Node<'a, ()> for NAwaitS<S>
where
    S: Signal<'a>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        self.0.await(sub_runtime, self.1);
    }
}


//     _                _ _   ___                              _ _       _       ____
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___|  _ \
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \ | | |
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/ |_| |
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|____/

/// Node awaiting a signal to be emitted,
/// and jumping to the next node at the same instant than the emission.
/// The signal is given as the node input.
#[derive(Clone, Copy)]
pub struct NAwaitImmediateD(pub usize);

impl<'a, S> Node<'a, S> for NAwaitImmediateD
where
    S: Signal<'a>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, sr: S) -> Self::Out {
        sr.await_immediate(sub_runtime, self.0);
    }
}

//     _                _ _   ___                              _ _       _       ____
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___/ ___|
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \___ \
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/___) |
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|____/

/// Node awaiting a signal to be emitted,
/// and jumping to the next node at the same instant than the emission.
/// The signal is known at the node creation.
#[derive(Clone)]
pub struct NAwaitImmediateS<S>(pub S, pub usize);

impl<'a, S> Node<'a, ()> for NAwaitImmediateS<S>
where
    S: Signal<'a>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        self.0.await_immediate(sub_runtime, self.1);
    }
}


//  ____                           _   ____
// |  _ \ _ __ ___  ___  ___ _ __ | |_|  _ \
// | |_) | '__/ _ \/ __|/ _ \ '_ \| __| | | |
// |  __/| | |  __/\__ \  __/ | | | |_| |_| |
// |_|   |_|  \___||___/\___|_| |_|\__|____/


/// Node jumping to node_true if the signal is emitted in the current instant,
/// and jumping to node_false at the next instant otherwise.
/// The signal is given as the node input
#[derive(Clone, Copy)]
pub struct NPresentD {
    pub node_true: usize,
    pub node_false: usize,
}

impl<'a, S> Node<'a, S> for NPresentD
where
    S: Signal<'a>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, sr: S) -> Self::Out {
        sr.present(sub_runtime, self.node_true, self.node_false);
    }
}


//  ____                           _   ____
// |  _ \ _ __ ___  ___  ___ _ __ | |_/ ___|
// | |_) | '__/ _ \/ __|/ _ \ '_ \| __\___ \
// |  __/| | |  __/\__ \  __/ | | | |_ ___) |
// |_|   |_|  \___||___/\___|_| |_|\__|____/

/// Node jumping to node_true if the signal is emitted in the current instant,
/// and jumping to node_false at the next instant otherwise.
/// The signal is known at the node creation.
#[derive(Clone)]
pub struct NPresentS<S> {
    pub node_true: usize,
    pub node_false: usize,
    pub signal_runtime: S,
}

impl<'a, S> Node<'a, ()> for NPresentS<S>
where
    S: Signal<'a>,
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        self.signal_runtime.present(
            sub_runtime,
            self.node_true,
            self.node_false,
        );
    }
}
