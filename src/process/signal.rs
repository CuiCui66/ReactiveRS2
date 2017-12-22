use node::*;
use super::*;

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

impl<'a, In: 'a, E: 'a, SV: 'a> GProcess<'a, ((SignalRuntimeRef<SV>, E),In)> for EmitD
    where
    SV: SignalValue<E=E>,
{
    type Out = In;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitD\"];",num);
        (num,num)
    }
}

impl<'a, In: 'a, E: 'a, SV: 'a> Process<'a, ((SignalRuntimeRef<SV>, E),In)> for EmitD
where
    SV: SignalValue<E=E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Mark = IsIm;
    type NIO = NEmitD;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        NEmitD {}
    }
}
impl<'a, E: 'a, SV: 'a> GProcess<'a, (SignalRuntimeRef<SV>, E)> for EmitD
    where
    SV: SignalValue<E = E>,
{
    type Out = ();
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitD\"];",num);
        (num,num)
    }
}

impl<'a, E: 'a, SV: 'a> Process<'a, (SignalRuntimeRef<SV>, E)> for EmitD
where
    SV: SignalValue<E = E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type Mark = IsIm;
    type NIO = NEmitD;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        NEmitD {}
    }
}

impl<'a, In: 'a, E: 'a, SV: 'a> GProcess<'a, (Vec<(SignalRuntimeRef<SV>, E)>,In)> for EmitD
    where
    SV: SignalValue<E=E>,
{
    type Out = In;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitD\"];",num);
        (num,num)
    }
}


impl<'a, In: 'a, E: 'a, SV: 'a> Process<'a, (Vec<(SignalRuntimeRef<SV>, E)>,In)> for EmitD
    where
        SV: SignalValue<E=E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Mark = IsIm;
    type NIO = NEmitD;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        NEmitD {}
    }
}

impl<'a, E: 'a, SV: 'a> GProcess<'a, Vec<(SignalRuntimeRef<SV>, E)>> for EmitD
    where
    SV: SignalValue<E = E>,
{
    type Out = ();
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitD\"];",num);
        (num,num)
    }
}


impl<'a, E: 'a, SV: 'a> Process<'a, Vec<(SignalRuntimeRef<SV>, E)>> for EmitD
    where
        SV: SignalValue<E = E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type Mark = IsIm;
    type NIO = NEmitD;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        NEmitD {}
    }
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

pub fn emit<SV>(sr: SignalRuntimeRef<SV>) -> EmitS<SV, SV::E>
where
    SV:SignalValue
{
    EmitS(sr, PhantomData)
}

impl<'a, E: 'a, SV: 'a> GProcess<'a, E> for EmitS<SV, E>
    where
    SV: SignalValue<E = E>,
{
    type Out = ();
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitS\"];",num);
        (num,num)
    }
}


impl<'a, E: 'a, SV: 'a> Process<'a, E> for EmitS<SV, E>
where
    SV: SignalValue<E = E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type Mark = IsIm;
    type NIO = NEmitS<SV, E>;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        NEmitS(self.0, PhantomData)
    }
}

impl<'a, SV: 'a, E: 'a, In: 'a> GProcess<'a, (E,In)> for EmitS<SV, E>
    where
    SV: SignalValue<E = E>,
{
    type Out = In;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitS\"];",num);
        (num,num)
    }
}



impl<'a, SV: 'a, E: 'a, In: 'a> Process<'a, (E,In)> for EmitS<SV, E>
where
    SV: SignalValue<E = E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type NIO = NEmitS<SV, E>;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        NEmitS(self.0, PhantomData)
    }
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

pub fn emit_vec<SV>(sr: Vec<SignalRuntimeRef<SV>>) -> EmitVecS<SV>
where
    SV:SignalValue
{
    EmitVecS(sr)
}

impl<'a, E: 'a, SV: 'a> GProcess<'a, Vec<E>> for EmitVecS<SV>
    where
    SV: SignalValue<E = E>,
{
    type Out = ();
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitVecS\"];",num);
        (num,num)
    }
}


impl<'a, E: 'a, SV: 'a> Process<'a, Vec<E>> for EmitVecS<SV>
where
    SV: SignalValue<E = E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type Mark = IsIm;
    type NIO = NEmitVecS<SV>;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVecS(self.0)
    }
}

impl<'a, SV: 'a, E: 'a, In: 'a> GProcess<'a, (Vec<E>,In)> for EmitVecS<SV>
    where
    SV: SignalValue<E = E>,
{
    type Out = In;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitVecS\"];",num);
        (num,num)
    }
}



