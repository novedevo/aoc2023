#![allow(unused)]
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use memoize::memoize;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../../data/day16.txt");
    let matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut max = 0;
    use FromDirection::*;
    for ((row, rowdir), (col, coldir)) in [(0, Up), (matrix.len() - 1, Down)]
        .into_iter()
        .cartesian_product([(0, Left), (matrix[0].len() - 1, Right)])
    {
        max = max
            .max(try_edge(&matrix, (row, col, rowdir)))
            .max(try_edge(&matrix, (row, col, coldir)));
    }

    for row in 1..matrix.len() - 1 {
        max = max
            .max(try_edge(&matrix, (row, 0, Left)))
            .max(try_edge(&matrix, (row, matrix[0].len() - 1, Right)))
    }
    for col in 1..matrix[0].len() - 1 {
        max = max
            .max(try_edge(&matrix, (0, col, Up)))
            .max(try_edge(&matrix, (matrix.len() - 1, col, Down)))
    }
    dbg!(max);
}
fn try_edge(matrix: &[Vec<char>], first_state: (usize, usize, FromDirection)) -> usize {
    let mut meowtrix = matrix.to_owned();

    let mut previous_beamstates = HashSet::with_capacity(40_000);
    let mut bfs_frontier = VecDeque::with_capacity(100);

    previous_beamstates.insert(first_state);
    bfs_frontier.push_back(first_state);

    while let Some(next_state) = bfs_frontier.pop_front() {
        previous_beamstates.insert(next_state);
        let next_states = next(
            next_state.0,
            next_state.1,
            next_state.2,
            matrix,
            &mut meowtrix,
        );
        bfs_frontier.extend(
            next_states
                .iter()
                .filter(|state| previous_beamstates.get(state).is_none()),
        );
    }
    dbg!(
        meowtrix.iter().flatten().filter(|c| **c == '🐈').count(),
        // meowtrix
        //     .into_iter()
        //     .map(|line| line.iter().collect::<String>())
        //     .collect_vec()
    )
}

fn next(
    row: usize,
    col: usize,
    from: FromDirection,
    matrix: &[Vec<char>],
    meowtrix: &mut [Vec<char>],
) -> Vec<(usize, usize, FromDirection)> {
    use FromDirection::*;
    let current = matrix[row][col];
    // dbg!(row, col, current, from);
    meowtrix[row][col] = '🐈';
    let row = row as isize;
    let col = col as isize;
    let retval = match current {
        '|' => match from {
            Down => vec![(row - 1, col, Down)],
            Up => vec![(row + 1, col, Up)],
            _ => vec![(row - 1, col, Down), (row + 1, col, Up)],
        },
        '-' => match from {
            Left => vec![(row, col + 1, Left)],
            Right => vec![(row, col - 1, Right)],
            _ => vec![(row, col + 1, Left), (row, col - 1, Right)],
        },
        '/' => match from {
            Down => vec![(row, col + 1, Left)],
            Up => vec![(row, col - 1, Right)],
            Left => vec![(row - 1, col, Down)],
            Right => vec![(row + 1, col, Up)],
        },
        '\\' => match from {
            Down => vec![(row, col - 1, Right)],
            Up => vec![(row, col + 1, Left)],
            Left => vec![(row + 1, col, Up)],
            Right => vec![(row - 1, col, Down)],
        },
        '.' => match from {
            Down => vec![(row - 1, col, Down)],
            Up => vec![(row + 1, col, Up)],
            Left => vec![(row, col + 1, Left)],
            Right => vec![(row, col - 1, Right)],
        },
        _ => unreachable!(),
    };
    // dbg!(&retval);
    let retval = retval
        .into_iter()
        .filter(|(r, c, _)| *r >= 0 && *c >= 0)
        .map(|(r, c, f)| (r as usize, c as usize, f))
        .filter(|(r, c, _)| *r < matrix.len() && *c < matrix[0].len())
        .collect();

    // dbg!(&retval);

    retval
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
enum FromDirection {
    Down,
    Up,
    Left,
    Right,
}
