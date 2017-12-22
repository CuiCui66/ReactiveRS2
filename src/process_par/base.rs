use node::*;
use super::*;

//  _   _       _   _     _
// | \ | | ___ | |_| |__ (_)_ __   __ _
// |  \| |/ _ \| __| '_ \| | '_ \ / _` |
// | |\  | (_) | |_| | | | | | | | (_| |
// |_| \_|\___/ \__|_| |_|_|_| |_|\__, |
//                                |___/

impl<'a> ProcessPar<'a, ()> for PNothing {
    type NI = DummyN<()>;
    type NO = DummyN<()>;
    type NIO = Nothing;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;

    fn compileIm_par(self, _: &mut GraphPar) -> Self::NIO {
        Nothing {}
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
    type NI = DummyN<()>;
    type NO = DummyN<V>;
    type NIO = NValue<V>;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;

    fn compileIm_par(self, _: &mut GraphPar) -> Self::NIO {
        NValue(self.0)
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
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = NFnOnce<F>;
    type Mark = IsIm;
    type MarkOnce = SOnce;

    fn compileIm_par(self, _: &mut GraphPar) -> Self::NIO {
        NFnOnce(Some(self.0))
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
    type NI = DummyN<()>;
    type NO = DummyN<Out>;
    type NIO = FnMutN<F>;
    type Mark = IsIm;
    type MarkOnce = SNotOnce;



    fn compileIm_par(self, _: &mut GraphPar) -> Self::NIO {
        FnMutN(self)
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
    type NI = NSeq<ArcStore<In>, NJump>;
    type NO = ArcLoad<In>;
    type NIO = DummyN<In>;
    type Mark = NotIm;
    type MarkOnce = SNotOnce;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let arcin = new_amutex();
        let arcout = arcin.clone();
        let out = g.reserve();
        (node!(store_par(arcin) >> jump(out)), out, load_par(arcout))
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
    type NI = NSeq<ArcStore<In>, NPause>;
    type NO = ArcLoad<In>;
    type NIO = DummyN<In>;
    type Mark = NotIm;
    type MarkOnce = SNotOnce;

    fn compile_par(self, g: &mut GraphPar<'a>) -> (Self::NI, usize, Self::NO) {
        let rcin = new_amutex();
        let rcout = rcin.clone();
        let out = g.reserve();
        (node!(store_par(rcin) >> pause(out)), out, load_par(rcout))
    }
}
