#![allow(non_snake_case)]
#![feature(specialization)]
#![feature(log_syntax)]
#![feature(box_syntax, box_patterns)]
#![feature(plugin)]
#![feature(test)]
#![plugin(promacros)]
#![feature(core_intrinsics)]
#![feature(arbitrary_self_types)]
#![feature(conservative_impl_trait)]
#![feature(associated_type_defaults)]

// #[macro_use] extern crate log;
// extern crate env_logger;
extern crate test;


#[macro_use]
pub mod macros;


//#[cfg(not(feature = "par"))]
pub mod engine;

#[cfg(not(feature = "par"))]
mod all{

    pub trait OptSend{}
    impl<T> OptSend for T{}

}


// #[cfg(feature = "par")]
// mod engine_par;
// #[cfg(feature = "par")]
// pub use engine_par as engine;

#[cfg(feature = "par")]
mod all {

    // pub trait OptSend{}
    // impl<T> OptSend for T{}

    pub trait OptSend : Send{}
    impl<T : Send> OptSend for T{}

}

/// Trait of value that can be passed in reactive processes
pub trait Val<'a> : 'a + OptSend {}
impl<'a,T : 'a + OptSend> Val<'a> for T{}

pub use all::*;

pub mod node;
pub mod process;
//pub mod process_par;
pub mod signal;
//pub mod signal_par;
mod take;
mod tname;

#[cfg(test)]
mod tests {
    use engine::*;
    use process::*;
    use node::ChoiceData::*;
    use signal::*;
    // use signal_par::*;
    use test::test::Bencher;
    use std::sync::{Arc, Mutex};
    use std::rc::*;
    use std::cell::*;


    #[test]
    fn instant_action_no_macro() {
        let mut i = 0;
        {
            run!(|_:()| { i += 42 });
        }
        assert_eq!(i, 42);
    }

    #[test]
    fn instant_action(){
        let mut i = 0;
        run!(|_| { i += 42; });
        assert_eq!(i, 42);
    }


    #[test]
    fn sequence() {
        let mut i = 0;
        run!{|_ :()| 42;
             |v : usize| i = v;
        };
        assert_eq!(i, 42);
    }

    #[cfg(not(feature = "par"))]
    #[test]
    fn pause() {
        let i = RefCell::new(0);
        {
            let mut r = rt!{
                |_| 42;
                Pause;
                |v| *i.borrow_mut() = v
            };
            r.instant();
            assert_eq!(*i.borrow(), 0);
            r.instant();
        }
        assert_eq!(*i.borrow(), 42);
    }


    #[cfg(feature = "par")] // TODO do this to all other tests
    #[test]
    fn pause() {
        let mut i = Mutex::new(0);
        {
            let mut r =
                rt!{
                    |_| { 42 };
                    Pause;
                    |v| { *i.lock().unwrap() = v }
                };
            r.instant();
            assert_eq!(*i.lock().unwrap(), 0);
            r.instant();
        }
        assert_eq!(*i.lock().unwrap(), 42);
    }

    #[test]
    fn choice() {
        let mut i = 0;
        run!{
            |_| { True(42) };
            choice {
                |v| i=v
            } {
                |()| unreachable!()
            }
        }
        assert_eq!(i, 42);
    }

    #[test]
    fn choice_pause() {
        let mut i = 0;
        run!{
            |_| True(42);
            pause();
            choice {
                pause();
                |v :usize| i = v
            } {
                |()| unreachable!()
            }
        }
        assert_eq!(i, 42);
    }

    #[test]
    fn loop_test() {
        run!{
            |_| 0;
            loop {
                |i : usize| if i < 42 {
                    True(i+1)
                }
                else{
                    False(i)
                };
                pause()
            };
            |i| {
                assert_eq!(i,42)
            }
        }
    }

    #[test]
    fn emit_d_test() {
        let mut value = 0;
        let signal = SignalRuntimeRef::new_mc(21, box |e: i32, v: &mut i32| { *v *= e;});
        {
            run! {
                |_| {
                    (signal.clone(),2)
                };
                emit_d();
                pause();
                |_| {
                    value = signal.clone().pre();
                }
            }
        }
        assert_eq!(value, 42);
    }

    #[test]
    fn emit_d_in_test() {
        let mut value = 0;
        let signal = SignalRuntimeRef::new_mc(7, box |e: i32, v: &mut i32| { *v *= e;});
        {
            run! {
                |_| {
                    ((signal.clone(),2), 3)
                };
                emit_d_in();
                pause();
                |val| {
                    value = val * signal.clone().pre();
                }
            }
        }
        assert_eq!(value, 42);
    }

