use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::mem;
use take::take;
use engine::{SubRuntime, Tasks, EndOfInstantCallback};
use super::*;

//  ____  _                   _ ____              _   _
// / ___|(_) __ _ _ __   __ _| |  _ \ _   _ _ __ | |_(_)_ __ ___   ___
// \___ \| |/ _` | '_ \ / _` | | |_) | | | | '_ \| __| | '_ ` _ \ / _ \
//  ___) | | (_| | | | | (_| | |  _ <| |_| | | | | |_| | | | | | |  __/
// |____/|_|\__, |_| |_|\__,_|_|_| \_\\__,_|_| |_|\__|_|_| |_| |_|\___|
//          |___/

/// Structure representing a signal runtime
pub(crate) struct SignalRuntime<SV>
{
    /// The last instant where the signal was set
    pub(crate) last_set: usize,

    /// The before last instant where the signal was set
    pub(crate) pre_last_set: usize,

    /// The last instant where the values were updated
    pub(crate) last_update: usize,

    /// Contains the ids of the nodes that await the signal
    pub(crate) pending_await: Vec<usize>,

    /// Contains the ids of the nodes that await_immediate the signal
    pub(crate) pending_await_immediate: Vec<usize>,

    /// Contains the ids of the nodes that present the signal
    pub(crate) pending_present: Vec<(usize,usize)>,

    /// Contains the values of the signal
    pub(crate) values: SV,
}


impl<SV> SignalRuntime<SV>
where
    SV: SignalValue,
{
    /// Create a new signal runtime, given a structure representing its value
    pub(crate) fn new(signal_value: SV) -> Self
    {
        SignalRuntime {
            last_set: 0,
            pre_last_set: 0,
            last_update: 2,
            pending_await: vec![],
            pending_await_immediate: vec![],
            pending_present: vec![],
            values: signal_value
        }
    }

    /// Process pending await nodes on signal emission
    fn process_pending_await<'a>(&mut self, sub_runtime: &mut SubRuntime<'a>) {
        let nodes = take(&mut self.pending_await);
        for node in nodes {
            sub_runtime.add_next(node);
        }
    }

    /// Process pending await_immediate nodes on signal emission
    fn process_pending_await_immediate<'a>(&mut self, sub_runtime: &mut SubRuntime<'a>) {
        let nodes = take(&mut self.pending_await_immediate);
        for node in nodes {
            sub_runtime.add_current(node);
        }
    }

    /// Process pending present nodes on signal emission
    fn process_pending_present<'a>(&mut self, sub_runtime: &mut SubRuntime<'a>) {
        let nodes = take(&mut self.pending_present);
        for node in nodes {
            sub_runtime.add_current(node.0);
        }
    }

    /// Update the values
    fn update_values(&mut self, current_instant: usize) {
        if self.last_update + 1 < current_instant {
            self.values.reset_value();
            self.values.reset_value();
            self.last_update = current_instant;
        } else if self.last_update < current_instant {
            self.values.reset_value();
            self.last_update = current_instant;
        }
    }

    /// Await the signal to be emitted, and then execute the node at the next instant,
    pub(crate) fn await(&mut self, sub_runtime: &mut SubRuntime, node: usize) {
        if self.last_set == sub_runtime.get_current_instant() {
            sub_runtime.add_next(node);
        } else {
            self.pending_await.push(node);
        }
    }

    /// Await the signal to be emitted, and then exexute the node at the current instant
    pub(crate) fn await_immediate(&mut self, sub_runtime: &mut SubRuntime, node: usize) {
        if self.last_set == sub_runtime.get_current_instant() {
            sub_runtime.add_current(node);
        } else {
            self.pending_await_immediate.push(node);
        }
    }

    /// If the signal is present at the current instant, execute node_true.
    /// Otherwise, execute node_false at the next instant.
    fn present<'a>(&mut self, sub_runtime: &mut SubRuntime<'a>, node_true: usize, node_false: usize) {
        if self.last_set == sub_runtime.get_current_instant() {
            sub_runtime.add_current(node_true);
        } else {
            self.pending_present.push((node_true, node_false));

            /*if self.pending_present.len() == 1 {
                sub_runtime.eoi.pending.push(box (*self).clone());
            }*/
        }
    }

    /// Return true if the signal was set at the last instant
    fn pre_set(&self, current_instant: usize) -> bool {
        self.pre_last_set + 1 == current_instant
    }


    /// Return true if the signal is set at the current instant
    /// This function should not be used in user mode, but Rust do not allow us to put
    /// this function in pub(crate), since it is part of a public trait
    fn is_set(&self, current_instant: usize) -> bool {
        self.last_set == current_instant
    }

    /// Emit a value to the signal
    fn emit(&mut self, emit_value: SV::E, sub_runtime: &mut SubRuntime) {
        // If the signal is already set, we are finished
        if self.last_set == sub_runtime.get_current_instant() {
            self.values.gather(emit_value);
            return;
        }

        self.pre_last_set = self.last_set;
        self.last_set = sub_runtime.get_current_instant();

        self.update_values(sub_runtime.get_current_instant());

        self.values.gather(emit_value);

        // We process the awaiting nodes
        self.process_pending_await_immediate(sub_runtime);
        self.process_pending_await(sub_runtime);
        self.process_pending_present(sub_runtime);
    }

    /// Return the value of the last instant
    fn get_pre_value(&mut self, current_instant: usize) -> SV::V {
        self.update_values(current_instant);
        self.values.get_pre_value()
    }

    fn on_end_of_instant(&mut self, sub_runtime: &mut SubRuntime) {
        let nodes = take(&mut self.pending_present);
        for node in nodes {
            sub_runtime.add_current(node.1);
        }
    }
}


