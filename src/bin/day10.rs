fn main() {
    let mut input = include_str!("../../data/day10.txt")
        .lines()
        .filter_map(|line| {
            let chars = line.chars();
            let mut stack = vec![];
            for char in chars {
                if "{[(<".contains(char) {
                    stack.push(char)
                } else {
                    match (char, *stack.last().unwrap()) {
                        ('}', '{') => stack.pop(),
                        (')', '(') => stack.pop(),
                        (']', '[') => stack.pop(),
                        ('>', '<') => stack.pop(),
                        _ => return None,
                    };
                }
            }
            Some(stack)
        })
        .map(|mut stack| {
            stack.reverse();
            stack
                .iter()
                .map(|char| match char {
                    '{' => 3,
                    '(' => 1,
                    '[' => 2,
                    '<' => 4,
                    _ => unreachable!(),
                })
                .fold(0, |init, curr| init * 5 + curr)
        })
        .collect::<Vec<u64>>();
    input.sort_unstable();
    dbg!(input[input.len() / 2]);
}
