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

pub fn store_clone<T>(rc: RCell<T>) -> RcStoreClone<T> {
    RcStoreClone { p: rc }
}

impl<'a, T: 'a> Node<'a, T> for RcStoreClone<T>
where
    T: Clone,
{
    type Out = T;
    fn call(&mut self, _: &mut SubRuntime<'a>, val: T) -> Self::Out {
        self.p.set(Some(val.clone()));
        val
    }
}



pub struct RcStoreCloneFirst<T> {
    p: RCell<T>,
}

pub fn store_clone_first<T>(rc: RCell<T>) -> RcStoreCloneFirst<T> {
    RcStoreCloneFirst { p: rc }
}

impl<'a, C: 'a, V: 'a> Node<'a, (C, V)> for RcStoreCloneFirst<(C, V)>
where
    C: Clone + 'a,
{
    type Out = C;

    fn call(&mut self, _: &mut SubRuntime<'a>, (clone_val, val): (C, V)) -> Self::Out {
        let out_val = clone_val.clone();
        self.p.set(Some((clone_val, val)));
        out_val
    }
}

