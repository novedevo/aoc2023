fn main() {
    let input = include_str!("../../data/day9.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    dbg!(get_low_points(input));
}
fn get_low_points(map: Vec<Vec<u32>>) -> u32 {
    let mut true_risk = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            let risk = map[row][col];
            let (top, bottom, left, right) = (
                row == 0 || map[row - 1][col] > risk,
                row == map.len() - 1 || map[row + 1][col] > risk,
                col == 0 || map[row][col - 1] > risk,
                col == map[0].len() - 1 || map[row][col + 1] > risk,
            );
            if top && bottom && left && right {
                true_risk += dbg!(risk) + 1;
            }
        }
    }
    true_risk
}
