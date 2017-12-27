use super::*;
use signal::*;
use std::marker::PhantomData;

//  _____           _ _   ____
// | ____|_ __ ___ (_) |_|  _ \
// |  _| | '_ ` _ \| | __| | | |
// | |___| | | | | | | |_| |_| |
// |_____|_| |_| |_|_|\__|____/

/// Process representing the emission of a signal,
/// where the signal and the value is given as the process input.
#[derive(Copy, Clone)]
pub struct EmitD {}

#[allow(non_upper_case_globals)]
pub static EmitD: EmitD = EmitD {};

impl<'a, In: 'a, SV: 'a> IntProcess<'a, ((SignalRuntimeRef<SV>, SV::E), In)> for EmitD
where SV: SignalValue {
    type Out = In;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"EmitD\"];", num);
        (num, num)
    }
}

impl<'a, In: 'a, SV: 'a> IntProcessIm<'a, ((SignalRuntimeRef<SV>, SV::E), In)> for EmitD
where
    SV: SignalValue
{
    type NIO = NEmitD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NEmitD {}
    }
}

impl<'a, SV: 'a> IntProcess<'a, (SignalRuntimeRef<SV>, SV::E)> for EmitD
where
    SV: SignalValue,
{
    type Out = ();

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"EmitD\"];", num);
        (num, num)
    }
}

impl<'a, SV: 'a> IntProcessIm<'a, (SignalRuntimeRef<SV>, SV::E)> for EmitD
where
    SV: SignalValue,
{
    type NIO = NEmitD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NEmitD {}
    }
}


impl<'a, In: 'a, SV: 'a> IntProcess<'a, (Vec<(SignalRuntimeRef<SV>, SV::E)>, In)> for EmitD
where
    SV: SignalValue,
{
    type Out = In;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"EmitD\"];", num);
        (num, num)
    }
}

impl<'a, In: 'a, SV: 'a> IntProcessIm<'a, (Vec<(SignalRuntimeRef<SV>, SV::E)>, In)> for EmitD
where
    SV: SignalValue,
{
    type NIO = NEmitD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NEmitD {}
    }
}


impl<'a, SV: 'a> IntProcess<'a, Vec<(SignalRuntimeRef<SV>, SV::E)>> for EmitD
    where
        SV: SignalValue,
{
    type Out = ();

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"EmitD\"];", num);
        (num, num)
    }
}

impl<'a, SV: 'a> IntProcessIm<'a, Vec<(SignalRuntimeRef<SV>, SV::E)>> for EmitD
where
    SV: SignalValue,
{
    type NIO = NEmitD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NEmitD {}
    }
}


pub fn emit_d_in<'a, SV: 'a, In: 'a>()
    -> ProcessIm<'a, ((SignalRuntimeRef<SV>, SV::E), In), In, NEmitD>
where
    SV: SignalValue
{
    ProcessIm(box EmitD {})
}

pub fn emit_d<'a, SV: 'a>()
    -> ProcessIm<'a, (SignalRuntimeRef<SV>, SV::E), (), NEmitD>
where
    SV: SignalValue
{
    ProcessIm(box EmitD {})
}


pub fn emit_d_vec_in<'a, SV: 'a, In: 'a>()
    -> ProcessIm<'a, (Vec<(SignalRuntimeRef<SV>, SV::E)>, In), In, NEmitD>
where
    SV: SignalValue
{
    ProcessIm(box EmitD {})
}

pub fn emit_d_vec<'a, SV: 'a>()
    -> ProcessIm<'a, Vec<(SignalRuntimeRef<SV>, SV::E)>, (), NEmitD>
where
    SV: SignalValue
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
pub struct EmitS<SV, E>(pub SignalRuntimeRef<SV>, pub PhantomData<E>);

impl<'a, SV: 'a> IntProcess<'a, SV::E> for EmitS<SV, SV::E>
where
    SV: SignalValue,
{
    type Out = ();

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitS\"];",num);
        (num,num)
    }
}

impl<'a, SV: 'a> IntProcessIm<'a, SV::E> for EmitS<SV, SV::E>
where
    SV: SignalValue,
{
    type NIO = NEmitS<SV, SV::E>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        let s = *self;
        NEmitS(s.0, s.1)
    }
}

