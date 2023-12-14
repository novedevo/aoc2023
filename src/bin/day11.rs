#![allow(unused)]
use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day11.txt");
    let mut starfield = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let empty_rows = starfield
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|c| *c == '.'))
        .map(|(i, _)| i)
        .collect_vec();
    let mut empty_cols = vec![];
    for col in 0..starfield[0].len() {
        let galactic = starfield.iter().map(|row| row[col]).all(|c| c == '.');
        if galactic {
            empty_cols.push(col);
        }
    }
    for row in &empty_rows {
        starfield[*row] = vec![':'; starfield[0].len()];
    }
    for col in &empty_cols {
        for row in &mut starfield {
            row[*col] = ':';
        }
    }

    let galaxies = galactate(&starfield);
    let diff = 10 - 1;

    let pairs = galaxies.iter().cartesian_product(galaxies.iter());
    let distances = pairs
        .filter(|(p1, p2)| p1 != p2)
        .map(|((row1, col1), (row2, col2))| {
            let temp_rowdiff = row2.abs_diff(*row1);
            let temp_coldiff = col2.abs_diff(*col1);
            let double_rows = empty_rows
                .iter()
                .filter(|&row| row > row1 && row < row2 || row > row2 && row < row1)
                .count();
            let double_cols = empty_cols
                .iter()
                .filter(|&col| col > col1 && col < col2 || col > col2 && col < col1)
                .count();
            (
                temp_rowdiff + double_rows * 999999,
                temp_coldiff + double_cols * 999999,
            )
        })
        .map(|(rowdiff, coldiff)| rowdiff + coldiff)
        .sum::<usize>();

    dbg!(distances / 2);
}

fn galactate(starfield: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut retval = vec![];

    for row in 0..starfield.len() {
        for col in 0..starfield[0].len() {
            if starfield[row][col] == '#' {
                retval.push((row, col))
            }
        }
    }
    retval
}
