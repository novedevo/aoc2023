fn main() {
    let (dots, folds) = include_str!("../../data/day13.txt")
        .split_once("\n\n")
        .unwrap();
    let mut dots = dots
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Vec<(usize, usize)>>();
    let folds = folds.lines().map(|line| {
        let coord = line
            .strip_prefix("fold along ")
            .unwrap()
            .split_once('=')
            .unwrap();
        (coord.0 == "x", coord.1.parse::<usize>().unwrap())
    });

    // println!("{:?}\n{}", dots, dots.len());
    for (vertical_line, coord) in folds {
        if vertical_line {
            for (x, _) in dots.iter_mut().filter(|dot| dot.0 > coord) {
                *x = coord - (*x - coord);
            }
        } else {
            for (_, y) in dots.iter_mut().filter(|dot| dot.1 > coord) {
                *y = coord - (*y - coord);
            }
        }
        dots.sort_unstable();
        dots.dedup();
        println!("{}", dots.len());
    }
    let mut output = [[1u8; 40]; 6];
    for dot in dots {
        output[dot.1][dot.0] = 8;
    }
    for line in output {
        println!("{:?}", line)
    }
}
