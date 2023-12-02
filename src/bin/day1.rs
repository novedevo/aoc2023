use std::collections::HashMap;

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
    let map: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into();

    let numbers: String = map.keys().fold(
        String::with_capacity(map.keys().map(|k| k.len() + 1).sum()),
        |mut acc, elem| {
            if !acc.is_empty() {
                acc.push('|')
            }
            acc.push_str(elem);
            acc
        },
    );

    let first_pattern = format!("({})|(\\d)", numbers);
    let last_pattern = format!(".*(({})|(\\d))", numbers);

    return get_regex_capture(&first_pattern, input, &map)
        .iter()
        .zip(get_regex_capture(&last_pattern, input, &map))
        .map(|(d1, d2)| (d1 * 10) + d2)
        .sum();
}

fn get_regex_capture(pattern: &str, input: &str, map: &HashMap<&str, u32>) -> Vec<u32> {
    let regex = Regex::new(pattern).unwrap();
    return input
        .lines()
        .map(|s| regex.captures(s).unwrap().iter().flatten().last().unwrap())
        .map(|m| str_to_digit(m.as_str(), map))
        .collect();
}

fn str_to_digit(s: &str, map: &HashMap<&str, u32>) -> u32 {
    if s.len() == 1 {
        s.parse().unwrap()
    } else {
        *map.get(s).unwrap()
    }
}

fn main() {
    let input = include_str!("../../data/day1.txt");
    println!("sum of first and second digits is {}", sum_digits(input));
    println!(
        "tracking spelled out numbers, sum of first and second digits is {}",
        sum_numbers(input)
    );
}
