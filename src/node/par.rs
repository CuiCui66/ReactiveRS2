use engine::*;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::cell::*;
use take::take;
use super::*;


//  ___
// |_ _|__ _ _ __   ___  _ __ ___
//  | |/ _` | '_ \ / _ \| '__/ _ \
//  | | (_| | | | | (_) | | |  __/
// |___\__, |_| |_|\___/|_|  \___|
//     |___/

#[derive(Clone, Copy)]
pub struct Ignore {}

#[allow(non_upper_case_globals)]
pub static Ignore: Ignore = Ignore {};

impl<'a, In: 'a> Node<'a, In> for Ignore {
    type Out = ();
    fn call(&mut self, _: &mut SubRuntime<'a>, _: In) -> Self::Out {}
}

#[derive(Clone, Copy)]
pub struct GenP {}

#[allow(non_upper_case_globals)]
pub static GenP: GenP = GenP {};

impl<'a> Node<'a, ()> for GenP {
    type Out = ((), ());
    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        ((), ())
    }
}



#[derive(Clone, Copy)]
pub struct Ignore1 {}

#[allow(non_upper_case_globals)]
pub static Ignore1: Ignore1 = Ignore1 {};

impl<'a, In1: 'a, In2: 'a> Node<'a, (In1, In2)> for Ignore1 {
    type Out = In2;
    fn call(&mut self, _: &mut SubRuntime<'a>, (_, val): (In1, In2)) -> Self::Out {
        val
    }
}

#[derive(Clone, Copy)]
pub struct Ignore2 {}

#[allow(non_upper_case_globals)]
pub static Ignore2: Ignore2 = Ignore2 {};

impl<'a, In1: 'a, In2: 'a> Node<'a, (In1, In2)> for Ignore2 {
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

pub struct NPar<N1, N2> {
    pub n1: N1,
    pub n2: N2,
}

impl<'a, N1, N2, In1: 'a, In2: 'a, Out1: 'a, Out2: 'a> Node<'a, (In1,In2)> for NPar<N1, N2>
    where
    N1: Node<'a, In1, Out = Out1>,
    N2: Node<'a, In2, Out = Out2>,
{
    type Out = (Out1, Out2);
    fn call(&mut self, t: &mut SubRuntime<'a>, (val1,val2):(In1,In2) ) -> Self::Out {
        (self.n1.call(t, val1), self.n2.call(t, val2))
    }
}

pub struct JoinPoint<T1, T2> {
    o1: Option<T1>,
    o2: Option<T2>,
}

impl<T1, T2> Default for JoinPoint<T1, T2> {
    fn default() -> Self {
        JoinPoint { o1: None, o2: None }
    }
}

pub fn new_rcjp<T1, T2>() -> Rc<RefCell<JoinPoint<T1, T2>>> {
    Rc::new(RefCell::new(JoinPoint::default()))
}


pub struct NSetVar1<T1, T2> {
    rc: Rc<RefCell<JoinPoint<T1, T2>>>,
    dest: usize,
}

pub fn set1<T1, T2>(rc: Rc<RefCell<JoinPoint<T1, T2>>>, dest: usize) -> NSetVar1<T1, T2> {
    NSetVar1 { rc, dest }
}

impl<'a, T1: 'a, T2: 'a> Node<'a, T1> for NSetVar1<T1, T2> {
    type Out = ();
    fn call(&mut self, t: &mut SubRuntime<'a>, val: T1) {
        self.rc.borrow_mut().o1 = Some(val);
        if !self.rc.borrow().o2.is_none() {
            t.tasks.current.push(self.dest);
        }
    }
}



pub struct NSetVar2<T1, T2> {
    rc: Rc<RefCell<JoinPoint<T1, T2>>>,
    dest: usize,
}

pub fn set2<T1, T2>(rc: Rc<RefCell<JoinPoint<T1, T2>>>, dest: usize) -> NSetVar2<T1, T2> {
    NSetVar2 { rc, dest }
}

impl<'a, T1: 'a, T2: 'a> Node<'a, T2> for NSetVar2<T1, T2> {
    type Out = ();
    fn call(&mut self, t: &mut SubRuntime<'a>, val: T2) {
        self.rc.borrow_mut().o2 = Some(val);
        if !self.rc.borrow().o1.is_none() {
            t.tasks.current.push(self.dest);
        }
    }
}



pub struct NMerge<T1, T2> {
    rc: Rc<RefCell<JoinPoint<T1, T2>>>,
}

pub fn merge<T1, T2>(rc: Rc<RefCell<JoinPoint<T1, T2>>>) -> NMerge<T1, T2> {
    NMerge { rc }
}

impl<'a, T1: 'a, T2: 'a> Node<'a, ()> for NMerge<T1, T2> {
    type Out = (T1, T2);
    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        let jp = take(&mut *self.rc.borrow_mut());
        (jp.o1.unwrap(), jp.o2.unwrap())
    }
}

