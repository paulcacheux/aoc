use std::cmp::Ordering;
use std::collections::HashMap;

use crate::aoc2023::Aoc2023;
use crate::traits::days::Day19;
use crate::traits::ParseInput;
use crate::traits::Solution;

pub struct Input {
    workflows: HashMap<String, Vec<Rule>>,
    ratings: Vec<Rating>,
}

#[derive(Debug, Default)]
struct Rating {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug)]
enum RuleField {
    None,
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Rule {
    field: RuleField,
    operator: std::cmp::Ordering,
    value: u64,
    target: String,
}

impl ParseInput<Day19> for Aoc2023 {
    type Parsed = Input;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut in_ratings = false;
        let mut ratings = Vec::new();
        let mut workflows = HashMap::new();

        for line in input.lines() {
            if line.trim().is_empty() {
                in_ratings = true;
                continue;
            }

            if in_ratings {
                let line = &line[1..line.len() - 1];
                let mut rating = Rating::default();
                for group in line.split(',') {
                    let (field, value) = group.split_once('=').unwrap();
                    let value = value.parse().unwrap();
                    match field {
                        "x" => rating.x = value,
                        "m" => rating.m = value,
                        "a" => rating.a = value,
                        "s" => rating.s = value,
                        _ => unreachable!(),
                    };
                }
                ratings.push(rating);
            } else {
                let (name, rules) = line.split_once('{').unwrap();
                let name = name.to_owned();
                let rules = &rules[..rules.len() - 1];

                let mut parsed_rules = Vec::new();

                for rule in rules.split(',') {
                    if let Some((eval, target)) = rule.split_once(':') {
                        let mut chars = eval.chars();
                        let field = match chars.next().unwrap() {
                            'x' => RuleField::X,
                            'm' => RuleField::M,
                            'a' => RuleField::A,
                            's' => RuleField::S,
                            _ => unreachable!(),
                        };
                        let operator = match chars.next().unwrap() {
                            '>' => Ordering::Greater,
                            '<' => Ordering::Less,
                            _ => unreachable!(),
                        };
                        let value = eval[2..].parse().unwrap();
                        parsed_rules.push(Rule {
                            field,
                            operator,
                            value,
                            target: target.to_owned(),
                        });
                    } else {
                        parsed_rules.push(Rule {
                            field: RuleField::None,
                            operator: Ordering::Equal,
                            value: 0,
                            target: rule.to_owned(),
                        });
                    }
                }
                workflows.insert(name, parsed_rules);
            }
        }

        Input { workflows, ratings }
    }
}

impl Solution<Day19> for Aoc2023 {
    type Part1Output = u64;
    type Part2Output = u64;

    fn part1(input: &Input) -> u64 {
        let mut score = 0;
        for rating in &input.ratings {
            let mut current = "in";

            'wf: loop {
                if current == "A" {
                    score += rating.x + rating.m + rating.a + rating.s;
                    break;
                } else if current == "R" {
                    break;
                }

                let workflow = input.workflows.get(current).unwrap();
                for rule in workflow {
                    let rating_value = match rule.field {
                        RuleField::None => {
                            current = &rule.target;
                            continue 'wf;
                        }
                        RuleField::X => rating.x,
                        RuleField::M => rating.m,
                        RuleField::A => rating.a,
                        RuleField::S => rating.s,
                    };

                    if rating_value.cmp(&rule.value) == rule.operator {
                        current = &rule.target;
                        continue 'wf;
                    }
                }
            }
        }
        score
    }

    fn part2(_input: &Input) -> u64 {
        todo!()
    }
}
