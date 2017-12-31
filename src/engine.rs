use std::vec::Vec;
use crossbeam;

use node::*;
use graph::*;
use process::*;
#[macro_use]
use utility;
use utility::*;

use super::*;
use std::ops::DerefMut;



pub trait EndOfInstantCallback<'a>: Val<'a> {
    fn on_end_of_instant(&self, sub_runtime: &mut SubRuntime<'a>);
}



#[cfg(not(feature = "par"))]
mod runtime {
    use super::*;

    /// Contains the remaining node to be executed
    pub(crate) struct Tasks {
        /// Contains nodes to be executed on the current instants.
        /// Nodes can add other nodes' id to continue the execution in an other node
        /// on the same instant.
        pub(crate) current: Vec<usize>,
        /// Contains nodes to be executed on the next instants.
        /// Nodes can add other nodes' id and stop to pause their execution until the next instant.
        pub(crate) next: Vec<usize>,
    }


    /// Contains a list of [signal](../signal/index.html)
    /// related continuation to be run at the end of the instant.
    pub(crate) struct EndOfInstant<'a> {
        pub(crate) pending: Vec<Box<EndOfInstantCallback<'a> + 'a>>,
    }

    /// The part of the runtime that is passed to Nodes, see
    /// [Node::call](../node/trait.Node.html#tymethod.call).
    pub struct SubRuntime<'a> {
        /// The tasks lists
        pub(crate) tasks: Tasks,
        /// The end of instant continuations.
        pub(crate) eoi: EndOfInstant<'a>,
        /// The id of the current instant.
        pub(crate) current_instant: usize,
    }


    use super::*;
    impl<'a> SubRuntime<'a> {
        /// Add a new main node to be executed on current instant
        pub fn add_current(&mut self, ind: usize) {
            self.tasks.current.push(ind);
        }
        /// Add a new main node to be executed on next instant
        pub fn add_next(&mut self, ind: usize) {
            self.tasks.next.push(ind);
        }
        /// Add a new en of instant object
        pub fn add_eoi(&mut self, box_eoi: Box<EndOfInstantCallback<'a>>) {
            self.eoi.pending.push(box_eoi);
        }
        pub fn get_current_instant(&mut self) -> usize {
            self.current_instant
        }
        /// Ask for ending the execution, useless in sequential mode.
        pub fn end(&mut self) {}
    }


    /// Runtime for running reactive graph.
    ///
    /// It contains all the information needed to execute of a reactive process.
    pub struct Runtime<'a> {
        /// The reactive control-flow graph in non-optional version.
        /// See [`Graph`](struct.Graph.html).
        pub(super) nodes: Vec<Box<Node<'a, (), Out = ()>>>,

        /// The SubRuntime containing all runtime info.
        sub_runtime: SubRuntime<'a>,
    }

    impl<'a> Runtime<'a> {
        /// Executes the whole reactive process until it ends.
        pub fn execute(&mut self) {
            while self.instant() {}
        }
        pub fn instantn(&mut self, n: usize) -> bool {
            for _ in 0..n {
                if !self.instant() {
                    return false;
                }
            }
            return true;
        }


        /// Executes an single instant of the reactive process loaded in the runtime.
        ///
        /// Returns whether the process should continue.
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

            self.sub_runtime.current_instant += 1;

            self.sub_runtime.tasks.current.len() > 0 || self.sub_runtime.eoi.pending.len() > 0
        }

        pub fn printDot(&mut self) {
            println!("digraph {{");
            let mut cfgd = CFGDrawer::new();
            for (i, node) in self.nodes.iter_mut().enumerate() {
                cfgd.printNode(i, node.deref_mut());
            }
            println!("}}");
        }

        /// Creates a runtime from a Graph.
        ///
        /// The graph must be complete i.e any reserved id must not be empty.
        /// If the graph is not complete, it panics.
        /// This function does not setup a start point:
        fn fromgraph(g: Graph<'a>) -> Self {
            let mut r = Self::newempty();
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


        /// [gf]: ../process/trait..html
        /// [mp]: ../process/struct.MarkedProcess.html

        /// Creates a Runtime by using a value implementing [``][gf].
        ///
        /// After this function, the runtime is ready to be used
        /// Normally,types that implement [``][gf] are [`MarkedProcess`][mp]
        pub fn new<GF>(gf: GF) -> Self
        where
            GF: GraphFiller<'a>,
        {
            let (g,start) = gf.compile_to_graph();
            let mut r = Runtime::fromgraph(g);
            r.sub_runtime.add_current(start);
            r
        }


        /// Creates a new empty runtime.
        pub(crate) fn newempty() -> Self {
            Runtime::<'a> {
                nodes: vec![],
                sub_runtime: SubRuntime {
                    current_instant: 3,
                    tasks: Tasks {
                        current: vec![],
                        next: vec![],
                    },
                    eoi: EndOfInstant { pending: vec![] },
                },
            }
        }
    }
}

