fn main() {
    let input = include_str!("../../data/day10.txt")
        .lines()
        .map(|line| {
            let chars = line.chars();
            let mut stack = vec![];
            for char in chars {
                if "{[(<".contains(char) {
                    stack.push(char)
                } else {
                    let a = match (char, *stack.last().unwrap()) {
                        ('}', '{') => stack.pop(),
                        (')', '(') => stack.pop(),
                        (']', '[') => stack.pop(),
                        ('>', '<') => stack.pop(),
                        _ => None,
                    };
                    if a.is_none() {
                        return match char {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => unreachable!(),
                        };
                    }
                }
            }
            0
        })
        .sum::<u32>();
    dbg!(input);
}
