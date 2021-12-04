use std::iter::repeat;

fn main() {
    let input = include_str!("../../data/day3.txt").lines();
    let (gamma, epsilon) = generate_gamma_epsilon(input.clone());

    let (g, e) = (
        u32::from_str_radix(&gamma, 2).unwrap(),
        u32::from_str_radix(&epsilon, 2).unwrap(),
    );
    println!("gamma ({}) * epsilon ({}) = {}", g, e, g * e);

    let o2 = recursive_filter(input.clone().collect(), 0, '1');
    let co2 = recursive_filter(input.collect(), 0, '0');
    let (o, c) = (
        u32::from_str_radix(o2, 2).unwrap(),
        u32::from_str_radix(co2, 2).unwrap(),
    );
    println!("o2 * co2 = {}", o * c)
}

fn generate_gamma_epsilon<'a>(input: impl Iterator<Item = &'a str> + Clone) -> (String, String) {
    let length = input.clone().count();
    let popcount: Vec<usize> =
        input
            .fold(
                Box::new(repeat(0usize)) as Box<dyn std::iter::Iterator<Item = usize>>,
                |acc, current| {
                    Box::new(current.chars().zip(acc).map(|(current_char, acc)| {
                        acc + current_char.to_digit(2).unwrap() as usize
                    }))
                },
            )
            .collect();
    let gamma = popcount
        .iter()
        .map(|count| if count * 2 >= length { '1' } else { '0' })
        .collect::<String>();
    let epsilon = popcount
        .iter()
        .map(|count| if count * 2 >= length { '0' } else { '1' })
        .collect::<String>();

    (gamma, epsilon)
}

fn get_digit_prevalence(input: &[&str], index: usize) -> std::cmp::Ordering {
    let vertical_slice: Vec<char> = input
        .iter()
        .map(|line| line.chars().nth(index).unwrap())
        .collect();
    let ones = vertical_slice.iter().filter(|&&item| item == '1').count();
    let zeroes = vertical_slice.iter().filter(|&&item| item == '0').count();
    ones.cmp(&zeroes)
}

fn recursive_filter(input: Vec<&str>, index: usize, preferred_digit: char) -> &str {
    if input.len() == 1 || index >= input[0].chars().count() {
        return input[0];
    }
    let input = input
        .iter()
        .filter(|item| {
            let ordering = get_digit_prevalence(&input, index);
            let current = item.chars().nth(index).unwrap();
            use std::cmp::Ordering::{Equal, Greater, Less};
            match preferred_digit {
                '1' => match ordering {
                    Greater => current == '1',
                    Less => current == '0',
                    Equal => current == preferred_digit,
                },
                '0' => match ordering {
                    Greater => current == '0',
                    Less => current == '1',
                    Equal => current == preferred_digit,
                },
                _ => unreachable!(),
            }
        })
        .copied();
    recursive_filter(input.collect(), index + 1, preferred_digit)
}