//  ____                 _ _      _
// |  _ \ __ _ _ __ __ _| | | ___| |
// | |_) / _` | '__/ _` | | |/ _ \ |
// |  __/ (_| | | | (_| | | |  __/ |
// |_|   \__,_|_|  \__,_|_|_|\___|_|



#[cfg(feature = "par")]
mod runtime {


    use super::*;
    use crossbeam_deque::{Deque, Stealer, Steal};
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::AtomicBool;
    use std::sync::Mutex;
    use std::sync::atomic::Ordering::*;
    use std::mem;
    use std::ptr;
    use std::sync::Arc;

    const nb_th: usize = 4;

//  _   _           _       ____     _ _
// | \ | | ___   __| | ___ / ___|___| | |
// |  \| |/ _ \ / _` |/ _ \ |   / _ \ | |
// | |\  | (_) | (_| |  __/ |__|  __/ | |
// |_| \_|\___/ \__,_|\___|\____\___|_|_|

    #[cfg(not(feature = "funsafe"))]
    mod node_cell {
        use super::*;
        pub struct NodeCell<'a>(pub(crate) Mutex<Box<Node<'a, (), Out = ()>>>);

        impl<'a> NodeCell<'a> {
            pub fn new(b: Box<Node<'a, (), Out = ()>>) -> Self {
                NodeCell(Mutex::new(b))
            }
            pub fn call(&self, sub: &mut SubRuntime<'a>) {
                self.0.lock().unwrap().call(sub,());
            }
        }
    }
    #[cfg(feature = "funsafe")]
    mod node_cell {
        use std::cell::UnsafeCell;
        use super::*;
        pub struct NodeCell<'a>(pub(crate) UnsafeCell<Box<Node<'a, (), Out = ()>>>);

        impl<'a> NodeCell<'a> {
            pub fn new(b: Box<Node<'a, (), Out = ()>>) -> Self {
                NodeCell(UnsafeCell::new(b))
            }
            pub fn call(&self, sub: &mut SubRuntime<'a>) {
                unsafe {self.0.get().as_mut().unwrap()}.call(sub,());
            }
        }

        unsafe impl<'a> Sync for NodeCell<'a>{}
    }

    use self::node_cell::*;



//  ___           _              _   ____        _
// |_ _|_ __  ___| |_ __ _ _ __ | |_|  _ \  __ _| |_ __ _
//  | || '_ \/ __| __/ _` | '_ \| __| | | |/ _` | __/ _` |
//  | || | | \__ \ || (_| | | | | |_| |_| | (_| | || (_| |
// |___|_| |_|___/\__\__,_|_| |_|\__|____/ \__,_|\__\__,_|


    /// Contains reference to all current instant data.
    pub(crate) struct InstantData {
        pub(crate) ws: Box<WorkStealing>,
        /// The number of threads that have finished this instant.
        pub(crate) nbf: Arc<AtomicUsize>,
    }

    /// Contains access to the work-stealing system of a given instant.
    pub(crate) struct WorkStealing {
        pub(crate) deque: Deque<usize>,
        pub(crate) stealers: [Stealer<usize>; nb_th - 1],
    }

    impl WorkStealing {
        fn new(deque: Deque<usize>, vstealers: Vec<Stealer<usize>>) -> WorkStealing {
            WorkStealing {
                deque,
                stealers: vec2array!(vstealers, nb_th - 1),
            }
        }
    }


