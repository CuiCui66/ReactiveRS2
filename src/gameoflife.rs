#![feature(box_syntax, box_patterns)]
#![feature(plugin)]
#![plugin(promacros)]

#[macro_use] extern crate ReactiveRS2;
extern crate time;

use ReactiveRS2::process::*;
use ReactiveRS2::signal::*;
use ReactiveRS2::engine::*;
use ReactiveRS2::node::ChoiceData::*;
use time::SteadyTime;

/// The type of the cell signal
type CellSignal = SignalRuntimeRef<MCSignalValue<usize,usize>>;

/// The type of the board signal
type BoardSignal = SignalRuntimeRef<BoardData>;

/// The board data
/// The first element represent the current instant id
/// (which is not necessarily the same value than the id of the current instant in the engine).
struct BoardData {
    /// The id of the current instant
    /// (which is not necessarily the same value than the id of the current instant in the engine)
    current_instant: usize,
    /// The data of the board
    /// The value of a cell is the last instant id where the cell was active
    data: Vec<Vec<usize>>,
}

impl BoardData {
    /// Create a new board with given width and height
    fn new(width: usize, height: usize) -> BoardData {
        BoardData {
            current_instant: 1,
            data: vec![vec![0;width];height],
        }
    }
}

/// The board can be considered as a signal value
/// Instead of resetting the internal value each iteration, we
/// change the current instant id.
impl SignalValue for BoardData {
    type E = (usize,usize);
    type V = Vec<Vec<usize>>;

    fn get_pre_value(&self) -> Self::V {
        self.data.clone()
    }

    fn reset_value(&mut self) {
        self.current_instant += 1;
    }

    fn gather(&mut self, (i,j): (usize, usize)) {
        self.data[i][j] = self.current_instant;
    }
}

/// The board, containing the signals representing the cells
struct Board {
    /// The signals, each one representing one cell
    signals: Vec<Vec<CellSignal>>,
}

impl Board {
    /// Create a new board with a given height and width
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
            signals,
        }
    }
}


fn main() {
    let width = 1000;
    let height = 1000;
    let board = Board::new(width,height);
    let board_signal = BoardSignal::new(BoardData::new(width, height));

    // This vector will contain one process per cell.
    let mut processes = vec![];
    for i in 0..height {
        for j in 0..width {

            // The signals representing the current cell (s) and its neighbors
            let s  = board.signals [i]                   [j]                 .clone();
            let s1 = board.signals [(i+height-1)%height] [(j+width-1)%width] .clone();
            let s2 = board.signals [(i+height-1)%height] [j]                 .clone();
            let s3 = board.signals [(i+height-1)%height] [(j+1)%width]       .clone();
            let s4 = board.signals [i]                   [(j+width-1)%width] .clone();
            let s5 = board.signals [i]                   [(j+1)%width]       .clone();
            let s6 = board.signals [(i+1)%height]        [(j+width-1)%width] .clone();
            let s7 = board.signals [(i+1)%height]        [j]                 .clone();
            let s8 = board.signals [(i+1)%height]        [(j+1)%width]       .clone();

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

            // The process for a single cell
            let process = pro!(
                loop {
                    value(());
                    // The cell await for neighbors to emit to this signal.
                    await_s(s.clone());
                    // When at least one neighbor has emit, we check that
                    // 3 neighbors emitted, or 2 emitted, plus the current cell.
                    // When a neighbor emit, it emits 2, and when a cell emit to itself, it emits 1.
                    move |v: usize| {
                        if v == 5 || v == 6 || v == 7  {
                            True(())
                        } else {
                            False(())
                        }
                    };
                    choice {
                        // If the cell is active at this instant,
                        value(());
                        // we emit to the neighbors, for the computation of the next instant,
                        emit_vec_vs(v);
                        // and we emit to the board, so it knows the cell is active
                        emit_vs(board_signal.clone(), (i as usize,j as usize));
                        value(True(()))
                    } {
                        // If the cell is inactive, we don't do anything
                        value(True(()))
                    }
                }
            );

            processes.push(process);
        }
    }

    // We join all the processes
    let rt1 = pro!(big_join(processes));
    let mut v = vec![];

    // We activate 200*200*3 cells (it is 200*200 blinkers, so the same number of operations are needed each steps)
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

    // We do n iterations, and we compute the average time needed to compute one iteration
    let n = 10;
    let start = SteadyTime::now();
    rt.instantn(n);
    let frequency = (n as f32) / ((SteadyTime::now() - start).num_nanoseconds().unwrap() as f32 / 1_000_000_000.);
    println!("{} iterations of the Game of Life can be executed each second.", frequency);
}
