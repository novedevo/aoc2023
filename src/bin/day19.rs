#![allow(unused)]
use std::{
    collections::{HashMap, VecDeque},
    convert::Infallible,
    ops::{Index, IndexMut, Range},
    str::FromStr,
};

use enum_iterator::{all, Sequence};
use itertools::Itertools;
use memoize::memoize;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::{Captures, Regex};

fn main() {
    let input = include_str!("../../data/day19_test.txt");
    let (workflows, part_ratings) = input.split_once("\n\n").unwrap();

    let rule_regex = Regex::new(r"([xmas])(<|>)(\d+):(.+)").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| line.split_once('{').unwrap())
        .map(|(name, rules)| {
            (
                name,
                rules
                    .strip_suffix('}')
                    .unwrap()
                    .split(',')
                    .map(|rule| {
                        (
                            rule,
                            rule_regex.captures(rule).and_then(|captures| {
                                captures
                                    .iter()
                                    .skip(1)
                                    .flatten()
                                    .map(|matc| matc.as_str().to_owned())
                                    .collect_tuple()
                            }),
                        )
                    })
                    .map(|(rule, option)| {
                        if let Some((category, comparator, value, next)) = option {
                            Rule::ConditionalNext(
                                category.chars().next().unwrap(),
                                comparator.chars().next().unwrap(),
                                value.parse().unwrap(),
                                next,
                            )
                        } else {
                            match rule {
                                "A" => Rule::Accept,
                                "R" => Rule::Reject,
                                other => Rule::InstantJump(other),
                            }
                        }
                    })
                    .collect_vec(),
            )
        })
        .collect::<HashMap<&'static str, _>>();

    //had the idea to run constant propagation until a fixed point, but that would only reduce complexity by about half i think

    // let reg = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    //part2 idea: make a tree of all possible paths. shouldn't be too many since this is a dag and doesn't really have exponentials like that.

    dbg!(count(
        &workflows,
        "in",
        [1..4001, 1..4001, 1..4001, 1..4001]
    ));
}

fn count(
    workflows: &HashMap<&'static str, Vec<Rule>>,
    current: &'static str,
    ranges: [Range<usize>; 4],
) -> usize {
    for rule in workflows[current].iter() {
        let valid_ranges = match rule {
            Rule::Accept => todo!(),
            Rule::Reject => [0..0, 0..0, 0..0, 0..0],
            Rule::ConditionalNext(_, _, _, _) => todo!(),
            Rule::InstantJump(_) => todo!(),
        };
    }
    todo!()
}

fn accepts(workflows: &HashMap<&'static str, Vec<Rule>>, part: &Part) -> bool {
    let mut workflow = workflows.get("in").unwrap();

    loop {
        let mut next_workflow = None;
        use Rule::*;
        for rule in workflow {
            match rule {
                Accept => return true,
                Reject => return false,
                ConditionalNext(category, comparator, value, next) => {
                    let relevant_category = part[*category];
                    let cont = match comparator {
                        '<' => relevant_category < *value,
                        '>' => relevant_category > *value,
                        _ => unreachable!(),
                    };
                    if cont {
                        next_workflow = Some(next.as_str());
                        break;
                    }
                }
                InstantJump(other) => {
                    next_workflow = Some(other);
                    break;
                }
            }
        }
        let next_workflow = next_workflow.unwrap();
        if next_workflow == "A" {
            return true;
        } else if next_workflow == "R" {
            return false;
        }
        workflow = workflows.get(next_workflow).unwrap();
    }

    unreachable!()
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
enum Rule {
    Accept,
    Reject,
    ConditionalNext(char, char, usize, String),
    InstantJump(&'static str),
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Index<char> for Part {
    type Output = usize;

    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            _ => unreachable!(),
        }
    }
}

impl Part {
    fn new(captures: Captures) -> Self {
        let mut meow = captures
            .iter()
            .skip(1)
            .map(|matc| matc.unwrap().as_str().parse::<usize>().unwrap());
        Self {
            x: meow.next().unwrap(),
            m: meow.next().unwrap(),
            a: meow.next().unwrap(),
            s: meow.next().unwrap(),
        }
    }
}
