use std::simd::Simd;
use std::simd::SimdPartialEq;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day11;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Default, Clone, Copy)]
pub enum Operation {
    Add(Simd<u64, CHUNK_WIDTH>),
    Mul(Simd<u64, CHUNK_WIDTH>),
    #[default]
    Square,
}

#[derive(Default, Debug, Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_div_by: Simd<u64, CHUNK_WIDTH>,
    if_true: Simd<i64, CHUNK_WIDTH>,
    if_false: Simd<i64, CHUNK_WIDTH>,
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
                        "+" => Operation::Add(Simd::splat(operation[2..].parse().unwrap())),
                        "*" => {
                            if &operation[2..] == "old" {
                                Operation::Square
                            } else {
                                Operation::Mul(Simd::splat(operation[2..].parse().unwrap()))
                            }
                        }
                        _ => unreachable!(),
                    };
                    3
                }
                3 => {
                    current_monkey.test_div_by = Simd::splat(
                        line.trim_start_matches("Test: divisible by ")
                            .parse()
                            .unwrap(),
                    );
                    4
                }
                4 => {
                    current_monkey.if_true = Simd::splat(
                        line.trim_start_matches("If true: throw to monkey ")
                            .parse()
                            .unwrap(),
                    );
                    5
                }
                5 => {
                    current_monkey.if_false = Simd::splat(
                        line.trim_start_matches("If false: throw to monkey ")
                            .parse()
                            .unwrap(),
                    );
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

const CHUNK_WIDTH: usize = 8;

fn solve(monkeys: &[Monkey], rounds: usize, div_by_3: bool) -> usize {
    let mut monkeys = monkeys.to_vec();
    let mut counter = vec![0; monkeys.len()];

    let modulo = Simd::splat(monkeys.iter().map(|m| m.test_div_by[0]).product::<u64>());

    for _ in 0..rounds {
        for mi in 0..monkeys.len() {
            let current_items = std::mem::take(&mut monkeys[mi].items);
            counter[mi] += current_items.len();

            for chunk in current_items.chunks(CHUNK_WIDTH) {
                let mut item: Simd<_, CHUNK_WIDTH> = Simd::splat(0);
                item.as_mut_array()[..chunk.len()].clone_from_slice(chunk);

                let mut item = match monkeys[mi].operation {
                    Operation::Add(rhs) => item + rhs,
                    Operation::Mul(rhs) => item * rhs,
                    Operation::Square => item * item,
                };

                if div_by_3 {
                    item /= Simd::splat(3);
                } else {
                    item %= modulo;
                }

                let next_idx_mask = (item % monkeys[mi].test_div_by).simd_eq(Simd::splat(0));
                let next_indices = next_idx_mask.select(monkeys[mi].if_true, monkeys[mi].if_false);

                for i in 0..chunk.len() {
                    monkeys[next_indices[i] as usize].items.push(item[i]);
                }
            }
        }
    }

    counter.sort();
    counter.reverse();
    counter[0] * counter[1]
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
