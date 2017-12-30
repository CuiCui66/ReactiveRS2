use engine::*;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::cell::*;
use utility::take;
use super::*;


//  ___
// |_ _|__ _ _ __   ___  _ __ ___
//  | |/ _` | '_ \ / _ \| '__/ _ \
//  | | (_| | | | | (_) | | |  __/
// |___\__, |_| |_|\___/|_|  \___|
//     |___/

/// Node that ignore it input value
///
/// Signature : `T -> ()`
#[derive(Clone, Copy)]
pub struct Ignore {}

impl<'a, In: Val<'a>> Node<'a, In> for Ignore {
    type Out = ();
    fn call(&mut self, _: &mut SubRuntime<'a>, _: In) -> Self::Out {}
}




/// Node that create a pair of unit
///
/// Signature : `() -> ((),())`
#[derive(Clone, Copy)]
pub struct GenP {}

impl<'a> Node<'a, ()> for GenP {
    type Out = ((), ());
    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        ((), ())
    }
}




/// Node that ignore the first argument of a pair
///
/// Signature : `(T,U) -> U`
#[derive(Clone, Copy)]
pub struct Ignore1 {}

impl<'a, In1: Val<'a>, In2: Val<'a>> Node<'a, (In1, In2)> for Ignore1 {
    type Out = In2;
    fn call(&mut self, _: &mut SubRuntime<'a>, (_, val): (In1, In2)) -> Self::Out {
        val
    }
}




/// Node that ignore the second argument of a pair
///
/// Signature : `(T,U) -> T`
#[derive(Clone, Copy)]
pub struct Ignore2 {}

impl<'a, In1: Val<'a>, In2: Val<'a>> Node<'a, (In1, In2)> for Ignore2 {
    type Out = In1;
    fn call(&mut self, _: &mut SubRuntime<'a>, (val, _): (In1, In2)) -> Self::Out {
        val
    }
}

//  ____
// |  _ \ __ _ _ __
// | |_) / _` | '__|
// |  __/ (_| | |
// |_|   \__,_|_|

/// Node that take a pair, run n1 on the first argument, n2 on the second
/// and returns the pair of the results
///
/// Signature : `(T1,T2) -> U1,U2` where `n1 : T1 -> U1` and `n2: T2 -> U2`
pub struct NPar<N1, N2> {
    pub n1: N1,
    pub n2: N2,
}

impl<'a, N1, N2, In1: Val<'a>, In2: Val<'a>, Out1: Val<'a>, Out2: Val<'a>> Node<'a, (In1, In2)>
    for NPar<N1, N2>
where
    N1: Node<'a, In1, Out = Out1>,
    N2: Node<'a, In2, Out = Out2>,
{
    type Out = (Out1, Out2);
    fn call(&mut self, t: &mut SubRuntime<'a>, (val1, val2): (In1, In2)) -> Self::Out {
        (self.n1.call(t, val1), self.n2.call(t, val2))
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        print!("");
        self.n1.printDot(cfgd);
        print!("| |");
        self.n2.printDot(cfgd);
        print!("");
    }
}


/// A structure for saving the value of the two branch of a `process::Par` while waiting for
/// completion
pub struct JoinPoint<T1, T2> {
    o1: Option<T1>,
    o2: Option<T2>,
}

impl<T1, T2> Default for JoinPoint<T1, T2> {
    fn default() -> Self {
        JoinPoint { o1: None, o2: None }
    }
}

impl<T1,T2> JoinPoint<T1,T2> {
    pub fn set1(&mut self, t: T1) -> bool {
        self.o1 = Some(t);
        !self.o2.is_none()
    }
    pub fn set2(&mut self, t: T2) -> bool {
        self.o2 = Some(t);
        !self.o1.is_none()
    }
    pub fn get(self : Self) -> (T1, T2) {
        (self.o1.unwrap(), self.o2.unwrap())
    }
}

#[cfg(not(feature = "par"))]
mod content {
    use super::*;
    /// struct sharing and pointing to a `JoinPoint`
    pub struct Rcjp<T1, T2>(Rc<RefCell<JoinPoint<T1, T2>>>);