impl<'a, SV: 'a, E: 'a, In: 'a> Process<'a, (Vec<E>,In)> for EmitVecS<SV>
where
    SV: SignalValue<E = E>,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type NIO = NEmitVecS<SV>;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVecS(self.0)
    }
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

pub fn emit_value<SV, E>(sr: SignalRuntimeRef<SV>, value: E) -> EmitVS<SV, E>
    where
SV: SignalValue<E = E>,
E: Clone,
{
    EmitVS(sr, value)
}

impl<'a, In: 'a, E: 'a, SV: 'a> GProcess<'a, In> for EmitVS<SV, E>
    where
    SV: SignalValue<E = E>,
    E: Clone,
{
    type Out = In;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitVS\"];",num);
        (num,num)
    }
}



impl<'a, In: 'a, E: 'a, SV: 'a> Process<'a, In> for EmitVS<SV, E>
where
    SV: SignalValue<E = E>,
    E: Clone,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Mark = IsIm;
    type NIO = NEmitVS<SV, E>;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVS(self.0, self.1)
    }
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

pub fn emit_value_vec<SV, E>(values: Vec<(SignalRuntimeRef<SV>,E)>) -> EmitVVecS<SV, E>
    where
        SV: SignalValue<E = E>,
        E: Clone,
{
    EmitVVecS(values)
}

impl<'a, In: 'a, E: 'a, SV: 'a> GProcess<'a, In> for EmitVVecS<SV, E>
    where
    SV: SignalValue<E = E>,
    E: Clone,
{
    type Out = In;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"EmitVVecS\"];",num);
        (num,num)
    }
}



impl<'a, In: 'a, E: 'a, SV: 'a> Process<'a, In> for EmitVVecS<SV, E>
    where
        SV: SignalValue<E = E>,
        E: Clone,
{
    type NI = DummyN<()>;
    type NO = DummyN<In>;
    type Mark = IsIm;
    type NIO = NEmitVVecS<SV, E>;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph<'a>) -> Self::NIO {
        NEmitVVecS(self.0)
    }
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

impl<'a, V: 'a, SV: 'a> GProcess<'a, SignalRuntimeRef<SV>> for AwaitD
    where
    SV: SignalValue<V = V>,
{
    type Out = V;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitD\"];",num);
        (num,num)
    }
}



impl<'a, V: 'a, SV: 'a> Process<'a, SignalRuntimeRef<SV>> for AwaitD
where
    SV: SignalValue<V = V>,
{
    type Mark = NotIm;
    type NIO = DummyN<V>;
    type NI = NSeq<NAwaitD, RcStore<SignalRuntimeRef<SV>>>;
    type NO = NSeq<RcLoad<SignalRuntimeRef<SV>>, NGetD>;
    type MarkOnce = SNotOnce;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_rcell();
        let rc2 = rc.clone();

        let ni = node!(NAwaitD(out_id) >> store(rc));
        let no = node!(load(rc2) >> NGetD {});
        (ni, out_id, no)
    }
}

impl<'a, In: 'a, V: 'a, SV: 'a> GProcess<'a, (SignalRuntimeRef<SV>, In)> for AwaitD
where
    SV: SignalValue<V=V>,
{
    type Out = (V,In);
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitD\"];",num);
        (num,num)
    }
}



impl<'a, In: 'a, V: 'a, SV: 'a> Process<'a, (SignalRuntimeRef<SV>, In)> for AwaitD
where
    SV: SignalValue<V=V>,
{
    type Mark = NotIm;
    type NIO = DummyN<(V,In)>;
    type NI = NSeq<NPar<NAwaitD,NIdentity>,RcStore<(SignalRuntimeRef<SV>, In)>>;
    type NO = NSeq<RcLoad<(SignalRuntimeRef<SV>, In)>, NGetD>;
    type MarkOnce = SNotOnce;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
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

//     _                _ _   ____
//    / \__      ____ _(_) |_/ ___|
//   / _ \ \ /\ / / _` | | __\___ \
//  / ___ \ V  V / (_| | | |_ ___) |
// /_/   \_\_/\_/ \__,_|_|\__|____/


/// Process awaiting for the emission of a signal, and executing the next process the next instant,
/// where the signal is fixed.
#[derive(Clone)]
pub struct AwaitS<SV>(pub SignalRuntimeRef<SV>);

impl<'a, In: 'a, V: 'a, SV: 'a> GProcess<'a, In> for AwaitS<SV>
    where
    SV: SignalValue<V=V>,
{
    type Out = (V,In);
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitS\"];",num);
        (num,num)
    }
}


