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

impl<'a, In: Val<'a>, SV: Val<'a>> IntProcess<'a, ((SignalRuntimeRef<SV>, SV::E), In)> for EmitD
where SV: SignalValue {
    type Out = In;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"EmitD\"];", num);
        (num, num)
    }
}

impl<'a, In: Val<'a>, SV: Val<'a>> IntProcessIm<'a, ((SignalRuntimeRef<SV>, SV::E), In)> for EmitD
where
    SV: SignalValue
{
    type NIO = NEmitD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NEmitD {}
    }
}

impl<'a, SV: Val<'a>> IntProcess<'a, (SignalRuntimeRef<SV>, SV::E)> for EmitD
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

impl<'a, SV: Val<'a>> IntProcessIm<'a, (SignalRuntimeRef<SV>, SV::E)> for EmitD
where
    SV: SignalValue,
{
    type NIO = NEmitD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NEmitD {}
    }
}


impl<'a, In: Val<'a>, SV: Val<'a>> IntProcess<'a, (Vec<(SignalRuntimeRef<SV>, SV::E)>, In)> for EmitD
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

impl<'a, In: Val<'a>, SV: Val<'a>> IntProcessIm<'a, (Vec<(SignalRuntimeRef<SV>, SV::E)>, In)> for EmitD
where
    SV: SignalValue,
{
    type NIO = NEmitD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NEmitD {}
    }
}


impl<'a, SV: Val<'a>> IntProcess<'a, Vec<(SignalRuntimeRef<SV>, SV::E)>> for EmitD
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

impl<'a, SV: Val<'a>> IntProcessIm<'a, Vec<(SignalRuntimeRef<SV>, SV::E)>> for EmitD
where
    SV: SignalValue,
{
    type NIO = NEmitD;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NEmitD {}
    }
}


pub fn emit_d_in<'a, SV: Val<'a>, In: Val<'a>>()
    -> ProcessIm<'a, ((SignalRuntimeRef<SV>, SV::E), In), In, NEmitD>
where
    SV: SignalValue
{
    ProcessIm(box EmitD {})
}

pub fn emit_d<'a, SV: Val<'a>>()
    -> ProcessIm<'a, (SignalRuntimeRef<SV>, SV::E), (), NEmitD>
where
    SV: SignalValue
{
    ProcessIm(box EmitD {})
}


pub fn emit_d_vec_in<'a, SV: Val<'a>, In: Val<'a>>()
    -> ProcessIm<'a, (Vec<(SignalRuntimeRef<SV>, SV::E)>, In), In, NEmitD>
where
    SV: SignalValue
{
    ProcessIm(box EmitD {})
}

pub fn emit_d_vec<'a, SV: Val<'a>>()
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

impl<'a, SV: Val<'a>> IntProcess<'a, SV::E> for EmitS<SV, SV::E>
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

impl<'a, SV: Val<'a>> IntProcessIm<'a, SV::E> for EmitS<SV, SV::E>
where
    SV: SignalValue,
{
    type NIO = NEmitS<SV, SV::E>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        let s = *self;
        NEmitS(s.0, s.1)
    }
}

pub fn emit_s<'a, SV: Val<'a>>(signal_runtime: SignalRuntimeRef<SV>)
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

impl<'a, In: Val<'a>, SV: Val<'a>> IntProcess<'a, (SV::E, In)> for EmitSIn<SV, SV::E>
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

impl<'a, In: Val<'a>, SV: Val<'a>> IntProcessIm<'a, (SV::E, In)> for EmitSIn<SV, SV::E>
where
    SV: SignalValue,
{
    type NIO = NEmitS<SV, SV::E>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        let s = *self;
        NEmitS(s.0, s.1)
    }
}

pub fn emit_s_in<'a, In: Val<'a>, SV: Val<'a>>(signal_runtime: SignalRuntimeRef<SV>)
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

impl<'a, SV: Val<'a>> IntProcess<'a, Vec<SV::E>> for EmitVecS<SV>
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

