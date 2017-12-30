use engine::*;
use super::*;


//      _
//     | |_   _ _ __ ___  _ __
//  _  | | | | | '_ ` _ \| '_ \
// | |_| | |_| | | | | | | |_) |
//  \___/ \__,_|_| |_| |_| .__/
//                       |_|

/// Node that schedule a main node for the current instant
///
/// Signature : `() -> ()`
pub struct NJump {
    /// id of the main node this node points to.
    dest: usize,
}

/// Build a node that jumps to dest when called
pub fn njump(dest: usize) -> NJump {
    NJump { dest}
}

impl<'a> Node<'a, ()> for NJump {
    type Out = ();
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, _: ()) {
        sub_runtime.add_current(self.dest);
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        let ind = cfgd.get_node_ind();
        print!("<f{}> Jump",ind);
        cfgd.add_arrow((ind,self.dest));
    }
}


//  ____
// |  _ \ __ _ _   _ ___  ___
// | |_) / _` | | | / __|/ _ \
// |  __/ (_| | |_| \__ \  __/
// |_|   \__,_|\__,_|___/\___|

/// Node that schedule a main node for the next instant
///
/// Signature : `() -> ()`
pub struct NPause {
    /// id of the main node this node points to.
    dest: usize,
}

/// Build a node that schedule `dest` on the next instant
pub fn npause(pos: usize) -> NPause {
    NPause { dest: pos }
}


impl<'a> Node<'a, ()> for NPause {
    type Out = ();
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, _: ()) {
        sub_runtime.add_next(self.dest);
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer) {
        let ind = cfgd.get_node_ind();
        print!("<f{}> Pause",ind);
        cfgd.add_arrow((ind,self.dest));
    }
}

//   ____ _           _
//  / ___| |__   ___ (_) ___ ___
// | |   | '_ \ / _ \| |/ __/ _ \
// | |___| | | | (_) | | (_|  __/
//  \____|_| |_|\___/|_|\___\___|

/// Enum of any branching structure to do a choice between two branches
#[derive(Clone, Copy)]
pub enum ChoiceData<T, F> {
    True(T),
    False(F),
}
use self::ChoiceData::*;

/// Node that chooses between nt or nf depending on input
///
/// Signature : `ChoiceData<T,F> -> O` when `nt : T -> O` and `nf: F -> O`
pub struct NChoice<NT, NF> {
    pub nt: NT,
    pub nf: NF,
}

impl<'a,NT,NF, InT: Val<'a>, InF: Val<'a>, Out: Val<'a>> Node<'a, ChoiceData<InT, InF>> for NChoice<NT,NF>
    where
    NT : Node<'a,InT,Out = Out>,
    NF : Node<'a,InF,Out = Out>,
{
    type Out = Out;
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>,  val: ChoiceData<InT, InF>) -> Out {
        match val {
            True(t) => {
                self.nt.call(sub_runtime, t)
            }
            False(f) => {
                self.nf.call(sub_runtime, f)
            }
        }
    }
    fn printDot(&mut self, cfgd: &mut CFGDrawer){
        print!("");
        self.nt.printDot(cfgd);
        print!("| or |");
        self.nf.printDot(cfgd);
        print!("");
    }
}


//  _                     ___
// | |    ___   ___  _ __|_ _|_ __ ___
// | |   / _ \ / _ \| '_ \| || '_ ` _ \
// | |__| (_) | (_) | |_) | || | | | | |
// |_____\___/ \___/| .__/___|_| |_| |_|
//                  |_|

/// Node that loops another node
///
/// Signature : `I -> O` when `N : I -> ChoiceData<I,O>`
pub struct LoopIm<N>(pub N);

impl<'a, N, In: Val<'a>, Out: Val<'a>> Node<'a, In> for LoopIm<N>
where
    N: Node<'a, In, Out = ChoiceData<In, Out>>,
{
    type Out = Out;
    fn call(&mut self, sub_runtime: &mut SubRuntime<'a>, mut val: In) -> Out {
        let &mut LoopIm(ref mut p) = self;
        loop {
            match p.call(sub_runtime, val) {
                True(t) => {
                    val = t;
                }
                False(f) => {
                    return f;
                }
            }
        }
    }
}
