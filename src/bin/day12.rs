use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::bytes::Regex;

fn main() {
    let input = include_str!("../../data/day12_test.txt");
    let sum = input
        .lines()
        .par_bridge()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(conditions, groups)| {
            (
                format!("{conditions}?{conditions}?{conditions}?{conditions}?{conditions}"),
                build_regex(
                    &format!("{groups},{groups},{groups},{groups},{groups},{groups}")
                        .split(',')
                        .map(|group| group.parse::<usize>().unwrap())
                        .collect_vec(),
                ),
            )
        })
        .map(|(conditions, regex)| count_matches(&conditions, regex))
        .sum::<usize>();
    dbg!(sum);
}

fn build_regex(groups: &[usize]) -> Regex {
    let builder = groups
        .iter()
        .map(|cond| format!("#{{{cond}}}"))
        .join(r"\.+");

    Regex::new(&format!(r"^\.*{builder}\.*$")).unwrap()
}

fn count_matches(conditions: &str, regex: Regex) -> usize {
    let catcount = conditions.chars().filter(|&c| c == '?').count();
    if catcount == 0 {
        return 1;
    }
    let modifiable = conditions.to_string();
    let mut modifiable = modifiable.into_bytes();
    let unmodifiable = modifiable.clone();

    let mut retval = 0;
    for iterate in 0usize..(1 << catcount) {
        let mut iterate_index = 0;

        for character in modifiable.iter_mut() {
            if *character == b'?' {
                *character = if iterate & (1 << iterate_index) != 0 {
                    b'#'
                } else {
                    b'.'
                };
                iterate_index += 1;
            }
        }

        if regex.is_match(&modifiable) {
            // dbg!(
            //     iterate,
            //     String::from_utf8(modifiable.clone()).unwrap(),
            //     regex.as_str()
            // );
            retval += 1;
        }

        modifiable.copy_from_slice(&unmodifiable);
    }

    retval
}