impl<'a, SV: Val<'a>> IntProcessIm<'a, Vec<SV::E>> for EmitVecS<SV>
where
    SV: SignalValue
{
    type NIO = NEmitVecS<SV>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVecS(self.0)
    }
}

pub fn emit_vec_s<'a, SV: Val<'a>>(signal_runtimes: Vec<SignalRuntimeRef<SV>>)
    -> ProcessIm<'a, Vec<SV::E>, (), NEmitVecS<SV>>
where
    SV: SignalValue
{
    ProcessIm(box EmitVecS(signal_runtimes))
}



impl<'a, SV: Val<'a>, In: Val<'a>> IntProcess<'a, (Vec<SV::E>, In)> for EmitVecS<SV>
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

impl<'a, SV: Val<'a>, In: Val<'a>> IntProcessIm<'a, (Vec<SV::E>, In)> for EmitVecS<SV>
where
    SV: SignalValue
{
    type NIO = NEmitVecS<SV>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVecS(self.0)
    }
}


pub fn emit_vec_s_in<'a, In: Val<'a>, SV: Val<'a>>(signal_runtimes: Vec<SignalRuntimeRef<SV>>)
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

impl<'a, SV: Val<'a>, In: Val<'a>> IntProcess<'a, In> for EmitVS<SV, SV::E>
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

impl<'a, SV: Val<'a>, In: Val<'a>> IntProcessIm<'a, In> for EmitVS<SV, SV::E>
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


pub fn emit_vs<'a, In: Val<'a>, SV: Val<'a>>(signal_runtime: SignalRuntimeRef<SV>, value: SV::E)
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


impl<'a, SV: Val<'a>, In: Val<'a>> IntProcess<'a, In> for EmitVVecS<SV, SV::E>
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

impl<'a, SV: Val<'a>, In: Val<'a>> IntProcessIm<'a, In> for EmitVVecS<SV, SV::E>
where
    SV: SignalValue,
    SV::E: Clone
{
    type NIO = NEmitVVecS<SV, SV::E>;

    fn compileIm(self: Box<Self>, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVVecS(self.0)
    }
}

pub fn emit_vec_vs<'a, In: Val<'a>, SV: Val<'a>>(signal_values: Vec<(SignalRuntimeRef<SV>, SV::E)>)
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

impl<'a, SV: Val<'a>> IntProcess<'a, SignalRuntimeRef<SV>> for AwaitD
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

impl<'a, SV: Val<'a>> IntProcessNotIm<'a, SignalRuntimeRef<SV>> for AwaitD
where
    SV: SignalValue,
{
    type NI = NSeq<NAwaitD, NStore<SignalRuntimeRef<SV>>>;
    type NO = NSeq<NLoad<SignalRuntimeRef<SV>>, NGetD>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = RCell::new();
        let rc2 = rc.clone();

        let ni = node!(NAwaitD(out_id) >> store(rc));
        let no = node!(load(rc2) >> NGetD {});
        (ni, out_id, no)
    }
}

pub fn await_d<'a, SV: Val<'a>>()
    -> ProcessNotIm<'a, SignalRuntimeRef<SV>, SV::V, NSeq<NAwaitD, NStore<SignalRuntimeRef<SV>>>, NSeq<NLoad<SignalRuntimeRef<SV>>, NGetD>>
    where
        SV: SignalValue,
        SV::E: Clone,
{
    ProcessNotIm(box AwaitD {})
}

impl<'a, SV: Val<'a>, In: Val<'a>> IntProcess<'a, (SignalRuntimeRef<SV>, In)> for AwaitD
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

