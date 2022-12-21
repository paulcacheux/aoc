use ahash::HashMap;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelIterator;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day21;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Clone)]
pub enum Computation {
    Direct(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl ParseInput<Day21> for Aoc2022 {
    type Parsed = HashMap<String, Computation>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(str::trim)
            .map(|line| {
                let (name, comp) = line.split_once(": ").unwrap();

                let c = if let Ok(dir) = comp.parse() {
                    Computation::Direct(dir)
                } else {
                    let mut words = comp.split_ascii_whitespace();
                    let lhs = words.next().unwrap().to_owned();
                    let op = words.next().unwrap();
                    let rhs = words.next().unwrap().to_owned();
                    match op {
                        "+" => Computation::Add(lhs, rhs),
                        "-" => Computation::Sub(lhs, rhs),
                        "*" => Computation::Mul(lhs, rhs),
                        "/" => Computation::Div(lhs, rhs),
                        _ => unreachable!(),
                    }
                };

                (name.to_owned(), c)
            })
            .collect()
    }
}

#[derive(Debug)]
enum StackItem {
    Humn,
    Val(i64),
    Add,
    Sub,
    Mul,
    Div,
}

impl Solution<Day21> for Aoc2022 {
    type Part1Output = i64;
    type Part2Output = i64;

    fn part1(input: &HashMap<String, Computation>) -> i64 {
        let stack = build_stack(input, "root".to_owned(), false);
        eval_stack(&stack, 0)
    }

    fn part2(input: &HashMap<String, Computation>) -> i64 {
        let (lhs, rhs) = match input["root"].clone() {
            Computation::Add(lhs, rhs) => (lhs, rhs),
            _ => unreachable!(),
        };

        let lhs = build_stack(input, lhs, true);
        let lhs = opt_stack(lhs);
        let rhs = build_stack(input, rhs, true);
        let rhs = opt_stack(rhs);

        dbg!(&lhs);
        dbg!(&rhs);

        let res: Vec<_> = (0..100000000)
            .into_par_iter()
            .filter(|&humn_value| eval_stack(&lhs, humn_value) == eval_stack(&rhs, humn_value))
            .collect();
        dbg!(res);
        unreachable!()
    }
}

fn build_stack(
    input: &HashMap<String, Computation>,
    root: String,
    meta_humn: bool,
) -> Vec<StackItem> {
    let mut queue = vec![root];
    let mut stack = Vec::new();

    while let Some(curr) = queue.pop() {
        match curr.as_str() {
            "+" => stack.push(StackItem::Add),
            "-" => stack.push(StackItem::Sub),
            "*" => stack.push(StackItem::Mul),
            "/" => stack.push(StackItem::Div),
            "humn" if meta_humn => stack.push(StackItem::Humn),
            other => match &input[other] {
                Computation::Direct(val) => {
                    stack.push(StackItem::Val(*val));
                }
                Computation::Add(lhs, rhs) => {
                    queue.extend_from_slice(&["+".to_owned(), rhs.clone(), lhs.clone()]);
                }
                Computation::Sub(lhs, rhs) => {
                    queue.extend_from_slice(&["-".to_owned(), rhs.clone(), lhs.clone()]);
                }
                Computation::Mul(lhs, rhs) => {
                    queue.extend_from_slice(&["*".to_owned(), rhs.clone(), lhs.clone()]);
                }
                Computation::Div(lhs, rhs) => {
                    queue.extend_from_slice(&["/".to_owned(), rhs.clone(), lhs.clone()]);
                }
            },
        }
    }
    stack
}

fn opt_stack(stack: Vec<StackItem>) -> Vec<StackItem> {
    fn mix(a: &StackItem, b: &StackItem, f: fn(i64, i64) -> i64) -> Option<StackItem> {
        match (a, b) {
            (StackItem::Val(a), StackItem::Val(b)) => Some(StackItem::Val(f(*a, *b))),
            _ => None,
        }
    }

    let stack_len = stack.len();
    let mut opt = Vec::with_capacity(stack.len());
    for item in stack {
        match item {
            StackItem::Add => {
                let rhs = opt.pop().unwrap();
                let lhs = opt.pop().unwrap();
                if let Some(res) = mix(&lhs, &rhs, |a, b| a + b) {
                    opt.push(res);
                } else {
                    opt.extend([lhs, rhs, StackItem::Add]);
                }
            }
            StackItem::Sub => {
                let rhs = opt.pop().unwrap();
                let lhs = opt.pop().unwrap();
                if let Some(res) = mix(&lhs, &rhs, |a, b| a - b) {
                    opt.push(res);
                } else {
                    opt.extend([lhs, rhs, StackItem::Sub]);
                }
            }
            StackItem::Mul => {
                let rhs = opt.pop().unwrap();
                let lhs = opt.pop().unwrap();
                if let Some(res) = mix(&lhs, &rhs, |a, b| a * b) {
                    opt.push(res);
                } else {
                    opt.extend([lhs, rhs, StackItem::Mul]);
                }
            }
            StackItem::Div => {
                let rhs = opt.pop().unwrap();
                let lhs = opt.pop().unwrap();
                if let Some(res) = mix(&lhs, &rhs, |a, b| a / b) {
                    opt.push(res);
                } else {
                    opt.extend([lhs, rhs, StackItem::Div]);
                }
            }
            other => opt.push(other),
        }
    }
    println!("{} => {}", stack_len, opt.len());
    opt
}

fn eval_stack(stack: &[StackItem], humn_value: i64) -> i64 {
    let mut real_stack = Vec::new();
    for item in stack {
        match item {
            StackItem::Val(val) => real_stack.push(*val),
            StackItem::Humn => real_stack.push(humn_value),
            StackItem::Add => {
                let rhs = real_stack.pop().unwrap();
                let lhs = real_stack.pop().unwrap();
                real_stack.push(lhs + rhs);
            }
            StackItem::Sub => {
                let rhs = real_stack.pop().unwrap();
                let lhs = real_stack.pop().unwrap();
                real_stack.push(lhs - rhs);
            }
            StackItem::Mul => {
                let rhs = real_stack.pop().unwrap();
                let lhs = real_stack.pop().unwrap();
                real_stack.push(lhs * rhs);
            }
            StackItem::Div => {
                let rhs = real_stack.pop().unwrap();
                let lhs = real_stack.pop().unwrap();
                real_stack.push(lhs / rhs);
            }
        }
    }

    assert_eq!(real_stack.len(), 1);
    real_stack.pop().unwrap()
}