pub fn emit_s<'a, SV: 'a>(signal_runtime: SignalRuntimeRef<SV>)
    -> ProcessIm<'a, SV::E, (), NEmitS<SV,SV::E>>
where
    SV: SignalValue
{
    ProcessIm(box EmitS(signal_runtime, PhantomData))
}

/// Process representing the emission of a signal,
/// where the signal is fixed and the value is given as the process input.
/// This structure is needed to passe a value to the next process
/// Rust does not understand that (SV::E,In) can't be equal to SV::E
#[derive(Clone)]
pub struct EmitSIn<SV, E>(pub SignalRuntimeRef<SV>, pub PhantomData<E>);

impl<'a, In: 'a, SV: 'a> IntProcess<'a, (SV::E, In)> for EmitSIn<SV, SV::E>
where
    SV: SignalValue,
{
    type Out = In;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitS\"];",num);
        (num,num)
    }
}

impl<'a, In: 'a, SV: 'a> IntProcessIm<'a, (SV::E, In)> for EmitSIn<SV, SV::E>
where
    SV: SignalValue,
{
    type NIO = NEmitS<SV, SV::E>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        let s = *self;
        NEmitS(s.0, s.1)
    }
}

pub fn emit_s_in<'a, In: 'a, SV: 'a>(signal_runtime: SignalRuntimeRef<SV>)
    -> ProcessIm<'a, (SV::E, In), In, NEmitS<SV,SV::E>>
where
    SV: SignalValue
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
pub struct EmitVecS<SV>(pub Vec<SignalRuntimeRef<SV>>);

impl<'a, SV: 'a> IntProcess<'a, Vec<SV::E>> for EmitVecS<SV>
where
    SV: SignalValue
{
    type Out = ();

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitVecS\"];",num);
        (num,num)
    }
}

impl<'a, SV: 'a> IntProcessIm<'a, Vec<SV::E>> for EmitVecS<SV>
where
    SV: SignalValue
{
    type NIO = NEmitVecS<SV>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVecS(self.0)
    }
}

pub fn emit_vec_s<'a, SV: 'a>(signal_runtimes: Vec<SignalRuntimeRef<SV>>)
    -> ProcessIm<'a, Vec<SV::E>, (), NEmitVecS<SV>>
where
    SV: SignalValue
{
    ProcessIm(box EmitVecS(signal_runtimes))
}



impl<'a, SV: 'a, In: 'a> IntProcess<'a, (Vec<SV::E>, In)> for EmitVecS<SV>
where
    SV: SignalValue
{
    type Out = In;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitVecS\"];",num);
        (num,num)
    }
}

impl<'a, SV: 'a, In: 'a> IntProcessIm<'a, (Vec<SV::E>, In)> for EmitVecS<SV>
where
    SV: SignalValue
{
    type NIO = NEmitVecS<SV>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVecS(self.0)
    }
}


pub fn emit_vec_s_in<'a, In: 'a, SV: 'a>(signal_runtimes: Vec<SignalRuntimeRef<SV>>)
    -> ProcessIm<'a, (Vec<SV::E>,In), In, NEmitVecS<SV>>
where
    SV: SignalValue
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
pub struct EmitVS<SV, E>(pub SignalRuntimeRef<SV>, pub E);

impl<'a, SV: 'a, In: 'a> IntProcess<'a, In> for EmitVS<SV, SV::E>
where
    SV: SignalValue,
    SV::E: Clone,
{
    type Out = In;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitVecS\"];",num);
        (num,num)
    }
}

impl<'a, SV: 'a, In: 'a> IntProcessIm<'a, In> for EmitVS<SV, SV::E>
where
    SV: SignalValue,
    SV::E: Clone
{
    type NIO = NEmitVS<SV, SV::E>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        let s = *self;
        NEmitVS(s.0, s.1)
    }
}


pub fn emit_vs<'a, In: 'a, SV: 'a>(signal_runtime: SignalRuntimeRef<SV>, value: SV::E)
    -> ProcessIm<'a, In, In, NEmitVS<SV, SV::E>>
where
    SV: SignalValue,
    SV::E: Clone,
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
pub struct EmitVVecS<SV, E>(pub Vec<(SignalRuntimeRef<SV>,E)>);


