use node::*;
use super::*;

//  _   _       _   _     _
// | \ | | ___ | |_| |__ (_)_ __   __ _
// |  \| |/ _ \| __| '_ \| | '_ \ / _` |
// | |\  | (_) | |_| | | | | | | | (_| |
// |_| \_|\___/ \__|_| |_|_|_| |_|\__, |
//                                |___/

impl<'a> ProcessPar<'a, ()> for PNothing {
    type Out = ();
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type NIO = Nothing;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph) -> Self::NIO {
        Nothing {}
    }
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"Nothing\"];",num);
        (num,num)
    }
}

// __     __    _
// \ \   / /_ _| |_   _  ___
//  \ \ / / _` | | | | |/ _ \
//   \ V / (_| | | |_| |  __/
//    \_/ \__,_|_|\__,_|\___|

impl<'a, V: 'a> ProcessPar<'a, ()> for PValue<V>
where
    V: Clone + Send + Sync
{
    type Out = V;
    type NI = DummyN<()>;
    type NO = DummyN<V>;
    type NIO = NValue<V>;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;

    fn compileIm(self, _: &mut Graph) -> Self::NIO {
        NValue(self.0)
    }

    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"Value\"];",num);
        (num,num)
    }
}


//  _____       ___
// |  ___| __  / _ \ _ __   ___ ___
// | |_ | '_ \| | | | '_ \ / __/ _ \
// |  _|| | | | |_| | | | | (_|  __/
// |_|  |_| |_|\___/|_| |_|\___\___|

impl<'a, F: 'a, In: 'a, Out: 'a> ProcessPar<'a, In> for PFnOnce<F>
where
    F: FnOnce(In) -> Out + Send + Sync,
    Out: Send + Sync,
{
    type Out = Out;
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = NFnOnce<F>;
    type Mark = IsIm;
    type MarkOnce = SOnce;

    fn compileIm(self, _: &mut Graph) -> Self::NIO {
        NFnOnce(Some(self.0))
    }

    fn printDot(&mut self, curNum: &mut usize) -> (usize, usize) {
        let num = *curNum;
        *curNum += 1;
        println!("{} [shape = box, label= \"FnOnce\"];",num);
        (num,num)
    }
}

//  _____      __  __       _
// |  ___| __ |  \/  |_   _| |_
// | |_ | '_ \| |\/| | | | | __|
// |  _|| | | | |  | | |_| | |_
// |_|  |_| |_|_|  |_|\__,_|\__|


impl<'a, F: 'a, In: 'a, Out: 'a> ProcessPar<'a, In> for F
where
    F: FnMut(In) -> Out + Send + Sync,
    Out: Send + Sync,
{
    type Out = Out;
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = FnMutN<F>;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;



    fn compileIm(self, _: &mut Graph) -> Self::NIO {
        FnMutN(self)
    }
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

impl<'a, In: 'a> ProcessPar<'a, In> for Jump
where
    In: Send + Sync,
{
    type Out = In;
    type NI = NSeq<ArcStore<In>, NJump>;
    type NO = ArcLoad<In>;
    type NIO = DummyN<In>;
    type Mark = NotIm;
    type MarkOnce = SNotOnce;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let arcin = new_amutex();
        let arcout = arcin.clone();
        let out = g.reserve();
        (node!(store_par(arcin) >> jump(out)), out, load_par(arcout))
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

impl<'a, In: 'a> ProcessPar<'a, In> for Pause
where
    In: Send + Sync,
{
    type Out = In;
    type NI = NSeq<ArcStore<In>, NPause>;
    type NO = ArcLoad<In>;
    type NIO = DummyN<In>;
    type Mark = NotIm;
    type MarkOnce = SNotOnce;

    fn compile(self, g: &mut Graph<'a>) -> (Self::NI, usize, Self::NO) {
        let rcin = new_amutex();
        let rcout = rcin.clone();
        let out = g.reserve();
        (node!(store_par(rcin) >> pause(out)), out, load_par(rcout))
    }
    fn printDot(&mut self,curNum : &mut usize) -> (usize,usize){
        let num = *curNum;
        *curNum +=1;
        println!("{} [shape = box, label= \"Pause\"];",num);
        (num,num)
    }
}
