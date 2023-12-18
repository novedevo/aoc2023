use std::collections::{HashMap, VecDeque};

use enum_iterator::{all, Sequence};
use itertools::Itertools;
// use memoize::memoize;
// use rayon::iter::{ParallelBridge, ParallelIterator};

//1261 is TOO HIGH
fn main() {
    let input = include_str!("../../data/day17.txt");
    let matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();

    let mut previous_beamstates = HashMap::with_capacity(40_000);
    let mut bfs_frontier = VecDeque::with_capacity(100);

    let start_right = State::new(0, 0, FromDirection::Left, 1).unwrap();
    let start_down = State::new(0, 0, FromDirection::Up, 1).unwrap();

    // previous_beamstates.insert(start_down, 0);
    // previous_beamstates.insert(start_right, 0);

    bfs_frontier.push_back((start_right, 0));
    bfs_frontier.push_back((start_down, 0));

    while let Some((next_state, heat_loss)) = bfs_frontier.pop_front() {
        if &heat_loss >= previous_beamstates.get(&next_state).unwrap_or(&usize::MAX) {
            continue;
        }
        previous_beamstates.insert(next_state, heat_loss);
        let next_states = next(next_state, matrix.len() - 1, matrix[0].len() - 1)
            .into_iter()
            .map(|state| (state, heat_loss + matrix[state.row][state.col]));
        bfs_frontier.extend(next_states);
    }

    let min = previous_beamstates
        .into_iter()
        .filter(|(k, _)| k.row == matrix.len() - 1 && k.col == matrix[0].len() - 1)
        .map(|(_, v)| v)
        .min()
        .unwrap();

    dbg!(min);
}

fn next(state: State, max_row: usize, max_col: usize) -> Vec<State> {
    let nds = next_directions(state.from);
    let retval = nds
        .iter()
        .filter_map(|&direction| {
            let (row, col) = direction.step(state.row, state.col, max_row, max_col)?;
            let consecutive = if direction == state.from {
                state.consecutive + 1
            } else if state.consecutive >= 4 {
                1
            } else {
                return None;
            };
            State::new(row, col, direction, consecutive)
        })
        .collect();
    retval
}

fn next_directions(direction: FromDirection) -> Vec<FromDirection> {
    all::<FromDirection>()
        .filter(|d| d.opposite() != direction)
        .collect()
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct State {
    row: usize,
    col: usize,
    from: FromDirection,
    consecutive: u8,
}
impl State {
    fn new(row: usize, col: usize, from: FromDirection, consecutive: u8) -> Option<Self> {
        if consecutive > 10 {
            None
        } else {
            Some(Self {
                row,
                col,
                from,
                consecutive,
            })
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, Sequence)]
enum FromDirection {
    Down,
    Left,
    Up,
    Right,
}

impl FromDirection {
    fn opposite(&self) -> Self {
        use FromDirection::*;
        match self {
            Down => Up,
            Left => Right,
            Up => Down,
            Right => Left,
        }
    }
    fn step(
        &self,
        row: usize,
        col: usize,
        max_row: usize,
        max_col: usize,
    ) -> Option<(usize, usize)> {
        use FromDirection::*;
        match self {
            Up => {
                if row == max_row {
                    None
                } else {
                    Some((row + 1, col))
                }
            }
            Down => {
                if row == 0 {
                    None
                } else {
                    Some((row - 1, col))
                }
            }
            Right => {
                if col == 0 {
                    None
                } else {
                    Some((row, col - 1))
                }
            }
            Left => {
                if col == max_col {
                    None
                } else {
                    Some((row, col + 1))
                }
            }
        }
    }
}
