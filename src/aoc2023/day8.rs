use std::collections::HashMap;

use crate::aoc2023::Aoc2023;
use crate::traits::days::Day8;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct GameDef {
    instructions: Vec<Direction>,
    edges: HashMap<String, (String, String)>,
}

impl ParseInput<Day8> for Aoc2023 {
    type Parsed = GameDef;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut iter = input.lines();

        let instructions = iter
            .next()
            .unwrap()
            .trim()
            .bytes()
            .map(|b| match b {
                b'L' => Direction::Left,
                b'R' => Direction::Right,
                _ => unreachable!(),
            })
            .collect();

        iter.next().unwrap(); // skip empty line

        let edges = iter
            .map(|line| {
                let src = line[0..3].to_owned();
                let left = line[7..10].to_owned();
                let right = line[12..15].to_owned();
                (src, (left, right))
            })
            .collect();

        GameDef {
            instructions,
            edges,
        }
    }
}

impl Solution<Day8> for Aoc2023 {
    type Part1Output = u32;
    type Part2Output = usize;

    fn part1(input: &GameDef) -> u32 {
        let mut current = "AAA";
        let mut inst_stream = InstIterator::new(&input.instructions);

        let mut step = 0;
        while current != "ZZZ" {
            step += 1;
            let next = input.edges.get(current).unwrap();
            match inst_stream.next().1 {
                Direction::Left => {
                    current = &next.0;
                }
                Direction::Right => {
                    current = &next.1;
                }
            }
        }
        step
    }

    fn part2(input: &GameDef) -> usize {
        let mut factors = Vec::new();
        for key in input.edges.keys() {
            if !key.ends_with('A') {
                continue;
            }

            let mut current = key;
            let mut inst_stream = InstIterator::new(&input.instructions);
            let mut states = HashMap::new();
            let mut step = 0usize;

            loop {
                let (iter_state, next_dir) = inst_stream.next();
                // iter_state == 0 is a complete hack, but it works
                if current.ends_with('Z') && iter_state == 0 {
                    if let Some(&previous_step) = states.get(&current) {
                        let delta = step - previous_step;
                        assert_eq!(previous_step, delta);
                        factors.push(delta);
                        break;
                    } else {
                        states.insert(current, step);
                    }
                }
                step += 1;

                let next = input.edges.get(current).unwrap();
                match next_dir {
                    Direction::Left => {
                        current = &next.0;
                    }
                    Direction::Right => {
                        current = &next.1;
                    }
                }
            }
        }
        lcm(&factors)
    }
}

struct InstIterator<'d> {
    state: usize,
    data: &'d [Direction],
}
impl<'d> InstIterator<'d> {
    fn new(data: &'d [Direction]) -> Self {
        InstIterator { state: 0, data }
    }

    fn next(&mut self) -> (usize, Direction) {
        let state = self.state;
        let value = self.data[state];
        self.state = (self.state + 1) % self.data.len();
        (state, value)
    }
}

pub fn lcm(nums: &[usize]) -> usize {
    assert!(!nums.is_empty());
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}