impl<'a, SV: 'a, In: 'a> IntProcess<'a, In> for EmitVVecS<SV, SV::E>
where
    SV: SignalValue,
    SV::E: Clone,
{
    type Out = In;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitVecS\"];",num);
        (num,num)
    }
}

impl<'a, SV: 'a, In: 'a> IntProcessIm<'a, In> for EmitVVecS<SV, SV::E>
where
    SV: SignalValue,
    SV::E: Clone
{
    type NIO = NEmitVVecS<SV, SV::E>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVVecS(self.0)
    }
}

pub fn emit_vec_vs<'a, In: 'a, SV: 'a>(signal_values: Vec<(SignalRuntimeRef<SV>, SV::E)>)
    -> ProcessIm<'a, In, In, NEmitVVecS<SV, SV::E>>
where
    SV: SignalValue,
    SV::E: Clone,
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

#[allow(non_upper_case_globals)]
pub static AwaitD: AwaitD = AwaitD {};

impl<'a, SV: 'a> IntProcess<'a, SignalRuntimeRef<SV>> for AwaitD
where
    SV: SignalValue,
{
    type Out = SV::V;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitD\"];",num);
        (num,num)
    }
}

impl<'a, SV: 'a> IntProcessNotIm<'a, SignalRuntimeRef<SV>> for AwaitD
where
    SV: SignalValue,
{
    type NI = NSeq<NAwaitD, RcStore<SignalRuntimeRef<SV>>>;
    type NO = NSeq<RcLoad<SignalRuntimeRef<SV>>, NGetD>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_rcell();
        let rc2 = rc.clone();

        let ni = node!(NAwaitD(out_id) >> store(rc));
        let no = node!(load(rc2) >> NGetD {});
        (ni, out_id, no)
    }
}

pub fn await_d<'a, SV: 'a>()
    -> ProcessNotIm<'a, SignalRuntimeRef<SV>, SV::V, NSeq<NAwaitD, RcStore<SignalRuntimeRef<SV>>>, NSeq<RcLoad<SignalRuntimeRef<SV>>, NGetD>>
    where
        SV: SignalValue,
        SV::E: Clone,
{
    ProcessNotIm(box AwaitD {})
}

impl<'a, SV: 'a, In: 'a> IntProcess<'a, (SignalRuntimeRef<SV>, In)> for AwaitD
where
    SV: SignalValue,
{
    type Out = (SV::V, In);

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitD\"];",num);
        (num,num)
    }
}

impl<'a, In: 'a, SV: 'a> IntProcessNotIm<'a, (SignalRuntimeRef<SV>, In)> for AwaitD
where
    SV: SignalValue,
{
    type NI = NSeq<NPar<NAwaitD,NIdentity>,RcStore<(SignalRuntimeRef<SV>, In)>>;
    type NO = NSeq<RcLoad<(SignalRuntimeRef<SV>, In)>, NGetD>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_rcell();
        let rc2 = rc.clone();

        // Type inference won't work here
        let ni_first = <NAwaitD as Node<'a,SignalRuntimeRef<SV>>>::njoin::<In, NIdentity>(NAwaitD(out_id), NIdentity {});
        let ni = node!(ni_first >> store(rc));
        let no = node!(load(rc2) >> NGetD{});
        (ni, out_id, no)
    }
}

pub fn await_d_in<'a, In: 'a, SV: 'a>()
    -> ProcessNotIm<'a, (SignalRuntimeRef<SV>,In), (SV::V, In), NSeq<NPar<NAwaitD,NIdentity>,RcStore<(SignalRuntimeRef<SV>, In)>>, NSeq<RcLoad<(SignalRuntimeRef<SV>, In)>, NGetD>>
where
    SV: SignalValue,
    SV::E: Clone,
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
pub struct AwaitS<SV>(pub SignalRuntimeRef<SV>);


impl<'a, SV: 'a> IntProcess<'a, ()> for AwaitS<SV>
where
    SV: SignalValue,
{
    type Out = SV::V;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitS\"];",num);
        (num,num)
    }
}