//  ____        _     ____              _   _
// / ___| _   _| |__ |  _ \ _   _ _ __ | |_(_)_ __ ___   ___
// \___ \| | | | '_ \| |_) | | | | '_ \| __| | '_ ` _ \ / _ \
//  ___) | |_| | |_) |  _ <| |_| | | | | |_| | | | | | |  __/
// |____/ \__,_|_.__/|_| \_\\__,_|_| |_|\__|_|_| |_| |_|\___|


    /// The part of the runtime that is passed to Nodes, see
    /// [Node::call](../node/trait.Node.html#tymethod.call).
    pub struct SubRuntime<'a> {
        /// reference to previous, current and next instant data
        /// if `current_instant % 3 = 2`, `current.ws = &ws2`, `next.ws = &ws0` and
        /// `previous.ws = &ws1`
        previous: InstantData,
        current: InstantData,
        next: InstantData,

        /// The end of instant continuations.
        pub(crate) eoi: Vec<Box<EndOfInstantCallback<'a> + 'a>>,

        /// The id of the current instant.
        pub(crate) current_instant: usize,

        /// When set to true, this is the end of the global process.
        pub(crate) aend: Arc<AtomicBool>,
    }

    impl<'a> SubRuntime<'a> {
        fn new(mut ids: Vec<InstantData>, aend: Arc<AtomicBool>) -> Self {
            assert_eq!(ids.len(), 3);
            // start instant must be 3
            let previous = ids.pop().unwrap();
            let next = ids.pop().unwrap();
            let current = ids.pop().unwrap();
            SubRuntime {
                previous,
                current,
                next,
                eoi: vec![],
                current_instant: 3,
                aend,
            }

        }
        /// Add a new main node to be executed on current instant
        pub fn add_current(&mut self, ind: usize) {
            self.current.ws.deque.push(ind);
        }
        /// Add a new main node to be executed on next instant
        pub fn add_next(&mut self, ind: usize) {
            self.next.ws.deque.push(ind);
        }
        /// Add a new en of instant object
        pub fn add_eoi(&mut self, box_eoi: Box<EndOfInstantCallback<'a>>) {
            self.eoi.push(box_eoi);
        }
        pub fn get_current_instant(&mut self) -> usize {
            self.current_instant
        }
        /// Ask for ending the execution, all threads will stop at the end of this instant.
        pub fn end(&mut self) {
            //println!("END\n");
            self.aend.store(true, Relaxed);
        }
    }

