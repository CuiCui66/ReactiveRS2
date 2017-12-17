#![feature(box_syntax, box_patterns)]
#![feature(plugin)]
#![plugin(promacros)]

#[macro_use] extern crate ReactiveRS2;

use ReactiveRS2::process::*;
use ReactiveRS2::signal::*;
use ReactiveRS2::engine::*;
use ReactiveRS2::node::ChoiceData::*;
use std::fmt::{Display, Formatter, Error};

const N: usize = 10;

type Signal = SignalRuntimeRef<MCSignalValue<(),usize>>;

struct Board {
    width: usize,
    height: usize,
    signals: Vec<Vec<Signal>>,
    data: Vec<Vec<bool>>
}

impl Board {

    fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            signals: vec![vec![Signal::new_mc(0, box |_:(), a: &mut usize| {*a += 1;} ); width]; height],
            data: vec![vec![false; width]; height],
        }
    }

    fn reset(&mut self) {
        for line in &mut self.data {
            for cell in line {
                *cell = false;
            }
        }
    }
}

impl Display for Board {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        let mut s = String::new();
        for line in &self.data {
            for cell in line {
                if *cell {
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
    let mut board = Board::new(20,10);

    {
        let mut processes = vec![];
        let height = board.height as i32;
        let width = board.width as i32;
        for (i, line) in &mut board.data.iter_mut().enumerate() {
            for (j, cell) in line.iter_mut().enumerate() {
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
                let process = pro!(
                    loop {
                        move |_:()| {
                            s.clone()
                        };
                        AwaitD;
                        move |val:usize| {
                            let e1 = (s1.clone(), ());
                            let e2 = ((s2.clone(), ()), e1);
                            let e3 = ((s3.clone(), ()), e2);
                            let e4 = ((s4.clone(), ()), e3);
                            let e5 = ((s5.clone(), ()), e4);
                            let e6 = ((s6.clone(), ()), e5);
                            let e7 = ((s7.clone(), ()), e6);
                            let e8 = ((s8.clone(), ()), e7);
                            e8
                        };
                        EmitD;
                        EmitD;
                        EmitD;
                        EmitD;
                        EmitD;
                        EmitD;
                        EmitD;
                        EmitD;
                        move |_| {
                            *cell = true;
                            if true {
                                False(())
                            } else {
                                True(())
                            }
                        }
                    }
                );
                processes.push(process);
            }
        }
    }

    println!("{}", board);
}