impl<'a, In: Val<'a>, SV: Val<'a>> IntProcessNotIm<'a, (SignalRuntimeRef<SV>, In)> for AwaitD
where
    SV: SignalValue,
{
    type NI = NSeq<NPar<NAwaitD,NIdentity>,NStore<(SignalRuntimeRef<SV>, In)>>;
    type NO = NSeq<NLoad<(SignalRuntimeRef<SV>, In)>, NGetD>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = RCell::new();
        let rc2 = rc.clone();

        // Type inference won't work here
        let ni_first = <NAwaitD as Node<'a,SignalRuntimeRef<SV>>>::njoin::<In, NIdentity>(NAwaitD(out_id), NIdentity {});
        let ni = node!(ni_first >> store(rc));
        let no = node!(load(rc2) >> NGetD{});
        (ni, out_id, no)
    }
}

pub fn await_d_in<'a, In: Val<'a>, SV: Val<'a>>()
    -> ProcessNotIm<'a, (SignalRuntimeRef<SV>,In), (SV::V, In), NSeq<NPar<NAwaitD,NIdentity>,NStore<(SignalRuntimeRef<SV>, In)>>, NSeq<NLoad<(SignalRuntimeRef<SV>, In)>, NGetD>>
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


impl<'a, SV: Val<'a>> IntProcess<'a, ()> for AwaitS<SV>
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

impl<'a, SV: Val<'a>> IntProcessNotIm<'a, ()> for AwaitS<SV>
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

pub fn await_s<'a, SV: Val<'a>>(signal_runtime: SignalRuntimeRef<SV>)
    -> ProcessNotIm<'a, (), SV::V, NAwaitS<SV>, NGetS<SV>>
where
    SV: SignalValue,
    SV::E: Clone,
{
    ProcessNotIm(box AwaitS(signal_runtime))
}



#[derive(Clone)]
pub struct AwaitSIn<SV>(pub SignalRuntimeRef<SV>);


impl<'a, In: Val<'a>, SV: Val<'a>> IntProcess<'a, In> for AwaitSIn<SV>
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

impl<'a, In: Val<'a>, SV: Val<'a>> IntProcessNotIm<'a, In> for AwaitSIn<SV>
where
    SV: SignalValue,
{
    type NI = NSeq<NStore<In>,NAwaitS<SV>>;
    type NO = NSeq<GenP, NPar<NGetS<SV>, NLoad<In>>>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = RCell::new();
        let rc2 = rc.clone();

        let ni = node!(store(rc) >> NAwaitS(self.0.clone(), out_id));
        let no = node!( GenP >> (NGetS(self.0) || load(rc2)));
        (ni, out_id, no)
    }
}

pub fn await_s_in<'a, In: Val<'a>, SV: Val<'a>>(signal_runtime: SignalRuntimeRef<SV>)
    -> ProcessNotIm<'a, In, (SV::V, In), NSeq<NStore<In>,NAwaitS<SV>>, NSeq<GenP, NPar<NGetS<SV>, NLoad<In>>>>
where
    SV: SignalValue,
    SV::E: Clone,
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

impl<'a, SV: Val<'a>> IntProcess<'a, SignalRuntimeRef<SV>> for AwaitImmediateD
where
    SV: SignalValue,
{
    type Out = ();

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitImmediateD\"];",num);
        (num,num)
    }
}

impl<'a, SV: Val<'a>> IntProcessNotIm<'a, SignalRuntimeRef<SV>> for AwaitImmediateD
where
    SV: SignalValue,
{
    type NI = NAwaitImmediateD;
    type NO = Nothing;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        (NAwaitImmediateD(out_id), out_id, Nothing {})
    }
}

pub fn await_immediate_d<'a, SV: Val<'a>>()
    -> ProcessNotIm<'a, SignalRuntimeRef<SV>, (), NAwaitImmediateD, Nothing>
where
    SV: SignalValue,
{
    ProcessNotIm(box AwaitImmediateD {})
}


impl<'a, SV: Val<'a>, In: Val<'a>> IntProcess<'a, (SignalRuntimeRef<SV>, In)> for AwaitImmediateD
where
    SV: SignalValue,
{
    type Out = In;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitImmediateD\"];",num);
        (num,num)
    }
}

