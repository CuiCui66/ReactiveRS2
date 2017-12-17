use std::rc::Rc;
use std::cell::RefCell;
use std::mem;
use take::take;
use engine::{SubRuntime, Tasks};

//  ____  _                   _ ____              _   _
// / ___|(_) __ _ _ __   __ _| |  _ \ _   _ _ __ | |_(_)_ __ ___   ___
// \___ \| |/ _` | '_ \ / _` | | |_) | | | | '_ \| __| | '_ ` _ \ / _ \
//  ___) | | (_| | | | | (_| | |  _ <| |_| | | | | |_| | | | | | |  __/
// |____/|_|\__, |_| |_|\__,_|_|_| \_\\__,_|_| |_|\__|_|_| |_| |_|\___|
//          |___/

/// Structure representing a signal runtime
pub(crate) struct SignalRuntime<SV>
{
    /// Is used to be able to do passive waiting,
    /// every instant there is an emitted value, this id increase (modulo 2)
    pub(crate) id: RefCell<i32>,
    /// Indicates if the signal is set or not at the current instant
    pub(crate) is_set: RefCell<bool>,
    /// Indicates if the signal was set or not at the last instant
    pub(crate) pre_set: RefCell<bool>,
    /// Contains the nodes id
    pub(crate) pending_await: RefCell<Vec<usize>>,
    pub(crate) pending_await_immediate: RefCell<Vec<usize>>,
    pub(crate) pending_present: RefCell<Vec<(usize,usize)>>,
    pub(crate) values: SV
}

impl<SV> SignalRuntime<SV>
{
    fn new(signal_value: SV) -> Self {
        SignalRuntime {
            id: RefCell::new(0),
            is_set: RefCell::new(false),
            pre_set: RefCell::new(false),
            pending_await: RefCell::new(vec![]),
            pending_await_immediate: RefCell::new(vec![]),
            pending_present: RefCell::new(vec![]),
            values: signal_value
        }
    }
}

impl SignalRuntime<PureSignalValue> {
    pub fn new_pure() -> Self {
        SignalRuntime::new(PureSignalValue::new())
    }
}

