use std::mem;
use take::take;
use engine::{SubRuntime, Tasks, EndOfInstantCallback};
use std::sync::{Arc, Mutex, MutexGuard};
use signal::*;

//  ____  _                   _ ____              _   _                ____       __
// / ___|(_) __ _ _ __   __ _| |  _ \ _   _ _ __ | |_(_)_ __ ___   ___|  _ \ ___ / _|
// \___ \| |/ _` | '_ \ / _` | | |_) | | | | '_ \| __| | '_ ` _ \ / _ \ |_) / _ \ |_
//  ___) | | (_| | | | | (_| | |  _ <| |_| | | | | |_| | | | | | |  __/  _ <  __/  _|
// |____/|_|\__, |_| |_|\__,_|_|_| \_\\__,_|_| |_|\__|_|_| |_| |_|\___|_| \_\___|_|
//          |___/

/// A shared pointer to a signal runtime
pub struct SignalRuntimeParRef<SV> {
    /// The shared signal runtime
    pub(crate) signal_runtime: Arc<Mutex<SignalRuntime<SV>>>,
}

impl SignalRuntimeParRef<PureSignalValue> {
    /// Create a shared pointer to a new pure signal runtime
    pub fn new_pure() -> Self {
        SignalRuntimeParRef {
            signal_runtime: Arc::new(Mutex::new(SignalRuntime::new_pure()))
        }
    }
}

impl<E,V> SignalRuntimeParRef<MCSignalValue<E,V>>
where
    V: Clone
{
    /// Create a shared pointer to a new multi consumer signal runtime
    pub fn new_mc(default_value: V, gather: Box<FnMut(E, &mut V)>) -> Self {
        SignalRuntimeParRef {
            signal_runtime: Arc::new(Mutex::new(SignalRuntime::new_mc(default_value,gather)))
        }
    }
}

impl<SV> SignalRuntimeParRef<SV>
where
    SV: SignalValue
{
    /// Create a shared pointer to a new signal runtime, given its value manager
    pub fn new(signal_value: SV) -> Self {
        SignalRuntimeParRef {
            signal_runtime: Arc::new(Mutex::new(SignalRuntime::new(signal_value)))
        }
    }
}


impl<SV> Clone for SignalRuntimeParRef<SV>
{
    fn clone(&self) -> Self {
        SignalRuntimeParRef {
            signal_runtime: self.signal_runtime.clone(),
        }
    }
}


impl<'a, E: 'a, V: 'a, SV: 'a> SignalRuntimeParRef<SV>
where
    SV: SignalValue<E=E, V=V>,
{

    /// Process pending await nodes on signal emission
    fn process_pending_await(&self, tasks: &mut Tasks, signal_runtime: &mut MutexGuard<SignalRuntime<SV>>) {
        let nodes = take(&mut signal_runtime.pending_await);
        for node in nodes {
            tasks.next.push(node);
        }
    }

    /// Process pending await_immediate nodes on signal emission
    fn process_pending_await_immediate(&self, tasks: &mut Tasks, signal_runtime: &mut MutexGuard<SignalRuntime<SV>>) {
        let nodes = take(&mut signal_runtime.pending_await_immediate);
        for node in nodes {
            tasks.current.push(node);
        }
    }

    /// Process pending present nodes on signal emission
    fn process_pending_present(&self, tasks: &mut Tasks, signal_runtime: &mut MutexGuard<SignalRuntime<SV>>) {
        let nodes = take(&mut signal_runtime.pending_present);
        for node in nodes {
            tasks.current.push(node.0);
        }
    }

    /// Update the values
    pub(crate) fn update_values(&self, current_instant: usize, signal_runtime: &mut MutexGuard<SignalRuntime<SV>>) {
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
        let mut signal_runtime = self.signal_runtime.lock().unwrap();

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
        let mut signal_runtime = self.signal_runtime.lock().unwrap();
        if signal_runtime.last_set == sub_runtime.current_instant {
            sub_runtime.tasks.next.push(node);
        } else {
            signal_runtime.pending_await.push(node);
        }
    }

    /// Await the signal to be emitted, and then exexute the node at the current instant
    pub(crate) fn await_immediate(&self, sub_runtime: &mut SubRuntime<'a>, node: usize) {
        let mut signal_runtime = self.signal_runtime.lock().unwrap();
        if signal_runtime.last_set == sub_runtime.current_instant {
            sub_runtime.tasks.current.push(node);
        } else {
            signal_runtime.pending_await_immediate.push(node);
        }
    }

    /// If the signal is present at the current instant, execute node_true.
    /// Otherwise, execute node_false at the next instant.
    pub(crate) fn present(&self, sub_runtime: &mut SubRuntime<'a>, node_true: usize, node_false: usize) {
        let mut signal_runtime = self.signal_runtime.lock().unwrap();

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
        self.signal_runtime.lock().unwrap().pre_last_set + 1 == current_instant
    }
}

impl<'a, SV: 'a> EndOfInstantCallback<'a> for SignalRuntimeParRef<SV>
where
    SV: SignalValue
{
    fn on_end_of_instant(&self, sub_runtime: &mut SubRuntime<'a>) {
        let mut signal_runtime = self.signal_runtime.lock().unwrap();

        let nodes = take(&mut signal_runtime.pending_present);
        for node in nodes {
            sub_runtime.tasks.current.push(node.1);
        }
    }
}
