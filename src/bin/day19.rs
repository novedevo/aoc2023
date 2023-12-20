#![allow(unused)]
use std::{
    collections::{HashMap, VecDeque},
    convert::Infallible,
    ops::{Index, IndexMut},
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

    let reg = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    let parts = part_ratings
        .lines()
        .map(|part| Part::new(reg.captures(part).unwrap()))
        .filter(|part| accepts(&workflows, part))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<usize>();
    dbg!(parts);
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
        let mut workflow = workflows.get(next_workflow.unwrap()).unwrap();
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
