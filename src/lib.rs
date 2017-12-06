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
mod take;




#[cfg(test)]
mod tests {
    use engine::*;
    use process::*;

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
            let mut r = Runtime::new(mp((|_: ()| 42).seq(Pause).seq(|v| { i = v; })));
            r.instant();
            unsafe {
                assert_eq!(*p, 0);
            }
            r.instant();
        }
        assert_eq!(i, 42);
    }
    #[test]
    fn macrot() {
        let mut i = 0;
        run!((|_| 42) >> Pause >> (|v| { i = v; }));
        assert_eq!(i, 42);
    }

    #[test]
    fn pmacro() {
        let mut i = 0;
        {
            let mut r = Runtime::new(mp(ppro!{
                    |_| 42;
                    Pause;
                    |val| {i = val;}
                }));
            r.execute();
        }
        assert_eq!(i, 42);

    }
}
