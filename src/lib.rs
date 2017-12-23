#![allow(non_snake_case)]
#![feature(specialization)]
#![feature(log_syntax)]
#![feature(box_syntax, box_patterns)]
#![feature(plugin)]
#![feature(test)]
#![plugin(promacros)]
#![feature(core_intrinsics)]
#![feature(arbitrary_self_types)]

extern crate core;
#[macro_use] extern crate log;
extern crate env_logger;
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


#[cfg(test)]
mod tests {
    use engine::*;
    //use engine_par::*;
    use process::*;
    //use process_par::*;
    use node::ChoiceData::*;
    use signal::*;
    // use signal_par::*;
    use test::test::Bencher;
    // use std::sync::{Arc, Mutex};
    use std::rc::*;
    use std::cell::*;


    #[test]
    fn instant_action() {
        let mut i = 0;
        {
            run!(|_:()| { i += 42 });
        }
        assert_eq!(i, 42);
    }

    // #[test]
    // fn instant_action_par() {
    //     let mut i = 0;
    //     {
    //         runp!(|_:()| { i += 42 });
    //     }
    //     assert_eq!(i, 42);
    // }

    #[test]
    fn sequence() {
        let mut i = 0;
        {
            run!(
                |_: ()| { 42 };
                |v| { i = v }
            );
        }
        assert_eq!(i, 42);
    }

    // #[test]
    // fn sequence_par() {
    //     let mut i = 0;
    //     {
    //         runp!(
    //             |_: ()| { 42 };
    //             |v| { i = v }
    //         );
    //     }
    //     assert_eq!(i, 42);
    // }

    #[cfg(not(feature = "par"))]
    #[test]
    fn pause() {
        let i = RefCell::new(0);
        {
            let mut r =
                rt!{
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

    // #[test]
    // fn choice_par() {
    //     let mut i = 0;
    //     runp!{
    //         |_| True(42);
    //         choice {
    //             |v| i=v
    //         } {
    //             |()| unreachable!()
    //         }
    //     }
    //     assert_eq!(i, 42);
    // }

    #[test]
    fn choice_pause() {
        let mut i = 0;
        run!{
            |_| True(42);
            Pause;
            choice {
                Pause;
                |v :usize| i = v
            } {
                |()| unreachable!()
            }
        }
        assert_eq!(i, 42);
    }

    // #[test]
    // fn choice_pause_par() {
    //     let mut i = 0;
    //     runp!{
    //         |_| True(42);
    //         Pause;
    //         choice {
    //             Pause;
    //             |v :usize| i = v
    //         } {
    //             |()| unreachable!()
    //         }
    //     }
    //     assert_eq!(i, 42);
    // }

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
                Pause
            };
            |i| {
                assert_eq!(i,42)
            }
        }
    }

