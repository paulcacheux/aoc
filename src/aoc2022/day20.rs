use std::collections::VecDeque;

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
    type Part2Output = i32;

    fn part1(input: &Vec<i32>) -> i32 {
        let mut ring: VecDeque<_> = input.iter().copied().enumerate().collect();
        mix(&mut ring, input.len());
        extract_res(ring)
    }

    fn part2(_input: &Vec<i32>) -> i32 {
        todo!()
    }
}

#[allow(dead_code)]
fn dbg_ring(ring: &VecDeque<(usize, i32)>) {
    for (_, val) in ring.iter() {
        print!("{val}, ")
    }
    println!()
}

fn mix(ring: &mut VecDeque<(usize, i32)>, count: usize) {
    for i in 0..count {
        let (j, (_, val)) = ring
            .iter()
            .copied()
            .enumerate()
            .find(|&(_, (index, _))| index == i)
            .unwrap();

        // pop elem
        ring.rotate_left(j % ring.len());
        // ring.pop_front();
        let t = ring.pop_front().unwrap();
        assert_eq!(t.1, val);

        // push elem
        if val >= 0 {
            ring.rotate_left((val as usize) % ring.len());
            ring.push_front((i, val));
        } else {
            ring.rotate_right((-val as usize) % ring.len());
            ring.push_front((i, val));
        }

        // dbg_ring(&ring);
    }
}

fn extract_res(mut ring: VecDeque<(usize, i32)>) -> i32 {
    let index0 = ring
        .iter()
        .copied()
        .find_position(|&(_, val)| val == 0)
        .unwrap()
        .0;

    ring.rotate_left(index0);

    let mut res = 0;
    for i in 1..=3 {
        res += ring[(1000 * i) % ring.len()].1;
    }

    res
}
