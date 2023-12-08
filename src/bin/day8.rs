use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day8.txt");

    let (instructions, network) = input.split_once("\n\n").unwrap();
    let network = network.lines().collect_vec();

    let mut visited = HashSet::new();

    let aaa = network
        .iter()
        .filter(|line| line.chars().nth(2).unwrap() == 'A')
        .copied()
        .collect_vec();

    let mut sum = 0;
    let mut current = aaa;
    for instruction in instructions.chars().cycle() {
        let debug = sum % 100000 == 0;
        // dbg!(&current, instruction);
        current = next(debug, &current, &network, instruction == 'L');
        sum += 1;
        if debug {
            dbg!(&current, sum);
        }
        if done(&current) {
            break;
        }
        if visited.contains(&current) {
            dbg!(&current, instruction, sum);
            return;
        }
        visited.insert(current.clone());
    }

    dbg!(sum);
}

fn next<'a>(debug: bool, lines: &[&'a str], network: &[&'a str], left: bool) -> Vec<&'a str> {
    lines
        .iter()
        .map(|line| {
            let next_node = if left {
                line.chars().skip(7).take(3).collect::<String>()
            } else {
                line.chars().skip(7).skip(5).take(3).collect::<String>()
            };
            if (debug) {
                dbg!(line, &next_node);
            }

            *network.iter().find(|l| l.starts_with(&next_node)).unwrap()
        })
        .collect()
}

fn done(line: &[&'static str]) -> bool {
    line.iter().all(|line| line.chars().nth(2).unwrap() == 'Z')
}

struct Node {}
