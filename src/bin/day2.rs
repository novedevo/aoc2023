use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day2.txt");
    let sum: u32 = input
        .lines()
        .map(|line| line.strip_prefix("Game ").unwrap())
        .map(|line| {
            let (id, data) = line.split_once(": ").unwrap();
            let id: u32 = id.parse().unwrap();
            let handfuls = data.split("; ").map(|handful| {
                handful
                    .split(", ")
                    .map(|cubecount| cubecount.split_once(' ').unwrap())
                    .map(|(number, color)| (number.parse::<u32>().unwrap(), color))
                    .collect_vec()
            });
            (id, handfuls.collect_vec())
        })
        .map(|(_, handfuls)| {
            handfuls
                .iter()
                .fold((0u32, 0u32, 0u32), |(r, g, b), handful| {
                    handful
                        .iter()
                        .fold((r, g, b), |(r, g, b), (number, color)| match *color {
                            "red" => (r.max(*number), g, b),
                            "green" => (r, g.max(*number), b),
                            "blue" => (r, g, b.max(*number)),
                            _ => unreachable!(),
                        })
                })
        })
        .map(|(r, g, b)| r * g * b)
        .sum();
    dbg!(sum);
}
