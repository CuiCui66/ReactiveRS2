use std::marker::PhantomData;
use std::vec::Vec;


use node::*;
use process::*;
use take::take;

pub type Graph<'a> = Vec<Option<Box<Node<'a, (), Out = ()>>>>;

pub struct Tasks {
    pub current: Vec<usize>,
    pub next: Vec<usize>,
}

/// Runtime for running reactive graph.
pub struct Runtime<'a> {
    lifemarker: PhantomData<&'a ()>,
    nodes: Vec<Box<Node<'a, (), Out = ()>>>,
    tasks: Tasks,
}

impl<'a> Runtime<'a> {
    fn newtest() -> Self {
        Runtime::<'a> {
            lifemarker: PhantomData,
            nodes: vec![],
            tasks: Tasks {
                current: vec![],
                next: vec![],
            },
        }
    }

    pub fn fromgraph(g: Graph<'a>) -> Self {
        let mut r = Self::newtest();
        for n in g {
            match n {
                Some(b) => {
                    r.nodes.push(b);
                }
                None => panic!("nope"),
            }
        }
        r
    }

    pub fn new<GF>(gf: GF) -> Self
    where
        GF: Graphfiller<'a>,
    {
        let mut g = vec![];
        let start = gf.fill_graph(& mut g);
        let mut r = Runtime::fromgraph(g);
        r.tasks.current.push(start);
        r
    }





    pub fn execute(&mut self) {
        while self.instant() {}
    }

    pub fn instant(&mut self) -> bool {
        while self.tasks.current.len() > 0 {
            let v = take(&mut self.tasks.current);
            for i in v {
                self.nodes[i].call(&mut self.tasks, ());
            }
        }
        self.tasks.current = take(&mut self.tasks.next);
        self.tasks.current.len() > 0
    }
}

