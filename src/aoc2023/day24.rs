use crate::aoc2023::Aoc2023;
use crate::traits::days::Day24;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl ParseInput<Day24> for Aoc2023 {
    type Parsed = Vec<Ball>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let (pos, speed) = line.split_once(" @ ").unwrap();
                let mut pos = pos.split(',').map(|val| val.trim().parse().unwrap());
                let px = pos.next().unwrap();
                let py = pos.next().unwrap();
                let pz = pos.next().unwrap();

                let mut speed = speed.split(',').map(|val| val.trim().parse().unwrap());
                let vx = speed.next().unwrap();
                let vy = speed.next().unwrap();
                let vz = speed.next().unwrap();

                Ball {
                    px,
                    py,
                    pz,
                    vx,
                    vy,
                    vz,
                }
            })
            .collect()
    }
}

impl Solution<Day24> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = u32;

    fn part1(input: &Vec<Ball>) -> usize {
        const MIN: f64 = 200000000000000.0;
        const MAX: f64 = 400000000000000.0;
        let bounds = MIN..=MAX;

        let mut counter = 0;
        for (ia, a) in input.iter().enumerate() {
            for b in &input[ia + 1..] {
                let t = ((a.px - b.px) * -b.vy - (a.py - b.py) * -b.vx)
                    / (-a.vx * -b.vy - -a.vy * -b.vx);
                let u = ((a.px - b.px) * -a.vy - (a.py - b.py) * -a.vx)
                    / (-a.vx * -b.vy - -a.vy * -b.vx);

                if t < 0.0 || u < 0.0 {
                    continue;
                }

                let interx = a.px + t * a.vx;
                let intery = a.py + t * a.vy;

                if bounds.contains(&interx) && bounds.contains(&intery) {
                    counter += 1;
                }
            }
        }
        counter
    }

    fn part2(_input: &Vec<Ball>) -> u32 {
        todo!()
    }
}
