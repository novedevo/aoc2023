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

// fn count_increases_sliding_window(input: &str) -> u32 {
//     input
//         .lines()
//         .map(|s| s.parse().unwrap())
//         .fold(
//             (0, (u16::MAX as u32, u16::MAX as u32, u16::MAX as u32)),
//             |(sum, (ante, pen, ult)), depth| {
//                 (
//                     if depth + ult + pen > ult + pen + ante {
//                         sum + 1
//                     } else {
//                         sum
//                     },
//                     (pen, ult, depth),
//                 )
//             },
//         )
//         .0
// }

fn main() {
    let input = include_str!("../../data/day1.txt");
    println!("sum of first and second digits is {}", sum_digits(input));
    // println!(
    //     "using a sliding window, depth increased {} times",
    //     count_increases_sliding_window(input)
    // );
}
