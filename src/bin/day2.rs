use std::{convert::Infallible, str::FromStr};

fn main() {
    let input = include_str!("../../data/day2.txt")
        .lines()
        .map(|line| Direction::from_str(line).unwrap());
    let naive_position = input
        .clone()
        .fold((0, 0), |(x, y), direction| match direction {
            Direction::Forward(distance) => (x + distance, y),
            Direction::Down(distance) => (x, y + distance),
            Direction::Up(distance) => (x, y - distance),
        });

    let position = input.fold((0, 0, 0), |(x, y, aim), direction| match direction {
        Direction::Forward(distance) => (x + distance, y + aim * distance, aim),
        Direction::Down(distance) => (x, y, aim + distance),
        Direction::Up(distance) => (x, y, aim - distance),
    });

    println!(
        "Naive position is {}, {}; this is {} when multiplied",
        naive_position.0,
        naive_position.1,
        naive_position.0 * naive_position.1
    );

    println!(
        "correct position is {}, {}; this is {} when multiplied",
        position.0,
        position.1,
        position.0 * position.1
    );
}

#[derive(Clone, Copy)]
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Direction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_once(' ').unwrap();
        Ok(match direction {
            "forward" => Direction::Forward(distance.parse().unwrap()),
            "down" => Direction::Down(distance.parse().unwrap()),
            "up" => Direction::Up(distance.parse().unwrap()),
            &_ => unreachable!(),
        })
    }
}
