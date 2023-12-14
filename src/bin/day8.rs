#![allow(unused)]
use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day8.txt");

    let (instructions, network) = input.split_once("\n\n").unwrap();
    let network = network.lines().collect_vec();

    let aaa = network
        .iter()
        .filter(|line| line.chars().nth(2).unwrap() == 'A')
        .collect_vec();

    let xs = aaa
        .iter()
        .map(|&a| {
            let mut sum = 0;
            let mut current = *a;
            for instruction in instructions.chars().cycle() {
                current = next(current, &network, instruction == 'L');
                sum += 1;
                // dbg!(current, instruction);
                if current.chars().nth(2).unwrap() == 'Z' {
                    break;
                }
            }
            dbg!(sum)
        })
        .collect_vec();

    dbg!(lcm(&xs));
}

fn next<'a>(line: &str, network: &[&'a str], left: bool) -> &'a str {
    let next_node = if left {
        line.chars().skip(7).take(3).collect::<String>()
    } else {
        line.chars().skip(7).skip(5).take(3).collect::<String>()
    };

    network
        .iter()
        .find(|line| line.starts_with(&next_node))
        .unwrap()
}

fn lcm(xs: &[usize]) -> usize {
    let first = xs[0];
    let mut lcm = first;
    dbg!(xs.iter().product::<usize>());
    loop {
        if xs.iter().all(|x| lcm % x == 0) {
            return lcm;
        }
        lcm += first;
    }
}

struct Node {}
