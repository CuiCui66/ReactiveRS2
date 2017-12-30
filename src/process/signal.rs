//! This module is about processes that interact with signals.
//! If a process implementation ends with a D, it means that it takes
//! the signal it operates on at runtime as an input value.
//! If a process implementation ends with a S, it means that it takes
//! the signal it operates on, during its construction at compile time

use super::*;
use signal::*;
use std::marker::PhantomData;


//  ____           ____
// |  _ \ _ __ ___|  _ \
// | |_) | '__/ _ \ | | |
// |  __/| | |  __/ |_| |
// |_|   |_|  \___|____/

/// Process implementation returning the last value V of the input signal S,
/// where the signal is given as the process input.
///
/// Possible signatures are S -> V and (S,In) -> (V,In)
#[derive(Copy, Clone)]
pub struct PreD {}

impl<'a, S> IntProcess<'a, S> for PreD
where
    S: Signal<'a>
{
    type Out = S::V;
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"PreD\"];", num);
        (num, num)
    }
}


impl<'a, S> IntProcessIm<'a, S> for PreD
where
    S: Signal<'a>
{
    type NIO = NGetD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NGetD {}
    }
}

/// Builds boxed `PreD` for signature S -> V
pub fn pre_d<'a, S>() -> ProcessIm<'a, S, S::V, NotOnce, NGetD>
where
    S: Signal<'a>
{
    ProcessIm(box PreD {})
}

impl<'a, S, In: Val<'a>> IntProcess<'a, (S, In)> for PreD
where
    S: Signal<'a>
{
    type Out = (S::V, In);
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"PreD\"];", num);
        (num, num)
    }
}


impl<'a, S, In: Val<'a>> IntProcessIm<'a, (S, In)> for PreD
where
    S: Signal<'a>
{
    type NIO = NGetD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NGetD {}
    }
}

/// Builds boxed `PreD` for signature (S,In) -> (V,In)
pub fn pre_d_in<'a, S, In: Val<'a>>() -> ProcessIm<'a, (S, In), (S::V, In), NotOnce, NGetD>
where
    S: Signal<'a>
{
    ProcessIm(box PreD {})
}

//  ____           ____
// |  _ \ _ __ ___/ ___|
// | |_) | '__/ _ \___ \
// |  __/| | |  __/___) |
// |_|   |_|  \___|____/


/// Process implementation returning the last value V of a signal,
/// where the signal is given at construction.
///
/// Signature is () -> V.
///
/// See `PreSIn` to have a side value
#[derive(Copy, Clone)]
pub struct PreS<S>(pub S);

impl<'a, S> IntProcess<'a, ()> for PreS<S>
where
    S: Signal<'a>
{
    type Out = S::V;
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"PreS\"];", num);
        (num, num)
    }
}


impl<'a, S: Val<'a>> IntProcessIm<'a, ()> for PreS<S>
where
    S: Signal<'a>
{
    type NIO = NGetS<S>;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NGetS(self.0)
    }
}

/// Builds boxed `PreS`
pub fn pre_s<'a, S>(signal: S) -> ProcessIm<'a, (), S::V, NotOnce, NGetS<S>>
where
    S: Signal<'a>
{
    ProcessIm(box PreS(signal))
}

/// Process implementation returning the last value V of a signal,
/// where the signal is given at construction.
///
/// Signature is In -> (V,In).
///
/// See `PreS` to not have a side value.
#[derive(Copy, Clone)]
pub struct PreSIn<S>(pub S);


impl<'a, S: Val<'a>, In: Val<'a>> IntProcess<'a, In> for PreSIn<S>
where
    S: Signal<'a>
{
    type Out = (S::V, In);
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"PreS\"];", num);
        (num, num)
    }
}


impl<'a, S: Val<'a>, In: Val<'a>> IntProcessIm<'a, In> for PreSIn<S>
where
    S: Signal<'a>
{
    type NIO = NGetSIn<S>;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NGetSIn(self.0)
    }
}

/// Builds boxed `PreSIn`
pub fn pre_s_in<'a, S, In: Val<'a>>(signal: S) -> ProcessIm<'a, In, (S::V, In), NotOnce, NGetSIn<S>>
where
    S: Signal<'a>
{
    ProcessIm(box PreSIn(signal))
}

//  _____           _ _   ____
// | ____|_ __ ___ (_) |_|  _ \
// |  _| | '_ ` _ \| | __| | | |
// | |___| | | | | | | |_| |_| |
// |_____|_| |_| |_|_|\__|____/

/// Process implementation representing the emission of a signal,
/// where the signal S and the value V are given as the process input.
///
/// Possible signatures are:
///
///   - `((S,V),In) -> In`
///   - `(S,V) -> ()`
///   - `(Vec<(S,V)>,In) -> In`
///   - `Vec<(S,V)> -> ()`

#[derive(Copy, Clone)]
pub struct EmitD {}

