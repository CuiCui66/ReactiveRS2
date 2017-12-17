use engine::*;
use std::rc::Rc;
use std::cell::*;
use super::*;


type RCell<T> = Rc<Cell<Option<T>>>;

pub fn new_rcell<T>() -> Rc<Cell<Option<T>>> {
    Rc::new(Cell::new(None))
}



pub struct RcStore<T> {
    p: RCell<T>,
}

pub fn store<T>(rc: RCell<T>) -> RcStore<T> {
    RcStore { p: rc }
}

impl<'a, T: 'a> Node<'a, T> for RcStore<T> {
    type Out = ();
    fn call(&mut self, _: &mut SubRuntime<'a>, val: T) {
        self.p.set(Some(val));
    }
}



pub struct RcLoad<T> {
    p: RCell<T>,
}

pub fn load<T>(rc: RCell<T>) -> RcLoad<T> {
    RcLoad { p: rc }
}

impl<'a, T: 'a> Node<'a, ()> for RcLoad<T> {
    type Out = T;
    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> T {
        self.p.take().unwrap()
    }
}

pub struct RcStoreClone<T> {
    p: RCell<T>,
}
