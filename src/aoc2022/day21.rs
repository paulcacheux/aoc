use ahash::HashMap;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day21;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug)]
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
    Val(i64),
    Add,
    Sub,
    Mul,
    Div,
}

impl Solution<Day21> for Aoc2022 {
    type Part1Output = i64;
    type Part2Output = u32;

    fn part1(input: &HashMap<String, Computation>) -> i64 {
        let mut queue = vec!["root".to_owned()];
        let mut stack = Vec::new();

        while let Some(curr) = queue.pop() {
            match curr.as_str() {
                "+" => stack.push(StackItem::Add),
                "-" => stack.push(StackItem::Sub),
                "*" => stack.push(StackItem::Mul),
                "/" => stack.push(StackItem::Div),
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

        let mut real_stack = Vec::new();
        for item in stack {
            match item {
                StackItem::Val(val) => real_stack.push(val),
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

    fn part2(_input: &HashMap<String, Computation>) -> u32 {
        todo!()
    }
}
