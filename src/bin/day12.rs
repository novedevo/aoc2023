use std::collections::HashSet;

use Node::*;
fn main() {
    let edges = include_str!("../../data/day12_test1.txt")
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(start, end)| [Node::from_str(start), Node::from_str(end)])
        .collect::<Vec<_>>();
    let mut stack = vec![Start];
    // let mut paths = vec![];
    let mut visited = HashSet::new();
    let mut solutions = 0;
    while let Some(current_node) = stack.pop() {
        for next_node in edges.iter().filter_map(|edge| {
            if edge[0] == current_node {
                Some(edge[1])
            } else if edge[1] == current_node {
                Some(edge[0])
            } else {
                None
            }
        }) {
            dbg!(next_node);
            if next_node == End {
                solutions += 1;
            }
            if !visited.contains(&next_node) {
                stack.push(next_node);
                let _ = match next_node {
                    Start | End | Small(_) => visited.insert(next_node),
                    Large(_) => false,
                };
            }
        }
    }
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
