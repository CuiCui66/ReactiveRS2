use std::marker::PhantomData;

use engine::*;
use std::rc::Rc;
use std::cell::*;
use std::fmt;
use std::fmt::Debug;

pub trait Node<'a, In: 'a>: 'a + Debug {
    type Out;
    fn call(&mut self, tasks: &mut Tasks, val: In) -> Self::Out;
    fn nseq<N2>(self, n2: N2) -> NSeq<Self, N2>
    where
        N2: Node<'a, Self::Out> + Sized,
        Self: Sized,
    {
        NSeq { n1: self, n2: n2 }
    }
}
#[derive(Debug)]
pub struct Nothing {}

impl<'a> Node<'a, ()> for Nothing {
    type Out = ();
    fn call(&mut self, _tasks: &mut Tasks, _val: ()) -> Self::Out {}
}


/// Partial nodes during compilation
/// normally there is a In and Out such that
/// NI: Node<In,Out=()>
/// NO: Node<(),Out=Out>
/// NIO: Node<In,Out=Out>
pub enum PNode<NI, NO, NIO> {
    InOut(NIO),
    Halves(NI, usize, NO),
}

//  _____                 _
// | ____|_ __ ___  _ __ | |_ _   _
// |  _| | '_ ` _ \| '_ \| __| | | |
// | |___| | | | | | |_) | |_| |_| |
// |_____|_| |_| |_| .__/ \__|\__, |
//                 |_|        |___/

pub struct DummyN<Out> {
    dummy: PhantomData<Out>,
}
impl<Out> Debug for DummyN<Out> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DummyN")
    }
}


impl<'a, In: 'a, Out: 'a> Node<'a, In> for DummyN<Out>
where
    Out: 'a,
{
    type Out = Out;
    fn call(&mut self, _: &mut Tasks, _: In) -> Out {
        panic!("Called empty node");
    }
}


//  _____      __  __       _
// |  ___| __ |  \/  |_   _| |_
// | |_ | '_ \| |\/| | | | | __|
// |  _|| | | | |  | | |_| | |_
// |_|  |_| |_|_|  |_|\__,_|\__|

pub struct FnMutN<F>(pub F);

impl<F> Debug for FnMutN<F>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Func")
    }
}

impl<'a, F, In: 'a, Out: 'a> Node<'a, In> for FnMutN<F>
where
    F: FnMut(In) -> Out + 'a,
{
    type Out = Out;
    fn call(&mut self, _: &mut Tasks, val: In) -> Out {
        let &mut FnMutN(ref mut f) = self;
        f(val)
    }
}

//  ____
// / ___|  ___  __ _
// \___ \ / _ \/ _` |
//  ___) |  __/ (_| |
// |____/ \___|\__, |
//                |_|

#[derive(Debug)]
pub struct NSeq<N1, N2> {
    n1: N1,
    n2: N2,
}

impl<'a, N1, N2, In: 'a, Mid: 'a, Out: 'a> Node<'a, In> for NSeq<N1, N2>
where
    N1: Node<'a, In, Out = Mid>,
    N2: Node<'a, Mid, Out = Out>,
{
    type Out = Out;
    fn call(&mut self, t: &mut Tasks, val: In) -> Out {
        let valm = self.n1.call(t, val);
        self.n2.call(t, valm)
    }
}

//  ____        __  __             _
// |  _ \ ___  |  \/  | __ _ _ __ (_)_ __
// | |_) / __| | |\/| |/ _` | '_ \| | '_ \
// |  _ < (__  | |  | | (_| | | | | | |_) |
// |_| \_\___| |_|  |_|\__,_|_| |_|_| .__/
//                                  |_|
pub struct RcStore<T> {
    p: Rc<Cell<T>>,
}

impl<Out> Debug for RcStore<Out> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RcStore")
    }
}


impl<T> RcStore<T> {
    pub fn new(rc: Rc<Cell<T>>) -> Self {
        RcStore { p: rc }
    }
}

impl<'a, T: 'a> Node<'a, T> for RcStore<T> {
    type Out = ();
    fn call(&mut self, _tasks: &mut Tasks, val: T) {
        self.p.set(val);
    }
}

pub struct RcLoad<T> {
    p: Rc<Cell<T>>,
}

impl<Out> Debug for RcLoad<Out> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RcStore")
    }
}


impl<T> RcLoad<T> {
    pub fn new(rc: Rc<Cell<T>>) -> Self {
        RcLoad { p: rc }
    }
}


impl<'a, T: 'a> Node<'a, ()> for RcLoad<T>
where
    T: Default,
{
    type Out = T;
    fn call(&mut self, _tasks: &mut Tasks, _: ()) -> T {
        self.p.take()
    }
}

//  ____
// |  _ \ __ _ _   _ ___  ___
// | |_) / _` | | | / __|/ _ \
// |  __/ (_| | |_| \__ \  __/
// |_|   \__,_|\__,_|___/\___|

#[derive(Debug)]
pub struct NPause {
    dest: usize,
}
impl NPause {
    pub fn new(rc: usize) -> Self {
       NPause { dest: rc }
    }
}

impl<'a> Node<'a, ()> for NPause {
    type Out = ();
    fn call(&mut self, tasks: &mut Tasks, _: ()) {
        tasks.next.push(self.dest);
    }
}
