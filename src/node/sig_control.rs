use engine::*;
use signal::*;
use super::*;
use self::ControlSignal::{DoWhen, DoUntil};

pub enum ControlSignal<'a> {
    DoWhen(Box<PureSignal<'a> + 'a>),
    DoUntil(Box<PureSignal<'a> + 'a>),
}

impl<'a> Clone for ControlSignal<'a> {
    fn clone(&self) -> ControlSignal<'a> {
        match self {
            &DoWhen(ref signal) => DoWhen(signal.clone2()),
            &DoUntil(ref signal) => DoUntil(signal.clone2()),
        }
    }
}


pub(crate) struct ControlNode<'a> {
    pub(crate) id: usize,
    pub(crate) node: Box<Node<'a, (), Out=()>>,
    pub(crate) control_sig: Vec<ControlSignal<'a>>,
}


impl<'a> Node<'a, ()> for ControlNode<'a>
{
    type Out = ();

    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, _: ()) -> () {
        for control_sig in &self.control_sig {
            match control_sig {
                &DoWhen(ref signal) => {
                    if !signal.is_set(sub_runtime.current_instant) {
                        signal.await_immediate(sub_runtime, self.id);
                        return;
                    }
                },
                &DoUntil(ref signal) => {
                    if signal.pre_set(sub_runtime.current_instant) {
                        return;
                    }
                },
            }
        }
        self.node.call(sub_runtime, ());
    }
}
