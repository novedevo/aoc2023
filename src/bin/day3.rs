use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day3.txt");
    let array = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut sum: u32 = 0;

    let mut running_str = String::with_capacity(10);
    let mut adj_symbol = false;
    for r in 0..array.len() {
        if !running_str.is_empty() {
            if adj_symbol {
                sum += dbg!(&running_str).parse::<u32>().unwrap();
                adj_symbol = false;
            }
            running_str.clear();
        }
        adj_symbol = false;

        for c in 0..array[r].len() {
            let entry = array[r][c];
            if !entry.is_ascii_digit() {
                if !running_str.is_empty() {
                    if adj_symbol {
                        sum += dbg!(&running_str).parse::<u32>().unwrap();
                        adj_symbol = false;
                    }
                    running_str.clear();
                }
            } else {
                running_str.push(entry);
                adj_symbol = adj_symbol
                    || get_neighbours(array[r].len(), array.len(), r, c)
                        .iter()
                        .map(|(row, column)| is_symbol(&array, *row, *column))
                        .any(|b| b);
            }
        }
    }
    dbg!(sum);
}

fn get_neighbours(width: usize, height: usize, r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut retval = vec![];

    if c != 0 {
        retval.push((r, c - 1))
    }
    if r != 0 {
        retval.push((r - 1, c))
    }
    if r != 0 && c != 0 {
        retval.push((r - 1, c - 1))
    }

    if r != height - 1 {
        retval.push((r + 1, c))
    }
    if c != width - 1 {
        retval.push((r, c + 1))
    }
    if r != height - 1 && c != width - 1 {
        retval.push((r + 1, c + 1))
    }

    if r != height - 1 && c != 0 {
        retval.push((r + 1, c - 1))
    }
    if r != 0 && c != width - 1 {
        retval.push((r - 1, c + 1))
    }

    retval
}

fn print_with_context(array: &[Vec<char>], row: usize, column: usize, length: usize) {}

fn is_symbol(array: &[Vec<char>], row: usize, column: usize) -> bool {
    let entry = array[row][column];
    !entry.is_ascii_digit() && entry != '.'
}