impl<E,V> SignalRuntime<MCSignalValue<E,V>>
where
    V: Clone
{
    pub fn new_mc(default_value: V, gather: Box<FnMut(E, &mut V)>) -> Self {
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
    type E;
    type V;
    fn get_pre_value(&self) -> Self::V;
    fn gather(&self, emit_value: Self::E);
    fn reset_value(&self);
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
    fn gather(&self, _emit_value: ()) {}
    fn reset_value(&self) {}
}


//  __  __  ____ ____  _                   ___     __    _
// |  \/  |/ ___/ ___|(_) __ _ _ __   __ _| \ \   / /_ _| |_   _  ___
// | |\/| | |   \___ \| |/ _` | '_ \ / _` | |\ \ / / _` | | | | |/ _ \
// | |  | | |___ ___) | | (_| | | | | (_| | | \ V / (_| | | |_| |  __/
// |_|  |_|\____|____/|_|\__, |_| |_|\__,_|_|  \_/ \__,_|_|\__,_|\___|
//                       |___/

pub struct MCSignalValue<E,V> {
    default_value: V,
    current_value: RefCell<V>,
    pre_value: RefCell<V>,
    gather: RefCell<Box<FnMut(E, &mut V)>>,
}

impl<E,V> MCSignalValue<E,V>
where
    V: Clone
{
    fn new(default_value: V, gather: Box<FnMut(E, &mut V)>) -> Self {
        MCSignalValue {
            default_value: default_value.clone(),
            current_value: RefCell::new(default_value.clone()),
            pre_value: RefCell::new(default_value),
            gather: RefCell::new(gather),
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
        self.pre_value.borrow().clone()
    }

    fn gather(&self, emit_value: E) {
        (&mut *self.gather.borrow_mut())(emit_value, &mut *self.current_value.borrow_mut())
    }

    fn reset_value(&self) {
        let mut current_value = self.current_value.borrow_mut();
        mem::swap(&mut *self.pre_value.borrow_mut(), &mut *current_value);
        *current_value = self.default_value.clone();
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
    pub(crate) signal_runtime: Rc<SignalRuntime<SV>>,
}

impl SignalRuntimeRef<PureSignalValue> {
    pub fn new_pure() -> Self {
        SignalRuntimeRef {
            signal_runtime: Rc::new(SignalRuntime::new_pure())
        }
    }
}

impl<E,V> SignalRuntimeRef<MCSignalValue<E,V>>
where
    V: Clone
{
    pub fn new_mc(default_value: V, gather: Box<FnMut(E, &mut V)>) -> Self {
        SignalRuntimeRef {
            signal_runtime: Rc::new(SignalRuntime::new_mc(default_value,gather))
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


impl<'a, E, V, SV> SignalRuntimeRef<SV>
where
    E: 'a,
    V: 'a,
    SV: SignalValue<E=E, V=V> + 'a,
{

    /// Process pending await nodes
    fn process_pending_await(&self, tasks: &mut Tasks) {
        let mut nodes = take(&mut *self.signal_runtime.pending_await.borrow_mut());
        for node in nodes {
            tasks.next.push(node);
        }
    }

    /// Process pending await nodes
    fn process_pending_await_immediate(&self, tasks: &mut Tasks) {
        let mut nodes = take(&mut *self.signal_runtime.pending_await_immediate.borrow_mut());
        for node in nodes {
            tasks.current.push(node);
        }
    }

    fn process_pending_present(&self, tasks: &mut Tasks) {
        let mut nodes = take(&mut *self.signal_runtime.pending_present.borrow_mut());
        for node in nodes {
            tasks.next.push(node.0);
        }
    }

    /// Emit a value to the signal
    pub(crate) fn emit(&self, emit_value: E, sub_runtime: &mut SubRuntime<'a>) {
        self.signal_runtime.values.gather(emit_value);

        if *self.signal_runtime.is_set.borrow() {
            return;
        }
        *self.signal_runtime.is_set.borrow_mut() = true;

        {
            let mut id = self.signal_runtime.id.borrow_mut();
            *id += 1;
            *id %= 42;
        }

        self.process_pending_await_immediate(&mut sub_runtime.tasks);
        self.process_pending_await(&mut sub_runtime.tasks);
        self.process_pending_present(&mut sub_runtime.tasks);

        let signal_runtime_ref = (*self).clone();
        let current_id = *self.signal_runtime.id.borrow();


        sub_runtime.eoi.continuations.push(box move |sr: &mut SubRuntime<'a>| {
            *signal_runtime_ref.signal_runtime.pre_set.borrow_mut() = true;
            *signal_runtime_ref.signal_runtime.is_set.borrow_mut() = false;
            signal_runtime_ref.signal_runtime.values.reset_value();

            let signal_runtime_ref2 = signal_runtime_ref.clone();

            // Update pre_set if no emit is made in the next instant
            // Since the id is modified each instant that has an emit,
            sr.eoi.continuations.push(box move |sr: &mut SubRuntime<'a>| {
                let future_id = *signal_runtime_ref2.signal_runtime.id.borrow();
                if future_id == current_id {
                    signal_runtime_ref2.signal_runtime.values.reset_value();
                    *signal_runtime_ref2.signal_runtime.pre_set.borrow_mut() = false;
                }
            });
        });
    }

    pub(crate) fn await(&self, tasks: &mut Tasks, node: usize) {
        if *self.signal_runtime.is_set.borrow() {
            tasks.next.push(node);
        } else {
            self.signal_runtime.pending_await.borrow_mut().push(node);
        }
    }

    pub(crate) fn await_immediate(&self, tasks: &mut Tasks, node: usize) {
        if *self.signal_runtime.is_set.borrow() {
            tasks.current.push(node);
        } else {
            self.signal_runtime.pending_await_immediate.borrow_mut().push(node);
        }
    }

    pub(crate) fn present(&self, sr: &mut SubRuntime<'a>, node_true: usize, node_false: usize) {
        if *self.signal_runtime.is_set.borrow() {
            sr.tasks.current.push(node_true);
        } else {
            let signal_runtime_ref = self.clone();
            sr.eoi.continuations.push(box move |sr: &mut SubRuntime| {
                let mut nodes = take(&mut *signal_runtime_ref.signal_runtime.pending_present.borrow_mut());
                for node in nodes {
                    sr.tasks.current.push(node.1);
                }
            });
            self.signal_runtime.pending_present.borrow_mut().push((node_true, node_false));
        }
    }

    pub fn pre(&self) -> V {
        self.signal_runtime.values.get_pre_value()
    }
}
