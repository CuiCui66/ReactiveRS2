use node::*;
use super::*;

//  _   _       _   _     _
// | \ | | ___ | |_| |__ (_)_ __   __ _
// |  \| |/ _ \| __| '_ \| | '_ \ / _` |
// | |\  | (_) | |_| | | | | | | | (_| |
// |_| \_|\___/ \__|_| |_|_|_| |_|\__, |
//                                |___/

/// A process implementation that does nothing : it just force its input-output
/// to be ()
pub struct PNothing {}
impl<'a> IntProcess<'a, ()> for PNothing {
    type Out = ();
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"Nothing\"];", num);
        (num, num)
    }
}


impl<'a> IntProcessIm<'a, ()> for PNothing {
    type NIO = Nothing;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        Nothing {}
    }
}

/// Builds a boxed `PNothing`
pub fn nothing<'a>() -> ProcessIm<'a, (), (), NotOnce, Nothing> {
    ProcessIm(box PNothing {})
}

// __     __    _
// \ \   / /_ _| |_   _  ___
//  \ \ / / _` | | | | |/ _ \
//   \ V / (_| | | |_| |  __/
//    \_/ \__,_|_|\__,_|\___|

/// A process implementation that returns always the same value (that must be clone).
pub struct PValue<V>(pub V);

/// Builds a boxed `PValue`
pub fn value<'a, V: Val<'a>>(value: V) -> ProcessIm<'a, (), V, NotOnce, NValue<V>>
where
    V: Clone,
{
    ProcessIm(box PValue(value))
}

impl<'a, V: Val<'a>> IntProcess<'a, ()> for PValue<V>
where
    V: Clone,
{
    type Out = V;
    type MarkOnce = NotOnce;
    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"Value\"];", num);
        (num, num)
    }
}

impl<'a, V: Val<'a>> IntProcessIm<'a, ()> for PValue<V>
where
    V: Clone,
{
    type NIO = NValue<V>;
    fn compileIm(self :Box<Self>, _: &mut Graph) -> Self::NIO {
        NValue(self.0)
    }
}


//  _____      __  __       _
// |  ___| __ |  \/  |_   _| |_
// | |_ | '_ \| |\/| | | | | __|
// |  _|| | | | |  | | |_| | |_
// |_|  |_| |_|_|  |_|\__,_|\__|


/// A process implementation that call a FnMut function. (Immediate)
pub struct PFnMut<F>(pub F);

/// Builds a boxed `PFnMut` from a instance of `FnMut`
pub fn fnmut2pro<'a, F: Val<'a>, In: Val<'a>, Out: Val<'a>>(
    f: F,
) -> ProcessIm<'a, In, Out, NotOnce, FnMutN<F>>
    where
    F: FnMut(In) -> Out,
{
    ProcessIm(box PFnMut(f))
}

impl<'a, F: Val<'a>, In: Val<'a>, Out: Val<'a>> IntProcess<'a, In> for PFnMut<F>
where
    F: FnMut(In) -> Out
{
    type Out = Out;
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"FnMut\"];", num);
        (num, num)
    }
}

impl<'a, F: Val<'a>, In: Val<'a>, Out: Val<'a>> IntProcessIm<'a, In> for PFnMut<F>
where
    F: FnMut(In) -> Out,
{
    type NIO = FnMutN<F>;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        FnMutN(self.0)
    }
}



//  _____       ___
// |  ___| __  / _ \ _ __   ___ ___
// | |_ | '_ \| | | | '_ \ / __/ _ \
// |  _|| | | | |_| | | | | (_|  __/
// |_|  |_| |_|\___/|_| |_|\___\___|


/// A process implementation that call a FnOnce function. (Immediate)
/// This process implementation can only be called Once.
/// The typechecking will normally forbid you to put it into a loop
pub struct PFnOnce<F>(pub F);

/// Builds a boxed `PFnOnce` from a instance of `FnOnce`
pub fn fnonce2pro<'a, F: Val<'a>, In: Val<'a>, Out: Val<'a>>(
    f: F,
) -> ProcessIm<'a, In, Out, IsOnce, NFnOnce<F>>
where
    F: FnOnce(In) -> Out,
{
    ProcessIm(box PFnOnce(f))
}

impl<'a, F: Val<'a>, In: Val<'a>, Out: Val<'a>> IntProcess<'a, In> for PFnOnce<F>
    where
    F: FnOnce(In) -> Out,
{
    type Out = Out;
    type MarkOnce = IsOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"FnOnce\"];", num);
        (num, num)
    }
}

impl<'a, F: Val<'a>, In: Val<'a>, Out: Val<'a>> IntProcessIm<'a, In> for PFnOnce<F>
    where
    F: FnOnce(In) -> Out,
{
    type NIO = NFnOnce<F>;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        NFnOnce(Some(self.0))
    }
}

//      _
//     | |_   _ _ __ ___  _ __
//  _  | | | | | '_ ` _ \| '_ \
// | |_| | |_| | | | | | | |_) |
//  \___/ \__,_|_| |_| |_| .__/
//                       |_|

/// An identity process that does nothing on semantics but break static node dependency,
/// and thus inlining.
///
/// If you have a long sequence of immediate process and Rust compile time is too slow,
/// You can insert some of these to break the sequence into multiple part.
///
/// Even if this process is executed in a single instant, it is still marked as non-immediate
/// in order to allow it to break the static dependency.
#[derive(Copy, Clone)]
pub(crate) struct PJump {}


/// Builds a boxed `PJump`
pub fn jump<'a, In: Val<'a>>()
                             -> ProcessNotIm<'a, In, In, NotOnce, NSeq<NStore<In>, NJump>, NLoad<In>>
{
    ProcessNotIm(box PJump {})
}


impl<'a, In: Val<'a>> IntProcess<'a, In> for PJump {
    type Out = In;
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"Jump\"];", num);
        (num, num)
    }
}


impl<'a, In: Val<'a>> IntProcessNotIm<'a, In> for PJump {
    type NI = NSeq<NStore<In>, NJump>;
    type NO = NLoad<In>;
    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rcin = RCell::new();
        let rcout = rcin.clone();
        let out = g.reserve();
        (node!(store(rcin) >> njump(out)), out, load(rcout))
    }
}

//  ____
// |  _ \ __ _ _   _ ___  ___
// | |_) / _` | | | / __|/ _ \
// |  __/ (_| | |_| \__ \  __/
// |_|   \__,_|\__,_|___/\___|

/// A pause process that correspond to the pause construct of RML.
/// Wait an instant before continuing execution. Any value is passed through without changes.
#[derive(Copy, Clone)]
pub(crate) struct PPause {}

/// Builds a boxed `PPause`
pub fn pause<'a, In: Val<'a>>()
                              -> ProcessNotIm<'a, In, In, NotOnce, NSeq<NStore<In>, NPause>, NLoad<In>>
{
    ProcessNotIm(box PPause {})
}

impl<'a, In: Val<'a>> IntProcess<'a, In> for PPause {
    type Out = In;
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"Pause\"];", num);
        (num, num)
    }
}

impl<'a, In: Val<'a>> IntProcessNotIm<'a, In> for PPause {
    type NI = NSeq<NStore<In>, NPause>;
    type NO = NLoad<In>;
    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rcin = RCell::new();
        let rcout = rcin.clone();
        let out = g.reserve();
        (node!(store(rcin) >> npause(out)), out, load(rcout))
    }
}