    #[test]
    fn emit_d_vec_test() {
        let mut value = 0;
        let signal = SignalRuntimeRef::new_mc(7, box |e: i32, v: &mut i32| { *v *= e;});
        {
            run! {
                |_| {
                    vec![(signal.clone(),2), (signal.clone(), 3)]
                };
                emit_d_vec();
                pause();
                |_| {
                    value = signal.clone().pre();
                }
            }
        }
        assert_eq!(value, 42);
    }

    #[test]
    fn emit_d_vec_in_test() {
        let mut value = 0;
        let signal = SignalRuntimeRef::new_mc(1, box |e: i32, v: &mut i32| { *v *= e;});
        {
            run! {
                |_| {
                    (vec![(signal.clone(),2), (signal.clone(), 3)], 7)
                };
                emit_d_vec_in();
                pause();
                |val| {
                    value = val * signal.clone().pre();
                }
            }
        }
        assert_eq!(value, 42);
    }

    #[test]
    fn emit_s_test() {
        let mut value = 0;
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            run! {
                |_| {
                    21
                };
                emit_s(signal.clone());
                pause();
                |_| {
                    value = signal.clone().pre();
                }
            }
        }
        assert_eq!(value, 42);
    }

    #[test]
    fn emit_s_in_test() {
        let mut value = 0;
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            run! {
                |_| {
                    (7,3)
                };
                emit_s_in(signal.clone());
                pause();
                |val| {
                    value = val * signal.clone().pre();
                }
            }
        }
        assert_eq!(value, 42);
    }


    #[test]
    fn emit_vec_s_test() {
        let mut value = 0;
        let signal1 = SignalRuntimeRef::new_mc(1, box |e: i32, v: &mut i32| { *v *= e;});
        let signal2 = SignalRuntimeRef::new_mc(1, box |e: i32, v: &mut i32| { *v *= e;});
        {
            run! {
                |_| {
                    vec![2,14]
                };
                emit_vec_s(vec![signal1.clone(), signal2.clone()]);
                pause();
                |_:()| {
                    value = 7 * signal1.clone().pre() + 2 * signal2.clone().pre();
                }
            }
        }
        assert_eq!(value, 42);
    }

    #[test]
    fn emit_vec_s_in_test() {
        let mut value = 0;
        let signal1 = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        let signal2 = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            run! {
                |_| {
                    (vec![2,14], 2)
                };
                emit_vec_s_in(vec![signal1.clone(), signal2.clone()]);
                pause();
                |val| {
                    value = (7 * signal1.clone().pre() + 2 * signal2.clone().pre()) / val;
                }
            }
        }
        assert_eq!(value, 42);
    }

    #[test]
    fn emit_vs_test() {
        let mut value = 0;
        let signal1 = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            run! {
                |_| {
                    ()
                };
                emit_vs(signal1.clone(), 21);
                pause();
                |_| {
                    value = signal1.clone().pre();
                }
            }
        }
        assert_eq!(value, 42);
    }

    #[test]
    fn emit_vec_vs_test() {
        let mut value = 0;
        let signal1 = SignalRuntimeRef::new_mc(1, box |e: i32, v: &mut i32| { *v *= e;});
        let signal2 = SignalRuntimeRef::new_mc(1, box |e: i32, v: &mut i32| { *v *= e;});
        {
            run! {
                |_| {
                    7
                };
                emit_vec_vs(vec![(signal1.clone(), 2), (signal2.clone(), 3)]);
                pause();
                |val| {
                    value = val * signal1.clone().pre() * signal2.clone().pre();
                }
            }
        }
        assert_eq!(value, 42);
    }


    #[test]
    fn await_d_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    signal.clone()
                };
                emit_vs(signal.clone(), 21);
                await_d();
                |val| {
                    *value.borrow_mut() = val
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 0);
            rt.instant();
            assert_eq!(*value.borrow(), 42);
        }
    }

    #[cfg(feature = "par")]
    #[test]
    fn emitd_await() {
        let value = Mutex::new(0);
        let signal = SignalRuntimeRef::new_mc(0, box |e:i32, v:&mut i32| { *v = e;});
        {
            let mut rt = rt! {
                move |_| {
                    let signal2 = signal.clone();
                    let signal3 = signal.clone();
                    ((signal2,42), signal3)
                };
                EmitD;
                AwaitD;
                |v| {
                    *value.lock().unwrap() = v;
                }
            };
            rt.instant();
            assert_eq!(*value.lock().unwrap(), 0);
            rt.instant();
            assert_eq!(*value.lock().unwrap(), 42);
        }
    }

    #[cfg(not(feature = "par"))]
    #[test]
    fn non_await_d_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    signal.clone()
                };
                await_d();
                |val| {
                    *value.borrow_mut() = val
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 0);
            rt.instant();
            assert_eq!(*value.borrow(), 0);
        }
    }

    #[test]
    fn await_d_in_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    (signal.clone(), 7)
                };
                emit_vs(signal.clone(), 3);
                await_d_in();
                |(val1, val2)| {
                    *value.borrow_mut() = val1 * val2
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 0);
            rt.instant();
            assert_eq!(*value.borrow(), 42);
        }
    }


    #[cfg(feature = "par")]
    #[test]
    fn emits_await() {
        let value = Mutex::new(0);
        let signal = SignalRuntimeRef::new_mc(0, box |e:i32, v:&mut i32| { *v = e;});
        {
            let mut rt = rt! {
                |_| {
                    42
                };
                emit(signal.clone());
                |_| {
                    signal.clone()
                };
                AwaitD;
                |v| { *value.lock().unwrap() = v; }
            };
            rt.instant();
            assert_eq!(*value.lock().unwrap(), 0);
            rt.instant();
            assert_eq!(*value.lock().unwrap(), 42);
        }
    }

    #[cfg(not(feature = "par"))]
    #[test]
    fn non_await_d_in_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    (signal.clone(), 7)
                };
                await_d_in();
                |(val1, val2)| {
                    *value.borrow_mut() = val1 * val2
                }
            };
            rt.instant();
            assert_eq!(*value.borrow(), 0);
            rt.instant();
            assert_eq!(*value.borrow(), 0);
        }
    }

    #[test]
    fn await_s_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    ()
                };
                emit_vs(signal.clone(), 21);
                await_s(signal.clone());
                |val| {
                    *value.borrow_mut() = val
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 0);
            rt.instant();
            assert_eq!(*value.borrow(), 42);
        }
    }

    #[cfg(feature = "par")]
    #[test]
    fn emit_await_immediate() {
        let value = Mutex::new(0);
        let signal = SignalRuntimeRef::new_mc(0, box |e:i32, v:&mut i32| { *v = e; });
        {
            let mut rt = rt! {
                |_| {
                    let signal2 = signal.clone();
                    let signal3 = signal.clone();
                    ((signal2,42), signal3)
                };
                EmitD;
                AwaitImmediateD;
                |()| { *value.lock().unwrap() = 42; }
            };
            rt.instant();
            assert_eq!(*value.lock().unwrap(), 42);
        }
    }


    #[test]
    fn non_await_s_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    ()
                };
                await_s(signal.clone());
                |val| {
                    *value.borrow_mut() = val
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 0);
            rt.instant();
            assert_eq!(*value.borrow(), 0);
        }
    }


    #[cfg(not(feature = "par"))]
    #[test]
    fn await_s_in_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    7
                };
                emit_vs(signal.clone(), 3);
                await_s_in(signal.clone());
                |(val1, val2)| {
                    *value.borrow_mut() = val1 * val2
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 0);
            rt.instant();
            assert_eq!(*value.borrow(), 42);
        }
    }

    #[test]
    fn non_await_s_in_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    7
                };
                await_s_in(signal.clone());
                |(val1, val2)| {
                    *value.borrow_mut() = val1 * val2
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 0);
            rt.instant();
            assert_eq!(*value.borrow(), 0);
        }
    }


    #[cfg(feature = "par")]
    #[test]
    fn non_await_immediate() {
        let value = Mutex::new(0);
        let signal = SignalRuntimeRef::new_mc(0, box |e:i32, v:&mut i32| { *v = e; });
        {
            let mut rt = rt! {
                |_| {
                    let signal3 = signal.clone();
                    signal3
                };
                AwaitImmediateD;
                |()| { *value.lock().unwrap() = 42; }
            };
            rt.instant();
            assert_eq!(*value.lock().unwrap(), 0);
            rt.instant();
            assert_eq!(*value.lock().unwrap(), 0);
        }
    }

    fn await_immediate_d_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    signal.clone()
                };
                emit_vs(signal.clone(), 21);
                await_immediate_d();
                |_| {
                    *value.borrow_mut() = 42
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 42);
        }
    }

    #[test]
    fn non_await_immediate_d_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    signal.clone()
                };
                await_immediate_d();
                |_| {
                    *value.borrow_mut() = 42
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 0);
            rt.instant();
            assert_eq!(*value.borrow(), 0);
        }
    }

    #[test]
    fn await_immediate_d_in_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    (signal.clone(), 7)
                };
                emit_vs(signal.clone(), 3);
                await_immediate_d_in();
                |val| {
                    *value.borrow_mut() = 6 * val
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 42);
        }
    }

    #[cfg(not(feature = "par"))]
    #[test]
    fn non_await_immediate_d_in_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    (signal.clone(), 7)
                };
                await_immediate_d_in();
                |val| {
                    *value.borrow_mut() = val
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 0);
            rt.instant();
            assert_eq!(*value.borrow(), 0);
        }
    }

    #[cfg(feature = "par")]
    #[test]
    fn present_true() {
        let value = Mutex::new(0);
        let signal = SignalRuntimeRef::new_pure();
        {
            let mut rt = rt! {
                |_| {
                    ((signal.clone(),()), signal.clone())
                };
                EmitD;
                present
                    {|_:()| {
                        *value.lock().unwrap() = 42;
                    }} {
                    |_:()| {
                        *value.lock().unwrap() = 21;
                    }}
            };
            rt.instant();
            assert_eq!(*value.lock().unwrap(), 42);
        }
    }

    #[cfg(not(feature = "par"))]
    #[test]
    fn await_immediate_s_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    7
                };
                emit_vs(signal.clone(), 3);
                await_immediate_s(signal.clone());
                |val| {
                    *value.borrow_mut() = 6 * val
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 42);
        }
    }

    #[test]
    fn non_await_immediate_s_test() {
        let value = Rc::new(RefCell::new(0));
        let signal = SignalRuntimeRef::new_mc(2, box |e: i32, v: &mut i32| { *v *= e;});
        {
            let mut rt = rt! {
                |_| {
                    7
                };
                await_immediate_s(signal.clone());
                |val| {
                    *value.borrow_mut() = val
                }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 0);
            rt.instant();
            assert_eq!(*value.borrow(), 0);
        }
    }

    #[test]
    fn par() {
        run!{
            |_| (0,0);
            {
                loop {
                    |i : usize|
                    if i < 21 {
                        True(i+1)
                    }
                    else{
                        False(i)
                    };
                    pause()
                } || loop {
                    |i : usize|
                    if i < 21 {
                        True(i+1)
                    }
                    else{
                        False(i)
                    };
                    pause()
                }
            };
            |(v1,v2)| v1 + v2;
            pause();
            |i| {
                assert_eq!(i,42)
            }
        }
    }

    #[test]
    fn par_half_im() {
        run!{
            |_| (0,0);
            {
                loop {
                    |i : usize|
                    if i < 21 {
                        True(i+1)
                    }
                    else{
                        False(i)
                    };
                    pause()
                } || |_| 21
            };
            |(v1,v2)| v1 + v2;
            pause();
            |i| {
                assert_eq!(i,42)
            }
        }
    }

    #[cfg(feature = "par")]
    #[test]
    fn present_false() {
        let value = Mutex::new(0);
        let signal = SignalRuntimeRef::new_pure();
        {
            let mut rt = rt! {
                |_| {
                    signal.clone()
                };
                present
                    {|_:()| {
                        *value.lock().unwrap() = 42;
                    }} {
                    |_:()| {
                        *value.lock().unwrap() = 21;
                    }}
            };
            rt.instant();
            assert_eq!(*value.lock().unwrap(), 0);
            rt.instant();
            assert_eq!(*value.lock().unwrap(), 21);
        }
    }

    #[test]
    fn par_im() {
        run!{
            |_| (0,0);
            {
                |_| 21 || |_| 21
            };
            |(v1,v2)| v1 + v2;
            pause();
            |i| {
                assert_eq!(i,42)
            }
        }
    }

    #[cfg(not(feature = "par"))]
    #[test]
    fn bigpar(){
        let value = Rc::new(RefCell::new(-3));
        {
            let mut processes = vec![];

            for i in 0..10{
                let value2 = value.clone();
                processes.push(pro!{
                    move |_|{
                        *value2.borrow_mut() += i;
                    };
                    pause()
                });
            }
            run!(big_join(processes));
        }
        assert_eq!(*value.borrow(), 42);
    }

    #[cfg(feature = "par")]
    #[test]
    fn bigpar(){
        let value = Arc::new(Mutex::new(-3));
        {
            let mut processes = vec![];
            for i in 0..10{
                let value2 = value.clone();
                processes.push(pro!{
                    move |_|{
                        *value2.lock().unwrap() += i;
                    };
                    Pause
                });
            }
            run!(big_join(processes));
        }
        assert_eq!(*value.lock().unwrap(), 42);
    }

    #[test]
    fn fnonce() {
        let mut value = -3;
        {
            run!(
                once(|_:()| {
                    42
                });
                |v:i32| {
                    value = v;
                }
            );
        }
        assert_eq!(value, 42);
    }

    #[bench]
    fn bench_emitd_pause(bencher: &mut Bencher) {
        let signal = SignalRuntimeRef::new_pure();
        let mut rt = rt! {
            loop {
                |_| { (signal.clone(), ()) };
                emit_d();
                pause();
                |_| {
                    True(())
                }
            }
        };
        bencher.iter(|| {
            for _ in 0..1000 {
                rt.instant();
            }
        });
    }

    // #[bench]
    // fn bench_emitd_pause_par(bencher: &mut Bencher) {
    //     let signal = SignalRuntimeRef::new_pure();
    //     let mut rt = rtp! {
    //         loop {
    //             value((signal.clone(), ()));
    //             EmitD;
    //             Pause;
    //             value(True(()))
    //         }
    //     };
    //     bencher.iter(|| {
    //         for _ in 0..1000 {
    //             rt.instant();
    //         }
    //     });
    // }

    #[bench]
    fn bench_emits_pause(bencher: &mut Bencher) {
        let signal = SignalRuntimeRef::new_pure();
        let mut rt = rt! {
            loop {
                |_:()| { };
                emit_vs(signal.clone(),());
                pause();
                |_| {
                    True(())
                }
            }
        };
        bencher.iter(|| {
            for _ in 0..1000 {
                rt.instant();
            }
        });
    }

    // #[bench]
    // fn bench_emits_pause_par(bencher: &mut Bencher) {
    //     let signal = SignalRuntimeRef::new_pure();
    //     let mut rt = rtp! {
    //         loop {
    //             value(());
    //             emit_value(signal.clone(),());
    //             Pause;
    //             value(True(()))
    //         }
    //     };
    //     bencher.iter(|| {
    //         for _ in 0..1000 {
    //             rt.instant();
    //         }
    //     });
    // }

    #[bench]
    fn bench_emitd_await(bencher: &mut Bencher) {
        let signal = SignalRuntimeRef::new_pure();
        let mut rt = rt! {
            loop {
                |_| { ((signal.clone(), ()), signal.clone()) };
                emit_d_in();
                await_d();
                |_:()| {
                    True(())
                }
            }
        };
        bencher.iter(|| {
            for _ in 0..1000 {
                rt.instant();
            }
        });
    }

    // #[bench]
    // fn bench_emitd_await_par(bencher: &mut Bencher) {
    //     let signal = SignalRuntimeRef::new_pure();
    //     let mut rt = rtp! {
    //         loop {
    //             value(((signal.clone(), ()), signal.clone()));
    //             EmitD;
    //             AwaitD;
    //             value(True(()))
    //         }
    //     };
    //     bencher.iter(|| {
    //         for _ in 0..1000 {
    //             rt.instant();
    //         }
    //     });
    // }

    #[bench]
    fn bench_emits_await(bencher: &mut Bencher) {
        let signal = SignalRuntimeRef::new_pure();
        let mut rt = rt! {
            loop {
                |_| {()};
                emit_vs(signal.clone(), ());
                await_s(signal.clone());
                |_:()| {
                    True(())
                }
            }
        };
        bencher.iter(|| {
            for _ in 0..1000 {
                rt.instant();
            }
        });
    }


    // #[bench]
    // fn bench_emits_await_par(bencher: &mut Bencher) {
    //     let signal = SignalRuntimeRef::new_pure();
    //     let mut rt = rtp! {
    //         loop {
    //             value(());
    //             emit_value(signal.clone(), ());
    //             AwaitS(signal.clone());
    //             |_:((),())| {
    //                 True(())
    //             }
    //         }
    //     };
    //     bencher.iter(|| {
    //         for _ in 0..1000 {
    //             rt.instant();
    //         }
    //     });
    // }

}