//  _____ _                        _ ____              _   _
// |_   _| |__  _ __ ___  __ _  __| |  _ \ _   _ _ __ | |_(_)_ __ ___   ___
//   | | | '_ \| '__/ _ \/ _` |/ _` | |_) | | | | '_ \| __| | '_ ` _ \ / _ \
//   | | | | | | | |  __/ (_| | (_| |  _ <| |_| | | | | |_| | | | | | |  __/
//   |_| |_| |_|_|  \___|\__,_|\__,_|_| \_\\__,_|_| |_|\__|_|_| |_| |_|\___|


    /// The part of the runtime that is passed to Nodes, see
    /// [Node::call](../node/trait.Node.html#tymethod.call).
    pub struct ThreadRuntime<'a> {
        pub(super) sub: SubRuntime<'a>,
        pub(super) nodes: Arc<Vec<NodeCell<'a>>>,
    }


    impl<'a> ThreadRuntime<'a> {
        fn new(ids: Vec<InstantData>, end: Arc<AtomicBool>, nodes: Arc<Vec<NodeCell<'a>>>) -> Self {
            ThreadRuntime {
                sub: SubRuntime::new(ids, end),
                nodes,
            }

        }

        /// Change formally of instant (switch previous, current and next)
        /// The synchronization signal is when current.nbf gets to nb_th.
        /// It cannot go down if it reaches nb_th at any point of time because
        /// reaching nb_th means
        fn step(&mut self) {
            self.sub.previous.nbf.store(0, Relaxed);
            swap3(
                &mut self.sub.previous,
                &mut self.sub.current,
                &mut self.sub.next,
            );
            self.sub.current_instant += 1;
        }
        /// run a node by id
        fn run_node(&mut self, num: usize) {
            self.nodes[num].call(&mut self.sub);
        }

        /// Runs the nodes of an instant by work stealing, then synchronize with other threads,
        /// for changing instant then run the eoi routines.
        fn instant(&mut self) {
            'instant: loop {
                while let Some(nb) = self.sub.current.ws.deque.pop() {
                    self.run_node(nb);
                }
                self.sub.current.nbf.fetch_add(1, SeqCst);
                while (self.sub.current.nbf.load(SeqCst) < nb_th) {
                    for i in 0..nb_th - 1 {
                        if (!self.sub.current.ws.stealers[i].is_empty()) {
                            self.sub.current.nbf.fetch_sub(1, SeqCst);
                            if let Steal::Data(nb) = self.sub.current.ws.stealers[i].steal() {
                                self.run_node(nb);
                                continue 'instant;
                            }
                            self.sub.current.nbf.fetch_add(1, SeqCst);
                        }
                    }
                    cpu_pause();
                }
                break 'instant;
            } // end 'instant
            self.step();
            let eois = take(&mut self.sub.eoi);
            for eoi in eois {
                eoi.on_end_of_instant(&mut self.sub);
            }

        }
        /// Runs `n` instant or until end flag is raised
        fn instantn(&mut self, n: usize) {
            for _ in 0..n {
                if self.sub.aend.load(Relaxed) {
                    self.sub.current.nbf.fetch_add(1, Relaxed);
                    break;
                }
                self.instant();
            }
        }
        /// Runs instants until end flag is raised.
        fn execute(&mut self) {
            while !self.sub.aend.load(Relaxed) {
                self.instant();
            }
            self.sub.current.nbf.fetch_add(1, Relaxed);
        }
    }

