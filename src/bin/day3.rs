fn main() {
    let input = include_str!("../../data/day3_test.txt").lines();
    let (gamma, epsilon) = generate_gamma_epsilon(input.clone());

    let (g, e) = (
        u32::from_str_radix(&gamma, 2).unwrap(),
        u32::from_str_radix(&epsilon, 2).unwrap(),
    );
    println!("gamma ({}) * epsilon ({}) = {}", g, e, g * e);

    let _o2 = recursive_filter(input, 0, '1');
}

fn generate_gamma_epsilon<'a>(input: impl Iterator<Item = &'a str> + Clone) -> (String, String) {
    let length = input.clone().count();
    let mut i = 0;
    let mut popcount = [0; 12];
    'outer: loop {
        for line in input.clone() {
            if let Some(digit) = line.chars().nth(i) {
                if digit == '1' {
                    popcount[i] += 1
                }
            } else {
                break 'outer;
            }
        }
        i += 1;
    }
    let gamma = popcount
        .get(..i)
        .unwrap()
        .iter()
        .map(|count| if count * 2 >= length { '1' } else { '0' })
        .collect::<String>();
    let epsilon = popcount
        .get(..i)
        .unwrap()
        .iter()
        .map(|count| if count * 2 >= length { '0' } else { '1' })
        .collect::<String>();

    (gamma, epsilon)
}

fn recursive_filter<'a>(
    mut input: impl Iterator<Item = &'a str> + Clone,
    index: usize,
    preferred_digit: char,
) -> &'a str {
    let (gamma, epsilon) = generate_gamma_epsilon(input.clone());
    if index >= gamma.chars().count() {
        return input.next().unwrap();
    }
    let input = input.filter(|item| {
        let (current, (gamma_char, epsilon_char)) = item
            .chars()
            .zip(gamma.chars().zip(epsilon.chars()))
            .nth(index)
            .unwrap();

        let (most, least) = (current == gamma_char, current == epsilon_char);
        if most && least {
            current == preferred_digit
        } else {
            most
        }
    });
    recursive_filter(input, index, preferred_digit)
}
