use std::collections::HashSet;

use Node::*;
fn main() {
    let edges = include_str!("../../data/day12.txt")
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(start, end)| [Node::from_str(start), Node::from_str(end)])
        .collect::<Vec<_>>();
    let solutions = solve(Start, &edges, HashSet::new());
    println!("{}", solutions);
}
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Node {
    Start,
    End,
    Small(&'static str),
    Large(&'static str),
}
impl Node {
    fn from_str(str: &'static str) -> Self {
        if str == "start" {
            Start
        } else if str == "end" {
            End
        } else if str.to_uppercase() == str {
            Large(str)
        } else {
            Small(str)
        }
    }
}
fn solve(current_node: Node, graph: &[[Node; 2]], mut visited: HashSet<Node>) -> usize {
    if current_node == End {
        return 1;
    }
    let _ = match current_node {
        Start | End | Small(_) => visited.insert(current_node),
        Large(_) => false,
    };
    graph
        .iter()
        .filter_map(|edge| {
            if edge[0] == current_node {
                Some(edge[1])
            } else if edge[1] == current_node {
                Some(edge[0])
            } else {
                None
            }
        })
        .filter(|node| !visited.contains(node))
        .map(|node| solve(node, graph, visited.clone()))
        .sum()
}