#[allow(non_upper_case_globals)]
pub static EmitD: EmitD = EmitD {};

impl<'a, In: Val<'a>, S: Val<'a>> IntProcess<'a, ((S, S::E), In)> for EmitD
where S: Signal<'a> {
    type Out = In;
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"EmitD\"];", num);
        (num, num)
    }
}

impl<'a, In: Val<'a>, S: Val<'a>> IntProcessIm<'a, ((S, S::E), In)> for EmitD
where
    S: Signal<'a>
{
    type NIO = NEmitD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NEmitD {}
    }
}

impl<'a, S: Val<'a>> IntProcess<'a, (S, S::E)> for EmitD
where
    S: Signal<'a>,
{
    type Out = ();
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"EmitD\"];", num);
        (num, num)
    }
}

impl<'a, S: Val<'a>> IntProcessIm<'a, (S, S::E)> for EmitD
where
    S: Signal<'a>,
{
    type NIO = NEmitD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NEmitD {}
    }
}


impl<'a, In: Val<'a>, S: Val<'a>> IntProcess<'a, (Vec<(S, S::E)>, In)> for EmitD
where
    S: Signal<'a>,
{
    type Out = In;
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"EmitD\"];", num);
        (num, num)
    }
}

impl<'a, In: Val<'a>, S: Val<'a>> IntProcessIm<'a, (Vec<(S, S::E)>, In)> for EmitD
where
    S: Signal<'a>,
{
    type NIO = NEmitD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NEmitD {}
    }
}


impl<'a, S: Val<'a>> IntProcess<'a, Vec<(S, S::E)>> for EmitD
where
    S: Signal<'a>,
{
    type Out = ();
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"EmitD\"];", num);
        (num, num)
    }
}

impl<'a, S: Val<'a>> IntProcessIm<'a, Vec<(S, S::E)>> for EmitD
where
    S: Signal<'a>,
{
    type NIO = NEmitD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NEmitD {}
    }
}


/// Builds boxed `EmitD` for signature ((S,V),In) -> In
pub fn emit_d_in<'a, S: Val<'a>, In: Val<'a>>()
    -> ProcessIm<'a, ((S, S::E), In), In, NotOnce, NEmitD>
where
    S: Signal<'a>
{
    ProcessIm(box EmitD {})
}

/// Builds boxed `EmitD` for signature (S,V) -> ()
pub fn emit_d<'a, S: Val<'a>>()
    -> ProcessIm<'a, (S, S::E), (), NotOnce, NEmitD>
where
    S: Signal<'a>
{
    ProcessIm(box EmitD {})
}

/// Builds boxed `EmitD` for signature (Vec<(S,V)>,In) -> In
pub fn emit_d_vec_in<'a, S: Val<'a>, In: Val<'a>>()
    -> ProcessIm<'a, (Vec<(S, S::E)>, In), In, NotOnce, NEmitD>
where
    S: Signal<'a>
{
    ProcessIm(box EmitD {})
}

/// Builds boxed `EmitD` for signature Vec<(S,V)> -> ()
pub fn emit_d_vec<'a, S: Val<'a>>()
    -> ProcessIm<'a, Vec<(S, S::E)>, (), NotOnce, NEmitD>
where
    S: Signal<'a>
{
    ProcessIm(box EmitD {})
}



//  _____           _ _   ____
// | ____|_ __ ___ (_) |_/ ___|
// |  _| | '_ ` _ \| | __\___ \
// | |___| | | | | | | |_ ___) |
// |_____|_| |_| |_|_|\__|____/

/// Process representing the emission of a signal,
/// where the signal is fixed and the value is given as the process input.
#[derive(Clone)]
pub struct EmitS<S, E>(pub S, pub PhantomData<E>);

impl<'a, S: Val<'a>> IntProcess<'a, S::E> for EmitS<S, S::E>
where
    S: Signal<'a>,
{
    type Out = ();
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitS\"];",num);
        (num,num)
    }
}

impl<'a, S: Val<'a>> IntProcessIm<'a, S::E> for EmitS<S, S::E>
where
    S: Signal<'a>,
{
    type NIO = NEmitS<S, S::E>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        let s = *self;
        NEmitS(s.0, s.1)
    }
}

pub fn emit_s<'a, S: Val<'a>>(signal_runtime: S)
    -> ProcessIm<'a, S::E, (), NotOnce, NEmitS<S,S::E>>
where
    S: Signal<'a>
{
    ProcessIm(box EmitS(signal_runtime, PhantomData))
}

/// Process representing the emission of a signal,
/// where the signal is fixed and the value is given as the process input.
/// This structure is needed to passe a value to the next process
/// Rust does not understand that (S::E,In) can't be equal to S::E
#[derive(Clone)]
pub struct EmitSIn<S, E>(pub S, pub PhantomData<E>);

impl<'a, In: Val<'a>, S: Val<'a>> IntProcess<'a, (S::E, In)> for EmitSIn<S, S::E>
where
    S: Signal<'a>,
{
    type Out = In;
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitS\"];",num);
        (num,num)
    }
}