impl SignalRuntime<PureSignalValue> {
    /// Create a new signal runtime, that has no value
    pub(crate) fn new_pure() -> Self {
        SignalRuntime::new(PureSignalValue::new())
    }
}


//  ____  _                   ___     __    _
// / ___|(_) __ _ _ __   __ _| \ \   / /_ _| |_   _  ___
// \___ \| |/ _` | '_ \ / _` | |\ \ / / _` | | | | |/ _ \
//  ___) | | (_| | | | | (_| | | \ V / (_| | | |_| |  __/
// |____/|_|\__, |_| |_|\__,_|_|  \_/ \__,_|_|\__,_|\___|
//          |___/

/// Trait used to represent the values stored in a signal runtime
pub trait SignalValue {

    /// The type of the values that are emitted
    type E;

    /// The value of the signal that is stored
    type V;

    /// Get the value of the signal of the last instant
    fn get_pre_value(&self) -> Self::V;

    /// Gather the emitted value
    fn gather(&mut self, emit_value: Self::E);

    /// Reset the value stored by the signal,
    /// and stored the last one as the last instant value
    fn reset_value(&mut self);
}


//  ____                 ____  _                   ___     __    _
// |  _ \ _   _ _ __ ___/ ___|(_) __ _ _ __   __ _| \ \   / /_ _| |_   _  ___
// | |_) | | | | '__/ _ \___ \| |/ _` | '_ \ / _` | |\ \ / / _` | | | | |/ _ \
// |  __/| |_| | | |  __/___) | | (_| | | | | (_| | | \ V / (_| | | |_| |  __/
// |_|    \__,_|_|  \___|____/|_|\__, |_| |_|\__,_|_|  \_/ \__,_|_|\__,_|\___|
//                               |___/

/// Structure representing the values of a pure signal
pub struct PureSignalValue {}

impl PureSignalValue {
    fn new() -> Self {
        PureSignalValue{}
    }
}

impl SignalValue for PureSignalValue {
    type E = ();
    type V = ();
    fn get_pre_value(&self) -> () {}
    fn gather(&mut self, _emit_value: ()) {}
    fn reset_value(&mut self) {}
}


//  __  __  ____ ____  _                   ___     __    _
// |  \/  |/ ___/ ___|(_) __ _ _ __   __ _| \ \   / /_ _| |_   _  ___
// | |\/| | |   \___ \| |/ _` | '_ \ / _` | |\ \ / / _` | | | | |/ _ \
// | |  | | |___ ___) | | (_| | | | | (_| | | \ V / (_| | | |_| |  __/
// |_|  |_|\____|____/|_|\__, |_| |_|\__,_|_|  \_/ \__,_|_|\__,_|\___|
//                       |___/

#[cfg(not(feature = "par"))]
mod mcvalue_content {
    use super::*;

    /// Structure representing the values of a multi consumer signal
    pub struct MCSignalValue<E, V> {
        /// The default value of the signal
        pub(crate) default_value: V,

        /// The value of the signal for the current instant
        pub(crate) current_value: V,

        /// The value of the signal at the last instant
        pub(crate) pre_value: V,

        /// The function used to gather the signals
        pub(crate) gather: Box<FnMut(E, &mut V)>,
    }

