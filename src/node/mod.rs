use std::marker::PhantomData;

use engine::*;
use utility::{take,tname};
use std::collections::HashMap;
use super::*;

mod mem_manip;
pub use self::mem_manip::*;
mod control;
pub use self::control::*;
mod par;
pub use self::par::*;
mod signal;
pub use self::signal::*;
pub mod sig_control;
pub use self::sig_control::*;


pub trait Node<'a, In: Val<'a>>: Val<'a> {
    type Out: Val<'a>;
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: In) -> Self::Out;
    fn printDot(&mut self, _: &mut CFGDrawer) {
        print!("{}",tname::<Self>())
    }
    fn nseq<N2>(self, n2: N2) -> NSeq<Self, N2>
    where
        N2: Node<'a, Self::Out> + Sized,
        Self: Sized,
    {
        NSeq { n1: self, n2: n2 }
    }
    fn alter<NF, In2: Val<'a>>(self, nf: NF) -> NChoice<Self, NF>
    where
        NF: Node<'a, In2, Out = Self::Out> + Sized,
        Self: Sized,
    {
        NChoice { nt: self, nf: nf }
    }
    fn njoin<In2: Val<'a>, N2>(self, n2: N2) -> NPar<Self, N2>
    where
        N2: Node<'a, In2> + Sized,
        Self: Sized,
    {
        NPar { n1: self, n2: n2 }
    }
}


#[derive(Clone, Copy)]
pub struct Nothing {}

impl<'a> Node<'a, ()> for Nothing {
    type Out = ();
    fn call(&mut self, _: &mut SubRuntime<'a>, _val: ()) -> Self::Out {}
}



#[derive(Clone, Copy)]
pub struct NIdentity {}

impl<'a, In: Val<'a>> Node<'a, In> for NIdentity {
    type Out = In;

    fn call(&mut self, _: &mut SubRuntime<'a>, val: In) -> Self::Out {
        val
    }
}


//  _____                 _
// | ____|_ __ ___  _ __ | |_ _   _
// |  _| | '_ ` _ \| '_ \| __| | | |
// | |___| | | | | | |_) | |_| |_| |
// |_____|_| |_| |_| .__/ \__|\__, |
//                 |_|        |___/

pub struct DummyN<Out> {
    dummy: PhantomData<Out>,
}


impl<'a, In: Val<'a>, Out: Val<'a>> Node<'a, In> for DummyN<Out>
{
    type Out = Out;
    fn call(&mut self, _: &mut SubRuntime<'a>, _: In) -> Out {
        panic!("Called empty node");
    }
}


// __     __    _
// \ \   / /_ _| |_   _  ___
//  \ \ / / _` | | | | |/ _ \
//   \ V / (_| | | |_| |  __/
//    \_/ \__,_|_|\__,_|\___|

pub struct NValue<V>(pub V);

impl<'a, V: Val<'a>> Node<'a, ()> for NValue<V>
where
    V: Clone,
{
    type Out = V;

    fn call(&mut self, _: &mut SubRuntime<'a>, _: ()) -> V {
        self.0.clone()
    }
}


//  _____      __  __       _
// |  ___| __ |  \/  |_   _| |_
// | |_ | '_ \| |\/| | | | | __|
// |  _|| | | | |  | | |_| | |_
// |_|  |_| |_|_|  |_|\__,_|\__|

pub struct FnMutN<F>(pub F);

impl<'a, F, In: Val<'a>, Out: Val<'a>> Node<'a, In> for FnMutN<F>
where
    F: FnMut(In) -> Out + Val<'a>,
{
    type Out = Out;
    fn call(&mut self, _: &mut SubRuntime<'a>, val: In) -> Out {
        (&mut self.0)(val)
    }
    fn printDot(&mut self, _: &mut CFGDrawer) {
        print!("FnMut : {} -\\> {}", tname::<In>(), tname::<Out>())
    }
}


//  _____       ___
// |  ___| __  / _ \ _ __   ___ ___
// | |_ | '_ \| | | | '_ \ / __/ _ \
// |  _|| | | | |_| | | | | (_|  __/
// |_|  |_| |_|\___/|_| |_|\___\___|

