use itertools::Itertools;
use regex::Regex;

fn sum_digits(input: &str) -> i32 {
    input
        .lines()
        .map(|s| {
            s.chars()
                .filter(char::is_ascii_digit)
                .collect::<Vec<char>>()
        })
        .map(|v| {
            [v.first().unwrap(), v.last().unwrap()]
                .into_iter()
                .collect::<String>()
        })
        .map(|s| s.parse::<i32>().unwrap())
        .sum()
}

fn sum_numbers(input: &str) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let first_pattern = format!("({})|(\\d)", numbers.join("|"));
    let first_regex = Regex::new(dbg!(first_pattern).as_str()).unwrap();
    let first_digits = input.lines().map(|s| {
        first_regex
            .captures(s)
            .unwrap()
            .iter()
            .flatten()
            .last()
            .unwrap()
    });

    let last_pattern = format!(".*(({})|(\\d))", numbers.join("|"));
    let last_regex = Regex::new(dbg!(last_pattern).as_str()).unwrap();
    let last_digits = input.lines().map(|s| {
        last_regex
            .captures(s)
            .unwrap()
            .iter()
            .flatten()
            .last()
            .unwrap()
    });

    return first_digits
        .zip(last_digits)
        .map(|(fm, lm)| (str_to_digit(fm.as_str()), str_to_digit(lm.as_str())))
        .map(|(d1, d2)| (d1 * 10) + d2)
        .sum();
}

fn str_to_digit(s: &str) -> u32 {
    if s.len() == 1 {
        s.parse().unwrap()
    } else {
        match s {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = include_str!("../../data/day1.txt");
    // println!("sum of first and second digits is {}", sum_digits(input));
    println!(
        "tracking spelled out numbers, sum of first and second digits is {}",
        sum_numbers(input)
    );
}
