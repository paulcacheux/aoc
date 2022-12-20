use std::collections::VecDeque;

use itertools::Itertools;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day20;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day20> for Aoc2022 {
    type Parsed = Vec<i64>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .split_ascii_whitespace()
            .map(|w| w.parse().unwrap())
            .collect()
    }
}

impl Solution<Day20> for Aoc2022 {
    type Part1Output = i64;
    type Part2Output = i64;

    fn part1(input: &Vec<i64>) -> i64 {
        let mut ring: VecDeque<_> = input.iter().copied().enumerate().collect();
        mix(&mut ring, input.len());
        extract_res(ring)
    }

    fn part2(input: &Vec<i64>) -> i64 {
        const DEC_KEY: i64 = 811589153;
        let mut ring: VecDeque<_> = input
            .iter()
            .copied()
            .map(|val| val * DEC_KEY)
            .enumerate()
            .collect();

        for _ in 0..10 {
            mix(&mut ring, input.len());
        }
        extract_res(ring)
    }
}

#[allow(dead_code)]
fn dbg_ring(ring: &VecDeque<(usize, i64)>) {
    for (_, val) in ring.iter() {
        print!("{val}, ")
    }
    println!()
}

fn mix(ring: &mut VecDeque<(usize, i64)>, count: usize) {
    for i in 0..count {
        let (j, (_, val)) = ring
            .iter()
            .copied()
            .find_position(|&(index, _)| index == i)
            .unwrap();

        // pop elem
        ring.rotate_left(j % ring.len());
        // ring.pop_front();
        let t = ring.pop_front().unwrap();
        assert_eq!(t.1, val);

        // push elem
        if val >= 0 {
            ring.rotate_left((val as usize) % ring.len());
        } else {
            ring.rotate_right((-val as usize) % ring.len());
        }
        ring.push_front((i, val));

        // dbg_ring(&ring);
    }
}

fn extract_res(mut ring: VecDeque<(usize, i64)>) -> i64 {
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
