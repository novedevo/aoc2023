use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day6.txt");

    let (time, distance) = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_ascii_digit()))
        .map(|s| s.collect::<String>().parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    dbg!(time, distance);

    let res = process(time, distance);

    dbg!(res);
}

fn process(time: usize, distance: usize) -> usize {
    (0..=time)
        .map(|held| calc_distance(time, held))
        .filter(|d| *d > distance)
        .count()
}

fn calc_distance(total_time: usize, held_time: usize) -> usize {
    held_time * (total_time - held_time)
}
