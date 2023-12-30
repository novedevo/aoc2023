#![allow(unused_imports)]
use std::{
    collections::{HashMap, VecDeque},
    convert::Infallible,
    ops::{Index, IndexMut, Range},
    str::FromStr,
};

use enum_iterator::{all, Sequence};
use itertools::Itertools;
use memoize::memoize;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::{Captures, Regex};

fn main() {
    let mut input = include_str!("../../data/day21_test.txt")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let s_coords = input
        .iter()
        .enumerate()
        .find(|(_, line)| line.contains(&'S'))
        .map(|(row, line)| {
            (
                row,
                line.iter().enumerate().find(|(_, c)| **c == 'S').unwrap().0,
            )
        })
        .unwrap();

    let (height, width) = (input.len() as isize, input[0].len() as isize);

    input[s_coords.0][s_coords.1] = '.';

    let mut steps = vec![((s_coords.0 as isize, s_coords.1 as isize), 1usize)];

    for _ in 0..50 {
        // for (row, col) in &steps {
        //     input[*row][*col] = '.';
        // }
        let meow = steps
            .into_iter()
            .map(|((row, col), count)| ((row as isize, col as isize), count))
            .map(|((row, col), count)| {
                let next_steps = vec![
                    (row - 1, col),
                    (row + 1, col),
                    (row, col - 1),
                    (row, col + 1),
                ];

                (
                    next_steps
                        .into_iter()
                        // .map(|(row, col)| (row.rem_euclid(height), col.rem_euclid(width)))
                        // .map(|(row, col)| (row as usize, col as usize))
                        .filter(|(row, col)| {
                            input[row.rem_euclid(height) as usize][col.rem_euclid(width) as usize]
                                != '#'
                        })
                        .collect_vec(),
                    count,
                )
            });
        let mut next_steps = HashMap::new();
        for step in meow {
            for sstep in step.0 {
                let sstep = (sstep.0.rem_euclid(height), sstep.1.rem_euclid(width));
                if let Some(ss) = next_steps.get(&sstep) {
                    if *ss < step.1 {
                        next_steps.insert(sstep, step.1);
                    }
                } else {
                    next_steps.insert(sstep, step.1);
                }
            }
        }

        steps = next_steps.into_iter().collect();

        // for (row, col) in &steps {
        //     input[*row][*col] = 'O';
        // }

        // for line in &input {
        //     println!("{}", line.iter().collect::<String>())
        // }
    }

    dbg!(steps.iter().map(|s| s.1).sum::<usize>());
}
