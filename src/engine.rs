use std::vec::Vec;


use node::*;
use process::*;
use take::take;

pub struct Graph<'a>(Vec<Option<Box<Node<'a, (), Out = ()>>>>);

impl<'a> Graph<'a> {

    pub(crate) fn new() -> Self {
        Graph(vec![])
    }

    pub(crate) fn reserve(&mut self) -> usize {
        let &mut Graph(ref mut v) = self;
        v.push(None);
        v.len() - 1
    }

    pub(crate) fn set(&mut self, pos: usize, val: Box<Node<'a, (), Out = ()>>) {
        let &mut Graph(ref mut v) = self;
        if let Some(_) = v[pos] {
            panic!("v[pos] != None in Graph::set")
        }
        v[pos] = Some(val);
    }

    pub(crate) fn add(&mut self, val: Box<Node<'a, (), Out = ()>>) -> usize {
        let &mut Graph(ref mut v) = self;
        v.push(Some(val));
        v.len() - 1
    }

    pub(crate) fn get(self) -> Vec<Option<Box<Node<'a, (), Out = ()>>>> {
        let Graph(v) = self;
        v
    }
}

pub(crate) struct Tasks {
    pub(crate) current: Vec<usize>,
    pub(crate) next: Vec<usize>,
}

pub trait EndOfInstantCallback<'a>{
    fn on_end_of_instant(&self, sub_runtime: &mut SubRuntime<'a>);
}

pub(crate) struct EndOfInstant<'a> {
    pub(crate) pending: Vec<Box<EndOfInstantCallback<'a> + 'a>>,
}

pub struct SubRuntime<'a> {
    pub(crate) tasks: Tasks,
    pub(crate) eoi: EndOfInstant<'a>,
}


/// Runtime for running reactive graph.
pub struct Runtime<'a> {
    nodes: Vec<Box<Node<'a, (), Out = ()>>>,
    sub_runtime: SubRuntime<'a>,
}

impl<'a> Runtime<'a> {
    fn newtest() -> Self {
        Runtime::<'a> {
            nodes: vec![],
            sub_runtime: SubRuntime {
                tasks: Tasks {
                    current: vec![],
                    next: vec![],
                },
                eoi: EndOfInstant {
                    pending: vec![],
                },
            }
        }
    }

    pub(crate) fn fromgraph(g: Graph<'a>) -> Self {
        let mut r = Self::newtest();
        for n in g.get() {
            match n {
                Some(b) => {
                    r.nodes.push(b);
                }
                None => unreachable!(),
            }
        }
        r
    }

    // Gf is theorically a process.
    pub fn new<GF>(gf: GF) -> Self
    where
        GF: Graphfiller<'a>,
    {
        let mut g = Graph::new();
        let start = gf.fill_graph(&mut g);
        let mut r = Runtime::fromgraph(g);
        r.sub_runtime.tasks.current.push(start);
        r
    }





    pub fn execute(&mut self) {
        while self.instant() {}
    }

    pub fn instant(&mut self) -> bool {
        while self.sub_runtime.tasks.current.len() > 0 {
            let v = take(&mut self.sub_runtime.tasks.current);
            for i in v {
                self.nodes[i].call(&mut self.sub_runtime, ());
            }
        }
        self.sub_runtime.tasks.current = take(&mut self.sub_runtime.tasks.next);

        let eois = take(&mut self.sub_runtime.eoi.pending);
        for eoi in eois {
            eoi.on_end_of_instant(&mut self.sub_runtime);
        }

        self.sub_runtime.tasks.current.len() > 0 || self.sub_runtime.eoi.pending.len() > 0
    }
}
