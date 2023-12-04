use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day3.txt");
    let array = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut sum: u32 = 0;
    for r in 0..array.len() {
        for c in 0..array[r].len() {
            sum += get_ratio(&array, r, c)
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

fn get_ratio(array: &[Vec<char>], r: usize, c: usize) -> u32 {
    if array[r][c] != '*' {
        return 0;
    }
    let numbers = get_neighbours(array[0].len(), array.len(), r, c)
        .iter()
        .filter_map(|(row, column)| get_part_number(array, *row, *column))
        .collect::<HashSet<_>>();

    if numbers.len() == 2 {
        numbers
            .iter()
            .map(|(number, _row, _column)| number)
            .product::<u32>()
    } else {
        0
    }
}

fn get_part_number(array: &[Vec<char>], r: usize, c: usize) -> Option<(u32, usize, usize)> {
    let entry = array[r][c];
    if !entry.is_ascii_digit() {
        return None;
    }
    let mut left_c = c;
    let mut right_c = c;
    let mut chars = vec![entry];
    while left_c != 0 {
        left_c -= 1;
        let entry = array[r][left_c];
        if !entry.is_ascii_digit() {
            break;
        }
        chars.insert(0, entry);
    }
    while right_c != array.len() - 1 {
        right_c += 1;
        let entry = array[r][right_c];
        if !entry.is_ascii_digit() {
            break;
        }
        chars.push(entry);
    }
    dbg!(left_c, right_c, c, r);
    Some((
        dbg!(chars.iter().collect::<String>()).parse().unwrap(),
        r,
        left_c,
    ))
}

// fn is_symbol(array: &[Vec<char>], row: usize, column: usize) -> bool {
//     let entry = array[row][column];
//     !entry.is_ascii_digit() && entry != '.'
// }
