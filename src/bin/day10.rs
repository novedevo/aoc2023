use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day10_test1.txt");
    let mut matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    // let mut meowtrix = matrix.clone();
    let spos = spos(&matrix);
    let mut next_state = next_from_spos(spos.0, spos.1, &matrix);
    let mut counter = 0;

    while matrix[next_state.0][next_state.1] != 'S' {
        next_state = next(next_state.0, next_state.1, next_state.2, &mut matrix);
        // counter += 1;
    }

    for row in 0..matrix.len() {
        let mut exterior = true;
        for col in 0..matrix[0].len() {
            if matrix[row][col] == '🐈' {
                exterior = !exterior;
            } else {
                matrix[row][col] = if exterior {
                    'O'
                } else {
                    counter += 1;
                    'I'
                }
            }
        }
    }

    dbg!(matrix);
    dbg!(counter);

    dbg!((counter as f64 / 2.0).ceil());
}

fn spos(matrix: &[Vec<char>]) -> (usize, usize) {
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 'S' {
                return (row, col);
            }
        }
    }
    unreachable!()
}

fn next_from_spos(srow: usize, scol: usize, matrix: &[Vec<char>]) -> (usize, usize, FromDirection) {
    use FromDirection::*;
    let above = matrix[srow - 1][scol];
    if above == '|' || above == '7' || above == 'F' {
        return (srow - 1, scol, Down);
    }
    let below = matrix[srow + 1][scol];
    if below == '|' || below == 'L' || below == 'J' {
        return (srow + 1, scol, Up);
    }
    let left = matrix[srow][scol - 1];
    if left == '-' || left == 'F' || left == 'L' {
        return (srow, scol - 1, Right);
    }
    let right = matrix[srow][scol + 1];
    if right == '-' || right == '7' || right == 'J' {
        return (srow, scol + 1, Left);
    }
    unreachable!();
}

fn next(
    row: usize,
    col: usize,
    from: FromDirection,
    matrix: &mut [Vec<char>],
) -> (usize, usize, FromDirection) {
    use FromDirection::*;
    let current = matrix[row][col];
    dbg!(row, col, current, from);
    let retval = match current {
        '|' => {
            if from == Down {
                (row - 1, col, Down)
            } else {
                (row + 1, col, Up)
            }
        }
        '-' => {
            if from == Left {
                (row, col + 1, Left)
            } else {
                (row, col - 1, Right)
            }
        }
        'L' => {
            if from == Up {
                (row, col + 1, Left)
            } else {
                (row - 1, col, Down)
            }
        }
        'J' => {
            if from == Up {
                (row, col - 1, Right)
            } else {
                (row - 1, col, Down)
            }
        }
        '7' => {
            if from == Down {
                (row, col - 1, Right)
            } else {
                (row + 1, col, Up)
            }
        }
        'F' => {
            if from == Down {
                (row, col + 1, Left)
            } else {
                (row + 1, col, Up)
            }
        }
        _ => unreachable!(),
    };

    matrix[row][col] = '🐈';
    retval
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum FromDirection {
    Down,
    Up,
    Left,
    Right,
}
