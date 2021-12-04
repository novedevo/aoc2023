#[derive(Clone, Default, Debug)]
struct Board {
    lines: Vec<Vec<u32>>,
    called: [[bool; 5]; 5],
}

impl Board {
    pub fn new(lines: Vec<Vec<u32>>) -> Self {
        Self {
            lines,
            called: [[false; 5]; 5],
        }
    }
    pub fn score(&self) -> u32 {
        let mut retvals = vec![];
        for row in 0..5 {
            for col in 0..5 {
                if !self.called[row][col] {
                    retvals.push(self.lines[row][col]);
                }
            }
        }
        retvals.into_iter().reduce(|acc, curr| acc + curr).unwrap()
    }
    pub fn call(&mut self, num: u32) {
        for row in 0..5 {
            for column in 0..5 {
                if self.lines[row][column] == num {
                    self.called[row][column] = true;
                }
            }
        }
    }
    pub fn check(&self) -> bool {
        for line in &self.called {
            if line.iter().all(|&x| x) {
                return true;
            }
        }
        for column in 0..5 {
            if self.called.iter().map(|line| line[column]).all(|x| x) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let input = include_str!("../../data/day4.txt");
    let (drawings, boards) = input.split_once("\n\n").unwrap();
    let boards = boards
        .split("\n\n")
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|digit| digit.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .map(Board::new)
        .collect::<Vec<Board>>();

    let drawings = drawings
        .split(',')
        .map(|digit| digit.parse::<u32>().unwrap())
        .collect();

    let (winning_board, draw) = run_bingo(boards, drawings);
    println!("{}", winning_board.score() * draw)
}

fn run_bingo(mut boards: Vec<Board>, drawings: Vec<u32>) -> (Board, u32) {
    for draw in drawings {
        let mut winners = vec![];
        for i in 0..boards.len() {
            boards[i].call(draw);
            if boards[i].check() {
                winners.push(i)
            }
        }
        winners.sort_unstable();
        // dbg!(&winners);

        for elem in winners.iter().rev() {
            println!("removing index {} from the {} boards", elem, boards.len());
            let mut removed = boards.remove(*elem);
            if boards.is_empty() {
                // removed.uncall(draw);
                return dbg!(removed, draw);
            }
        }
    }
    unreachable!()
}
