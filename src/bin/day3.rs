fn main() {
    let input = include_str!("../../data/day3_test.txt").lines();
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

    let (g, e) = (
        u32::from_str_radix(&gamma, 2).unwrap(),
        u32::from_str_radix(&epsilon, 2).unwrap(),
    );
    println!("gamma ({}) * epsilon ({}) = {}", g, e, g * e);

    let gamma = gamma.chars().collect::<Vec<char>>();
    let epsilon = epsilon.chars().collect::<Vec<char>>();

    // let mut i = 0;
    // let mut o2 = input.clone();

    // loop {
    let o2 = input
        .clone()
        .map(|line| -> usize {
            line.chars()
                .zip(gamma.iter())
                .zip(epsilon.iter())
                .map(|((current, &gamma_char), &epsilon_char)| -> bool {
                    let (most, least) = (current == gamma_char, current == epsilon_char);
                    if most && least {
                        current == '1'
                    } else {
                        most
                    }
                })
                .take_while(|&digit| digit)
                .count()
        })
        .enumerate()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;

    println!(
        "line {} of the test file is {}",
        o2,
        input.clone().nth(o2).unwrap()
    )
    // i += 1;
    // }
}
