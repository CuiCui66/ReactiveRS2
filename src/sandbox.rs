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
        |_| 0;
        loop {
            |i|{
                if i < 42{
                    True(i+1)
                }
                else {
                    False(i)
                }
            };
            Pause
        };
        |i| ()
    };
    print_graph(&mut p);

    // let mut r = rt!(p);
    // r.printDot();

}