pub struct NFnOnce<F>(pub Option<F>);

impl<'a, F, In: Val<'a>, Out: Val<'a>> Node<'a, In> for NFnOnce<F>
where
    F: FnOnce(In) -> Out + Val<'a>,
{
    type Out = Out;

    fn call(&mut self, _: &mut SubRuntime<'a>, val: In) -> Out {
        let option = take(&mut self.0);
        if let Some(f) = option {
            f(val)
        } else {
            panic!("NFnOnce was called twice!");
        }
    }
    fn printDot(&mut self, _: &mut CFGDrawer) {
        print!("FnOne : {} -> {}", tname::<In>(), tname::<Out>())
    }
}

//  ____
// / ___|  ___  __ _
// \___ \ / _ \/ _` |
//  ___) |  __/ (_| |
// |____/ \___|\__, |
//                |_|

pub struct NSeq<N1, N2> {
    n1: N1,
    n2: N2,
}

impl<'a, N1, N2, In: Val<'a>, Mid: Val<'a>, Out: Val<'a>> Node<'a, In> for NSeq<N1, N2>
where
    N1: Node<'a, In, Out = Mid>,
    N2: Node<'a, Mid, Out = Out>,
{
    type Out = Out;
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, val: In) -> Out {
        let valm = self.n1.call(sub_runtime, val);
        self.n2.call(sub_runtime, valm)
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        print!("{{{{");
        self.n1.printDot(cfgd);
        print!("}}|{{");
        self.n2.printDot(cfgd);
        print!("}}}}");
    }
}

//  _____           _
// | ____|_ __   __| |
// |  _| | '_ \ / _` |
// | |___| | | | (_| |
// |_____|_| |_|\__,_|

pub struct NEnd{}
impl<'a> Node<'a, ()> for NEnd
{
    type Out = ();
    fn call(&mut self, sub: &mut SubRuntime<'a>, _: ()) -> () {
        sub.end();
    }
}





//   ____                 _
//  / ___|_ __ __ _ _ __ | |__
// | |  _| '__/ _` | '_ \| '_ \
// | |_| | | | (_| | |_) | | | |
//  \____|_|  \__,_| .__/|_| |_|
//                 |_|

pub struct CFGDrawer {
    map: HashMap<usize, usize>,
    current_ind: usize,
    node: usize,
    node_ind: usize,
    arrow: Vec<(usize, usize)>,
}


impl CFGDrawer {
    pub fn new() -> CFGDrawer {
        CFGDrawer {
            map: HashMap::new(),
            current_ind: 0,
            node: 0,
            node_ind: 0,
            arrow: vec![]
        }
    }
    fn start_node(&mut self, node : usize){
        self.arrow.clear();
        self.node = node;
    }
    fn get_ind<T>(&mut self, ptr: *const T) -> usize {
        let u = ptr as usize;
        match self.map.get(&u) {
            Some(ind) => {return *ind; }
            None => {}
        }
        let ind = self.current_ind;
        self.current_ind += 1;
        self.map.insert(u,ind);
        ind
    }
    fn get_node(&mut self) -> usize{
        self.node
    }
    fn get_node_ind(&mut self) -> usize{
        let ind = self.node_ind;
        self.node_ind +=1;
        ind
    }
    fn add_arrow(&mut self, arr: (usize, usize)){
        self.arrow.push(arr)
    }
    fn get_arrow(&mut self) -> &[(usize, usize)]{
        &self.arrow
    }
}


pub fn printNode<'a,N : ?Sized>(ind: usize, n: &mut N, cfgd : &mut CFGDrawer)
    where N : Node<'a,(),Out =()>
{
    print!("{} [shape=record,label=\"",ind);
    cfgd.start_node(ind);
    n.printDot(cfgd);
    println!("\"]");
    for &(s,i) in cfgd.get_arrow(){
        println!("{}:f{} -> {}",ind,s,i);
    }
}
