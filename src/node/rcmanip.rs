use engine::*;
use std::rc::Rc;
use std::cell::*;
use super::*;


pub(super) type RCell<T> = Rc<Cell<Option<T>>>;

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
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        print!(
            "Store : {} in {}",
            tname::<T>(),
            cfgd.get_ind(Rc::into_raw(self.p.clone()))
        )
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
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        print!(
            "Load : {} in {}",
            tname::<T>(),
            cfgd.get_ind(Rc::into_raw(self.p.clone()))
        )
    }
}


pub struct RcLoadCopy<T> {
    p: RCell<T>,
}

pub fn load_copy<T>(rc: RCell<T>) -> RcLoadCopy<T>
where
    T: Copy,
{
    RcLoadCopy { p: rc }
}

impl<'a, T: 'a> Node<'a, ()> for RcLoadCopy<T>
where
    T: Copy,
{
    type Out = T;
    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> T {
        self.p.get().unwrap()
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        print!(
            "LoadCopy : {} in {}",
            tname::<T>(),
            cfgd.get_ind(Rc::into_raw(self.p.clone()))
        )
    }
}