impl<'a, In: Val<'a>, S: Val<'a>> IntProcessIm<'a, (S::E, In)> for EmitSIn<S, S::E>
where
    S: Signal<'a>,
{
    type NIO = NEmitS<S, S::E>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        let s = *self;
        NEmitS(s.0, s.1)
    }
}

pub fn emit_s_in<'a, In: Val<'a>, S: Val<'a>>(signal_runtime: S)
    -> ProcessIm<'a, (S::E, In), In, NotOnce, NEmitS<S,S::E>>
where
    S: Signal<'a>
{
    ProcessIm(box EmitSIn(signal_runtime, PhantomData))
}


//  _____           _ _ __     __        ____
// | ____|_ __ ___ (_) |\ \   / /__  ___/ ___|
// |  _| | '_ ` _ \| | __\ \ / / _ \/ __\___ \
// | |___| | | | | | | |_ \ V /  __/ (__ ___) |
// |_____|_| |_| |_|_|\__| \_/ \___|\___|____/

/// Process representing the emission of multiple signals,
/// where the signals are fixed and the values are given as the process input.
#[derive(Clone)]
pub struct EmitVecS<S>(pub Vec<S>);

impl<'a, S: Val<'a>> IntProcess<'a, Vec<S::E>> for EmitVecS<S>
where
    S: Signal<'a>
{
    type Out = ();
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitVecS\"];",num);
        (num,num)
    }
}

impl<'a, S: Val<'a>> IntProcessIm<'a, Vec<S::E>> for EmitVecS<S>
where
    S: Signal<'a>
{
    type NIO = NEmitVecS<S>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVecS(self.0)
    }
}

pub fn emit_vec_s<'a, S: Val<'a>>(signal_runtimes: Vec<S>)
    -> ProcessIm<'a, Vec<S::E>, (), NotOnce, NEmitVecS<S>>
where
    S: Signal<'a>
{
    ProcessIm(box EmitVecS(signal_runtimes))
}



impl<'a, S: Val<'a>, In: Val<'a>> IntProcess<'a, (Vec<S::E>, In)> for EmitVecS<S>
where
    S: Signal<'a>
{
    type Out = In;
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitVecS\"];",num);
        (num,num)
    }
}

impl<'a, S: Val<'a>, In: Val<'a>> IntProcessIm<'a, (Vec<S::E>, In)> for EmitVecS<S>
where
    S: Signal<'a>
{
    type NIO = NEmitVecS<S>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVecS(self.0)
    }
}


pub fn emit_vec_s_in<'a, In: Val<'a>, S: Val<'a>>(signal_runtimes: Vec<S>)
    -> ProcessIm<'a, (Vec<S::E>,In), In, NotOnce, NEmitVecS<S>>
where
    S: Signal<'a>
{
    ProcessIm(box EmitVecS(signal_runtimes))
}



//  _____           _ _ __     ______
// | ____|_ __ ___ (_) |\ \   / / ___|
// |  _| | '_ ` _ \| | __\ \ / /\___ \
// | |___| | | | | | | |_ \ V /  ___) |
// |_____|_| |_| |_|_|\__| \_/  |____/


/// Process representing the emission of a signal,
/// where the signal and the value are fixed.
#[derive(Clone)]
pub struct EmitVS<S, E>(pub S, pub E);

impl<'a, S: Val<'a>, In: Val<'a>> IntProcess<'a, In> for EmitVS<S, S::E>
where
    S: Signal<'a>,
    S::E: Clone,
{
    type Out = In;
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitVecS\"];",num);
        (num,num)
    }
}

impl<'a, S: Val<'a>, In: Val<'a>> IntProcessIm<'a, In> for EmitVS<S, S::E>
where
    S: Signal<'a>,
    S::E: Clone
{
    type NIO = NEmitVS<S, S::E>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        let s = *self;
        NEmitVS(s.0, s.1)
    }
}


pub fn emit_vs<'a, In: Val<'a>, S: Val<'a>>(signal_runtime: S, value: S::E)
    -> ProcessIm<'a, In, In, NotOnce, NEmitVS<S, S::E>>
where
    S: Signal<'a>,
    S::E: Clone,
{
    ProcessIm(box EmitVS(signal_runtime, value))
}


//  _____           _ _ __     ____     __        ____
// | ____|_ __ ___ (_) |\ \   / /\ \   / /__  ___/ ___|
// |  _| | '_ ` _ \| | __\ \ / /  \ \ / / _ \/ __\___ \
// | |___| | | | | | | |_ \ V /    \ V /  __/ (__ ___) |
// |_____|_| |_| |_|_|\__| \_/      \_/ \___|\___|____/


/// Process representing the emission of multiple signals,
/// where the signals and the values are fixed.
#[derive(Clone)]
pub struct EmitVVecS<S, E>(pub Vec<(S,E)>);


