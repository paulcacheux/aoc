use std::collections::HashMap;
use std::collections::VecDeque;

use crate::aoc2023::day8::lcm;
use crate::aoc2023::Aoc2023;
use crate::traits::days::Day20;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug)]
pub struct Piping {
    sigil: Option<char>,
    from: String,
    to: Vec<String>,
}

impl ParseInput<Day20> for Aoc2023 {
    type Parsed = Vec<Piping>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let (before, after) = line.split_once(" -> ").unwrap();
                let to = after.split(", ").map(str::to_owned).collect();

                let sigil = match before.chars().next().unwrap() {
                    c @ ('&' | '%') => Some(c),
                    _ => None,
                };

                let from = if sigil.is_some() {
                    before[1..].to_owned()
                } else {
                    before.to_owned()
                };

                Piping { sigil, from, to }
            })
            .collect()
    }
}

impl Solution<Day20> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<Piping>) -> usize {
        let mut graph = Graph::new(input);

        let mut low_counter = 0;
        let mut high_counter = 0;

        for _ in 0..1000 {
            graph.click(|_, _, signal| {
                match signal {
                    Signal::High => high_counter += 1,
                    Signal::Low => low_counter += 1,
                };
            });
        }

        low_counter * high_counter
    }

    fn part2(input: &Vec<Piping>) -> usize {
        let mut parents: HashMap<String, Vec<String>> = HashMap::new();
        for pipe in input {
            for target in &pipe.to {
                parents
                    .entry(target.clone())
                    .or_default()
                    .push(pipe.from.clone());
            }
        }

        let rx_parent = &parents.get("rx").unwrap()[0];
        let rx_grand_parents_count = parents.get(rx_parent).unwrap().len();

        let mut factors = HashMap::new();
        let mut graph = Graph::new(input);

        let mut counter = 0;

        while factors.len() != rx_grand_parents_count {
            counter += 1;
            graph.click(|from, target, signal| {
                if target == rx_parent && signal == Signal::High && !factors.contains_key(from) {
                    factors.insert(from.to_owned(), counter);
                }
            });
        }

        let factors: Vec<_> = factors.values().copied().collect();

        lcm(&factors)
    }
}

#[derive(Debug)]
enum Module {
    BroadCaster,
    FlipFlop(bool),
    Inverter,
    Conjunction {
        memory: HashMap<String, Signal>,
        source_count: usize,
    },
}

impl Module {
    fn bip(&mut self, from: &str, signal: Signal) -> Option<Signal> {
        match (self, signal) {
            (Module::BroadCaster, signal) => Some(signal),
            (Module::FlipFlop(_), Signal::High) => None,
            (Module::FlipFlop(state), Signal::Low) => {
                *state = !*state;
                Some(if *state { Signal::High } else { Signal::Low })
            }
            (Module::Inverter, Signal::High) => Some(Signal::Low),
            (Module::Inverter, Signal::Low) => Some(Signal::High),
            (
                Module::Conjunction {
                    memory,
                    source_count,
                },
                signal,
            ) => {
                memory.insert(from.to_owned(), signal);
                if memory.values().filter(|&&sig| sig == Signal::High).count() == *source_count {
                    Some(Signal::Low)
                } else {
                    Some(Signal::High)
                }
            }
        }
    }
}

#[derive(Debug)]
struct ModuleGroup {
    module: Module,
    targets: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Signal {
    Low,
    High,
}

struct Graph {
    state: HashMap<String, ModuleGroup>,
}

impl Graph {
    fn new(pipes: &[Piping]) -> Self {
        let mut directed_to: HashMap<String, usize> = HashMap::new();

        for pipe in pipes {
            for target in &pipe.to {
                *directed_to.entry(target.clone()).or_default() += 1;
            }
        }

        let mut graph = HashMap::new();

        for pipe in pipes {
            let module = match (pipe.sigil, pipe.from.as_str()) {
                (Some('%'), _) => Module::FlipFlop(false),
                (Some('&'), name) => {
                    let source_count = directed_to.get(name).copied().unwrap_or(0);
                    if source_count == 1 {
                        Module::Inverter
                    } else {
                        Module::Conjunction {
                            memory: HashMap::new(),
                            source_count,
                        }
                    }
                }
                (None, "broadcaster") => Module::BroadCaster,
                _ => unreachable!(),
            };
            graph.insert(
                pipe.from.clone(),
                ModuleGroup {
                    module,
                    targets: pipe.to.clone(),
                },
            );
        }

        Graph { state: graph }
    }

    fn click<F>(&mut self, mut cb: F)
    where
        F: FnMut(&str, &str, Signal),
    {
        let mut signal_queue = VecDeque::new();
        signal_queue.push_back(("button".to_owned(), "broadcaster".to_owned(), Signal::Low));

        while let Some((from, target, signal)) = signal_queue.pop_front() {
            cb(&from, &target, signal);

            if let Some(mg) = self.state.get_mut(&target) {
                if let Some(next_sig) = mg.module.bip(&from, signal) {
                    for next_target in &mg.targets {
                        signal_queue.push_back((target.clone(), next_target.clone(), next_sig));
                    }
                }
            }
        }
    }
}
