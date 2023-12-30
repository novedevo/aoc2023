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
    let input = include_str!("../../data/day20.txt");
    let mut modules = input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(module_name, destination_modules)| {
            let module = Module::new(module_name, destination_modules);
            (module.name, module)
        })
        .collect::<HashMap<&'static str, Module>>();

    for module in modules.clone().values() {
        for &output in &module.outputs {
            let Some(modu) = modules.get_mut(output) else {
                continue;
            };
            modu.inputs.insert(module.name, false);
        }
    }
    let res = (0..10000000)
        .map(|_| push_button(&mut modules)).collect_vec();
    let res = res.iter()
        .enumerate()
        .find(|(_, option)| option.is_none())
        .unwrap();
    dbg!(res);
}

//returns number of pulses sent
fn push_button(modules: &mut HashMap<&'static str, Module>) -> Option<Vec<usize>> {
    let mut high_pulses = 0;
    let mut low_pulses = 0;

    let mut pulses_to_be_processed = VecDeque::new();
    pulses_to_be_processed.push_back(("", false, "broadcaster"));

    while let Some(current_pulse) = pulses_to_be_processed.pop_front() {
        // if pulsecount > 15 {
        //     panic!("meowwww")
        // }
        // dbg!(current_pulse);
        if current_pulse.1 {
            high_pulses += 1;
        } else {
            low_pulses += 1;
        }

        let Some(pulsing_module) = modules.get_mut(current_pulse.2) else {
            continue;
        };

        if pulsing_module.name == "rx" {
            return None;
        }

        let pulseheight = match pulsing_module.ttype {
            'b' => current_pulse.1,
            '%' => {
                if current_pulse.1 {
                    continue;
                } else {
                    pulsing_module.invert()
                }
            }
            '&' => pulsing_module.input(current_pulse.0, current_pulse.1),
            _ => unreachable!(),
        };

        pulses_to_be_processed.extend(
            pulsing_module
                .outputs
                .iter()
                .map(|&s| (pulsing_module.name, pulseheight, s)),
        );
    }
    let meow = ["km", "jd", "bv", "vn", "vv", "fc", "gm"]
        .into_iter()
        .map(|key| {
            modules
                .get(key)
                .unwrap()
                .flipflop_state
        })
        .filter(|v| *v).count();

    if  meow == 7{
        dbg!(meow);
        return None;
    } else {return Some(vec![])}
    // return Some(meow);
}

#[derive(Clone, Eq)]
struct Module {
    ttype: char,
    name: &'static str,
    inputs: HashMap<&'static str, bool>,
    outputs: Vec<&'static str>,
    flipflop_state: bool,
}

impl std::hash::Hash for Module {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl Module {
    fn invert(&mut self) -> bool {
        self.flipflop_state = !self.flipflop_state;
        self.flipflop_state
    }

    fn input(&mut self, from: &'static str, height: bool) -> bool {
        self.inputs.insert(from, height);
        !self.inputs.values().all(|v| *v)
    }

    fn new(module_name: &'static str, destination_modules: &'static str) -> Self {
        let outputs = destination_modules.split(", ").collect_vec();
        if module_name == "broadcaster" {
            Self {
                ttype: 'b',
                name: module_name,
                inputs: HashMap::new(),
                outputs,
                flipflop_state: false,
            }
        } else {
            let mut cs = module_name.chars();
            Self {
                ttype: cs.next().unwrap(),
                name: &module_name[1..],
                inputs: HashMap::new(),
                outputs,
                flipflop_state: false,
            }
        }
    }
}