impl<'a, S: Val<'a>, In: Val<'a>> IntProcess<'a, In> for EmitVVecS<S, S::E>
where
    S: Signal<'a>,
    S::E: Clone,
{
    type Out = In;
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitVecS\"];",num);
        (num,num)
    }
}

impl<'a, S: Val<'a>, In: Val<'a>> IntProcessIm<'a, In> for EmitVVecS<S, S::E>
where
    S: Signal<'a>,
    S::E: Clone
{
    type NIO = NEmitVVecS<S, S::E>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVVecS(self.0)
    }
}

pub fn emit_vec_vs<'a, In: Val<'a>, S: Val<'a>>(signal_values: Vec<(S, S::E)>)
    -> ProcessIm<'a, In, In, NotOnce, NEmitVVecS<S, S::E>>
where
    S: Signal<'a>,
    S::E: Clone,
{
    ProcessIm(box EmitVVecS(signal_values))
}



//     _                _ _   ____
//    / \__      ____ _(_) |_|  _ \
//   / _ \ \ /\ / / _` | | __| | | |
//  / ___ \ V  V / (_| | | |_| |_| |
// /_/   \_\_/\_/ \__,_|_|\__|____/


/// Process awaiting for the emission of a signal, and executing the next process the next instant,
/// where the signal is given as the process input.
#[derive(Clone, Copy)]
pub struct AwaitD {}

impl<'a, S: Val<'a>> IntProcess<'a, S> for AwaitD
where
    S: Signal<'a>,
{
    type Out = S::V;
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitD\"];",num);
        (num,num)
    }
}

impl<'a, S: Val<'a>> IntProcessNotIm<'a, S> for AwaitD
where
    S: Signal<'a>,
{
    type NI = NSeq<NAwaitD, NStore<S>>;
    type NO = NSeq<NLoad<S>, NGetD>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = RCell::new();
        let rc2 = rc.clone();

        let ni = node!(NAwaitD(out_id) >> store(rc));
        let no = node!(load(rc2) >> NGetD {});
        (ni, out_id, no)
    }
}

pub fn await_d<'a, S: Val<'a>>()
    -> ProcessNotIm<'a, S, S::V, NotOnce, NSeq<NAwaitD, NStore<S>>, NSeq<NLoad<S>, NGetD>>
where
    S: Signal<'a>,
    S::E: Clone,
{
    ProcessNotIm(box AwaitD {})
}

impl<'a, S: Val<'a>, In: Val<'a>> IntProcess<'a, (S, In)> for AwaitD
where
    S: Signal<'a>,
{
    type Out = (S::V, In);
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitD\"];",num);
        (num,num)
    }
}

impl<'a, In: Val<'a>, S: Val<'a>> IntProcessNotIm<'a, (S, In)> for AwaitD
where
    S: Signal<'a>,
{
    type NI = NSeq<NPar<NAwaitD,NIdentity>,NStore<(S, In)>>;
    type NO = NSeq<NLoad<(S, In)>, NGetD>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = RCell::new();
        let rc2 = rc.clone();

        // Type inference won't work here
        let ni_first = <NAwaitD as Node<'a,S>>::njoin::<In, NIdentity>(NAwaitD(out_id), NIdentity {});
        let ni = node!(ni_first >> store(rc));
        let no = node!(load(rc2) >> NGetD{});
        (ni, out_id, no)
    }
}

pub fn await_d_in<'a, In: Val<'a>, S: Val<'a>>()
    -> ProcessNotIm<'a, (S,In), (S::V, In), NotOnce, NSeq<NPar<NAwaitD,NIdentity>,NStore<(S, In)>>, NSeq<NLoad<(S, In)>, NGetD>>
where
    S: Signal<'a>,
    S::E: Clone,
{
    ProcessNotIm(box AwaitD {})
}


//     _                _ _   ____
//    / \__      ____ _(_) |_/ ___|
//   / _ \ \ /\ / / _` | | __\___ \
//  / ___ \ V  V / (_| | | |_ ___) |
// /_/   \_\_/\_/ \__,_|_|\__|____/


/// Process awaiting for the emission of a signal, and executing the next process the next instant,
/// where the signal is fixed.
#[derive(Clone)]
pub struct AwaitS<S>(pub S);


impl<'a, S: Val<'a>> IntProcess<'a, ()> for AwaitS<S>
where
    S: Signal<'a>,
{
    type Out = S::V;
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitS\"];",num);
        (num,num)
    }
}

impl<'a, S: Val<'a>> IntProcessNotIm<'a, ()> for AwaitS<S>
where
    S: Signal<'a> + Clone,
{
    type NI = NAwaitS<S>;
    type NO = NGetS<S>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let ni = NAwaitS(self.0.clone(), out_id);
        let no = NGetS(self.0);
        (ni, out_id, no)
    }
}

pub fn await_s<'a, S: Val<'a>>(signal_runtime: S)
    -> ProcessNotIm<'a, (), S::V, NotOnce, NAwaitS<S>, NGetS<S>>
