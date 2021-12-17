use std::cmp::Ordering;
use std::ops::Range;
use std::ops::RangeInclusive;

use crate::aoc2021::Aoc2021;
use advent_of_code_traits::days::Day17;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct TargetArea {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl TargetArea {
    fn contains(&self, x: i32, y: i32) -> bool {
        self.x.contains(&x) && self.y.contains(&y)
    }
}

impl ParseInput<Day17> for Aoc2021 {
    type Parsed = TargetArea;

    fn parse_input(input: &str) -> TargetArea {
        let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();

        let line = input.lines().next().unwrap();
        let captures = re.captures(line).unwrap();

        let xstart = captures[1].parse().unwrap();
        let xend = captures[2].parse().unwrap();
        let x = xstart..=xend;

        let ystart = captures[3].parse().unwrap();
        let yend = captures[4].parse().unwrap();
        let y = ystart..=yend;

        TargetArea { x, y }
    }
}

#[derive(Debug)]
struct State {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl State {
    fn new(vx: i32, vy: i32) -> Self {
        State { x: 0, y: 0, vx, vy }
    }

    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        match self.vx.cmp(&0) {
            Ordering::Less => self.vx += 1,
            Ordering::Equal => {}
            Ordering::Greater => self.vx -= 1,
        }
        self.vy -= 1;
    }

    fn can_go_to(&self, target: &TargetArea) -> bool {
        // if we go down, and we are already under the target, it's over
        if self.vy < 0 && self.y < *target.y.start() {
            return false;
        }

        // if we overshoot the target, it's over
        if self.x > *target.x.end() {
            return false;
        }

        if self.vx == 0 && !target.x.contains(&self.x) {
            return false;
        }

        true
    }
}

fn compute_local_max(target: &TargetArea, vx: i32, vy: i32) -> Option<i32> {
    let mut local_max = None;
    let mut state = State::new(vx, vy);
    while state.can_go_to(target) {
        state.step();

        match local_max {
            Some(m) if m < state.y => {
                local_max = Some(state.y);
            }
            None => {
                local_max = Some(state.y);
            }
            _ => {}
        }

        if target.contains(state.x, state.y) {
            return local_max;
        }
    }
    None
}

fn is_valid_vx(vx: i32, target: &RangeInclusive<i32>) -> bool {
    let mut vx = vx;
    let mut current = 0;
    while current <= *target.end() {
        if target.contains(&current) {
            return true;
        }
        current += vx;
        vx -= 1;
    }
    false
}

fn compute_vx_range<'t>(target: &'t TargetArea) -> impl Iterator<Item = i32> + 't {
    let vx_start = (*target.x.start() as f32).sqrt() as i32 - 1;
    let vx_end = *target.x.end() + 1;
    (vx_start..vx_end)
        .into_iter()
        .filter(|vx| is_valid_vx(*vx, &target.x))
}

impl Solution<Day17> for Aoc2021 {
    type Part1Output = i32;
    type Part2Output = usize;

    fn part1(input: &TargetArea) -> i32 {
        let mut max = None;

        for vx in compute_vx_range(input) {
            'yloop: for vy in 0..300 {
                if let Some(local_max) = compute_local_max(input, vx, vy) {
                    match max {
                        Some(m) if m < local_max => {
                            max = Some(local_max);
                        }
                        Some(m) if m > local_max => {
                            break 'yloop;
                        }
                        None => {
                            max = Some(local_max);
                        }
                        _ => {}
                    }
                }
            }
        }

        max.unwrap()
    }

    fn part2(input: &TargetArea) -> usize {
        let mut velocities = 0;

        for vx in compute_vx_range(input) {
            for vy in *input.y.start()..500 {
                if compute_local_max(input, vx, vy).is_some() {
                    velocities += 1;
                }
            }
        }

        velocities
    }
}
