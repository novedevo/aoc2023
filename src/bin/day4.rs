use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day4.txt");

    let mut map = vec![1; input.lines().count()];

    let array = input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|card| card.split_once(" | ").unwrap())
        .map(|(a, b)| {
            (
                splitparse(a).collect::<HashSet<u32>>(),
                splitparse(b).collect_vec(),
            )
        })
        .map(|(winning_numbers, mut held_numbers)| {
            held_numbers.retain(|num| winning_numbers.contains(num));
            held_numbers.len()
        })
        .enumerate()
        .map(|(i, val)| {
            dbg!((i, val));
            for next_card in i + 1..i + 1 + val {
                map[next_card] += map[i]
            }
            dbg!(&map);
            map[i]
        })
        .sum::<usize>();
    dbg!(array);
}

fn splitparse(s: &'static str) -> impl Iterator<Item = u32> {
    s.split_ascii_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
}
