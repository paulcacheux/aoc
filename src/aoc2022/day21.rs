use ahash::HashMap;

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
        eval_stack(&stack)
    }

    fn part2(input: &HashMap<String, Computation>) -> i64 {
        let (lhs, rhs) = match input["root"].clone() {
            Computation::Add(lhs, rhs) => (lhs, rhs),
            _ => unreachable!(),
        };

        // in the example and in the actual input, rhs is directly evaluable
        let rhs = build_stack(input, rhs, true);
        let rhs = eval_stack(&rhs);

        let lhs = build_stack(input, lhs, true);
        let lhs = opt_stack(lhs);
        let lhs = eval_stack_symbolic(&lhs);

        ((rhs * lhs.c) - lhs.a) / lhs.b
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
    opt
}

fn eval_stack(stack: &[StackItem]) -> i64 {
    let mut real_stack = Vec::new();
    for item in stack {
        match item {
            StackItem::Val(val) => real_stack.push(*val),
            StackItem::Humn => unimplemented!(),
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

// represents (a + b*x) / c
#[derive(Debug, Clone, Copy)]
struct HumanSymbol {
    a: i64,
    b: i64,
    c: i64,
}

impl HumanSymbol {
    fn from_humn() -> Self {
        HumanSymbol { a: 0, b: 1, c: 1 }
    }

    fn from_val(val: i64) -> Self {
        HumanSymbol { a: val, b: 0, c: 1 }
    }

    fn normalized(mut self) -> Self {
        // sign on top
        if self.c < 0 {
            self.a *= -1;
            self.b *= -1;
        }
        let gcd = gcd::binary_u64(self.a.abs() as u64, self.b.abs() as u64);
        let gcd = gcd::binary_u64(gcd, self.c.abs() as u64);
        let gcd = gcd as i64;

        self.a /= gcd;
        self.b /= gcd;
        self.c /= gcd;
        self
    }

    fn add(lhs: Self, rhs: Self) -> Self {
        let c = lhs.c * rhs.c;
        let a = lhs.a * rhs.c + rhs.a * lhs.c;
        let b = lhs.b * rhs.c + rhs.b * lhs.c;
        HumanSymbol { a, b, c }.normalized()
    }

    fn sub(lhs: Self, rhs: Self) -> Self {
        let c = lhs.c * rhs.c;
        let a = lhs.a * rhs.c - rhs.a * lhs.c;
        let b = lhs.b * rhs.c - rhs.b * lhs.c;
        HumanSymbol { a, b, c }.normalized()
    }

    fn mul(lhs: Self, rhs: Self) -> Self {
        if lhs.b != 0 && rhs.b != 0 {
            unimplemented!()
        }

        let c = lhs.c * rhs.c;
        let a = lhs.a * rhs.a;
        let b = lhs.a * rhs.b + rhs.a * lhs.b;
        HumanSymbol { a, b, c }.normalized()
    }

    fn div(lhs: Self, rhs: Self) -> Self {
        if rhs.b != 0 {
            unimplemented!()
        }

        let a = lhs.a * rhs.c;
        let b = lhs.b * rhs.c;
        let c = lhs.c * rhs.a;
        HumanSymbol { a, b, c }.normalized()
    }
}

fn eval_stack_symbolic(stack: &[StackItem]) -> HumanSymbol {
    let mut real_stack = Vec::new();
    for item in stack {
        match item {
            StackItem::Val(val) => real_stack.push(HumanSymbol::from_val(*val)),
            StackItem::Humn => real_stack.push(HumanSymbol::from_humn()),
            StackItem::Add => {
                let rhs = real_stack.pop().unwrap();
                let lhs = real_stack.pop().unwrap();
                real_stack.push(HumanSymbol::add(lhs, rhs));
            }
            StackItem::Sub => {
                let rhs = real_stack.pop().unwrap();
                let lhs = real_stack.pop().unwrap();
                real_stack.push(HumanSymbol::sub(lhs, rhs));
            }
            StackItem::Mul => {
                let rhs = real_stack.pop().unwrap();
                let lhs = real_stack.pop().unwrap();
                real_stack.push(HumanSymbol::mul(lhs, rhs));
            }
            StackItem::Div => {
                let rhs = real_stack.pop().unwrap();
                let lhs = real_stack.pop().unwrap();
                real_stack.push(HumanSymbol::div(lhs, rhs));
            }
        }
    }

    assert_eq!(real_stack.len(), 1);
    let hs = real_stack.pop().unwrap();
    hs
}
