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
            (Some(row), Some(col)) => unreachable!("both? {row} {col}"),
        })
        .collect_vec();

    dbg!(&sum, sum.iter().sum::<usize>());
}

fn get_rowcols(mut pattern: Vec<Vec<u8>>) -> (Option<usize>, Option<usize>) {
    let orig = get_rowcols_smudged(&pattern, (None, None));
    for row in 0..pattern.len() {
        for col in 0..pattern[0].len() {
            let o = pattern[row][col];
            pattern[row][col] = if o == b'.' { b'#' } else { b'.' };
            let smudgy = get_rowcols_smudged(&pattern, orig);
            if smudgy != (None, None) {
                dbg!(
                    orig,
                    pattern
                        .iter()
                        .map(|line| String::from_utf8(line.to_vec()).unwrap())
                        .collect_vec()
                );
                return smudgy;
            }
            let o = pattern[row][col];
            pattern[row][col] = if o == b'.' { b'#' } else { b'.' };
        }
    }
    unreachable!()
}

fn get_rowcols_smudged(
    pattern: &[Vec<u8>],
    (ignore_row, ignore_col): (Option<usize>, Option<usize>),
) -> (Option<usize>, Option<usize>) {
    let mut mirror_col = None;
    for col in 1..pattern[0].len() {
        let symmetric = pattern.iter().all(|line| {
            line[..col]
                .iter()
                .rev()
                .zip(line[col..].iter())
                .all(|(a, b)| a == b)
        });
        if symmetric && Some(col) != ignore_col {
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
        if symmetric && Some(row) != ignore_row {
            mirror_row = Some(row);
            break;
        }
    }

    (mirror_row, mirror_col)
}
