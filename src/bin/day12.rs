#![allow(unused)]
use itertools::Itertools;
// use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../../data/day12.txt");
    let sum = input
        .lines()
        // .par_bridge()
        .filter(|line| line.chars().all(|c| c != '.'))
        .map(|line| line.split_once(' ').unwrap())
        .map(to_part2)
        .map(|(conditions, groups)| {
            (
                conditions.to_string().into_bytes(),
                groups
                    .split(',')
                    .map(|group| group.parse::<usize>().unwrap())
                    .collect_vec(),
            )
        })
        .map(|(conditions, groups)| count_matches(conditions, groups))
        .map(|c| c / 1_000_000_000)
        .collect_vec();
    dbg!(sum);
}

fn to_part2((conditions, groups): (&str, &str)) -> (String, String) {
    (
        format!("{conditions}?{conditions}?{conditions}?{conditions}?{conditions}"),
        format!("{groups},{groups},{groups},{groups},{groups}"),
    )
}

fn count_matches(conditions: Vec<u8>, groups: Vec<usize>) -> usize {
    // let catcount = conditions.iter().filter(|&&c| c == b'?').count();
    let preexisting = get_preexisting_groups(&conditions);
    if preexisting == groups {
        // dbg!(String::from_utf8(conditions).unwrap(), groups);
        return 1;
    }

    // let combinations = (0..groups.len() + )

    get_upper_bound(&conditions, &groups)
}

fn get_preexisting_groups(conditions: &[u8]) -> Vec<usize> {
    let mut retval = vec![];
    let mut i = 0;
    while i < conditions.len() {
        if conditions[i] == b'#' {
            let mut group = 0;
            while i < conditions.len() && conditions[i] == b'#' {
                group += 1;
                i += 1;
            }
            retval.push(group);
        } else {
            i += 1
        }
    }

    retval
}

//fn split (?) - just do it recursively? split at dots? there's a limited size,,,

fn get_upper_bound(conditions: &[u8], groups: &[usize]) -> usize {
    let filled_in_cells = groups.iter().sum::<usize>();
    let filled_in_cells_with_gaps = filled_in_cells + groups.len() - 1;
    if filled_in_cells_with_gaps > conditions.len() {
        let conds = String::from_utf8(conditions.to_vec()).unwrap();
        dbg!(
            conds,
            groups,
            conditions.len(),
            filled_in_cells,
            filled_in_cells_with_gaps
        );
        unreachable!();
    }

    num::integer::binomial(
        groups.len() + conditions.len() - filled_in_cells_with_gaps,
        groups.len(),
    )
}