where
    S: Signal<'a> + Clone,
    S::E: Clone,
{
    ProcessNotIm(box AwaitS(signal_runtime))
}



#[derive(Clone)]
pub struct AwaitSIn<S>(pub S);


impl<'a, In: Val<'a>, S: Val<'a>> IntProcess<'a, In> for AwaitSIn<S>
where
    S: Signal<'a>,
{
    type Out = (S::V, In);
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitS\"];",num);
        (num,num)
    }
}

impl<'a, In: Val<'a>, S: Val<'a>> IntProcessNotIm<'a, In> for AwaitSIn<S>
where
    S: Signal<'a> + Clone,
{
    type NI = NSeq<NStore<In>,NAwaitS<S>>;
    type NO = NSeq<GenP, NPar<NGetS<S>, NLoad<In>>>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = RCell::new();
        let rc2 = rc.clone();

        let ni = node!(store(rc) >> NAwaitS(self.0.clone(), out_id));
        let no = node!( GenP{} >> (NGetS(self.0) || load(rc2)));
        (ni, out_id, no)
    }
}

pub fn await_s_in<'a, In: Val<'a>, S: Val<'a>>(signal_runtime: S)
    -> ProcessNotIm<'a, In, (S::V, In), NotOnce, NSeq<NStore<In>,NAwaitS<S>>, NSeq<GenP, NPar<NGetS<S>, NLoad<In>>>>
where
    S: Signal<'a> + Clone,
    S::E: Clone,
{
    ProcessNotIm(box AwaitSIn(signal_runtime))
}



//     _                _ _   ___                              _ _       _       ____
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___|  _ \
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \ | | |
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/ |_| |
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|____/


/// Process awaiting for the emission of a signal, and executing the next process the current instant,
/// where the signal is given as the process input.
#[derive(Clone, Copy)]
pub struct AwaitImmediateD {}

impl<'a, S: Val<'a>> IntProcess<'a, S> for AwaitImmediateD
where
    S: Signal<'a>,
{
    type Out = ();
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitImmediateD\"];",num);
        (num,num)
    }
}

impl<'a, S: Val<'a>> IntProcessNotIm<'a, S> for AwaitImmediateD
where
    S: Signal<'a>,
{
    type NI = NAwaitImmediateD;
    type NO = Nothing;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        (NAwaitImmediateD(out_id), out_id, Nothing {})
    }
}

pub fn await_immediate_d<'a, S: Val<'a>>()
    -> ProcessNotIm<'a, S, (), NotOnce, NAwaitImmediateD, Nothing>
where
    S: Signal<'a>,
{
    ProcessNotIm(box AwaitImmediateD {})
}


impl<'a, S: Val<'a>, In: Val<'a>> IntProcess<'a, (S, In)> for AwaitImmediateD
where
    S: Signal<'a>,
{
    type Out = In;
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitImmediateD\"];",num);
        (num,num)
    }
}

impl<'a, In: Val<'a>, S: Val<'a>> IntProcessNotIm<'a, (S,In)> for AwaitImmediateD
where
    S: Signal<'a>,
{
    type NI = NSeq<NSeq<NPar<NIdentity,NStore<In>>, Ignore2>,NAwaitImmediateD>;
    type NO = NLoad<In>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = RCell::new();
        let rc2 = rc.clone();

        let ni_first = <NIdentity as Node<'a,S>>::njoin::<In, NStore<In>>(NIdentity {}, store(rc));
        let ni_second = <NPar<NIdentity,NStore<In>> as Node<'a, (S, In)>>::nseq(ni_first, Ignore2 {});
        let ni = <NSeq<NPar<NIdentity,NStore<In>>,Ignore2> as Node<'a, (S, In)>>::nseq(ni_second, NAwaitImmediateD(out_id));
        let no = load(rc2);
        (ni, out_id, no)
    }
}

pub fn await_immediate_d_in<'a, In: Val<'a>, S: Val<'a>>()
    -> ProcessNotIm<'a, (S, In), In, NotOnce, NSeq<NSeq<NPar<NIdentity,NStore<In>>, Ignore2>,NAwaitImmediateD>, NLoad<In>>
where
    S: Signal<'a>,
{
    ProcessNotIm(box AwaitImmediateD {})
}


//     _                _ _   ___                              _ _       _       ____
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___/ ___|
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \___ \
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/___) |
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|____/


/// Process awaiting for the emission of a signal, and executing the next process the current instant,
/// where the signal is fixed.
#[derive(Clone)]
pub struct AwaitImmediateS<S>(pub S);

impl<'a, S: Val<'a>, In: Val<'a>> IntProcess<'a, In> for AwaitImmediateS<S>
where
    S: Signal<'a>,
{
    type Out = In;
    type MarkOnce = NotOnce;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitImmediateS\"];",num);
        (num,num)
    }
}

