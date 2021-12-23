use std::collections::HashMap;

fn main() {
    let input = include_str!("../../data/day6.txt")
        .split(',')
        .map(|fish| fish.parse::<u8>().unwrap());
    let mut fish_counter = 0usize;
    let mut cache = HashMap::new();
    for fish in input {
        fish_counter += simulate_fish(fish, 80, &mut cache);
    }
    println!("{}", fish_counter);
}

fn simulate_fish(timer: u8, days_remaining: u16, cache: &mut HashMap<(u8, u16), usize>) -> usize {
    let retval = if days_remaining == 0 {
        1
    } else if let Some(cached) = cache.get(&(timer, days_remaining)) {
        *cached
    } else if timer == 0 {
        simulate_fish(8, days_remaining - 1, cache) + simulate_fish(6, days_remaining - 1, cache)
    } else {
        simulate_fish(timer - 1, days_remaining - 1, cache)
    };
    cache.insert((timer, days_remaining), retval);
    retval
}
