#![allow(non_snake_case)]
#![feature(specialization)]
#![feature(log_syntax)]
#![feature(box_syntax, box_patterns)]
#![feature(plugin)]
#![plugin(promacros)]

#[macro_use]
pub mod macros;
pub mod engine;
pub mod node;
pub mod process;
pub mod signal;
mod take;




#[cfg(test)]
mod tests {
    use engine::*;
    use process::*;
    use node::*;
    use signal::*;

    #[test]
    fn instant_action() {
        let mut i = 0;
        {
            let mut r = Runtime::new(mp(|_: ()| { i += 1; }));
            r.execute();
        }
        assert_eq!(i, 1);
    }

    #[test]
    fn sequence() {
        let mut i = 0;
        {
            let mut r = Runtime::new(mp((|_: ()| 42).seq(|v| i = v)));
            r.execute();
        }
        assert_eq!(i, 42);
    }

    #[test]
    fn pause() {
        let mut i = 0;
        let p = &mut i as *mut i32;
        {
            let mut r =
                rt!{
                |_| 42;
                Pause;
                |v| i = v
            };
            r.instant();
            unsafe {
                assert_eq!(*p, 0);
            }
            r.instant();
        }
        assert_eq!(i, 42);
    }

    #[test]
    fn choice() {
        let mut i = 0;
        run!{
            |_| ChoiceData::True(42);
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
            |_| ChoiceData::True(42);
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

    #[test]
    fn emit() {
        let mut value = 0;
        let signal = SignalRuntimeRef::new_pure();
        {
            run! {
                |_| {
                    let signal = SignalRuntimeRef::new_pure();
                    let signal2 = signal.clone();
                ((signal,()), signal2)
                };
                Emit;
                |val| value = 42
            };
        }
        assert_eq!(value, 42);
    }
}