impl<'a, SV: 'a> IntProcessNotIm<'a, ()> for AwaitS<SV>
where
    SV: SignalValue,
{
    type NI = NAwaitS<SV>;
    type NO = NGetS<SV>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let ni = NAwaitS(self.0.clone(), out_id);
        let no = NGetS(self.0);
        (ni, out_id, no)
    }
}

pub fn await_s<'a, SV: 'a>(signal_runtime: SignalRuntimeRef<SV>)
    -> ProcessNotIm<'a, (), SV::V, NAwaitS<SV>, NGetS<SV>>
where
    SV: SignalValue,
    SV::E: Clone,
{
    ProcessNotIm(box AwaitS(signal_runtime))
}



#[derive(Clone)]
pub struct AwaitSIn<SV>(pub SignalRuntimeRef<SV>);


impl<'a, In: 'a, SV: 'a> IntProcess<'a, In> for AwaitSIn<SV>
where
    SV: SignalValue,
{
    type Out = (SV::V, In);

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitS\"];",num);
        (num,num)
    }
}

impl<'a, In: 'a, SV: 'a> IntProcessNotIm<'a, In> for AwaitSIn<SV>
where
    SV: SignalValue,
{
    type NI = NSeq<RcStore<In>,NAwaitS<SV>>;
    type NO = NSeq<GenP, NPar<NGetS<SV>, RcLoad<In>>>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_rcell();
        let rc2 = rc.clone();

        let ni = node!(store(rc) >> NAwaitS(self.0.clone(), out_id));
        let no = node!( GenP >> (NGetS(self.0) || load(rc2)));
        (ni, out_id, no)
    }
}

pub fn await_s_in<'a, In: 'a, SV: 'a>(signal_runtime: SignalRuntimeRef<SV>)
    -> ProcessNotIm<'a, In, (SV::V, In), NSeq<RcStore<In>,NAwaitS<SV>>, NSeq<GenP, NPar<NGetS<SV>, RcLoad<In>>>>
where
    SV: SignalValue,
    SV::E: Clone,
{
    ProcessNotIm(box AwaitSIn(signal_runtime))
}


