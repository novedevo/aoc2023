#![allow(unused)]
use itertools::Itertools;
use memoize::memoize;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../../data/day13.txt");
    let sum = input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.to_string().into_bytes())
                .collect_vec()
        })
        .map(get_rowcols)
        .map(|(row, col)| match (row, col) {
            (Some(row), None) => row * 100,
            (None, Some(col)) => col,
            (None, None) => unreachable!("neither?"),
            (Some(_), Some(_)) => unreachable!("both?"),
        })
        .collect_vec();

    dbg!(&sum, sum.iter().sum::<usize>());
}

fn get_rowcols(pattern: Vec<Vec<u8>>) -> (Option<usize>, Option<usize>) {
    let mut mirror_col = None;
    for col in 1..pattern[0].len() {
        let symmetric = pattern.iter().all(|line| {
            line[..col]
                .iter()
                .rev()
                .zip(line[col..].iter())
                .all(|(a, b)| a == b)
        });
        if symmetric {
            mirror_col = Some(col);
            break;
        }
    }

    let mut mirror_row = None;
    for row in 1..pattern.len() {
        let symmetric = (0..pattern[0].len()).all(|c| {
            pattern[..row]
                .iter()
                .rev()
                .map(|line| line[c])
                .zip(pattern[row..].iter().map(|line| line[c]))
                .all(|(a, b)| a == b)
        });
        if symmetric {
            mirror_row = Some(row);
            break;
        }
    }

    (mirror_row, mirror_col)
}
