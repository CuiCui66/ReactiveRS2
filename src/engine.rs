use std::marker::PhantomData;
use std::vec::Vec;

use node::Node;
use process::Process;

/// Runtime for running reactive graph.
pub struct Runtime<'a> {
    lifemarker: PhantomData<&'a ()>,
    nodes: Vec<Box<Node<'a>>>,
    current: Vec<u32>,
    next: Vec<u32>,
}

impl<'a> Runtime<'a> {
    pub fn new<P>(p: P) -> Self
    where
        P: Process<()>,
    {
        panic!("Not implemented");
    }

    pub(crate) fn newtest() -> Self {
        Runtime {
            lifemarker: PhantomData,
            nodes: vec![],
            current: vec![],
            next: vec![],
        }
    }

    pub fn execute(&mut self) {
        while self.instant() {}
    }

    pub fn instant(&mut self) -> bool {
        panic!("Not implemented")
    }
}