/*
//     _                _ _   ___                              _ _       _       ____
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___|  _ \
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \ | | |
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/ |_| |
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|____/


/// Process awaiting for the emission of a signal, and executing the next process the current instant,
/// where the signal is given as the process input.
#[derive(Clone, Copy)]
pub struct AwaitImmediateD {}

#[allow(non_upper_case_globals)]
pub static AwaitImmediateD: AwaitImmediateD = AwaitImmediateD {};

impl<'a, SV: 'a> Process<'a, SignalRuntimeRef<SV>> for AwaitImmediateD
    where
        SV: SignalValue,
{
    type Out = ();
    type Mark = NotIm;
    type NIO = DummyN<()>;
    type NI = NAwaitImmediateD;
    type NO = Nothing;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        (NAwaitImmediateD(out_id), out_id, Nothing {})
    }

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitImmD\"];",num);
        (num,num)
    }
}


impl<'a, In: 'a, SV: 'a> Process<'a, (SignalRuntimeRef<SV>, In)> for AwaitImmediateD
    where
        SV: SignalValue,
{
    type Out = In;
    type Mark = NotIm;
    type NIO = DummyN<In>;
    type NI = NSeq<NSeq<NPar<NIdentity,RcStore<In>>, Ignore2>,NAwaitImmediateD>;
    type NO = RcLoad<In>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_rcell();
        let rc2 = rc.clone();

        let ni_first = <NIdentity as Node<'a,SignalRuntimeRef<SV>>>::njoin::<In, RcStore<In>>(NIdentity {}, store(rc));
        let ni_second = <NPar<NIdentity,RcStore<In>> as Node<'a, (SignalRuntimeRef<SV>, In)>>::nseq(ni_first, Ignore2);
        let ni = <NSeq<NPar<NIdentity,RcStore<In>>,Ignore2> as Node<'a, (SignalRuntimeRef<SV>, In)>>::nseq(ni_second, NAwaitImmediateD(out_id));
        let no = load(rc2);
        (ni, out_id, no)
    }

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitImmD\"];",num);
        (num,num)
    }
}

//     _                _ _   ___                              _ _       _       ____
//    / \__      ____ _(_) |_|_ _|_ __ ___  _ __ ___   ___  __| (_) __ _| |_ ___/ ___|
//   / _ \ \ /\ / / _` | | __|| || '_ ` _ \| '_ ` _ \ / _ \/ _` | |/ _` | __/ _ \___ \
//  / ___ \ V  V / (_| | | |_ | || | | | | | | | | | |  __/ (_| | | (_| | ||  __/___) |
// /_/   \_\_/\_/ \__,_|_|\__|___|_| |_| |_|_| |_| |_|\___|\__,_|_|\__,_|\__\___|____/


/// Process awaiting for the emission of a signal, and executing the next process the current instant,
/// where the signal is fixed.
#[derive(Clone)]
pub struct AwaitImmediateS<SV>(pub SignalRuntimeRef<SV>);


impl<'a, In: 'a, V: 'a, SV: 'a> Process<'a, In> for AwaitImmediateS<SV>
    where
        SV: SignalValue<V=V>,
{
    type Out = In;
    type Mark = NotIm;
    type NIO = DummyN<In>;
    type NI = NSeq<RcStore<In>,NAwaitImmediateS<SV>>;
    type NO = RcLoad<In>;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_rcell();
        let rc2 = rc.clone();

        let ni = node!(store(rc) >> NAwaitImmediateS(self.0.clone(), out_id));
        let no = load(rc2);
        (ni, out_id, no)
    }

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitImmS\"];",num);
        (num,num)
    }
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

impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, SignalRuntimeRef<SV>>
    for PresentD<MarkedProcess<PT, NotIm>, MarkedProcess<PF, NotIm>>
where
    PT: Process<'a, (), Out=Out>,
    PF: Process<'a, (), Out=Out>,
    SV: SignalValue,
{
    type Out = Out;
    type NI = NPresentD;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_rcell();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let (ptni, ptind, ptno) = self.pt.p.compile(g);
        let (pfni, pfind, pfno) = self.pf.p.compile(g);

        let out_id = g.reserve();
        g.set(ptind, box node!(ptno >> store(rct) >> jump(out_id)));
        g.set(pfind, box node!(pfno >> store(rcf) >> jump(out_id)));
        let nit_id = g.add(box ptni);
        let nif_id = g.add(box pfni);

        let ni = NPresentD {
            node_true: nit_id,
            node_false: nif_id,
        };

        (ni, out_id, load(rc_out))
    }

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"PresentD\"];",num);
        (num,num)
    }
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, SignalRuntimeRef<SV>>
for PresentD<MarkedProcess<PT, IsIm>, MarkedProcess<PF, NotIm>>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type Out = Out;
    type NI = NPresentD;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_rcell();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let ptnio = self.pt.p.compileIm(g);
        let (pfni, pfind, pfno) = self.pf.p.compile(g);

        let out_id = g.reserve();
        let ptind = g.add(box node!(ptnio >> store(rct) >> jump(out_id)));
        g.set(pfind, box node!(pfno >> store(rcf) >> jump(out_id)));
        let nif_id = g.add(box pfni);

        let ni = NPresentD {
            node_true: ptind,
            node_false: nif_id,
        };

        (ni, out_id, load(rc_out))
    }

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"PresentD\"];",num);
        (num,num)
    }
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, SignalRuntimeRef<SV>>
for PresentD<MarkedProcess<PT, NotIm>, MarkedProcess<PF, IsIm>>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type Out = Out;
    type NI = NPresentD;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_rcell();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let pfnio = self.pf.p.compileIm(g);
        let (ptni, ptind, ptno) = self.pt.p.compile(g);

        let out_id = g.reserve();
        let pfind = g.add(box node!(pfnio >> store(rcf) >> jump(out_id)));
        g.set(ptind, box node!(ptno >> store(rct) >> jump(out_id)));
        let nit_id = g.add(box ptni);

        let ni = NPresentD {
            node_true: nit_id,
            node_false: pfind,
        };

        (ni, out_id, load(rc_out))
    }

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"PresentD\"];",num);
        (num,num)
    }
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, SignalRuntimeRef<SV>>
for PresentD<MarkedProcess<PT, IsIm>, MarkedProcess<PF, IsIm>>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type Out = Out;
    type NI = NPresentD;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_rcell();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let pfnio = self.pf.p.compileIm(g);
        let ptnio = self.pt.p.compileIm(g);

        let out_id = g.reserve();
        let pfind = g.add(box node!(pfnio >> store(rcf) >> jump(out_id)));
        let ptind = g.add(box node!(ptnio >> store(rct) >> jump(out_id)));

        let ni = NPresentD {
            node_true: ptind,
            node_false: pfind,
        };

        (ni, out_id, load(rc_out))
    }

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"PresentD\"];",num);
        (num,num)
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
pub struct PresentS<PT,PF,SV> {
    pub(crate) pt: PT,
    pub(crate) pf: PF,
    pub(crate) signal_runtime: SignalRuntimeRef<SV>,
}

impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, ()>
for PresentS<MarkedProcess<PT, NotIm>, MarkedProcess<PF, NotIm>, SV>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type Out = Out;
    type NI = NPresentS<SV>;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_rcell();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let (ptni, ptind, ptno) = self.pt.p.compile(g);
        let (pfni, pfind, pfno) = self.pf.p.compile(g);

        let out_id = g.reserve();
        g.set(ptind, box node!(ptno >> store(rct) >> jump(out_id)));
        g.set(pfind, box node!(pfno >> store(rcf) >> jump(out_id)));
        let nit_id = g.add(box ptni);
        let nif_id = g.add(box pfni);

        let ni = NPresentS {
            node_true: nit_id,
            node_false: nif_id,
            signal_runtime: self.signal_runtime,
        };

        (ni, out_id, load(rc_out))
    }

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"PresentS\"];",num);
        (num,num)
    }
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, ()>
for PresentS<MarkedProcess<PT, IsIm>, MarkedProcess<PF, NotIm>, SV>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type Out = Out;
    type NI = NPresentS<SV>;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_rcell();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let ptnio = self.pt.p.compileIm(g);
        let (pfni, pfind, pfno) = self.pf.p.compile(g);

        let out_id = g.reserve();
        let ptind = g.add(box node!(ptnio >> store(rct) >> jump(out_id)));
        g.set(pfind, box node!(pfno >> store(rcf) >> jump(out_id)));
        let nif_id = g.add(box pfni);

        let ni = NPresentS {
            node_true: ptind,
            node_false: nif_id,
            signal_runtime: self.signal_runtime,
        };

        (ni, out_id, load(rc_out))
    }

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"PresentS\"];",num);
        (num,num)
    }
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, ()>
for PresentS<MarkedProcess<PT, NotIm>, MarkedProcess<PF, IsIm>, SV>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type Out = Out;
    type NI = NPresentS<SV>;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_rcell();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let pfnio = self.pf.p.compileIm(g);
        let (ptni, ptind, ptno) = self.pt.p.compile(g);

        let out_id = g.reserve();
        let pfind = g.add(box node!(pfnio >> store(rcf) >> jump(out_id)));
        g.set(ptind, box node!(ptno >> store(rct) >> jump(out_id)));
        let nit_id = g.add(box ptni);

        let ni = NPresentS {
            node_true: nit_id,
            node_false: pfind,
            signal_runtime: self.signal_runtime,
        };

        (ni, out_id, load(rc_out))
    }

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"PresentS\"];",num);
        (num,num)
    }
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, ()>
for PresentS<MarkedProcess<PT, IsIm>, MarkedProcess<PF, IsIm>, SV>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type Out = Out;
    type NI = NPresentS<SV>;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rct = new_rcell();
        let rcf = rct.clone();
        let rc_out = rct.clone();
        let pfnio = self.pf.p.compileIm(g);
        let ptnio = self.pt.p.compileIm(g);

        let out_id = g.reserve();
        let pfind = g.add(box node!(pfnio >> store(rcf) >> jump(out_id)));
        let ptind = g.add(box node!(ptnio >> store(rct) >> jump(out_id)));

        let ni = NPresentS {
            node_true: ptind,
            node_false: pfind,
            signal_runtime: self.signal_runtime,
        };

        (ni, out_id, load(rc_out))
    }

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"PresentS\"];",num);
        (num,num)
    }
}
*/