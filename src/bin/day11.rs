use std::fmt::Display;

fn main() {
    let mut octopi = Octopi::new(
        include_str!("../../data/day11.txt")
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    println!("first:\n{}", octopi);
    for i in 0..1000 {
        println!("{}", i + 1);
        octopi.step();
    }
    println!("after:\n{}", octopi);
}

struct Octopi {
    octopi: Vec<Vec<u32>>,
    flash_count: u32,
    flashed: [[bool; 10]; 10],
}
impl Octopi {
    fn new(octopi: Vec<Vec<u32>>) -> Self {
        Self {
            octopi,
            flash_count: 0,
            flashed: [[false; 10]; 10],
        }
    }
    fn step(&mut self) {
        self.inc();
        self.flashed = [[false; 10]; 10];
        let old_flash_count = self.flash_count;
        for row in 0..10 {
            for col in 0..10 {
                self.flash(row, col);
            }
        }
        for row in 0..10 {
            for col in 0..10 {
                if self.octopi[row][col] > 9 {
                    self.octopi[row][col] = 0;
                }
            }
        }
        if self.flash_count - old_flash_count == 100 {
            panic!()
        }
    }
    fn inc(&mut self) {
        for row in self.octopi.iter_mut() {
            for col in row {
                *col += 1;
            }
        }
    }
    fn flash(&mut self, row: usize, col: usize) {
        if self.octopi[row][col] <= 9 || self.flashed[row][col] {
            return;
        }
        self.flash_count += 1;
        self.flashed[row][col] = true;
        for (n_row, n_col) in get_neighbours(row, col) {
            self.octopi[n_row][n_col] += 1;
            self.flash(n_row, n_col);
        }
    }
}
impl Display for Octopi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nflashed {} times",
            self.octopi
                .iter()
                .map(|row| row
                    .iter()
                    .map(|octopus| octopus.to_string())
                    .collect::<String>())
                .collect::<Vec<_>>()
                .join("\n"),
            self.flash_count
        )
    }
}
fn get_neighbours(row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    let (top, bottom, left, right) = (row > 0, row < 9, col > 0, col < 9);
    if top {
        neighbours.push((row - 1, col));
        if left {
            neighbours.push((row - 1, col - 1));
        }
        if right {
            neighbours.push((row - 1, col + 1));
        }
    }
    if bottom {
        neighbours.push((row + 1, col));
        if left {
            neighbours.push((row + 1, col - 1));
        }
        if right {
            neighbours.push((row + 1, col + 1));
        }
    }
    if left {
        neighbours.push((row, col - 1));
    }
    if right {
        neighbours.push((row, col + 1));
    }
    neighbours
}
