use crate::aoc2022::Aoc2022;
use crate::traits::days::Day5;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Default, Debug)]
pub struct Input {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

#[derive(Debug)]
pub struct Move {
    n: u32,
    from: u32,
    to: u32,
}

impl ParseInput<Day5> for Aoc2022 {
    type Parsed = Input;

    fn parse_input(input: &str) -> Self::Parsed {
        const STACK_END: &str = " 1   2   3";

        let mut res = Input::default();
        let mut in_move_part = false;

        for line in input.lines() {
            if line.starts_with(STACK_END) {
                in_move_part = true;
                continue;
            }

            if !in_move_part {
                for (i, c) in line.chars().enumerate() {
                    if i % 4 == 1 && c != ' ' {
                        let stack_index = i / 4;
                        if res.stacks.len() <= stack_index {
                            res.stacks.resize(stack_index + 1, Vec::new());
                        }
                        res.stacks[stack_index].push(c);
                    }
                }
            } else if line.starts_with("move") {
                let mut word_iter = line.split_ascii_whitespace();
                word_iter.next(); // skip move
                let n = word_iter.next().unwrap().parse().unwrap();
                word_iter.next(); // skip from
                let from = word_iter.next().unwrap().parse().unwrap();
                word_iter.next(); // skip to
                let to = word_iter.next().unwrap().parse().unwrap();
                res.moves.push(Move { n, from, to });
            }
        }

        for stack in res.stacks.iter_mut() {
            stack.reverse();
        }
        res
    }
}

fn compute_output(stacks: Vec<Vec<char>>) -> String {
    let mut output = String::with_capacity(stacks.len());
    for mut stack in stacks {
        let top = stack.pop().unwrap();
        output.push(top);
    }
    output
}

impl Solution<Day5> for Aoc2022 {
    type Part1Output = String;
    type Part2Output = String;

    fn part1(input: &Input) -> String {
        let mut stacks = input.stacks.clone();

        for m in &input.moves {
            let current = &mut stacks[m.from as usize - 1];
            let top = current.split_off(current.len() - m.n as usize);
            stacks[m.to as usize - 1].extend(top.into_iter().rev());
        }
        compute_output(stacks)
    }

    fn part2(input: &Input) -> String {
        let mut stacks = input.stacks.clone();

        for m in &input.moves {
            let current = &mut stacks[m.from as usize - 1];
            let top = current.split_off(current.len() - m.n as usize);
            stacks[m.to as usize - 1].extend(top);
        }
        compute_output(stacks)
    }
}
