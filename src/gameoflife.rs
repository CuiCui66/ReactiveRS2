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

type Signal = SignalRuntimeRef<MCSignalValue<(),usize>>;

struct Board {
    width: usize,
    height: usize,
    signals: Vec<Vec<Signal>>,
    data: Vec<Vec<RefCell<bool>>>
}

impl Board {

    fn new(width: usize, height: usize) -> Board {
        let mut signals = vec![vec![]];
        for i in 0..height {
            for _ in 0..width {
                signals[i].push(Signal::new_mc(0, box |_:(), a: &mut usize| {*a += 1;}));
            }
            signals.push(vec![]);
        }
        Board {
            width,
            height,
            signals,
            data: vec![vec![RefCell::new(false); width]; height],
        }
    }

    fn reset(&self) {
        for line in &self.data {
            for cell in line {
                *cell.borrow_mut() = false;
            }
        }
    }
}

impl Display for Board {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        let mut s = String::new();
        for line in &self.data {
            for cell in line {
                if *cell.borrow() {
                    s += "+";
                } else {
                    s += " ";
                }
            }
            s += "\n";
        }
        write!(formatter,"{}",s)
    }
}


fn main() {
    let board = Board::new(20,10);

    {
        let mut processes = vec![];
        let height = board.height as i32;
        let width = board.width as i32;
        for i in 0..board.height {
            for j in 0..board.width {
                let i = i as i32;
                let j = j as i32;
                let s = board.signals[i as usize][j as usize].clone();
                let s1 = board.signals[((i-1+height)%height) as usize][((j-1+width)%width) as usize].clone();
                let s2 = board.signals[((i-1+height)%height) as usize][j as usize].clone();
                let s3 = board.signals[((i-1+height)%height) as usize][((j+1)%width) as usize].clone();
                let s4 = board.signals[i as usize][((j-1+width)%width) as usize].clone();
                let s5 = board.signals[i as usize][((j+1)%width) as usize].clone();
                let s6 = board.signals[((i+1)%height) as usize][((j-1+width)%width) as usize].clone();
                let s7 = board.signals[((i+1)%height) as usize][j as usize].clone();
                let s8 = board.signals[((i+1)%height) as usize][((j+1)%width) as usize].clone();

                let v = vec![(s1.clone(),()),
                             (s2.clone(),()),
                             (s3.clone(),()),
                             (s4.clone(),()),
                             (s5.clone(),()),
                             (s6.clone(),()),
                             (s7.clone(),()),
                             (s8.clone(),())];

                let cell_value = &board.data[i as usize][j as usize];

                let process = pro!(
                    loop {
                        |_:()| {
                            ()
                        };
                        AwaitS(s.clone());
                        move |(v,_): (usize,())| {
                            if v == 3 || (v == 2 && *(*cell_value).borrow()) {
                                True(())
                            } else {
                                False(())
                            }
                        };
                        choice {
                            |_:()| {
                                ()
                            };
                            emit_value_vec(v);
                            move |_:()| {
                                *(*cell_value).borrow_mut() = true;
                                if true {
                                    False(())
                                } else {
                                    True(())
                                }}
                        } {
                            |_:()| {
                                if true {
                                    False(())
                                } else {
                                    True(())
                                }
                            }
                        }
                    }
                );

                processes.push(process);
            }
        }


        let rt1 = pro!(big_join(processes));


        let signal1 = board.signals[3][3].clone();
        let signal2 = board.signals[3][4].clone();
        let signal3 = board.signals[3][5].clone();


        let rt2 = pro!(
            |_:()| { () };
            emit_value(signal1.clone(), ());
            emit_value(signal2.clone(), ());
            emit_value(signal3.clone(), ())
        );

        let rt8 = pro!(
            |_:()| { () };
            emit_value(signal1.clone(), ());
            emit_value(signal2.clone(), ());
            emit_value(signal3.clone(), ())
        );

        let rt6 = pro!(
            |_:()| { () };
            emit_value(signal1.clone(), ());
            emit_value(signal2.clone(), ());
            emit_value(signal3.clone(), ())
        );

        let mut rt = rt!(rt2; rt6; rt8; rt1);
        rt.instant();
        rt.instant();
        rt.instant();
    }

    println!("{}", board);
}