    impl<E,V> MCSignalValue<E,V>
    where
        V: Clone
    {
        /// Creates a new multi consumer signal, given a default value and a gather function
        fn new(default_value: V, gather: Box<FnMut(E, &mut V)>) -> Self {
            MCSignalValue {
                default_value: default_value.clone(),
                current_value: default_value.clone(),
                pre_value: default_value,
                gather,
            }
        }
    }

    impl<E,V> SignalRuntime<MCSignalValue<E,V>>
    where
        V: Clone
    {
        /// Create a new signal runtime, which has a value that can be cloned
        pub(crate) fn new_mc(default_value: V, gather: Box<FnMut(E, &mut V)>) -> Self {
            SignalRuntime::new(MCSignalValue::new(default_value,gather))
        }
    }
}

#[cfg(feature = "par")]
mod mcvalue_content {
    use super::*;

    /// Structure representing the values of a multi consumer signal
    pub struct MCSignalValue<E, V> {
        /// The default value of the signal
        pub(crate) default_value: V,

        /// The value of the signal for the current instant
        pub(crate) current_value: V,

        /// The value of the signal at the last instant
        pub(crate) pre_value: V,

        /// The function used to gather the signals
        pub(crate) gather: Box<FnMut(E, &mut V) + Send>,
    }

    impl<E,V> MCSignalValue<E,V>
    where
        V: Clone
    {
        /// Creates a new multi consumer signal, given a default value and a gather function
        pub fn new(default_value: V, gather: Box<FnMut(E, &mut V) + Send>) -> Self {
            MCSignalValue {
                default_value: default_value.clone(),
                current_value: default_value.clone(),
                pre_value: default_value,
                gather,
            }
        }
    }

    impl<E,V> SignalRuntime<MCSignalValue<E,V>>
        where
            V: Clone
    {
        /// Create a new signal runtime, which has a value that can be cloned
        pub(crate) fn new_mc(default_value: V, gather: Box<FnMut(E, &mut V) + Send>) -> Self {
            SignalRuntime::new(MCSignalValue::new(default_value,gather))
        }
    }
}

pub use self::mcvalue_content::*;



impl<E,V> SignalValue for MCSignalValue<E,V>
where
    V: Clone
{
    type E = E;
    type V = V;
    fn get_pre_value(&self) -> V {
        self.pre_value.clone()
    }

    fn gather(&mut self, emit_value: E) {
        (&mut self.gather)(emit_value, &mut self.current_value)
    }

    fn reset_value(&mut self) {
        mem::swap(&mut self.pre_value, &mut self.current_value);
        self.current_value = self.default_value.clone();
    }
}


//  ____  _                   _
// / ___|(_) __ _ _ __   __ _| |
// \___ \| |/ _` | '_ \ / _` | |
//  ___) | | (_| | | | | (_| | |
// |____/|_|\__, |_| |_|\__,_|_|
//          |___/


