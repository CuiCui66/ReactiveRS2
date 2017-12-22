use node::*;
//use node::rcmanip::*;
//use node::control::*;
use super::*;
//  _   _       _   _     _
// | \ | | ___ | |_| |__ (_)_ __   __ _
// |  \| |/ _ \| __| '_ \| | '_ \ / _` |
// | |\  | (_) | |_| | | | | | | | (_| |
// |_| \_|\___/ \__|_| |_|_|_| |_|\__, |
//                                |___/

pub struct PNothing {}

impl<'a> GProcess<'a, ()> for PNothing {
    type Out = ();
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"Nothing\"];",num);
        (num,num)
    }
}


impl<'a> Process<'a, ()> for PNothing {
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type NIO = Nothing;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph) -> Self::NIO {
        Nothing {}
    }
}

// __     __    _
// \ \   / /_ _| |_   _  ___
//  \ \ / / _` | | | | |/ _ \
//   \ V / (_| | | |_| |  __/
//    \_/ \__,_|_|\__,_|\___|

pub struct PValue<V>(pub V);

pub fn value<V>(value: V) -> PValue<V> {
    PValue(value)
}

impl<'a, V: 'a> GProcess<'a, ()> for PValue<V>
    where
    V: Clone
{
    type Out = V;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"Value\"];",num);
        (num,num)
    }
}


impl<'a, V: 'a> Process<'a, ()> for PValue<V>
where
    V: Clone
{
    type NI = DummyN<()>;
    type NO = DummyN<V>;
    type NIO = NValue<V>;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph) -> Self::NIO {
        NValue(self.0)
    }
}


//  _____       ___
// |  ___| __  / _ \ _ __   ___ ___
// | |_ | '_ \| | | | '_ \ / __/ _ \
// |  _|| | | | |_| | | | | (_|  __/
// |_|  |_| |_|\___/|_| |_|\___\___|


pub struct PFnOnce<F>(pub F);

pub fn once<F>(f: F) -> PFnOnce<F> {
    PFnOnce(f)
}
impl<'a, F: 'a, In: 'a, Out: 'a> GProcess<'a, In> for PFnOnce<F>
    where
    F: FnOnce(In) -> Out,
{
    type Out = Out;
    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"FnOnce\"];",num);
        (num,num)
    }
}

impl<'a, F: 'a, In: 'a, Out: 'a> Process<'a, In> for PFnOnce<F>
where
    F: FnOnce(In) -> Out,
{
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = NFnOnce<F>;
    type Mark = IsIm;
    type MarkOnce = SOnce;

    fn compileIm(self, _: &mut Graph) -> Self::NIO {
        NFnOnce(Some(self.0))
    }
}

//  _____      __  __       _
// |  ___| __ |  \/  |_   _| |_
// | |_ | '_ \| |\/| | | | | __|
// |  _|| | | | |  | | |_| | |_
// |_|  |_| |_|_|  |_|\__,_|\__|

impl<'a, F: 'a, In: 'a, Out: 'a> GProcess<'a, In> for F
    where
    F: FnMut(In) -> Out,
{
    type Out = Out;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"FnMut\"];",num);
        (num,num)
    }
}

impl<'a, F: 'a, In: 'a, Out: 'a> Process<'a, In> for F
    where
    F: FnMut(In) -> Out,
{
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = FnMutN<F>;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;



    fn compileIm(self, _: &mut Graph) -> Self::NIO {
        FnMutN(self)
    }
}


//      _
//     | |_   _ _ __ ___  _ __
//  _  | | | | | '_ ` _ \| '_ \
// | |_| | |_| | | | | | | |_) |
//  \___/ \__,_|_| |_| |_| .__/
//                       |_|


#[derive(Copy, Clone, Debug)]
pub struct Jump {}

#[allow(non_upper_case_globals)]
pub static Jump: Jump = Jump {};

impl<'a, In: 'a> GProcess<'a, In> for Jump {
    type Out = In;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"Jump\"];",num);
        (num,num)
    }
}


impl<'a, In: 'a> Process<'a, In> for Jump {
    type NI = NSeq<RcStore<In>, NJump>;
    type NO = RcLoad<In>;
    type NIO = DummyN<In>;
    type Mark = NotIm;
    type MarkOnce = SNotOnce;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rcin = new_rcell();
        let rcout = rcin.clone();
        let out = g.reserve();
        (node!(store(rcin) >> jump(out)), out, load(rcout))
    }
}



//  ____
// |  _ \ __ _ _   _ ___  ___
// | |_) / _` | | | / __|/ _ \
// |  __/ (_| | |_| \__ \  __/
// |_|   \__,_|\__,_|___/\___|

#[derive(Copy, Clone, Debug)]
pub struct Pause {}

#[allow(non_upper_case_globals)]
pub static Pause: Pause = Pause {};


impl<'a, In: 'a> GProcess<'a, In> for Pause {
    type Out = In;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"Pause\"];",num);
        (num,num)
    }
}

impl<'a, In: 'a> Process<'a, In> for Pause {
    type NI = NSeq<RcStore<In>, NPause>;
    type NO = RcLoad<In>;
    type NIO = DummyN<In>;
    type Mark = NotIm;
    type MarkOnce = SNotOnce;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rcin = new_rcell();
        let rcout = rcin.clone();
        let out = g.reserve();
        (node!(store(rcin) >> pause(out)), out, load(rcout))
    }
}
