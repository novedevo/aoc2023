use std::collections::HashSet;

fn part1() -> usize {
    include_str!("../../data/day8.txt")
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(_, output)| {
            output
                .split_whitespace()
                .map(|digit| match digit.chars().count() {
                    2 => 1,
                    4 => 1,
                    3 => 1,
                    7 => 1,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    part1();
    let input = include_str!("../../data/day8_test1.txt")
        .lines()
        .map(Transmission::from_str)
        .map(|t| t.parse());
}

struct Transmission<'a> {
    output: [&'a str; 4],
    signal_patterns: [&'a str; 10],
}
impl Transmission<'static> {
    fn from_str(s: &'static str) -> Self {
        let (signal_patterns, output) = s.split_once(" | ").unwrap();
        let (signal_patterns, output) = (
            signal_patterns.split_whitespace().collect::<Vec<_>>(),
            output.split_whitespace().collect::<Vec<_>>(),
        );
        Self {
            signal_patterns: signal_patterns.try_into().unwrap(),
            output: output.try_into().unwrap(),
        }
    }
    fn parse(&self) {
        let one = self.find_single_char(2);
        let four = self.find_single_char(4);
        let seven = self.find_single_char(3);
        let eight = self.find_single_char(7);

        let two_three_five: [&str; 3] = self.by_char_count(5);
        let six_nine_zero: [&str; 3] = self.by_char_count(6);

        let middle_three_segments = Self::common(&two_three_five);
        let topleft_bottomright = Self::common(&six_nine_zero);

        let top_segment = seven
            .chars()
            .find(|char| middle_three_segments.contains(char))
            .unwrap();

        let (a, b, c) = (two_three_five[0], two_three_five[1], two_three_five[2]);
        let commonalities = (
            Self::common(&[a, b]).len() > 3,
            Self::common(&[a, c]).len() > 3,
            Self::common(&[b, c]).len() > 3,
        );

        let three = match commonalities {
            (true, true, false) => a,
            (true, false, true) => b,
            (false, true, true) => c,
            _ => unreachable!(),
        };
        let six = *six_nine_zero
            .iter()
            .find(|&&digit| Self::common(&[digit, one]).len() == 1)
            .unwrap();
        let nine = *six_nine_zero
            .iter()
            .find(|&&digit| Self::common(&[digit, four]).len() == 4)
            .unwrap();
        let zero = *six_nine_zero
            .iter()
            .find(|&&digit| digit != six && digit != nine)
            .unwrap();
    }

    fn find_single_char(&self, count: usize) -> &str {
        self.signal_patterns
            .iter()
            .find(|sig| sig.chars().count() == count)
            .unwrap()
    }

    fn by_char_count<const N: usize>(&self, count: usize) -> [&str; N] {
        self.signal_patterns
            .iter()
            .filter(|sig| sig.chars().count() == count)
            .copied()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn common(s: &[&str]) -> HashSet<char> {
        s.iter()
            .map(|digit| digit.chars().collect::<HashSet<_>>())
            .reduce(|acc, cur| {
                acc.iter()
                    .filter(|char| cur.contains(char))
                    .copied()
                    .collect()
            })
            .unwrap()
    }
}