impl<'a, In: Val<'a>, SV: Val<'a>> IntProcessNotIm<'a, (SignalRuntimeRef<SV>,In)> for AwaitImmediateD
where
    SV: SignalValue,
{
    type NI = NSeq<NSeq<NPar<NIdentity,NStore<In>>, Ignore2>,NAwaitImmediateD>;
    type NO = NLoad<In>;

    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = RCell::new();
        let rc2 = rc.clone();

        let ni_first = <NIdentity as Node<'a,SignalRuntimeRef<SV>>>::njoin::<In, NStore<In>>(NIdentity {}, store(rc));
        let ni_second = <NPar<NIdentity,NStore<In>> as Node<'a, (SignalRuntimeRef<SV>, In)>>::nseq(ni_first, Ignore2);
        let ni = <NSeq<NPar<NIdentity,NStore<In>>,Ignore2> as Node<'a, (SignalRuntimeRef<SV>, In)>>::nseq(ni_second, NAwaitImmediateD(out_id));
        let no = load(rc2);
        (ni, out_id, no)
    }
}

pub fn await_immediate_d_in<'a, In: Val<'a>, SV: Val<'a>>()
    -> ProcessNotIm<'a, (SignalRuntimeRef<SV>, In), In, NSeq<NSeq<NPar<NIdentity,NStore<In>>, Ignore2>,NAwaitImmediateD>, NLoad<In>>
where
    SV: SignalValue,
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
pub struct AwaitImmediateS<SV>(pub SignalRuntimeRef<SV>);

impl<'a, SV: Val<'a>, In: Val<'a>> IntProcess<'a, In> for AwaitImmediateS<SV>
where
    SV: SignalValue,
{
    type Out = In;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitImmediateS\"];",num);
        (num,num)
    }
}

impl<'a, In: Val<'a>, SV: Val<'a>> IntProcessNotIm<'a, In> for AwaitImmediateS<SV>
where
    SV: SignalValue,
{
    type NI = NSeq<NStore<In>,NAwaitImmediateS<SV>>;
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

pub fn await_immediate_s<'a, In: Val<'a>, SV: Val<'a>>(signal_runtime: SignalRuntimeRef<SV>)
    -> ProcessNotIm<'a, In, In, NSeq<NStore<In>,NAwaitImmediateS<SV>>, NLoad<In>>
where
    SV: SignalValue,
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

impl<'a, PT, PF, SV: Val<'a>, Out: Val<'a>> IntProcess<'a, SignalRuntimeRef<SV>> for PresentD<PT, PF>
where
    PT: Process<'a, (), Out = Out>,
    PF: Process<'a, (), Out = Out>,
    SV: SignalValue,
{
    type Out = Out;

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"PresentD\"];",num);
        (num,num)
    }
}