impl<'a, In: 'a, V: 'a, SV: 'a> Process<'a, In> for AwaitS<SV>
where
    SV: SignalValue<V=V>,
{
    type Mark = NotIm;
    type NIO = DummyN<(V,In)>;
    type NI = NSeq<RcStore<In>,NAwaitS<SV>>;
    type NO = NSeq<GenP, NPar<NGetS<SV>, RcLoad<In>>>;
    type MarkOnce = SNotOnce;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        let rc = new_rcell();
        let rc2 = rc.clone();

        let ni = node!(store(rc) >> NAwaitS(self.0.clone(), out_id));
        let no = node!( GenP >> (NGetS(self.0) || load(rc2)));
        (ni, out_id, no)
    }
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

#[allow(non_upper_case_globals)]
pub static AwaitImmediateD: AwaitImmediateD = AwaitImmediateD {};

impl<'a, SV: 'a> GProcess<'a, SignalRuntimeRef<SV>> for AwaitImmediateD
    where
        SV: SignalValue,
{
    type Out = ();
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"AwaitImmD\"];",num);
        (num,num)
    }
}

impl<'a, SV: 'a> Process<'a, SignalRuntimeRef<SV>> for AwaitImmediateD
    where
    SV: SignalValue,
{
    type Mark = NotIm;
    type NIO = DummyN<()>;
    type NI = NAwaitImmediateD;
    type NO = Nothing;
    type MarkOnce = SNotOnce;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let out_id = g.reserve();
        (NAwaitImmediateD(out_id), out_id, Nothing {})
    }
}

impl<'a, In: 'a, SV: 'a> GProcess<'a, (SignalRuntimeRef<SV>, In)> for AwaitImmediateD
    where
        SV: SignalValue,
{
    type Out = In;
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
    type Mark = NotIm;
    type NIO = DummyN<In>;
    type NI = NSeq<NSeq<NPar<NIdentity,RcStore<In>>, Ignore2>,NAwaitImmediateD>;
    type NO = RcLoad<In>;
    type MarkOnce = SNotOnce;

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


impl<'a, In: 'a, V: 'a, SV: 'a> GProcess<'a, In> for AwaitImmediateS<SV>
    where
        SV: SignalValue<V=V>,
{
    type Out = In;
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

impl<'a, PT, PF, SV: 'a, Out: 'a> GProcess<'a, SignalRuntimeRef<SV>>
    for PresentD<PT, PF>
where
    PT: GProcess<'a, (), Out=Out>,
    PF: GProcess<'a, (), Out=Out>,
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


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, SignalRuntimeRef<SV>>
    for PresentD<MarkedProcess<PT, NotIm>, MarkedProcess<PF, NotIm>>
where
    PT: Process<'a, (), Out=Out>,
    PF: Process<'a, (), Out=Out>,
    SV: SignalValue,
{
    type NI = NPresentD;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

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
}

impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, SignalRuntimeRef<SV>>
for PresentD<MarkedProcess<PT, IsIm>, MarkedProcess<PF, NotIm>>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type NI = NPresentD;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

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
}

impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, SignalRuntimeRef<SV>>
for PresentD<MarkedProcess<PT, NotIm>, MarkedProcess<PF, IsIm>>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type NI = NPresentD;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

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
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, SignalRuntimeRef<SV>>
for PresentD<MarkedProcess<PT, IsIm>, MarkedProcess<PF, IsIm>>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type NI = NPresentD;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

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

impl<'a, PT, PF, SV: 'a, Out: 'a> GProcess<'a, ()>
for PresentS<PT, PF, SV>
    where
        PT: GProcess<'a, (), Out=Out>,
        PF: GProcess<'a, (), Out=Out>,
        SV: SignalValue,
{
    type Out = Out;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"PresentS\"];",num);
        (num,num)
    }
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, ()>
for PresentS<MarkedProcess<PT, NotIm>, MarkedProcess<PF, NotIm>, SV>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type NI = NPresentS<SV>;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

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
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, ()>
for PresentS<MarkedProcess<PT, IsIm>, MarkedProcess<PF, NotIm>, SV>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type NI = NPresentS<SV>;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

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
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, ()>
for PresentS<MarkedProcess<PT, NotIm>, MarkedProcess<PF, IsIm>, SV>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type NI = NPresentS<SV>;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

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
}


impl<'a, PT, PF, SV: 'a, Out: 'a> Process<'a, ()>
for PresentS<MarkedProcess<PT, IsIm>, MarkedProcess<PF, IsIm>, SV>
    where
        PT: Process<'a, (), Out=Out>,
        PF: Process<'a, (), Out=Out>,
        SV: SignalValue,
{
    type NI = NPresentS<SV>;
    type NO = RcLoad<Out>;
    type NIO = DummyN<Out>;
    type Mark = NotIm;
    type MarkOnce = And<PT::MarkOnce, PF::MarkOnce>;

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
}
