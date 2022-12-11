use crate::aoc2022::Aoc2022;
use crate::traits::days::Day11;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Default, Clone, Copy)]
pub enum Operation {
    Add(u64),
    Mul(u64),
    #[default]
    Square,
}

#[derive(Default, Debug, Clone)]
pub struct Monkey {
    items: Vec<u64>,
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

fn solve(monkeys: &[Monkey], rounds: usize, div_by_3: bool) -> u64 {
    let mut monkeys = monkeys.to_vec();
    let mut counter = vec![0; monkeys.len()];

    let modulo = monkeys.iter().map(|m| m.test_div_by).product::<u64>();

    for _ in 0..rounds {
        for mi in 0..monkeys.len() {
            let current_items = std::mem::replace(&mut monkeys[mi].items, Vec::new());
            for item in current_items {
                let mut item = match monkeys[mi].operation {
                    Operation::Add(rhs) => item + rhs,
                    Operation::Mul(rhs) => item * rhs,
                    Operation::Square => item * item,
                };

                if div_by_3 {
                    item /= 3;
                }
                item %= modulo;

                let next_index = if item % monkeys[mi].test_div_by == 0 {
                    monkeys[mi].if_true
                } else {
                    monkeys[mi].if_false
                };
                monkeys[next_index].items.push(item);
                counter[mi] += 1;
            }
        }
    }

    counter.sort();
    counter.reverse();
    counter[0] * counter[1]
}

impl Solution<Day11> for Aoc2022 {
    type Part1Output = u64;
    type Part2Output = u64;

    fn part1(input: &Vec<Monkey>) -> u64 {
        solve(input, 20, true)
    }

    fn part2(input: &Vec<Monkey>) -> u64 {
        solve(input, 10000, false)
    }
}