    impl<T1,T2> Clone for Rcjp<T1,T2>{
        fn clone(&self) -> Self{
            Rcjp(self.0.clone())
        }
    }

    impl<T1, T2> Rcjp<T1, T2> {
        pub fn new() -> Self {
            Rcjp(Rc::new(RefCell::new(JoinPoint::default())))
        }
        pub fn set1(&self, t: T1) -> bool {
            self.0.borrow_mut().set1(t)
        }
        pub fn set2(&self, t: T2) -> bool {
            self.0.borrow_mut().set2(t)
        }
        pub fn get(&self) -> (T1, T2) {
            take(&mut *self.0.borrow_mut()).get()
        }
        pub fn get_ind(&self, cfgd: &mut CFGDrawer) -> usize {
            cfgd.get_ind(Rc::into_raw(self.0.clone()))
        }
    }

}

#[cfg(feature = "par")]
mod content {
    use std::sync::Arc;
    use std::sync::Mutex;
    use super::*;

    pub struct Rcjp<T1, T2>(Arc<Mutex<JoinPoint<T1, T2>>>);

    impl<T1: Send,T2: Send> Clone for Rcjp<T1,T2>{
        fn clone(&self) -> Self{
            Rcjp(self.0.clone())
        }
    }

    impl<T1: Send, T2: Send> Rcjp<T1, T2> {
        pub fn new() -> Self {
            Rcjp(Arc::new(Mutex::new(JoinPoint::default())))
        }
        pub fn set1(&self, t: T1) -> bool {
            self.0.lock().unwrap().set1(t)
        }
        pub fn set2(&self, t: T2) -> bool {
            self.0.lock().unwrap().set2(t)
        }
        pub fn get(&self) -> (T1, T2) {
            take(&mut *self.0.lock().unwrap()).get()
        }
        pub fn get_ind(&self, cfgd: &mut CFGDrawer) -> usize {
            cfgd.get_ind(Arc::into_raw(self.0.clone()))
        }
    }


}

pub use self::content::*;





/// Node that will set the first value of an Rcjp and then jump to dest if the
/// second is already set. See `process::Par`
///
/// Signature : `T1 -> ()`
pub struct NSetVar1<T1, T2> {
    rc: Rcjp<T1, T2>,
    dest: usize,
}

pub fn set1<T1, T2>(rc: Rcjp<T1, T2>, dest: usize) -> NSetVar1<T1, T2> {
    NSetVar1 { rc, dest }
}

impl<'a, T1: Val<'a>, T2: Val<'a>> Node<'a, T1> for NSetVar1<T1, T2> {
    type Out = ();
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: T1) {
        if self.rc.set1(val) {
            sub_runtime.add_current(self.dest);
        }
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        let ind = cfgd.get_node_ind();
        print!(
            "<f{}> Set1: {} in {}",
            ind,
            tname::<T1>(),
            self.rc.get_ind(cfgd)
        );
        cfgd.add_arrow((ind, self.dest));
    }
}




/// Node that will set the second value of an Rcjp and then jump to dest if the
/// first is already set. See `process::Par`
///
/// Signature : `T2 -> ()`
pub struct NSetVar2<T1, T2> {
    rc: Rcjp<T1, T2>,
    dest: usize,
}

pub fn set2<T1, T2>(rc: Rcjp<T1, T2>, dest: usize) -> NSetVar2<T1, T2> {
    NSetVar2 { rc, dest }
}

impl<'a, T1: Val<'a>, T2: Val<'a>> Node<'a, T2> for NSetVar2<T1, T2> {
    type Out = ();
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: T2) {
        if self.rc.set2(val) {
            sub_runtime.add_current(self.dest);
        }
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        let ind = cfgd.get_node_ind();
        print!(
            "<f{}> Set2: {} in {}",
            ind,
            tname::<T2>(),
            self.rc.get_ind(cfgd)
        );
        cfgd.add_arrow((ind, self.dest));
    }
}



/// Node that, when called, extract the two value set in Rcjp and returns them as a pair
///
/// Signature : `() -> (T1,T2)`

