#![allow(unused)]
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use memoize::memoize;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../../data/day15.txt");
    let meow = input.split(',').map(hash).collect_vec();
    dbg!(&meow, meow.iter().sum::<usize>());
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .map(|c| c as usize)
        .fold(0, |acc, c| ((acc + c) * 17) % 256)
}
