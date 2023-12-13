#![allow(unused)]
use itertools::Itertools;
use memoize::memoize;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../../data/day12.txt");
    let sum = input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.to_string().into_bytes())
                .collect_vec()
        })
        .map(get_rowcols)
        .map(|(row, col)| row * 100 + col)
        .collect_vec();

    dbg!(&sum, sum.iter().sum::<usize>());
}

fn get_rowcols(pattern: Vec<Vec<u8>>) -> (usize, usize) {
    (1..pattern.len() - 1)
        .map(|row| (row, 1..pattern[1].len() - 1))
        .find_map(|(row, mut cols)| Some((row, cols.find(|col| possible(&pattern, row, *col))?)))
        .unwrap()
}

fn possible(pattern: &[Vec<u8>], row: usize, col: usize) -> bool {
    let rowable = pattern.iter().all(|line| {
        line[..col]
            .iter()
            .zip(line[col..].iter().rev())
            .all(|(a, b)| a == b)
    });

    let colable = (0..pattern.len()).all(|c| {
        pattern[..row]
            .iter()
            .map(|line| line[c])
            .zip(pattern[row..].iter().map(|line| line[c]))
            .all(|(a, b)| a == b)
    });

    rowable && colable
}
