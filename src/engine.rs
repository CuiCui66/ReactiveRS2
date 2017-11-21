use std::marker::PhantomData;
use std::vec::Vec;


use node::*;
use process::Process;
use take::take;

pub type Graph<'a> = Vec<Option<Box<Node<'a, (), Out = ()>>>>;

pub struct Tasks {
    current: Vec<usize>,
    next: Vec<usize>,
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

    pub fn new<P>(p: P) -> Self
    where
        P: Process<'a, (), Out = ()>,
    {
        let mut g = vec![];
        match p.compile(&mut g) {
            PNode::InOut(nio) => {
                g.push(Some(Box::new(nio)));
                let mut r = Self::fromgraph(g);
                r.tasks.current.push(r.nodes.len() - 1);
                r
            }
            PNode::Halves(ni, val, no) => {

                g[val] = Some(Box::new(no));
                g.push(Some(Box::new(ni)));
                let mut r = Self::fromgraph(g);
                r.tasks.current.push(r.nodes.len() - 1);
                r

            }
        }
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

#[cfg(test)]
mod tests {
    use engine::*;

    #[test]
    fn instant_action() {
        let mut i = 0;
        {
            let mut r = Runtime::new(|_: ()| { i += 1; });
            r.execute();
        }
        assert_eq!(i, 1);
    }
}