    // #[test]
    // fn loop_test_par() {
    //     runp!{
    //         |_| 0;
    //         loop {
    //             |i : usize| if i < 42 {
    //                 True(i+1)
    //             }
    //             else{
    //                 False(i)
    //             };
    //             Pause
    //         };
    //         |i| {
    //             assert_eq!(i,42)
    //         }
    //     }
    // }

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
                    Pause
                } || loop {
                    |i : usize|
                    if i < 21 {
                        True(i+1)
                    }
                    else{
                        False(i)
                    };
                    Pause
                }
            };
            |(v1,v2)| v1 + v2;
            Pause;
            |i| {
                assert_eq!(i,42)
            }
        }
    }

    // #[test]
    // fn par_par() {
    //     runp!{
    //         |_| (0,0);
    //         {
    //             loop {
    //                 |i : usize|
    //                 if i < 21 {
    //                     True(i+1)
    //                 }
    //                 else{
    //                     False(i)
    //                 };
    //                 Pause
    //             } || loop {
    //                 |i : usize|
    //                 if i < 21 {
    //                     True(i+1)
    //                 }
    //                 else{
    //                     False(i)
    //                 };
    //                 Pause
    //             }
    //         };
    //         |(v1,v2)| v1 + v2;
    //         Pause;
    //         |i| {
    //             assert_eq!(i,42)
    //         }
    //     }
    // }

    #[test]
    fn boxp() {
        run!{
            |_| (0,0);
            {
                box {
                    loop {
                        |i : usize|
                        if i < 21 {
                            True(i+1)
                        }
                        else{
                            False(i)
                        };
                        Pause
                    }
                }|| loop {
                    |i : usize|
                    if i < 21 {
                        True(i+1)
                    }
                    else{
                        False(i)
                    };
                    Pause
                }
            };
            |(v1,v2)| v1 + v2;
            Pause;
            |i| {
                assert_eq!(i,42)
            }
        }
    }

    // #[test]
    // fn boxp_par() {
    //     runp!{
    //         |_| (0,0);
    //         {
    //             box {
    //                 loop {
    //                     |i : usize|
    //                     if i < 21 {
    //                         True(i+1)
    //                     }
    //                     else{
    //                         False(i)
    //                     };
    //                     Pause
    //                 }
    //             }|| loop {
    //                 |i : usize|
    //                 if i < 21 {
    //                     True(i+1)
    //                 }
    //                 else{
    //                     False(i)
    //                 };
    //                 Pause
    //             }
    //         };
    //         |(v1,v2)| v1 + v2;
    //         Pause;
    //         |i| {
    //             assert_eq!(i,42)
    //         }
    //     }
    // }

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
                    Pause
                } || |_| 21
            };
            |(v1,v2)| v1 + v2;
            Pause;
            |i| {
                assert_eq!(i,42)
            }
        }
    }

    // #[test]
    // fn par_half_im_par() {
    //     run!{
    //         |_| (0,0);
    //         {
    //             loop {
    //                 |i : usize|
    //                 if i < 21 {
    //                     True(i+1)
    //                 }
    //                 else{
    //                     False(i)
    //                 };
    //                 Pause
    //             } || |_| 21
    //         };
    //         |(v1,v2)| v1 + v2;
    //         Pause;
    //         |i| {
    //             assert_eq!(i,42)
    //         }
    //     }
    // }

    #[test]
    fn par_im() {
        run!{
            |_| (0,0);
            {
                |_| 21 || |_| 21
            };
            |(v1,v2)| v1 + v2;
            Pause;
            |i| {
                assert_eq!(i,42)
            }
        }
    }

    // #[test]
    // fn par_im_par() {
    //     runp!{
    //         |_| (0,0);
    //         {
    //             |_| 21 || |_| 21
    //         };
    //         |(v1,v2)| v1 + v2;
    //         Pause;
    //         |i| {
    //             assert_eq!(i,42)
    //         }
    //     }
    // }


    #[test]
    fn emitd_await() {
        let value = RefCell::new(0);
        let signal = SignalRuntimeRef::new_mc(0, box |e:i32, v:&mut i32| { *v = e;});
        {
            let mut rt = rt! {
                |_| {
                    let signal2 = signal.clone();
                    let signal3 = signal.clone();
                    ((signal2,42), signal3)
                };
                EmitD;
                AwaitD;
                |v| { *value.borrow_mut() = v; }
            };
            rt.instant();
            assert_eq!(*value.borrow_mut(), 0);
            rt.instant();
            assert_eq!(*value.borrow_mut(), 42);
        }
    }

    // #[test]
    // fn emitd_await_par() {
    //     let value = Mutex::new(0);
    //     let signal = SignalRuntimeParRef::new_mc(0, box |e:i32, v:&mut i32| { *v = e;});
    //     {
    //         let mut rt = rtp! {
    //             move |_| {
    //                 let signal2 = signal.clone();
    //                 let signal3 = signal.clone();
    //                 ((signal2,42), signal3)
    //             };
    //             EmitD;
    //             AwaitD;
    //             |v| {
    //                 *value.lock().unwrap() = v;
    //             }
    //         };
    //         rt.instant();
    //         assert_eq!(*value.lock().unwrap(), 0);
    //         rt.instant();
    //         assert_eq!(*value.lock().unwrap(), 42);
    //     }
    // }

    #[test]
    fn emits_await() {
        let value = RefCell::new(0);
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
                |v| { *value.borrow_mut() = v; }
            };
            rt.instant();
            assert_eq!(*value.borrow_mut(), 0);
            rt.instant();
            assert_eq!(*value.borrow_mut(), 42);
        }
    }


    // #[test]
    // fn emits_await_par() {
    //     let value = Mutex::new(0);
    //     let signal = SignalRuntimeParRef::new_mc(0, box |e:i32, v:&mut i32| { *v = e;});
    //     {
    //         let mut rt = rtp! {
    //             |_| {
    //                 42
    //             };
    //             emit(signal.clone());
    //             |_| {
    //                 signal.clone()
    //             };
    //             AwaitD;
    //             |v| { *value.lock().unwrap() = v; }
    //         };
    //         rt.instant();
    //         assert_eq!(*value.lock().unwrap(), 0);
    //         rt.instant();
    //         assert_eq!(*value.lock().unwrap(), 42);
    //     }
    // }

    #[test]
    fn emit_await_immediate() {
        let value = RefCell::new(0);
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
                |()| { *value.borrow_mut() = 42; }
            };
            rt.instant();
            assert_eq!(*value.borrow(), 42);
        }
    }

    // #[test]
    // fn emit_await_immediate_par() {
    //     let value = Mutex::new(0);
    //     let signal = SignalRuntimeParRef::new_mc(0, box |e:i32, v:&mut i32| { *v = e; });
    //     {
    //         let mut rt = rtp! {
    //             |_| {
    //                 let signal2 = signal.clone();
    //                 let signal3 = signal.clone();
    //                 ((signal2,42), signal3)
    //             };
    //             EmitD;
    //             AwaitImmediateD;
    //             |()| { *value.lock().unwrap() = 42; }
    //         };
    //         rt.instant();
    //         assert_eq!(*value.lock().unwrap(), 42);
    //     }
    // }


    #[test]
    fn non_await_immediate() {
        let value = RefCell::new(0);
        let signal = SignalRuntimeRef::new_mc(0, box |e:i32, v:&mut i32| { *v = e; });
        {
            let mut rt = rt! {
                |_| {
                    let signal3 = signal.clone();
                    signal3
                };
                AwaitImmediateD;
                |()| { *value.borrow_mut() = 42; }
            };

            rt.instant();
            assert_eq!(*value.borrow(), 0);
            rt.instant();
            assert_eq!(*value.borrow(), 0);
        }
    }


    // #[test]
    // fn non_await_immediate_par() {
    //     let value = Mutex::new(0);
    //     let signal = SignalRuntimeParRef::new_mc(0, box |e:i32, v:&mut i32| { *v = e; });
    //     {
    //         let mut rt = rtp! {
    //             |_| {
    //                 let signal3 = signal.clone();
    //                 signal3
    //             };
    //             AwaitImmediateD;
    //             |()| { *value.lock().unwrap() = 42; }
    //         };
    //         rt.instant();
    //         assert_eq!(*value.lock().unwrap(), 0);
    //         rt.instant();
    //         assert_eq!(*value.lock().unwrap(), 0);
    //     }
    // }

    #[test]
    fn present_true() {
        let value = RefCell::new(0);
        let signal = SignalRuntimeRef::new_pure();
        {
            let mut rt = rt! {
                |_| {
                    ((signal.clone(),()), signal.clone())
                };
                EmitD;
                present
                    {|_:()| {
                        *value.borrow_mut() = 42;
                    }} {
                    |_:()| {
                        *value.borrow_mut() = 21;
                    }}
            };
            rt.instant();
            assert_eq!(*value.borrow_mut(), 42);
        }
    }

    // #[test]
    // fn present_true_par() {
    //     let value = Mutex::new(0);
    //     let signal = SignalRuntimeParRef::new_pure();
    //     {
    //         let mut rt = rt! {
    //             |_| {
    //                 ((signal.clone(),()), signal.clone())
    //             };
    //             EmitD;
    //             present
    //                 {|_:()| {
    //                     *value.lock().unwrap() = 42;
    //                 }} {
    //                 |_:()| {
    //                     *value.lock().unwrap() = 21;
    //                 }}
    //         };
    //         rt.instant();
    //         assert_eq!(*value.lock().unwrap(), 42);
    //     }
    // }

    #[test]
    fn present_false() {
        let value = RefCell::new(0);
        let signal = SignalRuntimeRef::new_pure();
        {
            let mut rt = rt! {
                |_| {
                    signal.clone()
                };
                present
                    {|_:()| {
                        *value.borrow_mut() = 42;
                    }} {
                    |_:()| {
                        *value.borrow_mut() = 21;
                    }}
            };
            rt.instant();
            assert_eq!(*value.borrow_mut(), 0);
            rt.instant();
            assert_eq!(*value.borrow_mut(), 21);
        }
    }

    // #[test]
    // fn present_false_par() {
    //     let value = Mutex::new(0);
    //     let signal = SignalRuntimeParRef::new_pure();
    //     {
    //         let mut rt = rt! {
    //             |_| {
    //                 signal.clone()
    //             };
    //             present
    //                 {|_:()| {
    //                     *value.lock().unwrap() = 42;
    //                 }} {
    //                 |_:()| {
    //                     *value.lock().unwrap() = 21;
    //                 }}
    //         };
    //         rt.instant();
    //         assert_eq!(*value.lock().unwrap(), 0);
    //         rt.instant();
    //         assert_eq!(*value.lock().unwrap(), 21);
    //     }
    // }

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
                    Pause
                });
            }
            run!(big_join(processes));
        }
        assert_eq!(*value.borrow(), 42);
    }

    // #[test]
    // fn bigpar_par(){
    //     let value = Arc::new(Mutex::new(-3));
    //     {
    //         let mut processes = vec![];
    //         for i in 0..10{
    //             let value2 = value.clone();
    //             processes.push(pro!{
    //                 move |_|{
    //                     *value2.lock().unwrap() += i;
    //                 };
    //                 Pause
    //             });
    //         }
    //         run!(big_join(processes));
    //     }
    //     assert_eq!(*value.lock().unwrap(), 42);
    // }

    #[test]
    fn fnonce() {
        let value = Rc::new(RefCell::new(-3));
        {
            run!(
                once(|_:()| {
                    42
                });
                |v:i32| {
                    *value.borrow_mut() = v;
                }
            );
        }
        assert_eq!(*value.borrow(), 42);
    }

    // #[test]
    // fn fnonce_par() {
    //     let value = Mutex::new(-3);
    //     {
    //         run!(
    //             once(|_:()| {
    //                 42
    //             });
    //             |v:i32| {
    //                 *value.lock().unwrap() = v;
    //             }
    //         );
    //     }
    //     assert_eq!(*value.lock().unwrap(), 42);
    // }


    #[bench]
    fn bench_emitd_pause(bencher: &mut Bencher) {
        let signal = SignalRuntimeRef::new_pure();
        let mut rt = rt! {
            loop {
                value((signal.clone(), ()));
                EmitD;
                Pause;
                value(True(()))
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
    //     let signal = SignalRuntimeParRef::new_pure();
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
                value(());
                emit_value(signal.clone(),());
                Pause;
                value(True(()))
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
    //     let signal = SignalRuntimeParRef::new_pure();
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
                value(((signal.clone(), ()), signal.clone()));
                EmitD;
                AwaitD;
                value(True(()))
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
    //     let signal = SignalRuntimeParRef::new_pure();
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
                value(());
                emit_value(signal.clone(), ());
                AwaitS(signal.clone());
                |_:((),())| {
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
    //     let signal = SignalRuntimeParRef::new_pure();
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
