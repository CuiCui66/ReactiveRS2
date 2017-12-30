#![feature(box_syntax, box_patterns)]
#![feature(plugin)]
#![plugin(promacros)]

#[macro_use] extern crate ReactiveRS2;
extern crate time;

use ReactiveRS2::process::*;
use ReactiveRS2::signal::*;
use ReactiveRS2::engine::*;
use ReactiveRS2::node::ChoiceData::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use time::SteadyTime;

/// The type of the cell signal
type CellSignal = SignalRuntimeRef<MCSignalValue<usize,usize>>;

/// The type of the board signal
type BoardSignal = SignalRuntimeRef<BoardData>;

/// The board data
/// The first element represent the current instant number
/// The elements in the vectors represent the last instant where the cell was set
struct BoardData {
    data: Arc<Mutex<(usize, Vec<Vec<usize>>)>>,
}

impl BoardData {
    /// Create a new board with given width and height
    fn new(width: usize, height: usize) -> (BoardData, Arc<Mutex<(usize,Vec<Vec<usize>>)>>) {
        let board_data = BoardData {
            data: Arc::new(Mutex::new((1,vec![vec![0;width];height]))),
        };
        let board_values = board_data.data.clone();
        (board_data, board_values)
    }
}


impl SignalValue for BoardData {
    type E = (usize,usize);
    type V = Arc<Mutex<(usize,Vec<Vec<usize>>)>>;

    fn get_pre_value(&self) -> Self::V {
       self.data.clone()
    }

    fn reset_value(&mut self) {
        self.data.lock().unwrap().0 += 1;
    }

    fn gather(&mut self, (i,j): (usize, usize)) {
        let mut data = self.data.lock().unwrap();
        data.1[i][j] = data.0;
    }
}

/// A board
struct Board {
    width: usize,
    height: usize,
    signals: Vec<Vec<CellSignal>>,
}

impl Board {

    fn new(width: usize, height: usize) -> Board {
        let mut signals = vec![vec![]];
        for i in 0..height {
            for _ in 0..width {
                let gather = box |emit_value: usize, value: &mut usize| {
                    *value += emit_value;
                };
                signals[i].push(CellSignal::new_mc(0, gather));
            }
            signals.push(vec![]);
        }
        Board {
            width,
            height,
            signals,
        }
    }
}


fn main() {
    let width = 1000;
    let height = 1000;
    let board = Board::new(width,height);
    let (board_data, board_values) = BoardData::new(width, height);
    let board_signal = BoardSignal::new(board_data);
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

                let v =
                    vec![(s.clone(),1),
                         (s1.clone(),2),
                         (s2.clone(),2),
                         (s3.clone(),2),
                         (s4.clone(),2),
                         (s5.clone(),2),
                         (s6.clone(),2),
                         (s7.clone(),2),
                         (s8.clone(),2)];

                let process = pro!(
                    loop {
                        |_:()| {
                            ()
                        };
                        await_s(s.clone());
                        move |v: usize| {
                            if v == 5 || v == 6 || v == 7  {
                                True(())
                            } else {
                                False(())
                            }
                        };
                        choice {
                            |_:()| {
                                ()
                            };
                            emit_vec_vs(v);
                            emit_vs(board_signal.clone(), (i as usize,j as usize));
                            move |_:()| {
                                if false {
                                    False(())
                                } else {
                                    True(())
                                }}
                        } {
                            |_:()| {
                                if false {
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
        let mut v = vec![];
        for i in 0..200 {
            for j in 0..200 {
                let signal1 = board.signals[2 + 5*i][1 + 5*j].clone();
                let signal2 = board.signals[2 + 5*i][2 + 5*j].clone();
                let signal3 = board.signals[2 + 5*i][3 + 5*j].clone();
                v.push((signal1, 6));
                v.push((signal2, 6));
                v.push((signal3, 6));
            }
        }

        let rt2 = pro!(
            |_:()| { () };
            emit_vec_vs(v)
        );


        let mut rt = rt!(rt2; rt1);

        let n = 10;
        let start = SteadyTime::now();
        rt.instantn(n);
        println!("{}", (n as f32) / ((SteadyTime::now() - start).num_nanoseconds().unwrap() as f32 / 1_000_000_000.))
    }

    // print the values
    /*
    let values = board_values.lock().unwrap();
    for i in 0..height {
        for j in 0..width {
            if values.1[i][j] == values.0 {
                print!("+");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
    */
}
