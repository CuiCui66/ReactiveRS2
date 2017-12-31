#![feature(box_syntax, box_patterns)]
#![feature(plugin)]
#![plugin(promacros)]

#[macro_use] extern crate ReactiveRS2;

use ReactiveRS2::process::*;
use ReactiveRS2::signal::*;
use ReactiveRS2::engine::*;
use ReactiveRS2::node::ChoiceData::*;
use std::fmt::{Display, Formatter, Error};
use std::cell::RefCell;
use std::borrow::*;

fn main (){
    let mut p = pro!{
        |_| (0,0);
        {
            {
                |_| 20;
                pause();
                |i| i+1;
                pause()
            } || {
                |_| 20;
                pause();
                |i| i+1;
                pause()
            }
        };
        |(v1,v2)| v1 + v2;
        pause();
        |i| {
            ()
        }
    };
    print_graph(&mut p);

    // let mut r = rt!(p);
    // r.printDot();

}
