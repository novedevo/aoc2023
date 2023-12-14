#![allow(unused)]
use itertools::Itertools;
use memoize::memoize;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../../data/day14.txt");
    let mut mirror = input
        .lines()
        .map(|line| line.to_string().into_bytes())
        .collect_vec();
    let mut sum = 0;
    for col in 0..mirror[0].len() {
        for row in 0..mirror.len() {
            if mirror[row][col] == b'O' {
                sum += go_north_for_the_winter(&mut mirror, row, col);
            }
        }
    }

    dbg!(
        sum,
        &mirror
            .iter()
            .map(|line| String::from_utf8(line.to_vec()).unwrap())
            .collect_vec()
    );
}

fn go_north_for_the_winter(mirror: &mut [Vec<u8>], mut row: usize, col: usize) -> usize {
    while row > 0 {
        if mirror[row - 1][col] == b'.' {
            mirror[row - 1][col] = b'O';
            mirror[row][col] = b'.';
            row -= 1;
        } else {
            return mirror.len() - row;
        }
    }
    mirror.len() - row
}
