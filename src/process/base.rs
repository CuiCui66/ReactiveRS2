use node::*;
use super::*;

//  _   _       _   _     _
// | \ | | ___ | |_| |__ (_)_ __   __ _
// |  \| |/ _ \| __| '_ \| | '_ \ / _` |
// | |\  | (_) | |_| | | | | | | | (_| |
// |_| \_|\___/ \__|_| |_|_|_| |_|\__, |
//                                |___/

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

pub fn nothing<'a>() -> ProcessIm<'a,(),(),NotOnce, Nothing>{
    ProcessIm(box PNothing {})
}

//  _____      __  __       _
// |  ___| __ |  \/  |_   _| |_
// | |_ | '_ \| |\/| | | | | __|
// |  _|| | | | |  | | |_| | |_
// |_|  |_| |_|_|  |_|\__,_|\__|

impl<'a, F: Val<'a>, In: Val<'a>, Out: Val<'a>> IntProcess<'a, In> for F
where
    F: FnMut(In) -> Out,
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

impl<'a, F: Val<'a>, In: Val<'a>, Out: Val<'a>> IntProcessIm<'a, In> for F
where
    F: FnMut(In) -> Out,
{
    type NIO = FnMutN<F>;
    fn compileIm(self: Box<Self>, _: &mut Graph) -> Self::NIO {
        FnMutN(*self)
    }
}

pub fn fnmut2pro<'a, F: Val<'a>, In: Val<'a>, Out: Val<'a>>(f: F)
    -> ProcessIm<'a, In, Out, NotOnce, FnMutN<F>>
where
    F: FnMut(In) -> Out,
{
    ProcessIm(box f)
}


//  _____       ___
// |  ___| __  / _ \ _ __   ___ ___
// | |_ | '_ \| | | | '_ \ / __/ _ \
// |  _|| | | | |_| | | | | (_|  __/
// |_|  |_| |_|\___/|_| |_|\___\___|


pub struct PFnOnce<F>(pub F);

pub fn fnonce2pro<'a, F: Val<'a>, In: Val<'a>, Out: Val<'a>>(f: F)
    -> ProcessIm<'a, In, Out, IsOnce, NFnOnce<F>>
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


#[derive(Copy, Clone)]
pub(crate) struct Jump {}

impl<'a, In: Val<'a>> IntProcess<'a, In> for Jump {
    type Out = In;
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"Jump\"];", num);
        (num, num)
    }
}


impl<'a, In: Val<'a>> IntProcessNotIm<'a, In> for Jump {
    type NI = NSeq<NStore<In>, NJump>;
    type NO = NLoad<In>;
    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rcin = RCell::new();
        let rcout = rcin.clone();
        let out = g.reserve();
        (node!(store(rcin) >> njump(out)), out, load(rcout))
    }
}

pub fn jump<'a, In: Val<'a>>() -> ProcessNotIm<'a,In,In,NotOnce,NSeq<NStore<In>, NJump>,NLoad<In>> {
    ProcessNotIm(box Jump {})
}

//  ____
// |  _ \ __ _ _   _ ___  ___
// | |_) / _` | | | / __|/ _ \
// |  __/ (_| | |_| \__ \  __/
// |_|   \__,_|\__,_|___/\___|

#[derive(Copy, Clone)]
pub(crate) struct Pause {}

impl<'a, In: Val<'a>> IntProcess<'a, In> for Pause {
    type Out = In;
    type MarkOnce = NotOnce;

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"Pause\"];", num);
        (num, num)
    }
}


impl<'a, In: Val<'a>> IntProcessNotIm<'a, In> for Pause {
    type NI = NSeq<NStore<In>, NPause>;
    type NO = NLoad<In>;
    fn compile(self: Box<Self>, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rcin = RCell::new();
        let rcout = rcin.clone();
        let out = g.reserve();
        (node!(store(rcin) >> npause(out)), out, load(rcout))
    }
}

pub fn pause<'a, In: Val<'a>>() -> ProcessNotIm<'a,In,In,NotOnce,NSeq<NStore<In>, NPause>,NLoad<In>> {
    ProcessNotIm(box Pause {})
}
