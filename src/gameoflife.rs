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
type CellSignal = SignalRuntimeRef<MCSignalValue<bool,(bool,usize)>>;

/// The type of the board signal
type BoardSignal = SignalRuntimeRef<BoardData>;

/// The board data
/// The first element represent the current instant number
/// The elements in the vectors represent the last instant where the cell was set
#[derive(Clone)]
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
                let gather = box |is_self: bool, &mut (ref mut was_set, ref mut nb_neighbors): &mut (bool, usize)| {
                    if is_self {
                        *was_set = true;
                    } else {
                        *nb_neighbors += 1;
                    }
                };
                signals[i].push(CellSignal::new_mc((false, 0), gather));
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
    let width = 20;
    let height = 10;
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
                    vec![(s.clone(),true),
                         (s1.clone(),false),
                         (s2.clone(),false),
                         (s3.clone(),false),
                         (s4.clone(),false),
                         (s5.clone(),false),
                         (s6.clone(),false),
                         (s7.clone(),false),
                         (s8.clone(),false)];

                let process = pro!(
                    loop {
                        |_:()| {
                            ()
                        };
                        await_s(s.clone());
                        move |(was_set,v): (bool,usize)| {
                            if v == 3 || (v == 2 && was_set) {
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


        let signal1 = board.signals[2][1].clone();
        let signal2 = board.signals[2][2].clone();
        let signal3 = board.signals[2][3].clone();
        let signal4 = board.signals[7][1].clone();
        let signal5 = board.signals[7][2].clone();
        let signal6 = board.signals[7][3].clone();
        let signal7 = board.signals[2][16].clone();
        let signal8 = board.signals[2][17].clone();
        let signal9 = board.signals[2][18].clone();
        let signal10 = board.signals[7][16].clone();
        let signal11 = board.signals[7][17].clone();
        let signal12 = board.signals[7][18].clone();

        let v = vec![
            (signal1.clone(),false),(signal1.clone(),false),(signal1.clone(),false),
            (signal2.clone(),false),(signal2.clone(),false),(signal2.clone(),false),
            (signal3.clone(),false),(signal3.clone(),false),(signal3.clone(),false),
            (signal4.clone(),false),(signal4.clone(),false),(signal4.clone(),false),
            (signal5.clone(),false),(signal5.clone(),false),(signal5.clone(),false),
            (signal6.clone(),false),(signal6.clone(),false),(signal6.clone(),false),
            (signal7.clone(),false),(signal7.clone(),false),(signal7.clone(),false),
            (signal8.clone(),false),(signal8.clone(),false),(signal8.clone(),false),
            (signal9.clone(),false),(signal9.clone(),false),(signal9.clone(),false),
            (signal10.clone(),false),(signal10.clone(),false),(signal10.clone(),false),
            (signal11.clone(),false),(signal11.clone(),false),(signal11.clone(),false),
            (signal12.clone(),false),(signal12.clone(),false),(signal12.clone(),false)];

        let rt2 = pro!(
            |_:()| { () };
            emit_vec_vs(v)
        );


        let mut rt = rt!(rt2; rt1);

        let n = 100_000;
        let start = SteadyTime::now();
        rt.instantn(n);
        println!("{}", (n as f32) / ((SteadyTime::now() - start).num_nanoseconds().unwrap() as f32 / 1_000_000_000.))
    }

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
}
