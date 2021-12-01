fn count_increases(input: &str) -> i32 {
    input
        .lines()
        .map(|s| s.parse().unwrap())
        .fold((0, i32::MAX), |(sum, prev), depth| {
            (if depth > prev { sum + 1 } else { sum }, depth)
        })
        .0
}

fn count_increases_sliding_window(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.parse().unwrap())
        .fold(
            (0, (u16::MAX as u32, u16::MAX as u32, u16::MAX as u32)),
            |(sum, (ante, pen, ult)), depth| {
                (
                    if depth + ult + pen > ult + pen + ante {
                        sum + 1
                    } else {
                        sum
                    },
                    (pen, ult, depth),
                )
            },
        )
        .0
}

fn main() {
    let input = include_str!("../../data/day1.txt");
    println!(
        "using simple comparison, depth increased {} times",
        count_increases(input)
    );
    println!(
        "using a sliding window, depth increased {} times",
        count_increases_sliding_window(input)
    );
}