//  ____            ____
// |  _ \ __ _ _ __|  _ \ __ _ _ __
// | |_) / _` | '__| |_) / _` | '__|
// |  __/ (_| | |  |  __/ (_| | |
// |_|   \__,_|_|  |_|   \__,_|_|


pub fn new_arcjp<T1, T2>() -> Arc<Mutex<JoinPoint<T1, T2>>> {
    Arc::new(Mutex::new(JoinPoint::default()))
}


pub struct NSetVar1Par<T1, T2> {
    arc: Arc<Mutex<JoinPoint<T1, T2>>>,
    dest: usize,
}

pub fn set1_par<T1, T2>(arc: Arc<Mutex<JoinPoint<T1, T2>>>, dest: usize) -> NSetVar1Par<T1, T2> {
    NSetVar1Par { arc, dest }
}

impl<'a, T1: 'a, T2: 'a> Node<'a, T1> for NSetVar1Par<T1, T2> {
    type Out = ();
    fn call(&mut self, t: &mut SubRuntime<'a>, val: T1) {
        let mut arc = self.arc.lock().unwrap();
        arc.o1 = Some(val);
        if !arc.o2.is_none() {
            t.tasks.current.push(self.dest);
        }
    }
}



pub struct NSetVar2Par<T1, T2> {
    arc: Arc<Mutex<JoinPoint<T1, T2>>>,
    dest: usize,
}

pub fn set2_par<T1, T2>(arc: Arc<Mutex<JoinPoint<T1, T2>>>, dest: usize) -> NSetVar2Par<T1, T2> {
    NSetVar2Par { arc, dest }
}

impl<'a, T1: 'a, T2: 'a> Node<'a, T2> for NSetVar2Par<T1, T2> {
    type Out = ();
    fn call(&mut self, t: &mut SubRuntime<'a>, val: T2) {
        let mut arc = self.arc.lock().unwrap();
        arc.o2 = Some(val);
        if !arc.o1.is_none() {
            t.tasks.current.push(self.dest);
        }
    }
}



pub struct NMergePar<T1, T2> {
    arc: Arc<Mutex<JoinPoint<T1, T2>>>,
}

pub fn merge_par<T1, T2>(arc: Arc<Mutex<JoinPoint<T1, T2>>>) -> NMergePar<T1, T2> {
    NMergePar { arc }
}

impl<'a, T1: 'a, T2: 'a> Node<'a, ()> for NMergePar<T1, T2> {
    type Out = (T1, T2);
    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        let jp = take(&mut *self.arc.lock().unwrap());
        (jp.o1.unwrap(), jp.o2.unwrap())
    }
}

//  ____  _
// | __ )(_) __ _
// |  _ \| |/ _` |
// | |_) | | (_| |
// |____/|_|\__, |
//          |___/


pub struct NBigPar {
    pub(crate) dests: Vec<usize>,
}

impl<'a> Node<'a, ()> for NBigPar {
    type Out = ();
    fn call(&mut self, t: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        for d in &self.dests {
            t.tasks.current.push(*d);
        }
    }
}

pub struct BigJoinPoint {
    nb: usize,
    total: usize,
    dest: usize,
}
pub fn new_rcbjp(total: usize, dest: usize) -> Rc<RefCell<BigJoinPoint>> {
    Rc::new(RefCell::new(BigJoinPoint { nb: 0, total, dest }))
}


pub struct NBigMerge {
    rc: Rc<RefCell<BigJoinPoint>>,
}

pub fn big_merge(rc: Rc<RefCell<BigJoinPoint>>) -> NBigMerge {
    NBigMerge { rc }
}


impl<'a> Node<'a, ()> for NBigMerge {
    type Out = ();
    fn call(&mut self, t: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        let mut bjp = self.rc.borrow_mut();
        bjp.nb += 1;
        if bjp.nb == bjp.total {
            bjp.nb = 0;
            t.tasks.current.push(bjp.dest);
        }
    }
}

//  ____  _       ____
// | __ )(_) __ _|  _ \ __ _ _ __
// |  _ \| |/ _` | |_) / _` | '__|
// | |_) | | (_| |  __/ (_| | |
// |____/|_|\__, |_|   \__,_|_|
//          |___/


pub fn new_rcbjp_par(total: usize, dest: usize) -> Arc<Mutex<BigJoinPoint>> {
    Arc::new(Mutex::new(BigJoinPoint { nb: 0, total, dest }))
}


pub struct NBigMergePar {
    arc: Arc<Mutex<BigJoinPoint>>,
}

pub fn big_merge_par(arc: Arc<Mutex<BigJoinPoint>>) -> NBigMergePar {
    NBigMergePar { arc }
}


impl<'a> Node<'a, ()> for NBigMergePar {
    type Out = ();
    fn call(&mut self, t: &mut SubRuntime<'a>, _: ()) -> Self::Out {
        let mut bjp = self.arc.lock().unwrap();
        bjp.nb += 1;
        if bjp.nb == bjp.total {
            bjp.nb = 0;
            t.tasks.current.push(bjp.dest);
        }
    }
}
