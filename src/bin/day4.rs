use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day4.txt");
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
            if held_numbers.is_empty() {
                return 0;
            }
            1 << (held_numbers.len() - 1)
        })
        .sum::<u32>();
    dbg!(array);
}

fn splitparse(s: &'static str) -> impl Iterator<Item = u32> {
    s.split_ascii_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
}
