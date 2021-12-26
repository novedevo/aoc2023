use petgraph::algo::dijkstra;
use petgraph::graph::Graph;

fn main() {
    let mut input = [
        include_str!("../../data/day15.txt")
            .lines()
            .map(|line| {
                [
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap())
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 1)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 2)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 3)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 4)
                        .collect::<Vec<_>>(),
                ]
                .concat()
            })
            .collect::<Vec<_>>(),
        include_str!("../../data/day15.txt")
            .lines()
            .map(|line| {
                [
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 1)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 2)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 3)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 4)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 5)
                        .collect::<Vec<_>>(),
                ]
                .concat()
            })
            .collect::<Vec<_>>(),
        include_str!("../../data/day15.txt")
            .lines()
            .map(|line| {
                [
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 2)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 3)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 4)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 5)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 6)
                        .collect::<Vec<_>>(),
                ]
                .concat()
            })
            .collect::<Vec<_>>(),
        include_str!("../../data/day15.txt")
            .lines()
            .map(|line| {
                [
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 3)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 4)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 5)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 6)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 7)
                        .collect::<Vec<_>>(),
                ]
                .concat()
            })
            .collect::<Vec<_>>(),
        include_str!("../../data/day15.txt")
            .lines()
            .map(|line| {
                [
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 4)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 5)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 6)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 7)
                        .collect::<Vec<_>>(),
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() + 8)
                        .collect::<Vec<_>>(),
                ]
                .concat()
            })
            .collect::<Vec<_>>(),
    ]
    .concat();
    for row in input.iter_mut() {
        for col in row {
            if *col > 9 {
                *col -= 9
            }
        }
    }
    let mut graph: Graph<u32, u32, petgraph::Directed> = Graph::default();
    let mut indices = vec![];
    for row in 0..input.len() {
        indices.push(vec![]);
        for col in 0..input[0].len() {
            indices[row].push(graph.add_node(input[row][col]));
        }
    }
    let length = input.len();
    for row in 0..length {
        for col in 0..length {
            for (n_row, n_col) in get_neighbours(row, col, length - 1) {
                graph.add_edge(
                    indices[row][col],
                    indices[n_row][n_col],
                    input[n_row][n_col],
                );
            }
        }
    }
    // println!("{:?}", graph);
    let cost = *dijkstra(
        &graph,
        indices[0][0],
        Some(indices[length - 1][length - 1]),
        |a| *a.weight(),
    )
    .get(&indices[length - 1][length - 1])
    .unwrap();

    println!("{}", cost)
}
fn get_neighbours(row: usize, col: usize, length: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    let (top, bottom, left, right) = (row > 0, row < length, col > 0, col < length);
    if top {
        neighbours.push((row - 1, col));
        // if left {
        //     neighbours.push((row - 1, col - 1));
        // }
        // if right {
        //     neighbours.push((row - 1, col + 1));
        // }
    }
    if bottom {
        neighbours.push((row + 1, col));
        // if left {
        //     neighbours.push((row + 1, col - 1));
        // }
        // if right {
        //     neighbours.push((row + 1, col + 1));
        // }
    }
    if left {
        neighbours.push((row, col - 1));
    }
    if right {
        neighbours.push((row, col + 1));
    }
    neighbours
}
