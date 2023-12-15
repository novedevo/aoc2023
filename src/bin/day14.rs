#![allow(unused)]
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use memoize::memoize;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../../data/day14.txt");
    let mut mirror = input
        .lines()
        .map(|line| line.to_string().into_bytes())
        .collect_vec();

    let mut mirrors = HashMap::new();
    let mut iters = 0;
    let looped_iters = loop {
        iters += 1;
        if mirrors.contains_key(&mirror) {
            break mirrors.get(&mirror).unwrap();
        }
        mirrors.insert(mirror.to_owned(), iters);
        spin_cycle(&mut mirror);
    };

    let loop_length = iters - looped_iters;
    let remaining_iters = 1000000000 - iters;
    let remaining_real_iters = remaining_iters % loop_length;
    for i in 0..remaining_real_iters + 1 {
        spin_cycle(&mut mirror);
    }

    let mut sum = 0;
    for col in 0..mirror[0].len() {
        for row in 0..mirror.len() {
            if mirror[row][col] == b'O' {
                sum += mirror.len() - row;
            }
        }
    }

    dbg!(
        // iters,
        // looped_iters,
        // loop_length,
        // remaining_iters,
        // remaining_real_iters,
        sum,
        // &mirror
        //     .iter()
        //     .map(|line| String::from_utf8(line.to_vec()).unwrap())
        //     .collect_vec()
    );
}

fn spin_cycle(mirror: &mut [Vec<u8>]) {
    for col in 0..mirror[0].len() {
        for row in 0..mirror.len() {
            if mirror[row][col] == b'O' {
                go_north_for_the_winter(mirror, row, col);
            }
        }
    }
    for col in 0..mirror[0].len() {
        for row in 0..mirror.len() {
            if mirror[row][col] == b'O' {
                go_west_for_the_winter(mirror, row, col);
            }
        }
    }
    for col in 0..mirror[0].len() {
        for row in (0..mirror.len()).rev() {
            if mirror[row][col] == b'O' {
                go_south_for_the_winter(mirror, row, col);
            }
        }
    }
    for col in (0..mirror[0].len()).rev() {
        for row in 0..mirror.len() {
            if mirror[row][col] == b'O' {
                go_east_for_the_winter(mirror, row, col);
            }
        }
    }
}

fn go_north_for_the_winter(mirror: &mut [Vec<u8>], mut row: usize, col: usize) {
    while row > 0 {
        if mirror[row - 1][col] == b'.' {
            mirror[row - 1][col] = b'O';
            mirror[row][col] = b'.';
            row -= 1;
        } else {
            return;
        }
    }
}
fn go_south_for_the_winter(mirror: &mut [Vec<u8>], mut row: usize, col: usize) {
    while row < mirror.len() - 1 {
        if mirror[row + 1][col] == b'.' {
            mirror[row + 1][col] = b'O';
            mirror[row][col] = b'.';
            row += 1;
        } else {
            return;
        }
    }
}
fn go_west_for_the_winter(mirror: &mut [Vec<u8>], row: usize, mut col: usize) {
    while col > 0 {
        if mirror[row][col - 1] == b'.' {
            mirror[row][col - 1] = b'O';
            mirror[row][col] = b'.';
            col -= 1;
        } else {
            return;
        }
    }
}

fn go_east_for_the_winter(mirror: &mut [Vec<u8>], row: usize, mut col: usize) {
    while col < mirror[0].len() - 1 {
        if mirror[row][col + 1] == b'.' {
            mirror[row][col + 1] = b'O';
            mirror[row][col] = b'.';
            col += 1;
        } else {
            return;
        }
    }
}