// NI - NI
implNI! {
    SignalRuntimeRef<SV>,
    impl<'a, Out: Val<'a>, SV: Val<'a>, PTNI, PTNO, PFNI, PFNO>
        for PresentD<ProcessNotIm<'a, (), Out, PTNI, PTNO>, ProcessNotIm<'a, (), Out, PFNI, PFNO>>
        where
        PTNI: Node<'a, (), Out = ()>,
        PTNO: Node<'a, (), Out = Out>,
        PFNI: Node<'a, (), Out = ()>,
        PFNO: Node<'a, (), Out = Out>,
        SV: SignalValue,

    trait IntProcessNotIm<'a, SignalRuntimeRef<SV>>
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
    SignalRuntimeRef<SV>,
    impl<'a, Out: Val<'a>, SV: Val<'a>, PTNIO, PFNI, PFNO>
        for PresentD<ProcessIm<'a, (), Out, PTNIO>, ProcessNotIm<'a, (), Out, PFNI, PFNO>>
        where
        PTNIO: Node<'a, (), Out = Out>,
        PFNI: Node<'a, (), Out = ()>,
        PFNO: Node<'a, (), Out = Out>,
        SV: SignalValue,

    trait IntProcessNotIm<'a, SignalRuntimeRef<SV>>
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
    SignalRuntimeRef<SV>,
    impl<'a, Out: Val<'a>, SV: Val<'a>, PTNI, PTNO, PFNIO>
        for PresentD<ProcessNotIm<'a, (), Out, PTNI, PTNO>, ProcessIm<'a, (), Out, PFNIO>>
        where
        PFNIO: Node<'a, (), Out = Out>,
        PTNI: Node<'a, (), Out = ()>,
        PTNO: Node<'a, (), Out = Out>,
        SV: SignalValue,

    trait IntProcessNotIm<'a, SignalRuntimeRef<SV>>
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
    SignalRuntimeRef<SV>,
    impl<'a, Out: Val<'a>, SV: Val<'a>, PTNIO, PFNIO>
        for PresentD<ProcessIm<'a, (), Out, PTNIO>, ProcessIm<'a, (), Out, PFNIO>>
        where
        PFNIO: Node<'a, (), Out = Out>,
        PTNIO: Node<'a, (), Out = Out>,
        SV: SignalValue,

    trait IntProcessNotIm<'a, SignalRuntimeRef<SV>>
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
pub struct PresentS<PT,PF,SV> {
    pub(crate) pt: PT,
    pub(crate) pf: PF,
    pub(crate) signal_runtime: SignalRuntimeRef<SV>,
}

impl<'a, PT, PF, SV: Val<'a>, Out: Val<'a>> IntProcess<'a, ()> for PresentS<PT, PF, SV>
where
    PT: Process<'a, (), Out = Out>,
    PF: Process<'a, (), Out = Out>,
    SV: SignalValue,
{
    type Out = Out;

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
    impl<'a, Out: Val<'a>, SV: Val<'a>, PTNI, PTNO, PFNI, PFNO>
        for PresentS<ProcessNotIm<'a, (), Out, PTNI, PTNO>, ProcessNotIm<'a, (), Out, PFNI, PFNO>, SV>
        where
        PTNI: Node<'a, (), Out = ()>,
        PTNO: Node<'a, (), Out = Out>,
        PFNI: Node<'a, (), Out = ()>,
        PFNO: Node<'a, (), Out = Out>,
        SV: SignalValue,

    trait IntProcessNotIm<'a, ()>
    {
         type NI = NPresentS<SV>;
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
    impl<'a, Out: Val<'a>, SV: Val<'a>, PTNIO, PFNI, PFNO>
        for PresentS<ProcessIm<'a, (), Out, PTNIO>, ProcessNotIm<'a, (), Out, PFNI, PFNO>, SV>
        where
        PTNIO: Node<'a, (), Out = Out>,
        PFNI: Node<'a, (), Out = ()>,
        PFNO: Node<'a, (), Out = Out>,
        SV: SignalValue,

    trait IntProcessNotIm<'a, ()>
    {
         type NI = NPresentS<SV>;
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
    impl<'a, Out: Val<'a>, SV: Val<'a>, PTNI, PTNO, PFNIO>
        for PresentS<ProcessNotIm<'a, (), Out, PTNI, PTNO>, ProcessIm<'a, (), Out, PFNIO>, SV>
        where
        PFNIO: Node<'a, (), Out = Out>,
        PTNI: Node<'a, (), Out = ()>,
        PTNO: Node<'a, (), Out = Out>,
        SV: SignalValue,

    trait IntProcessNotIm<'a, ()>
    {
         type NI = NPresentS<SV>;
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
    impl<'a, Out: Val<'a>, SV: Val<'a>, PTNIO, PFNIO>
        for PresentS<ProcessIm<'a, (), Out, PTNIO>, ProcessIm<'a, (), Out, PFNIO>, SV>
        where
        PFNIO: Node<'a, (), Out = Out>,
        PTNIO: Node<'a, (), Out = Out>,
        SV: SignalValue,

    trait IntProcessNotIm<'a, ()>
    {
         type NI = NPresentS<SV>;
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

