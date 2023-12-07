use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../data/day7.txt");

    let hands = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bet)| (hand.parse::<Hand>(), bet.parse::<usize>().unwrap()))
        .sorted()
        // .rev()
        .enumerate()
        .collect_vec();
    // dbg!(&hands);
    let hands = hands
        .iter()
        .map(|(i, (_hand, bet))| (i + 1) * bet)
        .sum::<usize>();

    dbg!(hands);
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_cmp = other.categorize().cmp(&self.categorize());

        if type_cmp.is_eq() {
            self.cards.cmp(&other.cards)
        } else {
            type_cmp
        }
    }
}

impl FromStr for Hand {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cards: s.chars().map(|c| c.into()).collect_vec(),
        })
    }

    type Err = ();
}

impl Hand {
    fn categorize(&self) -> Type {
        let mut mapped = HashMap::new();
        for card in &self.cards {
            *mapped.entry(card).or_default() += 1;
        }

        let joker_count = mapped.remove(&Card { value: 1 }).unwrap_or_default();
        let max_card = mapped.iter().map(|(k, v)| (v, k)).sorted().last();
        if max_card.is_none() {
            dbg!(&mapped, &self);
        }
        let max_card = **max_card
            .map(|(_, count)| count)
            .unwrap_or(&&Card { value: 1 });
        *mapped.entry(&max_card).or_default() += joker_count;

        let prod: usize = mapped.values().product();
        match mapped.len() {
            1 => Type::Five,
            2 => match prod {
                4 => Type::Four,
                6 => Type::Full,
                _ => unreachable!(),
            },
            3 => match prod {
                3 => Type::Three,
                4 => Type::Two,
                _ => unreachable!("{prod}"),
            },
            4 => Type::One,
            5 => Type::High,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Card {
    pub value: u32,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        if c.is_ascii_digit() {
            Self {
                value: c.to_digit(10).unwrap(),
            }
        } else {
            let value = match c {
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            };

            Self { value }
        }
    }
}
