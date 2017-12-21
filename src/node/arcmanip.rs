use engine::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::cell::*;
use super::*;


pub(super) type AMutex<T> = Arc<Mutex<Option<T>>>;

pub fn new_amutex<T>() -> Arc<Mutex<Option<T>>> {
    Arc::new(Mutex::new(None))
}


pub struct ArcStore<T> {
    p: AMutex<T>,
}

pub fn store_par<T>(rc: AMutex<T>) -> ArcStore<T> {
    ArcStore { p: rc }
}

impl<'a, T: 'a> Node<'a, T> for ArcStore<T> {
    type Out = ();
    fn call(&mut self, _: &mut SubRuntime<'a>, val: T) {
        *self.p.lock().unwrap() = Some(val);
    }
}



pub struct ArcLoad<T> {
    p: AMutex<T>,
}

pub fn load_par<T>(rc: AMutex<T>) -> ArcLoad<T> {
    ArcLoad { p: rc }
}

impl<'a, T: 'a> Node<'a, ()> for ArcLoad<T> {
    type Out = T;
    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> T {
        *self.p.lock().unwrap()
    }
}

pub struct ArcLoadCopy<T> {
    p: AMutex<T>,
}

pub fn load_copy_par<T>(rc: AMutex<T>) -> ArcLoadCopy<T>
where
    T: Copy
{
    ArcLoadCopy { p: rc }
}

impl<'a, T: 'a> Node<'a, ()> for ArcLoadCopy<T>
where
    T: Copy,
{
    type Out = T;
    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> T {
        *self.p.lock().unwrap().copy()
    }
}
