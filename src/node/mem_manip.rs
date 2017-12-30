use engine::*;
use super::*;

#[cfg(not(feature = "par"))]
mod content {
    use std::rc::Rc;
    use std::cell::Cell;
    use super::*;

    pub struct RCell<T>(Rc<Cell<Option<T>>>);

    impl<T> Clone for RCell<T>{
        fn clone(&self) -> Self{
            RCell(self.0.clone())
        }
    }

    impl<T> RCell<T> {
        pub fn new() -> Self {
            RCell(Rc::new(Cell::new(None)))
        }
        pub fn set(&self, t: T) {
            self.0.set(Some(t));
        }
        pub fn get(&self) -> T {
            self.0.take().unwrap()
        }
        pub fn get_copy(&self) -> T
        where
            T: Copy,
        {
            self.0.get().unwrap()
        }
        pub fn get_ind(&self, cfgd: &mut CFGDrawer) -> usize {
            cfgd.get_ind(Rc::into_raw(self.0.clone()))
        }
    }
}

#[cfg(all(feature = "par", not(feature = "funsafe")))]
mod content {
    use std::sync::Arc;
    use std::sync::Mutex;
    use super::*;

    pub struct RCell<T>(Arc<Mutex<Option<T>>>);

    impl<T: Send> Clone for RCell<T>{
        fn clone(&self) -> Self{
            RCell(self.0.clone())
        }
    }

    impl<T: Send> RCell<T> {
        pub fn new() -> Self {
            RCell(Arc::new(Mutex::new(None)))
        }
        pub fn set(&self, t: T) {
            (*self.0.lock().unwrap()) = Some(t);
        }
        pub fn get(&self) -> T {
            (*self.0.lock().unwrap()).take().unwrap()
        }
        pub fn get_copy(&self) -> T
        where
            T: Copy,
        {
            (*self.0.lock().unwrap()).unwrap()
        }
        pub fn get_ind(&self, cfgd: &mut CFGDrawer) -> usize {
            cfgd.get_ind(Arc::into_raw(self.0.clone()))
        }
    }

}

#[cfg(all(feature = "par", feature = "funsafe"))]
mod content {
    use std::sync::Arc;
    use std::sync::Mutex;
    use super::*;

    pub struct RCell<T>(Arc<Mutex<Option<T>>>);

    impl<T: Send> Clone for RCell<T>{
        fn clone(&self) -> Self{
            RCell(self.0.clone())
        }
    }

    impl<T: Send> RCell<T> {
        pub fn new() -> Self {
            RCell(Arc::new(Mutex::new(None)))
        }
        pub fn set(&self, t: T) {
            (*self.0.lock().unwrap()) = Some(t);
        }
        pub fn get(&self) -> T {
            (*self.0.lock().unwrap()).take().unwrap()
        }
        pub fn get_copy(&self) -> T
            where
            T: Copy,
        {
            (*self.0.lock().unwrap()).unwrap()
        }
        pub fn get_ind(&self, cfgd: &mut CFGDrawer) -> usize {
            cfgd.get_ind(Arc::into_raw(self.0.clone()))
        }
    }
}

pub use self::content::*;

//  ____  _
// / ___|| |_ ___  _ __ ___
// \___ \| __/ _ \| '__/ _ \
//  ___) | || (_) | | |  __/
// |____/ \__\___/|_|  \___|


pub struct NStore<T : OptSend>(RCell<T>);

pub fn store<T : OptSend>(rc: RCell<T>) -> NStore<T> {
    NStore(rc)
}

impl<'a, T: Val<'a>> Node<'a, T> for NStore<T> {
    type Out = ();
    fn call(&mut self, _: &mut SubRuntime<'a>, val: T) {
        self.0.set(val);
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        print!("Store : {} in {}", tname::<T>(), self.0.get_ind(cfgd))
    }
}



//  _                    _
// | |    ___   __ _  __| |
// | |   / _ \ / _` |/ _` |
// | |__| (_) | (_| | (_| |
// |_____\___/ \__,_|\__,_|

pub struct NLoad<T : OptSend>(RCell<T>);

pub fn load<T: OptSend>(rc: RCell<T>) -> NLoad<T> {
    NLoad(rc)
}

impl<'a, T: Val<'a>> Node<'a, ()> for NLoad<T> {
    type Out = T;
    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> T {
        self.0.get()
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        print!("Load : {} in {}", tname::<T>(), self.0.get_ind(cfgd))
    }
}



//  _                    _  ____
// | |    ___   __ _  __| |/ ___|___  _ __  _   _
// | |   / _ \ / _` |/ _` | |   / _ \| '_ \| | | |
// | |__| (_) | (_| | (_| | |__| (_) | |_) | |_| |
// |_____\___/ \__,_|\__,_|\____\___/| .__/ \__, |
//                                   |_|    |___/

pub struct NLoadCopy<T : OptSend>(RCell<T>);

pub fn load_copy<T>(rc: RCell<T>) -> NLoadCopy<T>
where
    T: Copy + OptSend,
{
    NLoadCopy(rc)
}

impl<'a, T: Val<'a>> Node<'a, ()> for NLoadCopy<T>
where
    T: Copy ,
{
    type Out = T;
    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> T {
        self.0.get_copy()
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        print!("LoadCopy : {} in {}", tname::<T>(), self.0.get_ind(cfgd))
    }
}