pub struct NMerge<T1, T2> {
    rc: Rcjp<T1,T2>,
}

pub fn merge<T1, T2>(rc: Rcjp<T1, T2>) -> NMerge<T1, T2> {
    NMerge { rc }
}

impl<'a, T1: Val<'a>, T2: Val<'a>> Node<'a, ()> for NMerge<T1, T2> {
    type Out = (T1, T2);
    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        self.rc.get()
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        print!(
            "Merge: {} in {}",
            tname::<(T1, T2)>(),
            self.rc.get_ind(cfgd)
        )
    }
}


//  ____  _
// | __ )(_) __ _
// |  _ \| |/ _` |
// | |_) | | (_| |
// |____/|_|\__, |
//          |___/


/// Node that, when called, dump all the ids it owns, in the scheduler for current instant.
///
/// Signature : `() -> (T1,T2)`
pub struct NBigPar {
    pub(crate) dests: Vec<usize>,
}

impl<'a> Node<'a, ()> for NBigPar {
    type Out = ();
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        for d in &self.dests {
            sub_runtime.add_current(*d);
        }
    }
}

#[cfg(not(feature = "par"))]
mod content2 {
    use super::*;
    /// Struct to wait `total` thread and then jump to `dest`
    pub struct BigJoinPoint {
        nb: Cell<usize>,
        total: usize,
        dest: usize,
    }


    impl BigJoinPoint {
        pub fn new(total: usize, dest: usize) -> Self{
            BigJoinPoint { nb: Cell::new(0), total, dest }
        }
        pub fn incr(&self) -> Option<usize> {
            let mut val = self.nb.get();
            val+=1;
            if val == self.total {
                self.nb.set(0);
                return Some(self.dest);
            }
            else {
                self.nb.set(val);
            }
            return None;
        }
    }

    /// Struct owning and sharing a BigJoinPoint
    #[derive(Clone)]
    pub struct Rcbjp(Rc<BigJoinPoint>);

    impl Rcbjp {
        pub fn new(total: usize, dest: usize) -> Self {
            Rcbjp(Rc::new(BigJoinPoint::new(total,dest)))
        }
        pub fn incr(&self) -> Option<usize> {
            self.0.incr()
        }
        pub fn get_ind(&self, cfgd: &mut CFGDrawer) -> usize {
            cfgd.get_ind(Rc::into_raw(self.0.clone()))
        }
    }

}

#[cfg(all(feature = "par"))]
mod content2 {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering::*;
    use super::*;
    pub struct BigJoinPoint {
        nb: AtomicUsize,
        total: usize,
        dest: usize,
    }

    impl BigJoinPoint {
        pub fn new(total: usize, dest: usize) -> Self{
            BigJoinPoint { nb: AtomicUsize::new(0), total, dest }
        }
        pub fn incr(&self) -> Option<usize> {
            self.nb.fetch_add(1,SeqCst);
            if self.nb.compare_and_swap(self.total,0,SeqCst) == self.total {
                return Some(self.dest);
            }
            return None;
        }
    }

    #[derive(Clone)]
    pub struct Rcbjp(Arc<BigJoinPoint>);

    impl Rcbjp {
        pub fn new(total: usize, dest: usize) -> Self {
            Rcbjp(Arc::new(BigJoinPoint::new(total,dest)))
        }
        pub fn incr(&self) -> Option<usize> {
            self.0.incr()
        }
        pub fn get_ind(&self, cfgd: &mut CFGDrawer) -> usize {
            cfgd.get_ind(Arc::into_raw(self.0.clone()))
        }
    }
}
pub use self::content2::*;

/// Node that Increments the value of a Rcbjp and jump to the dest node it is the last
/// one to get to join point.
pub struct NBigMerge {
    rc: Rcbjp,
}

pub fn big_merge(rc: Rcbjp) -> NBigMerge {
    NBigMerge { rc }
}


impl<'a> Node<'a, ()> for NBigMerge {
    type Out = ();
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        if let Some(ind) = self.rc.incr(){
            sub_runtime.add_current(ind);
        }
    }
}
