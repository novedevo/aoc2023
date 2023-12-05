use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day5.txt");

    let mut sections = input.split("\n\n");

    let chunks = sections
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .chunks(2);
    let seeds = chunks
        .into_iter()
        .map(|chunk| chunk.collect_tuple().unwrap())
        .map(|(start, length)| start..start + length)
        .sorted_by(|r1, r2| r1.start.cmp(&r2.start))
        .collect_vec();

    dbg!(&seeds);

    panic!("{}", seeds.len());

    // let seedtosoil = sections.next().map(parse_ranges).unwrap();
    // let soiltofertilizer = sections.next().map(parse_ranges).unwrap();
    // let fertilizertowater = sections.next().map(parse_ranges).unwrap();
    // let watertolight = sections.next().map(parse_ranges).unwrap();
    // let lighttotemperature = sections.next().map(parse_ranges).unwrap();
    // let temperaturetohumidity = sections.next().map(parse_ranges).unwrap();
    // let humiditytolocation = sections.next().map(parse_ranges).unwrap();

    // let m = seeds
    //     .map(|seed| map_seed(seed, &seedtosoil))
    //     .map(|seed| map_seed(seed, &soiltofertilizer))
    //     .map(|seed| map_seed(seed, &fertilizertowater))
    //     .map(|seed| map_seed(seed, &watertolight))
    //     .map(|seed| map_seed(seed, &lighttotemperature))
    //     .map(|seed| map_seed(seed, &temperaturetohumidity))
    //     .map(|seed| map_seed(seed, &humiditytolocation))
    //     .min();
    // dbg!(m);
}

fn parse_ranges(lines: &str) -> Vec<(Range<usize>, Range<usize>)> {
    lines
        .split_once(":\n")
        .unwrap()
        .1
        .lines()
        .map(get_ranges)
        .collect()
}

fn get_ranges(line: &str) -> (Range<usize>, Range<usize>) {
    let numbers = line
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    (
        numbers[0]..numbers[0] + numbers[2],
        numbers[1]..numbers[1] + numbers[2],
    )
}

fn map_seed(seed: usize, ranges: &[(Range<usize>, Range<usize>)]) -> usize {
    for (destination, source) in ranges {
        if source.contains(&seed) {
            return (seed as isize + (destination.start as isize - source.start as isize)) as usize;
        } else {
            continue;
        }
    }
    seed
}
