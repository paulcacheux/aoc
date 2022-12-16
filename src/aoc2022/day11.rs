use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::Range;
use std::ops::RemAssign;
use std::simd::Simd;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day11;
use crate::traits::ParseInput;
use crate::traits::Solution;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Operation {
    #[default]
    Add,
    Mul,
    Square,
}

#[derive(Default, Debug, Clone)]
pub struct Monkey {
    items: SmallVec<[u64; 64]>,
    operation: Operation,
    rhs: u64,
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
                    current_monkey.rhs = operation[2..].parse().unwrap_or_default();
                    current_monkey.operation = match &operation[0..1] {
                        "+" => Operation::Add,
                        "*" => {
                            if &operation[2..] == "old" {
                                Operation::Square
                            } else {
                                Operation::Mul
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

const SIMD_LANES: usize = 4;

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
    fn range(&self, index: usize, aligned: bool) -> Range<usize> {
        let start = self.period * index;
        let mut size = self.sizes[index];
        if aligned {
            size = (size + (SIMD_LANES - 1)) & !(SIMD_LANES - 1);
        }
        let end = start + size;
        start..end
    }

    #[inline]
    fn step<const DIV3: bool>(&mut self, monkey: &Monkey, index: usize, modulo: u64) {
        let r = self.range(index, true);
        let slice = &mut self.items[r.clone()];
        let (start, middle, end) = slice.as_simd_mut::<SIMD_LANES>();

        assert_eq!(start.len(), 0);
        assert_eq!(end.len(), 0);

        for item in middle {
            compute_item::<DIV3, _>(
                item,
                Simd::splat(monkey.rhs),
                Simd::splat(3),
                Simd::splat(modulo),
                monkey.operation,
            );
        }

        for i in self.range(index, false) {
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

#[inline]
fn compute_item<const DIV3: bool, T>(item: &mut T, rhs: T, three: T, modulo: T, op: Operation)
where
    T: Copy + AddAssign<T> + MulAssign<T> + DivAssign<T> + RemAssign<T>,
{
    match op {
        Operation::Add => *item += rhs,
        Operation::Mul => *item *= rhs,
        Operation::Square => *item *= *item,
    };

    if DIV3 {
        *item /= three;
    } else {
        *item %= modulo;
    }
}

fn solve<const DIV3: bool>(monkeys: &[Monkey], rounds: usize) -> usize {
    let mut items = ItemBags::new(monkeys);
    let mut counter = vec![0; monkeys.len()];

    let modulo = monkeys.iter().map(|m| m.test_div_by).product::<u64>();

    for _ in 0..rounds {
        for (mi, current_monkey) in monkeys.iter().enumerate() {
            counter[mi] += items.sizes[mi];
            items.step::<DIV3>(current_monkey, mi, modulo);
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
        solve::<true>(input, 20)
    }

    fn part2(input: &Vec<Monkey>) -> usize {
        solve::<false>(input, 10000)
    }
}