/// Trait only used to store many signals in one vector, without having problems
/// with their types
/// Note that they cannot be Clone to be able to have trait object
pub trait PureSignal<'a> : Val<'a> {
    fn await(&self, sub_runtime: &mut SubRuntime<'a>, node: usize);
    fn await_immediate(&self, sub_runtime: &mut SubRuntime<'a>, node: usize);
    fn present(&self, sub_runtime: &mut SubRuntime<'a>, node_true: usize, node_false: usize);
    fn pre_set(&self, current_instant: usize) -> bool;

    /// This function should not be used in user mode, but Rust do not allow us to put
    /// this function in pub(crate), since it is part of a public trait
    fn is_set(&self, current_instant: usize) -> bool;

    /// This function is used to clone the signal without knowing statically its struct
    fn clone2(&self) -> Box<PureSignal<'a> + 'a>;
}

/// A Signal with a value
pub trait Signal<'a>: PureSignal<'a> {
    type E;
    type V;

    fn emit(&self, emit_value: Self::E, sub_runtime: &mut SubRuntime<'a>);
    fn get_pre_value(&self, current_instant: usize) -> Self::V;
}


//  ____  _                   _ ____              _   _                ____       __
// / ___|(_) __ _ _ __   __ _| |  _ \ _   _ _ __ | |_(_)_ __ ___   ___|  _ \ ___ / _|
// \___ \| |/ _` | '_ \ / _` | | |_) | | | | '_ \| __| | '_ ` _ \ / _ \ |_) / _ \ |_
//  ___) | | (_| | | | | (_| | |  _ <| |_| | | | | |_| | | | | | |  __/  _ <  __/  _|
// |____/|_|\__, |_| |_|\__,_|_|_| \_\\__,_|_| |_|\__|_|_| |_| |_|\___|_| \_\___|_|
//          |___/


/// A shared pointer to a signal runtime

#[cfg(not(feature = "par"))]
mod content {
    use std::rc::Rc;
    use std::cell::RefCell;
    use super::*;

    pub struct SignalRuntimeRef<SV> {
        pub ( crate ) signal_runtime: Rc<RefCell<SignalRuntime<SV>>>,
    }

    impl SignalRuntimeRef<PureSignalValue> {
        /// Create a shared pointer to a new pure signal runtime
        pub fn new_pure() -> Self {
            SignalRuntimeRef {
                signal_runtime: Rc::new(RefCell::new(SignalRuntime::new_pure()))
            }
        }
    }

    impl<E, V> SignalRuntimeRef<MCSignalValue<E, V>>
        where
            V: Clone
    {
        /// Create a shared pointer to a new multi consumer signal runtime
        pub fn new_mc(default_value: V, gather: Box<FnMut(E, &mut V)>) -> Self {
            SignalRuntimeRef {
                signal_runtime: Rc::new(RefCell::new(SignalRuntime::new_mc(default_value, gather)))
            }
        }
    }

    impl<SV> SignalRuntimeRef<SV>
        where
            SV: SignalValue
    {
        /// Create a shared pointer to a new signal runtime, given its value manager
        pub fn new(signal_value: SV) -> Self {
            SignalRuntimeRef {
                signal_runtime: Rc::new(RefCell::new(SignalRuntime::new(signal_value)))
            }
        }
    }


    impl<SV> Clone for SignalRuntimeRef<SV>
    {
        fn clone(&self) -> Self {
            SignalRuntimeRef {
                signal_runtime: self.signal_runtime.clone(),
            }
        }
    }


    impl<'a, E: 'a, V: 'a, SV: 'a> PureSignal<'a> for SignalRuntimeRef<SV>
    where
        SV: SignalValue<E=E,V=V>,
    {
        /// Await the signal to be emitted, and then execute the node at the next instant,
        fn await(&self, sub_runtime: &mut SubRuntime<'a>, node: usize) {
            self.signal_runtime.borrow_mut().await(sub_runtime, node);
        }

        /// Await the signal to be emitted, and then exexute the node at the current instant
        fn await_immediate(&self, sub_runtime: &mut SubRuntime<'a>, node: usize) {
            self.signal_runtime.borrow_mut().await_immediate(sub_runtime, node);
        }

        /// If the signal is present at the current instant, execute node_true.
        /// Otherwise, execute node_false at the next instant.
        fn present(&self, sub_runtime: &mut SubRuntime<'a>, node_true: usize, node_false: usize) {
            let mut signal_runtime = self.signal_runtime.borrow_mut();
            signal_runtime.present(sub_runtime, node_true, node_false);

            if signal_runtime.pending_present.len() == 1 {
                sub_runtime.add_eoi(box (*self).clone());
            }
        }

        /// Return true if the signal was set at the last instant
        fn pre_set(&self, current_instant: usize) -> bool {
            self.signal_runtime.borrow_mut().pre_set(current_instant)
        }


        /// Return true if the signal is set at the current instant
        /// This function should not be used in user mode, but Rust do not allow us to put
        /// this function in pub(crate), since it is part of a public trait
        fn is_set(&self, current_instant: usize) -> bool {
            self.signal_runtime.borrow_mut().is_set(current_instant)
        }


        /// This function is used to clone the signal without knowing statically its struct
        fn clone2(&self) -> Box<PureSignal<'a> + 'a> {
            box self.clone()
        }
    }

    impl<'a, E:'a, V: 'a, SV: 'a> Signal<'a> for SignalRuntimeRef<SV>
        where
            SV: SignalValue<E=E,V=V>
    {
        type E = E;
        type V = V;

        /// Emit a value to the signal
        fn emit(&self, emit_value: E, sub_runtime: &mut SubRuntime<'a>) {
            self.signal_runtime.borrow_mut().emit(emit_value, sub_runtime)
        }

        /// Return the value of the last instant
        fn get_pre_value(&self, current_instant: usize) -> V {
            self.signal_runtime.borrow_mut().get_pre_value(current_instant)
        }
    }

    impl<'a, SV: 'a> EndOfInstantCallback<'a> for SignalRuntimeRef<SV>
        where
            SV: SignalValue
    {
        fn on_end_of_instant(&self, sub_runtime: &mut SubRuntime<'a>) {
            self.signal_runtime.borrow_mut().on_end_of_instant(sub_runtime);
        }
    }
}

#[cfg(feature = "par")]
mod content {
    use std::sync::{Arc, Mutex};
    use super::*;

    pub struct SignalRuntimeRef<SV> {
        pub ( crate ) signal_runtime: Arc<Mutex<SignalRuntime<SV>>>,
    }

    impl SignalRuntimeRef<PureSignalValue> {
        /// Create a shared pointer to a new pure signal runtime
        pub fn new_pure() -> Self {
            SignalRuntimeRef {
                signal_runtime: Arc::new(Mutex::new(SignalRuntime::new_pure()))
            }
        }
    }

    impl<E, V> SignalRuntimeRef<MCSignalValue<E, V>>
    where
        V: Clone
    {
        /// Create a shared pointer to a new multi consumer signal runtime
        pub fn new_mc(default_value: V, gather: Box<FnMut(E, &mut V) + Send>) -> Self {
            SignalRuntimeRef {
                signal_runtime: Arc::new(Mutex::new(SignalRuntime::new_mc(default_value, gather)))
            }
        }
    }

    impl<SV> SignalRuntimeRef<SV>
        where
            SV: SignalValue
    {
        /// Create a shared pointer to a new signal runtime, given its value manager
        pub fn new(signal_value: SV) -> Self {
            SignalRuntimeRef {
                signal_runtime: Arc::new(Mutex::new(SignalRuntime::new(signal_value)))
            }
        }
    }


    impl<SV> Clone for SignalRuntimeRef<SV>
    {
        fn clone(&self) -> Self {
            SignalRuntimeRef {
                signal_runtime: self.signal_runtime.clone(),
            }
        }
    }


    impl<'a, E: 'a, V: 'a, SV: 'a> PureSignal<'a> for SignalRuntimeRef<SV>
    where
        SV: SignalValue<E=E,V=V> + Send,
    {
        /// Await the signal to be emitted, and then execute the node at the next instant,
        fn await(&self, sub_runtime: &mut SubRuntime<'a>, node: usize) {
            self.signal_runtime.lock().unwrap().await(sub_runtime, node);
        }

        /// Await the signal to be emitted, and then exexute the node at the current instant
        fn await_immediate(&self, sub_runtime: &mut SubRuntime<'a>, node: usize) {
            self.signal_runtime.lock().unwrap().await_immediate(sub_runtime, node);
        }

        /// If the signal is present at the current instant, execute node_true.
        /// Otherwise, execute node_false at the next instant.
        fn present(&self, sub_runtime: &mut SubRuntime<'a>, node_true: usize, node_false: usize) {
            let mut signal_runtime = self.signal_runtime.lock().unwrap();
            signal_runtime.present(sub_runtime, node_true, node_false);

            if signal_runtime.pending_present.len() == 1 {
                sub_runtime.add_eoi(box (*self).clone());
            }
        }

        /// Return true if the signal was set at the last instant
        fn pre_set(&self, current_instant: usize) -> bool {
            self.signal_runtime.lock().unwrap().pre_set(current_instant)
        }


        /// Return true if the signal is set at the current instant
        /// This function should not be used in user mode, but Rust do not allow us to put
        /// this function in pub(crate), since it is part of a public trait
        fn is_set(&self, current_instant: usize) -> bool {
            self.signal_runtime.lock().unwrap().is_set(current_instant)
        }


        /// This function is used to clone the signal without knowing statically its struct
        fn clone2(&self) -> Box<PureSignal<'a> + 'a> {
            box self.clone()
        }
    }

    impl<'a, E:'a, V: 'a, SV: 'a> Signal<'a> for SignalRuntimeRef<SV>
    where
        SV: SignalValue<E=E,V=V> + Send
    {
        type E = E;
        type V = V;

        /// Emit a value to the signal
        fn emit(&self, emit_value: E, sub_runtime: &mut SubRuntime<'a>) {
            self.signal_runtime.lock().unwrap().emit(emit_value, sub_runtime)
        }

        /// Return the value of the last instant
        fn get_pre_value(&self, current_instant: usize) -> V {
            self.signal_runtime.lock().unwrap().get_pre_value(current_instant)
        }
    }

    impl<'a, SV: Val<'a>> EndOfInstantCallback<'a> for SignalRuntimeRef<SV>
        where
            SV: SignalValue
    {
        fn on_end_of_instant(&self, sub_runtime: &mut SubRuntime<'a>) {
            self.signal_runtime.lock().unwrap().on_end_of_instant(sub_runtime);
        }
    }
}

pub use self::content::*;