impl<'a, In: Val<'a>, S: Val<'a>> IntProcessNotIm<'a, In> for AwaitImmediateS<S>
where
    S: Signal<'a> + Clone,
{
    type NI = NSeq<NStore<In>,NAwaitImmediateS<S>>;
    type NO = NLoad<In>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = RCell::new();
        let rc2 = rc.clone();

        let ni = node!(store(rc) >> NAwaitImmediateS(self.0.clone(), out_id));
        let no = load(rc2);
        (ni, out_id, no)
    }
}

pub fn await_immediate_s<'a, In: Val<'a>, S: Val<'a>>(signal_runtime: S)
    -> ProcessNotIm<'a, In, In, NotOnce, NSeq<NStore<In>,NAwaitImmediateS<S>>, NLoad<In>>
where
    S: Signal<'a> + Clone,
{
    ProcessNotIm(box AwaitImmediateS(signal_runtime))
}


//  ____                           _   ____
// |  _ \ _ __ ___  ___  ___ _ __ | |_|  _ \
// | |_) | '__/ _ \/ __|/ _ \ '_ \| __| | | |
// |  __/| | |  __/\__ \  __/ | | | |_| |_| |
// |_|   |_|  \___||___/\___|_| |_|\__|____/

/// Process that executes pt in the current instant if the signal is present this instant,
/// and executes pf in the next instant otherwise,
/// where the signal is given as the process input.
pub struct PresentD<PT, PF> {
    pub(crate) pt: PT,
    pub(crate) pf: PF,
}

impl<'a, PT, PF, S: Val<'a>, Out: Val<'a>> IntProcess<'a, S> for PresentD<PT, PF>
where
    PT: Process<'a, (), Out = Out>,
    PF: Process<'a, (), Out = Out>,
    S: Signal<'a>,
{
    type Out = Out;
    type MarkOnce = <And<PT::MarkOnce,PF::MarkOnce> as GiveOnce>::Once;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"PresentD\"];",num);
        (num,num)
    }
}

// NI - NI
implNI! {
    S,
    impl<'a, Out: Val<'a>, S: Val<'a>, MarkOnceT, MarkOnceF, PTNI, PTNO, PFNI, PFNO>
        for PresentD<ProcessNotIm<'a, (), Out, MarkOnceT, PTNI, PTNO>, ProcessNotIm<'a, (), Out, MarkOnceF, PFNI, PFNO>>
        where
        MarkOnceT: Once,
        MarkOnceF: Once,
        PTNI: Node<'a, (), Out = ()>,
        PTNO: Node<'a, (), Out = Out>,
        PFNI: Node<'a, (), Out = ()>,
        PFNO: Node<'a, (), Out = Out>,
        S: Signal<'a>,

    trait IntProcessNotIm<'a, S>
    {
         type NI = NPresentD;
         type NO = NLoad<Out>;

         fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let rct = RCell::new();
            let rcf = rct.clone();
            let rc_out = rct.clone();

            let s = *self;
            let (ptni, ptind, ptno) = s.pt.0.compile(g);
            let (pfni, pfind, pfno) = s.pf.0.compile(g);

            let out_id = g.reserve();
            g.set(ptind, box node!(ptno >> store(rct) >> njump(out_id)));
            g.set(pfind, box node!(pfno >> store(rcf) >> njump(out_id)));
            let nit_id = g.add(box ptni);
            let nif_id = g.add(box pfni);

            let ni = NPresentD {
                node_true: nit_id,
                node_false: nif_id,
            };

            (ni, out_id, load(rc_out))
        }
    }
}

// Im - NI
implNI! {
    S,
    impl<'a, Out: Val<'a>, S: Val<'a>, MarkOnceT, MarkOnceF, PTNIO, PFNI, PFNO>
        for PresentD<ProcessIm<'a, (), Out, MarkOnceT, PTNIO>, ProcessNotIm<'a, (), Out, MarkOnceF, PFNI, PFNO>>
        where
        MarkOnceT: Once,
        MarkOnceF: Once,
        PTNIO: Node<'a, (), Out = Out>,
        PFNI: Node<'a, (), Out = ()>,
        PFNO: Node<'a, (), Out = Out>,
        S: Signal<'a>,

    trait IntProcessNotIm<'a, S>
    {
         type NI = NPresentD;
         type NO = NLoad<Out>;

         fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let rct = RCell::new();
            let rcf = rct.clone();
            let rc_out = rct.clone();
            let s = *self;
            let ptnio = s.pt.0.compileIm(g);
            let (pfni, pfind, pfno) = s.pf.0.compile(g);

            let out_id = g.reserve();
            let ptind = g.add(box node!(ptnio >> store(rct) >> njump(out_id)));
            g.set(pfind, box node!(pfno >> store(rcf) >> njump(out_id)));
            let nif_id = g.add(box pfni);

            let ni = NPresentD {
                node_true: ptind,
                node_false: nif_id,
            };

            (ni, out_id, load(rc_out))
        }
    }
}

