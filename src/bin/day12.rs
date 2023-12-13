#![allow(unused)]
use itertools::Itertools;
use memoize::memoize;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../../data/day12_test.txt");
    let sum = input
        .lines()
        // .par_bridge()
        // .filter(|line| line.chars().all(|c| c != '.'))
        .flat_map(|line| line.split_once(' '))
        // .map(to_part2)
        .map(|(conditions, groups)| {
            (
                conditions.to_string().into_bytes(),
                groups
                    .split(',')
                    .map(|group| group.parse::<usize>().unwrap())
                    .collect_vec(),
            )
        })
        .map(|(conditions, groups)| count_matches(conditions, groups, vec![], false))
        // .map(|c| c / 1_000_000_000)
        .collect_vec();
    dbg!(sum);
}

fn to_part2((conditions, groups): (&str, &str)) -> (String, String) {
    (
        format!("{conditions}?{conditions}?{conditions}?{conditions}?{conditions}"),
        format!("{groups},{groups},{groups},{groups},{groups}"),
    )
}

// #[memoize]
fn count_matches(
    conditions: Vec<u8>,
    groups: Vec<usize>,
    mut history: Vec<u8>,
    required_to_continue: bool,
) -> usize {
    if conditions.is_empty() && groups.is_empty() {
        return 1;
    } else if conditions.is_empty() {
        return 0;
    } else if groups.is_empty() {
        if conditions.iter().all(|&c| c == b'.' || c == b'?') {
            return 1;
        } else {
            return 0;
        }
    }

    let mut next_conditions = conditions[1..].to_vec();
    if conditions[0] == b'.' {
        if required_to_continue {
            return 0;
        }
        history.push(b'.');
        count_matches(next_conditions, groups, history, false)
    } else if conditions[0] == b'#' {
        history.push(b'#');
        if groups[0] == 1 {
            //we finished a group, so the next one has to be either the end or a dot
            if next_conditions.is_empty() {
                if groups.len() == 1 {
                    1
                } else {
                    0
                }
            } else if next_conditions[0] == b'#' {
                0
            } else {
                next_conditions[0] = b'.';
                count_matches(next_conditions, groups[1..].to_vec(), history, false)
            }
        } else {
            let mut next_groups = groups;
            next_groups[0] -= 1;
            count_matches(next_conditions, next_groups, history, true)
        }
    } else if conditions[0] == b'?' {
        history.push(b'.');
        let placed_dot = if required_to_continue {
            0
        } else {
            count_matches(
                next_conditions.clone(),
                groups.clone(),
                history.clone(),
                false,
            )
        };
        history.pop();
        history.push(b'#');
        let placed_hash = if groups[0] == 1 {
            //we finished a group, so the next one has to be either the end or a dot
            if next_conditions.is_empty() {
                if groups.len() == 1 {
                    1
                } else {
                    0
                }
            } else if next_conditions[0] == b'#' {
                0
            } else {
                next_conditions[0] = b'.';
                count_matches(next_conditions, groups[1..].to_vec(), history, false)
            }
        } else {
            let mut next_groups = groups;
            next_groups[0] -= 1;
            count_matches(next_conditions, next_groups, history, true)
        };
        placed_dot + placed_hash
    } else {
        unreachable!();
    }
}
