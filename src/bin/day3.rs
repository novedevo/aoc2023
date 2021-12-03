use std::iter::repeat;

fn main() {
    let input = include_str!("../../data/day3_test.txt").lines();
    let (gamma, epsilon) = generate_gamma_epsilon(input.clone());

    let (g, e) = (
        u32::from_str_radix(&gamma, 2).unwrap(),
        u32::from_str_radix(&epsilon, 2).unwrap(),
    );
    println!("gamma ({}) * epsilon ({}) = {}", g, e, g * e);

    let _o2 = recursive_filter(input.clone().collect(), 0, '1');
    // let co2 = recursive_filter(input.collect(), 0, '0');
    // println!("{} {}", _o2, co2)
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

fn recursive_filter(input: Vec<&str>, index: usize, preferred_digit: char) -> &str {
    let input = input.into_iter();
    let (gamma, epsilon) = dbg!(generate_gamma_epsilon(input.clone()));
    if index >= gamma.chars().count() {
        return input.clone().next().unwrap();
    }
    let input = input.filter(|item| {
        let (current, (gamma_char, epsilon_char)) = item
            .chars()
            .zip(gamma.chars().zip(epsilon.chars()))
            .nth(index)
            .unwrap();

        let (most, least) = (current == gamma_char, current == epsilon_char);
        if most && least {
            dbg!(current == preferred_digit)
        } else {
            most
        }
    });
    recursive_filter(input.collect(), index + 1, preferred_digit)
}