// NI - Im
implNI! {
    S,
    impl<'a, Out: Val<'a>, S: Val<'a>, MarkOnceT, MarkOnceF, PTNI, PTNO, PFNIO>
        for PresentD<ProcessNotIm<'a, (), Out, MarkOnceT, PTNI, PTNO>, ProcessIm<'a, (), Out, MarkOnceF, PFNIO>>
        where
        MarkOnceT: Once,
        MarkOnceF: Once,
        PFNIO: Node<'a, (), Out = Out>,
        PTNI: Node<'a, (), Out = ()>,
        PTNO: Node<'a, (), Out = Out>,
        S: Signal<'a>,

    trait IntProcessNotIm<'a, S>
    {
         type NI = NPresentD;
         type NO = NLoad<Out>;

         fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let rct = RCell::new();
            let rcf = rct.clone();
            let rc_out = rct.clone();

            let s = *self;

            let pfnio = s.pf.0.compileIm(g);
            let (ptni, ptind, ptno) = s.pt.0.compile(g);

            let out_id = g.reserve();
            let pfind = g.add(box node!(pfnio >> store(rcf) >> njump(out_id)));
            g.set(ptind, box node!(ptno >> store(rct) >> njump(out_id)));
            let nit_id = g.add(box ptni);

            let ni = NPresentD {
                node_true: nit_id,
                node_false: pfind,
            };

            (ni, out_id, load(rc_out))
        }
    }
}

// Im - Im
implNI! {
    S,
    impl<'a, Out: Val<'a>, S: Val<'a>, MarkOnceT, MarkOnceF, PTNIO, PFNIO>
        for PresentD<ProcessIm<'a, (), Out, MarkOnceT, PTNIO>, ProcessIm<'a, (), Out, MarkOnceF, PFNIO>>
        where
        MarkOnceT: Once,
        MarkOnceF: Once,
        PFNIO: Node<'a, (), Out = Out>,
        PTNIO: Node<'a, (), Out = Out>,
        S: Signal<'a>,

    trait IntProcessNotIm<'a, S>
    {
         type NI = NPresentD;
         type NO = NLoad<Out>;

         fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let rct = RCell::new();
            let rcf = rct.clone();
            let rc_out = rct.clone();

            let s = *self;

            let pfnio = s.pf.0.compileIm(g);
            let ptnio = s.pt.0.compileIm(g);

            let out_id = g.reserve();
            let pfind = g.add(box node!(pfnio >> store(rcf) >> njump(out_id)));
            let ptind = g.add(box node!(ptnio >> store(rct) >> njump(out_id)));

            let ni = NPresentD {
                node_true: ptind,
                node_false: pfind,
            };

            (ni, out_id, load(rc_out))
        }
    }
}

//  ____                           _   ____
// |  _ \ _ __ ___  ___  ___ _ __ | |_/ ___|
// | |_) | '__/ _ \/ __|/ _ \ '_ \| __\___ \
// |  __/| | |  __/\__ \  __/ | | | |_ ___) |
// |_|   |_|  \___||___/\___|_| |_|\__|____/

/// Process that executes pt in the current instant if the signal is present this instant,
/// and executes pf in the next instant otherwise,
/// where the signal is given as the process input.
pub struct PresentS<PT,PF,S> {
    pub(crate) pt: PT,
    pub(crate) pf: PF,
    pub(crate) signal_runtime: S,
}

impl<'a, PT, PF, S: Val<'a>, Out: Val<'a>> IntProcess<'a, ()> for PresentS<PT, PF, S>
where
    PT: Process<'a, (), Out = Out>,
    PF: Process<'a, (), Out = Out>,
    S: Signal<'a>,
{
    type Out = Out;
    type MarkOnce = <And<PT::MarkOnce, PT::MarkOnce> as GiveOnce>::Once;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"PresentD\"];",num);
        (num,num)
    }
}

// NI - NI
implNI! {
    (),
    impl<'a, Out: Val<'a>, S: Val<'a>, MarkOnceT, MarkOnceF, PTNI, PTNO, PFNI, PFNO>
        for PresentS<ProcessNotIm<'a, (), Out, MarkOnceT, PTNI, PTNO>, ProcessNotIm<'a, (), Out, MarkOnceF, PFNI, PFNO>, S>
        where
        MarkOnceT: Once,
        MarkOnceF: Once,
        PTNI: Node<'a, (), Out = ()>,
        PTNO: Node<'a, (), Out = Out>,
        PFNI: Node<'a, (), Out = ()>,
        PFNO: Node<'a, (), Out = Out>,
        S: Signal<'a>,

    trait IntProcessNotIm<'a, ()>
    {
         type NI = NPresentS<S>;
         type NO = NLoad<Out>;

         fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let rct = RCell::new();
            let rcf = rct.clone();
            let rc_out = rct.clone();

            let s = *self;
            let (ptni, ptind, ptno) = s.pt.0.compile(g);
            let (pfni, pfind, pfno) = s.pf.0.compile(g);

            let out_id = g.reserve();
            g.set(ptind, box node!(ptno >> store(rct) >> njump(out_id)));
            g.set(pfind, box node!(pfno >> store(rcf) >> njump(out_id)));
            let nit_id = g.add(box ptni);
            let nif_id = g.add(box pfni);

            let ni = NPresentS {
                node_true: nit_id,
                node_false: nif_id,
                signal_runtime: s.signal_runtime,
            };

            (ni, out_id, load(rc_out))
        }
    }
}

