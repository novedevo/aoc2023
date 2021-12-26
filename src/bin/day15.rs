use petgraph::algo::dijkstra;
use petgraph::matrix_graph::MatrixGraph;

fn main() {
    let input = include_str!("../../data/day15_test.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut graph = MatrixGraph::new_undirected();
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
            for (n_row, n_col) in get_neighbours(row, col, length) {
                graph.add_edge(indices[row][col], indices[n_row][n_col], 0);
            }
        }
    }
    let cost = *dijkstra(
        &graph,
        indices[0][0],
        Some(indices[length - 1][length - 1]),
        |_| 0,
    )
    .get(&indices[length][length])
    .unwrap();

    println!("{}", cost)
}
fn get_neighbours(row: usize, col: usize, length: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    let (top, bottom, left, right) = (row > 0, row < length, col > 0, col < length);
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
