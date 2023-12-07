use std::collections::HashSet;

fn main() {
    let input = include_str!("../../data/day4.txt");

    let mut map = vec![1; input.lines().count()];

    let array = input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|card| card.split_once(" | ").unwrap())
        .map(|(a, b)| (splitparse(a).collect::<HashSet<u32>>(), splitparse(b)))
        .map(|(winning_numbers, held_numbers)| {
            held_numbers
                .filter(|num| winning_numbers.contains(num))
                .count()
        })
        .enumerate()
        .map(|(i, val)| {
            for next_card in i + 1..i + 1 + val {
                map[next_card] += map[i]
            }
            map[i]
        })
        .sum::<usize>();
    dbg!(array);
}

fn splitparse(s: &'static str) -> impl Iterator<Item = u32> {
    s.split_ascii_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
}
