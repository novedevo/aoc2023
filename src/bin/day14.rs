use std::collections::HashMap;

fn main() {
    let (string, rules) = include_str!("../../data/day14.txt")
        .split_once("\n\n")
        .unwrap();
    let rules = rules
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .collect::<std::collections::HashMap<_, _>>();
    let pairs = string
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|chars| chars.iter().collect())
        .collect::<Vec<String>>();
    let mut pair_count: HashMap<String, usize> = HashMap::new();
    for pair in pairs {
        *pair_count.entry(pair).or_default() += 1
    }

    for _ in 0..40 {
        for pair in pair_count.clone().iter() {
            if let Some(&result) = rules.get(pair.0 as &str) {
                *pair_count
                    .entry(format!("{}{}", pair.0.chars().next().unwrap(), result))
                    .or_default() += pair.1;
                *pair_count
                    .entry(format!("{}{}", result, pair.0.chars().last().unwrap()))
                    .or_default() += pair.1;
                *pair_count.entry(pair.0.clone()).or_default() -= pair.1;
            }
        }
        pair_count = pair_count
            .iter()
            .filter(|(_key, &value)| value > 0)
            .map(|(key, &value)| (key.clone(), value))
            .collect();
        println!("{:?}", pair_count)
    }
    println!("{}", get_frequencies(pair_count))
}

fn get_frequencies(pair_count: HashMap<String, usize>) -> usize {
    let mut frequencies: HashMap<char, usize> = HashMap::new();
    for letter in pair_count {
        for char in letter.0.chars() {
            *frequencies.entry(char).or_default() += letter.1;
        }
    }
    let mut frequencies = frequencies
        .iter()
        .map(|(_key, &value)| value)
        .collect::<Vec<_>>();
    frequencies.sort_unstable();
    ((frequencies.iter().last().unwrap() - frequencies[0]) + 1) / 2
}
