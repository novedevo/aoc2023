//371 is too low; it's not median
//ugh, i guess i'm just gonna brute force it.
//there's got to be a better way...

//93123016 is also too low, lol
fn main() {
    let crabs: Vec<i64> = include_str!("../../data/day7.txt")
        .split(',')
        .map(|crab| crab.parse().unwrap())
        .collect();
    let &max = crabs.iter().max().unwrap();
    let costs = (0..=max)
        .map(|alignment| fuel_cost(&crabs, alignment))
        .collect::<Vec<_>>();
    // println!("{:?}", costs);
    let optimal_alignment = costs
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.cmp(b.1))
        .unwrap();

    println!("{:?}", optimal_alignment)
}

fn fuel_cost(crabs: &[i64], alignment: i64) -> i64 {
    crabs.iter().fold(0, |acc, cur| {
        let n = (alignment - cur).abs();
        acc + n * (n + 1) / 2
    })
}