// Im - NI
implNI! {
    (),
    impl<'a, Out: Val<'a>, S: Val<'a>, MarkOnceT, MarkOnceF, PTNIO, PFNI, PFNO>
        for PresentS<ProcessIm<'a, (), Out, MarkOnceT, PTNIO>, ProcessNotIm<'a, (), Out, MarkOnceF, PFNI, PFNO>, S>
        where
        MarkOnceT: Once,
        MarkOnceF: Once,
        PTNIO: Node<'a, (), Out = Out>,
        PFNI: Node<'a, (), Out = ()>,
        PFNO: Node<'a, (), Out = Out>,
        S: Signal<'a>,

    trait IntProcessNotIm<'a, ()>
    {
         type NI = NPresentS<S>;
         type NO = NLoad<Out>;

         fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let rct = RCell::new();
            let rcf = rct.clone();
            let rc_out = rct.clone();

            let s = *self;
            let ptnio = s.pt.0.compileIm(g);
            let (pfni, pfind, pfno) = s.pf.0.compile(g);

            let out_id = g.reserve();
            let ptind = g.add(box node!(ptnio >> store(rct) >> njump(out_id)));
            g.set(pfind, box node!(pfno >> store(rcf) >> njump(out_id)));
            let nif_id = g.add(box pfni);

            let ni = NPresentS {
                node_true: ptind,
                node_false: nif_id,
                signal_runtime: s.signal_runtime,
            };

            (ni, out_id, load(rc_out))
        }
    }
}

// NI - Im
implNI! {
    (),
    impl<'a, Out: Val<'a>, S: Val<'a>, MarkOnceT, MarkOnceF, PTNI, PTNO, PFNIO>
        for PresentS<ProcessNotIm<'a, (), Out, MarkOnceT, PTNI, PTNO>, ProcessIm<'a, (), Out, MarkOnceF, PFNIO>, S>
        where
        MarkOnceT: Once,
        MarkOnceF: Once,
        PFNIO: Node<'a, (), Out = Out>,
        PTNI: Node<'a, (), Out = ()>,
        PTNO: Node<'a, (), Out = Out>,
        S: Signal<'a>,

    trait IntProcessNotIm<'a, ()>
    {
         type NI = NPresentS<S>;
         type NO = NLoad<Out>;

         fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let rct = RCell::new();
            let rcf = rct.clone();
            let rc_out = rct.clone();

            let s = *self;

            let pfnio = s.pf.0.compileIm(g);
            let (ptni, ptind, ptno) = s.pt.0.compile(g);

            let out_id = g.reserve();
            let pfind = g.add(box node!(pfnio >> store(rcf) >> njump(out_id)));
            g.set(ptind, box node!(ptno >> store(rct) >> njump(out_id)));
            let nit_id = g.add(box ptni);

            let ni = NPresentS {
                node_true: nit_id,
                node_false: pfind,
                signal_runtime: s.signal_runtime
            };

            (ni, out_id, load(rc_out))
        }
    }
}

// Im - Im
implNI! {
    (),
    impl<'a, Out: Val<'a>, S: Val<'a>, MarkOnceT, MarkOnceF, PTNIO, PFNIO>
        for PresentS<ProcessIm<'a, (), Out, MarkOnceT, PTNIO>, ProcessIm<'a, (), Out, MarkOnceF, PFNIO>, S>
        where
        MarkOnceT: Once,
        MarkOnceF: Once,
        PFNIO: Node<'a, (), Out = Out>,
        PTNIO: Node<'a, (), Out = Out>,
        S: Signal<'a>,

    trait IntProcessNotIm<'a, ()>
    {
         type NI = NPresentS<S>;
         type NO = NLoad<Out>;

         fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
            let rct = RCell::new();
            let rcf = rct.clone();
            let rc_out = rct.clone();

            let s = *self;

            let pfnio = s.pf.0.compileIm(g);
            let ptnio = s.pt.0.compileIm(g);

            let out_id = g.reserve();
            let pfind = g.add(box node!(pfnio >> store(rcf) >> njump(out_id)));
            let ptind = g.add(box node!(ptnio >> store(rct) >> njump(out_id)));

            let ni = NPresentS {
                node_true: ptind,
                node_false: pfind,
                signal_runtime: s.signal_runtime,
            };

            (ni, out_id, load(rc_out))
        }
    }
}

