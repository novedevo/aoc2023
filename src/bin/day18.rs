use std::{
    collections::{HashMap, VecDeque},
    convert::Infallible,
    str::FromStr,
};

use enum_iterator::{all, Sequence};
use itertools::Itertools;
// use memoize::memoize;
// use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("../../data/day18.txt");
    let meow = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|(direction, meters, hexcode)| {
            (
                direction.parse::<Direction>().unwrap(),
                meters.parse::<isize>().unwrap(),
                hexcode
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap(),
            )
        })
        .collect_vec();
    let coords = meow
        .iter()
        .fold(vec![(0, 0)], |mut coords, (direction, meters, hexcode)| {
            let (initrow, initcol) = coords.last().unwrap();
            coords.push(match direction {
                Direction::Up => (initrow - meters, *initcol),
                Direction::Down => (initrow + meters, *initcol),
                Direction::Left => (*initrow, initcol - meters),
                Direction::Right => (*initrow, initcol + meters),
            });
            coords
        });

    dbg!(&coords);

    //https://www.mathopenref.com/coordpolygonarea2.html
    let mut area = 0isize;
    let mut j = coords.len() - 1;
    for i in 0..coords.len() {
        let current = coords[i];
        let previous = coords[j];

        area += (current.0 + previous.0) * (previous.1 - current.1);

        j = i
    }

    let catarea = area / 2;
    let perimeter = meow.iter().map(|(_, meters, _)| meters).sum::<isize>();
    dbg!(catarea, perimeter, catarea + perimeter / 2 + 1);

    // let mut digsite = vec![vec!['.'; 6000]; 6000];
}

//shoutout https://www.mathopenref.com/coordpolygonarea2.html
fn polygon_area(x: &[isize], y: &[isize]) -> isize {
    let mut area = 0; // Accumulates area
    let mut j = x.len() - 1;

    for i in 0..x.len() {
        area += (x[j] + x[i]) * (y[j] - y[i]);
        j = i; //j is previous vertex to i
    }
    area
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, Sequence)]
enum Direction {
    Down,
    Left,
    Up,
    Right,
}

impl FromStr for Direction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        Ok(match s {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => unreachable!(),
        })
    }
}
