use std::ops::Range;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day11;
use crate::traits::ParseInput;
use crate::traits::Solution;
use smallvec::SmallVec;

#[derive(Debug, Default, Clone, Copy)]
pub enum Operation {
    Add(u64),
    Mul(u64),
    #[default]
    Square,
}

#[derive(Default, Debug, Clone)]
pub struct Monkey {
    items: SmallVec<[u64; 64]>,
    operation: Operation,
    test_div_by: u64,
    if_true: usize,
    if_false: usize,
}

impl ParseInput<Day11> for Aoc2022 {
    type Parsed = Vec<Monkey>;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut monkeys = Vec::new();
        let mut current_monkey = Monkey::default();
        let mut state = 0;
        for line in input.lines() {
            let line = line.trim();
            state = match state {
                0 => 1, // Monkey N: do nothing
                1 => {
                    current_monkey.items = line
                        .trim_start_matches("Starting items: ")
                        .split(", ")
                        .map(|item| item.parse().unwrap())
                        .collect();
                    2
                }
                2 => {
                    let operation = line.trim_start_matches("Operation: new = old ");
                    current_monkey.operation = match &operation[0..1] {
                        "+" => Operation::Add(operation[2..].parse().unwrap()),
                        "*" => {
                            if &operation[2..] == "old" {
                                Operation::Square
                            } else {
                                Operation::Mul(operation[2..].parse().unwrap())
                            }
                        }
                        _ => unreachable!(),
                    };
                    3
                }
                3 => {
                    current_monkey.test_div_by = line
                        .trim_start_matches("Test: divisible by ")
                        .parse()
                        .unwrap();
                    4
                }
                4 => {
                    current_monkey.if_true = line
                        .trim_start_matches("If true: throw to monkey ")
                        .parse()
                        .unwrap();
                    5
                }
                5 => {
                    current_monkey.if_false = line
                        .trim_start_matches("If false: throw to monkey ")
                        .parse()
                        .unwrap();
                    6
                }
                6 => {
                    // reset state at empty lines
                    monkeys.push(current_monkey);
                    current_monkey = Monkey::default();
                    0
                }
                _ => unreachable!(),
            };
        }
        if state != 0 {
            monkeys.push(current_monkey);
        }
        monkeys
    }
}

#[derive(Debug)]
struct ItemBags {
    items: Vec<u64>,
    period: usize,
    sizes: Vec<usize>,
}

impl ItemBags {
    fn new(monkeys: &[Monkey]) -> Self {
        let period = monkeys
            .iter()
            .map(|m| m.items.len())
            .sum::<usize>()
            .next_power_of_two();
        let mut items = vec![0; period * monkeys.len()];
        let mut sizes = vec![0; monkeys.len()];

        for (mi, monkey) in monkeys.iter().enumerate() {
            let start = period * mi;
            let size = monkey.items.len();
            items[start..start + size].clone_from_slice(&monkey.items);
            sizes[mi] = size;
        }

        ItemBags {
            items,
            period,
            sizes,
        }
    }

    #[inline]
    fn range(&self, index: usize) -> Range<usize> {
        let start = self.period * index;
        let end = start + self.sizes[index];
        start..end
    }

    #[inline]
    fn step(&mut self, monkey: &Monkey, index: usize, modulo: u64, div_by_3: bool) {
        for i in self.range(index) {
            let item = self.items[i];
            let mut item = match monkey.operation {
                Operation::Add(rhs) => item + rhs,
                Operation::Mul(rhs) => item * rhs,
                Operation::Square => item * item,
            };

            if div_by_3 {
                item /= 3;
            } else {
                item %= modulo;
            }
            self.items[i] = item;
        }

        for i in self.range(index) {
            let item = self.items[i];
            let next = if item % monkey.test_div_by == 0 {
                monkey.if_true
            } else {
                monkey.if_false
            };

            self.items[self.period * next + self.sizes[next]] = item;
            self.sizes[next] += 1;
        }
        self.sizes[index] = 0;
    }
}

fn solve(monkeys: &[Monkey], rounds: usize, div_by_3: bool) -> usize {
    let mut items = ItemBags::new(monkeys);
    let mut counter = vec![0; monkeys.len()];

    let modulo = monkeys.iter().map(|m| m.test_div_by).product::<u64>();

    for _ in 0..rounds {
        for (mi, current_monkey) in monkeys.iter().enumerate() {
            counter[mi] += items.sizes[mi];
            items.step(current_monkey, mi, modulo, div_by_3);
        }
    }

    let (mut max0, mut max1) = (0, 0);
    for c in counter {
        if c >= max0 {
            max1 = max0;
            max0 = c;
        } else if c > max1 {
            max1 = c;
        }
    }
    max0 * max1
}

impl Solution<Day11> for Aoc2022 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<Monkey>) -> usize {
        solve(input, 20, true)
    }

    fn part2(input: &Vec<Monkey>) -> usize {
        solve(input, 10000, false)
    }
}
