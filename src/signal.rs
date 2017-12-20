use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::mem;
use take::take;
use engine::{SubRuntime, Tasks, EndOfInstantCallback};
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
{
    /// Create a new signal runtime, given a structure representing its value
    fn new(signal_value: SV) -> Self {
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
}

impl SignalRuntime<PureSignalValue> {
    /// Create a new signal runtime, that has no value
    fn new_pure() -> Self {
        SignalRuntime::new(PureSignalValue::new())
    }
}

impl<E,V> SignalRuntime<MCSignalValue<E,V>>
where
    V: Clone
{
    /// Create a new signal runtime, which has a value that can be cloned
    fn new_mc(default_value: V, gather: Box<FnMut(E, &mut V)>) -> Self {
        SignalRuntime::new(MCSignalValue::new(default_value,gather))
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

/// Structure representing the values of a multi consumer signal
pub struct MCSignalValue<E,V> {

    /// The default value of the signal
    default_value: V,

    /// The value of the signal for the current instant
    current_value: V,

    /// The value of the signal at the last instant
    pre_value: V,

    /// The function used to gather the signals
    gather: Box<FnMut(E, &mut V)>,
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


//  ____  _                   _ ____              _   _                ____       __
// / ___|(_) __ _ _ __   __ _| |  _ \ _   _ _ __ | |_(_)_ __ ___   ___|  _ \ ___ / _|
// \___ \| |/ _` | '_ \ / _` | | |_) | | | | '_ \| __| | '_ ` _ \ / _ \ |_) / _ \ |_
//  ___) | | (_| | | | | (_| | |  _ <| |_| | | | | |_| | | | | | |  __/  _ <  __/  _|
// |____/|_|\__, |_| |_|\__,_|_|_| \_\\__,_|_| |_|\__|_|_| |_| |_|\___|_| \_\___|_|
//          |___/

/// A shared pointer to a signal runtime
pub struct SignalRuntimeRef<SV> {
    /// The shared signal runtime
    pub(crate) signal_runtime: Rc<RefCell<SignalRuntime<SV>>>,
}

impl SignalRuntimeRef<PureSignalValue> {
    /// Create a shared pointer to a new pure signal runtime
    pub fn new_pure() -> Self {
        SignalRuntimeRef {
            signal_runtime: Rc::new(RefCell::new(SignalRuntime::new_pure()))
        }
    }
}

impl<E,V> SignalRuntimeRef<MCSignalValue<E,V>>
where
    V: Clone
{
    /// Create a shared pointer to a new multi consumer signal runtime
    pub fn new_mc(default_value: V, gather: Box<FnMut(E, &mut V)>) -> Self {
        SignalRuntimeRef {
            signal_runtime: Rc::new(RefCell::new(SignalRuntime::new_mc(default_value,gather)))
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


impl<'a, E: 'a, V: 'a, SV: 'a> SignalRuntimeRef<SV>
where
    SV: SignalValue<E=E, V=V>,
{

    /// Process pending await nodes on signal emission
    fn process_pending_await(&self, tasks: &mut Tasks, signal_runtime: &mut RefMut<SignalRuntime<SV>>) {
        let nodes = take(&mut signal_runtime.pending_await);
        for node in nodes {
            tasks.next.push(node);
        }
    }

    /// Process pending await_immediate nodes on signal emission
    fn process_pending_await_immediate(&self, tasks: &mut Tasks, signal_runtime: &mut RefMut<SignalRuntime<SV>>) {
        let nodes = take(&mut signal_runtime.pending_await_immediate);
        for node in nodes {
            tasks.current.push(node);
        }
    }

    /// Process pending present nodes on signal emission
    fn process_pending_present(&self, tasks: &mut Tasks, signal_runtime: &mut RefMut<SignalRuntime<SV>>) {
        let nodes = take(&mut signal_runtime.pending_present);
        for node in nodes {
            tasks.current.push(node.0);
        }
    }

    /// Update the values
    pub(crate) fn update_values(&self, current_instant: usize, signal_runtime: &mut RefMut<SignalRuntime<SV>>) {
        if signal_runtime.last_update + 1 < current_instant {
            signal_runtime.values.reset_value();
            signal_runtime.values.reset_value();
            signal_runtime.last_update = current_instant;
        } else if signal_runtime.last_update < current_instant {
            signal_runtime.values.reset_value();
            signal_runtime.last_update = current_instant;
        }
    }

    /// Emit a value to the signal
    pub(crate) fn emit(&self, emit_value: E, sub_runtime: &mut SubRuntime<'a>) {
        let mut signal_runtime = self.signal_runtime.borrow_mut();

        // If the signal is already set, we are finished
        if signal_runtime.last_set == sub_runtime.current_instant {
            signal_runtime.values.gather(emit_value);
            return;
        }

        signal_runtime.pre_last_set = signal_runtime.last_set;
        signal_runtime.last_set = sub_runtime.current_instant;

        self.update_values(sub_runtime.current_instant, &mut signal_runtime);

        signal_runtime.values.gather(emit_value);

        // We process the awaiting nodes
        self.process_pending_await_immediate(&mut sub_runtime.tasks, &mut signal_runtime);
        self.process_pending_await(&mut sub_runtime.tasks, &mut signal_runtime);
        self.process_pending_present(&mut sub_runtime.tasks, &mut signal_runtime);
    }

    /// Await the signal to be emitted, and then execute the node at the next instant,
    pub(crate) fn await(&self, sub_runtime: &mut SubRuntime<'a>, node: usize) {
        let mut signal_runtime = self.signal_runtime.borrow_mut();
        if signal_runtime.last_set == sub_runtime.current_instant {
            sub_runtime.tasks.next.push(node);
        } else {
            signal_runtime.pending_await.push(node);
        }
    }

    /// Await the signal to be emitted, and then exexute the node at the current instant
    pub(crate) fn await_immediate(&self, sub_runtime: &mut SubRuntime<'a>, node: usize) {
        let mut signal_runtime = self.signal_runtime.borrow_mut();
        if signal_runtime.last_set == sub_runtime.current_instant {
            sub_runtime.tasks.current.push(node);
        } else {
            signal_runtime.pending_await_immediate.push(node);
        }
    }

    /// If the signal is present at the current instant, execute node_true.
    /// Otherwise, execute node_false at the next instant.
    pub(crate) fn present(&self, sub_runtime: &mut SubRuntime<'a>, node_true: usize, node_false: usize) {
        let mut signal_runtime = self.signal_runtime.borrow_mut();

        if signal_runtime.last_set == sub_runtime.current_instant {
            sub_runtime.tasks.current.push(node_true);
        } else {
            signal_runtime.pending_present.push((node_true, node_false));

            if signal_runtime.pending_present.len() == 1 {
                sub_runtime.eoi.pending.push(box (*self).clone());
            }
        }
    }

    /// Return true if the signal was set at the last instant
    pub fn pre_set(&self, current_instant: usize) -> bool {
        self.signal_runtime.borrow().pre_last_set + 1 == current_instant
    }
}

impl<'a, SV: 'a> EndOfInstantCallback<'a> for SignalRuntimeRef<SV>
where
    SV: SignalValue
{
    fn on_end_of_instant(&self, sub_runtime: &mut SubRuntime<'a>) {
        let mut signal_runtime = self.signal_runtime.borrow_mut();

        let nodes = take(&mut signal_runtime.pending_present);
        for node in nodes {
            sub_runtime.tasks.current.push(node.1);
        }
    }
}
