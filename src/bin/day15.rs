#![allow(unused)]
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use memoize::memoize;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../../data/day15.txt");
    let steps = input
        .split(',')
        .map(|step| match step.split_once('=') {
            Some((label, focal_length)) => (label, Some(focal_length.parse::<usize>().unwrap())),
            None => (step.strip_suffix('-').unwrap(), None),
        })
        .map(|(label, focal_length)| (hash(label), label, focal_length));
    let mut boxes = vec![vec![]; 256];
    for (box_num, label, focal_length) in steps {
        let boxxx = &mut boxes[box_num];
        if let Some(fl) = focal_length {
            if let Some(lens) = boxxx
                .iter_mut()
                .find(|lens: &&mut (&str, usize)| lens.0 == label)
            {
                *lens = (label, fl)
            } else {
                boxxx.push((label, fl));
            }
        } else if let Some((lens_index, _)) = boxxx
            .iter()
            .enumerate()
            .find(|(idx, (llabel, focall_llength))| llabel == &label)
        {
            boxxx.remove(lens_index);
        }
    }

    let sum = boxes
        .iter()
        .enumerate()
        .flat_map(|(idx, boxx)| {
            boxx.iter()
                .enumerate()
                .map(move |(iidx, &(_, focal_length))| (idx + 1) * (iidx + 1) * focal_length)
        })
        .collect_vec();
    let sumsum = sum.iter().sum::<usize>();

    dbg!(boxes, sum, sumsum);
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .map(|c| c as usize)
        .fold(0, |acc, c| ((acc + c) * 17) % 256)
}
