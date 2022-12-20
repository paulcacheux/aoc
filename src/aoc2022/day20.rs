use itertools::Itertools;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day20;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day20> for Aoc2022 {
    type Parsed = Vec<i32>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .split_ascii_whitespace()
            .map(|w| w.parse().unwrap())
            .collect()
    }
}

impl Solution<Day20> for Aoc2022 {
    type Part1Output = i32;
    type Part2Output = u32;

    fn part1(input: &Vec<i32>) -> i32 {
        let mut cycle = Cycle::new(input);

        let mut i = 0;
        for _ in 0..input.len() {
            let (j, (p, val)) = cycle
                .inner
                .iter()
                .copied()
                .enumerate()
                .find(|&(_, (pi, _))| pi == i)
                .unwrap();

            if val > 0 {
                for offset in 0..val {
                    let t = cycle.get(j, offset + 1);
                    cycle.set(j, offset, t);
                }
                cycle.set(j, val, (p, val));
            } else if val < 0 {
                let mut fake_val = val;
                while fake_val < 0 {
                    fake_val += cycle.inner.len() as i32;
                }
                fake_val -= 1;

                for offset in 0..fake_val {
                    let t = cycle.get(j, offset + 1);
                    cycle.set(j, offset, t);
                }
                cycle.set(j, fake_val, (p, val));
            }

            i += 1;

            let d = cycle
                .inner
                .iter()
                .copied()
                .map(|p| format!("{} - {}", p.1, p.0))
                .collect_vec();
            // dbg!(d);
        }

        let (index0, _) = cycle
            .inner
            .iter()
            .copied()
            .enumerate()
            .find(|(_, p)| p.1 == 0)
            .unwrap();

        let a = cycle.get(index0, 1000).1;
        let b = cycle.get(index0, 2000).1;
        let c = cycle.get(index0, 3000).1;
        a + b + c
    }

    fn part2(_input: &Vec<i32>) -> u32 {
        todo!()
    }
}

fn compute_new_index(base: usize, offset: i32, len: usize) -> usize {
    if offset >= 0 {
        return (base + offset as usize) % len;
    }

    (base as i64 + offset as i64).rem_euclid(len as _) as usize
}

struct Cycle {
    inner: Vec<(usize, i32)>,
}

impl Cycle {
    fn new(input: &[i32]) -> Self {
        Cycle {
            inner: input.iter().copied().enumerate().collect(),
        }
    }

    fn get(&self, base: usize, offset: i32) -> (usize, i32) {
        self.inner[compute_new_index(base, offset, self.inner.len())]
    }

    fn set(&mut self, base: usize, offset: i32, new_val: (usize, i32)) {
        let len = self.inner.len();
        self.inner[compute_new_index(base, offset, len)] = new_val;
    }
}
