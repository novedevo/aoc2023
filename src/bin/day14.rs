use std::collections::HashMap;

fn main() {
    let (string, rules) = include_str!("../../data/day14.txt")
        .split_once("\n\n")
        .unwrap();
    let rules = rules
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .collect::<std::collections::HashMap<_, _>>();
    let mut current_string = string.to_string();
    for _ in 0..20 {
        let mut next_string = String::new();
        for elem in current_string.chars().collect::<Vec<_>>().windows(2) {
            let elem_string = elem.iter().collect::<String>();
            if let Some(&result) = rules.get(&elem_string as &str) {
                next_string += &format!("{}{}", elem[0], result);
            } else {
                next_string += &elem_string
            }
        }
        current_string = next_string + &current_string.chars().last().unwrap().to_string();
    }
    let mut frequencies: HashMap<char, usize> = HashMap::new();
    for letter in current_string.chars() {
        *frequencies.entry(letter).or_default() += 1usize;
    }
    let mut frequencies = frequencies
        .iter()
        .map(|(key, &value)| value)
        .collect::<Vec<_>>();
    frequencies.sort();
    println!("{:?}", frequencies.iter().last().unwrap() - frequencies[0])
}