//  ____              _   _
// |  _ \ _   _ _ __ | |_(_)_ __ ___   ___
// | |_) | | | | '_ \| __| | '_ ` _ \ / _ \
// |  _ <| |_| | | | | |_| | | | | | |  __/
// |_| \_\\__,_|_| |_|\__|_|_| |_| |_|\___|

    /// Runtime for running reactive graph.
    ///
    /// It contains all the information needed to execute of a reactive process.
    pub struct Runtime<'a> {
        /// The reactive control-flow graph in non-optional version.
        /// See [`Graph`](struct.Graph.html).
        //TODO remove the mutex on funsafe
        pub(super) nodes: Arc<Vec<NodeCell<'a>>>,

        /// The SubRuntime containing all runtime info.
        thread_runtimes: Vec<ThreadRuntime<'a>>,

        /// TODO doc
        pub(crate) end: Arc<AtomicBool>,
    }

    impl<'a> Runtime<'a> {
        /// Executes the whole reactive process until it ends.
        pub fn execute(&mut self) {
            crossbeam::scope(|scope| for tr in self.thread_runtimes.iter_mut() {
                scope.spawn(move || tr.execute());
            });
            //assert!(self.end.load(SeqCst));

        }

        /// Executes an single instant of the reactive process loaded in the runtime.
        ///
        /// Returns whether the process should continue.
        pub fn instant(&mut self) -> bool {
            crossbeam::scope(|scope| for tr in self.thread_runtimes.iter_mut() {
                scope.spawn(move || tr.instant());
            });
            !self.end.load(SeqCst)
        }
        pub fn instantn(&mut self, n: usize) -> bool {
            crossbeam::scope(|scope| for tr in self.thread_runtimes.iter_mut() {
                scope.spawn(move || tr.instantn(n));
            });
            !self.end.load(SeqCst)
        }

        // pub fn printDot(&mut self) {
        //     println!("digraph {{");
        //     let mut cfgd = CFGDrawer::new();
        //     for (i, node) in self.nodes.iter_mut().enumerate() {
        //         printNode(i, *node.lock().unwrap().deref_mut(), &mut cfgd);
        //     }
        //     println!("}}");
        // }

        /// Creates a runtime from a Graph.
        ///
        /// The graph must be complete i.e any reserved id must not be empty.
        /// If the graph is not complete, it panics.
        /// This function does not setup a start point:
        fn fromgraph(g: Graph<'a>) -> Self {
            let mut r = vec![];
            for n in g.get() {
                match n {
                    Some(b) => {
                        r.push(NodeCell::new(b));
                    }
                    None => unreachable!(),
                }
            }
            Runtime::fromnodes(r)
        }


        /// [gf]: ../process/trait..html
        /// [mp]: ../process/struct.MarkedProcess.html

        /// Creates a Runtime by using a value implementing [``][gf].
        ///
        /// After this function, the runtime is ready to be used
        /// Normally,types that implement [``][gf] are [`MarkedProcess`][mp]
        pub fn new<GF>(gf: GF) -> Self
        where
            GF: GraphFiller<'a>,
        {
            let (g,start) = gf.compile_to_graph();
            let mut r = Runtime::fromgraph(g);
            r.thread_runtimes[0].sub.add_current(start);
            r
        }

        /// Creates a new empty runtime.
        fn fromnodes(nodes: Vec<NodeCell<'a>>) -> Self {
            let deques: Vec<Vec<Deque<usize>>> = (0..nb_th)
                .map(|_| (0..3).map(|_| Deque::new()).collect())
                .collect();

            let stealers: Vec<Vec<Vec<Stealer<usize>>>> = (0..nb_th)
                .map(|th| {
                    (0..3)
                        .map(|inst| {
                            (0..nb_th)
                                .filter(|sth| *sth != th)
                                .map(|sth| deques[sth][inst].stealer())
                                .collect()
                        })
                        .collect()
                })
                .collect();

            let workStealings: Vec<Vec<Box<WorkStealing>>> = deques
                .into_iter()
                .zip(stealers.into_iter())
                .map(|(vd, vs)| {
                    vd.into_iter()
                        .zip(vs.into_iter())
                        .map(|(deq, stl)| box WorkStealing::new(deq, stl))
                        .collect()
                })
                .collect();

            let nb_finish_base = [
                Arc::new(AtomicUsize::new(0)),
                Arc::new(AtomicUsize::new(0)),
                Arc::new(AtomicUsize::new(0)),
            ];

            let nb_finishs: Vec<Vec<Arc<AtomicUsize>>> = (0..nb_th)
                .map(|_| {
                    (0..3).map(|inst| nb_finish_base[inst].clone()).collect()
                })
                .collect();

            let instdatas: Vec<Vec<InstantData>> = workStealings
                .into_iter()
                .zip(nb_finishs.into_iter())
                .map(|(wss, nbfs)| {
                    wss.into_iter()
                        .zip(nbfs.into_iter())
                        .map(|(ws, nbf)| InstantData { ws, nbf })
                        .collect()
                })
                .collect();

            let end = Arc::new(AtomicBool::new(false));
            let arc_nodes = Arc::new(nodes);

            let subs: Vec<ThreadRuntime<'a>> = instdatas
                .into_iter()
                .map(|ids| {
                    ThreadRuntime::new(ids, end.clone(), arc_nodes.clone())
                })
                .collect();

            Runtime {
                end,
                thread_runtimes: subs,
                nodes: arc_nodes,
            }



        }
    }
}

pub use self::runtime::*;
