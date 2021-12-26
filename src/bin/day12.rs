use std::{collections::HashSet, fmt::Display};

use Node::*;
fn main() {
    let edges = include_str!("../../data/day12.txt")
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(start, end)| [Node::from_str(start), Node::from_str(end)])
        .collect::<Vec<_>>();
    let solutions = solve(Start, &edges, HashSet::new(), HashSet::new(), true, vec![]);
    println!("{}", solutions);
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Start => "start",
                End => "end",
                Small(inner) | Large(inner) => inner,
            }
        )
    }
}
fn solve(
    current_node: Node,
    graph: &[[Node; 2]],
    mut visited: HashSet<Node>,
    mut visited_small: HashSet<Node>,
    mut allowed_to_visit_small: bool,
    mut tracker: Vec<Node>,
) -> usize {
    tracker.push(current_node);
    if current_node == End {
        // println!("{:?}", tracker);
        return 1;
    }
    match current_node {
        Start | End => {
            visited.insert(current_node);
        }
        Small(_) => {
            if !allowed_to_visit_small || visited_small.contains(&current_node) {
                visited.insert(current_node);
                allowed_to_visit_small = false;
            }
            visited_small.insert(current_node);
        }
        Large(_) => (),
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
        .filter(|node| {
            !visited.contains(node) && (allowed_to_visit_small || !visited_small.contains(node))
        })
        .map(|node| {
            solve(
                node,
                graph,
                visited.clone(),
                visited_small.clone(),
                allowed_to_visit_small,
                tracker.clone(),
            )
        })
        .sum()
}
