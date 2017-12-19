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


impl<'a> Process<'a, ()> for PNothing {
    type Out = ();
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type NIO = Nothing;
    fn compileIm(self, _: &mut Graph) -> Self::NIO {
        Nothing {}
    }
    type Mark = IsIm;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"Nothing\"];",num);
        (num,num)
    }
}

//  _____      __  __       _
// |  ___| __ |  \/  |_   _| |_
// | |_ | '_ \| |\/| | | | | __|
// |  _|| | | | |  | | |_| | |_
// |_|  |_| |_|_|  |_|\__,_|\__|


impl<'a, F: 'a, In: 'a, Out: 'a> Process<'a, In> for F
    where
    F: FnMut(In) -> Out,
{
    type Out = Out;
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = FnMutN<F>;



    fn compileIm(self, _: &mut Graph) -> Self::NIO {
        FnMutN(self)
    }
    type Mark = IsIm;
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"FnMut\"];",num);
        (num,num)
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

impl<'a, In: 'a> Process<'a, In> for Jump {
    type Out = In;
    type NI = NSeq<RcStore<In>, NJump>;
    type NO = RcLoad<In>;
    type NIO = DummyN<In>;
    type Mark = NotIm;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rcin = new_rcell();
        let rcout = rcin.clone();
        let out = g.reserve();
        (node!(store(rcin) >> jump(out)), out, load(rcout))
    }
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"Jump\"];",num);
        (num,num)
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

impl<'a, In: 'a> Process<'a, In> for Pause {
    type Out = In;
    type NI = NSeq<RcStore<In>, NPause>;
    type NO = RcLoad<In>;
    type NIO = DummyN<In>;
    type Mark = NotIm;
    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rcin = new_rcell();
        let rcout = rcin.clone();
        let out = g.reserve();
        (node!(store(rcin) >> pause(out)), out, load(rcout))
    }
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"Pause\"];",num);
        (num,num)
    }
}
