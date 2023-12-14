use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day9.txt");
    let meow = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .collect_vec()
        })
        .map(extrapolate)
        .sum::<isize>();

    dbg!(meow);
}

fn extrapolate(history: Vec<isize>) -> isize {
    let mut diffs = vec![history];
    while !diffs.last().unwrap().iter().all(|v| *v == 0) {
        let next = diffs.last().unwrap().clone();
        let next = next.iter();
        let next = next.tuple_windows();
        let next = next.map(|(first, second)| second - first).collect();
        diffs.push(next);
    }
    calc_placeholders(diffs)
}

fn calc_placeholders(mut diffs: Vec<Vec<isize>>) -> isize {
    let mut index = diffs.len() - 1;
    while index > 0 {
        let delta = *diffs[index].first().unwrap();
        let first = *diffs[index - 1].first().unwrap();
        diffs[index - 1].insert(0, first - delta);
        index -= 1;
    }
    // dbg!(&diffs);
    *diffs.first().unwrap().first().unwrap()
}
