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
        .filter(|(_, handfuls)| {
            handfuls.iter().all(|handful| {
                handful.iter().all(|(number, color)| {
                    *number
                        <= match *color {
                            "red" => 12,
                            "blue" => 14,
                            "green" => 13,
                            _ => unreachable!(),
                        }
                })
            })
        })
        .map(|(id, _)| id)
        .sum();
    dbg!(sum);
}
