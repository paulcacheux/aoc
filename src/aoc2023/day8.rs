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
        let fast_mapper = FastEdgeMapper::new(&input.edges);

        let mut current = fast_mapper.str_to_id("AAA");
        let zzz = fast_mapper.str_to_id("ZZZ");
        let mut inst_stream = InstIterator::new(&input.instructions);

        let mut step = 0;
        while current != zzz {
            step += 1;
            let (_, dir) = inst_stream.next();
            current = fast_mapper.get(current, dir);
        }
        step
    }

    fn part2(input: &GameDef) -> usize {
        let fast_mapper = FastEdgeMapper::new(&input.edges);

        let mut factors = Vec::new();
        for key in input.edges.keys() {
            if !key.ends_with('A') {
                continue;
            }

            let mut current = fast_mapper.str_to_id(key);
            let mut inst_stream = InstIterator::new(&input.instructions);
            let mut states: HashMap<usize, usize> = HashMap::new();
            let mut step = 0usize;

            loop {
                let (iter_state, next_dir) = inst_stream.next();
                // iter_state == 0 is a complete hack, but it works
                if fast_mapper.ends_with_z(current) && iter_state == 0 {
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

                current = fast_mapper.get(current, next_dir);
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

    let mut res = nums[0];
    for &b in &nums[1..] {
        res = res * b / gcd(res, b);
    }
    res
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}

struct FastEdgeMapper {
    map: HashMap<String, usize>,
    edges: Vec<usize>,
    zs: Vec<usize>,
}

impl FastEdgeMapper {
    fn new(edges: &HashMap<String, (String, String)>) -> Self {
        let mut map = HashMap::new();
        let mut zs = Vec::new();

        let mut counter = 0;
        for from in edges.keys() {
            map.insert(from.clone(), counter);
            if from.ends_with('Z') {
                zs.push(counter);
            }
            counter += 1;
        }

        let mut data = vec![0; counter * 2];

        for (from, (left, right)) in edges {
            let from = *map.get(from).unwrap();
            let left = *map.get(left).unwrap();
            let right = *map.get(right).unwrap();

            data[2 * from] = left;
            data[2 * from + 1] = right;
        }

        FastEdgeMapper {
            map,
            edges: data,
            zs,
        }
    }

    fn str_to_id(&self, s: &str) -> usize {
        *self.map.get(s).unwrap()
    }

    fn ends_with_z(&self, id: usize) -> bool {
        self.zs.contains(&id)
    }

    fn get(&self, from: usize, dir: Direction) -> usize {
        match dir {
            Direction::Left => self.edges[2 * from],
            Direction::Right => self.edges[2 * from + 1],
        }
    }
}
