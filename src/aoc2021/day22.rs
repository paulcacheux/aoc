use std::ops::RangeInclusive;

use crate::aoc2021::Aoc2021;
use advent_of_code_traits::days::Day22;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use regex::Regex;

#[derive(Debug)]
pub struct InitStep {
    on: bool,
    xrange: RangeInclusive<i32>,
    yrange: RangeInclusive<i32>,
    zrange: RangeInclusive<i32>,
}

impl ParseInput<Day22> for Aoc2021 {
    type Parsed = Vec<InitStep>;

    fn parse_input(input: &str) -> Vec<InitStep> {
        let re = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)")
            .unwrap();
        let mut steps = Vec::new();
        for line in input.lines() {
            let line = line.trim();
            let captures = re.captures(line).unwrap();

            let on = match &captures[1] {
                "on" => true,
                "off" => false,
                _ => unreachable!(),
            };

            let xstart = captures[2].parse().unwrap();
            let xend = captures[3].parse().unwrap();
            let xrange = xstart..=xend;

            let ystart = captures[4].parse().unwrap();
            let yend = captures[5].parse().unwrap();
            let yrange = ystart..=yend;

            let zstart = captures[6].parse().unwrap();
            let zend = captures[7].parse().unwrap();
            let zrange = zstart..=zend;

            steps.push(InitStep {
                on,
                xrange,
                yrange,
                zrange,
            })
        }
        steps
    }
}

#[derive(Debug)]
struct Part1Grid {
    inner: Vec<bool>,
}

const PART1_LEN: usize = 101;

impl Part1Grid {
    fn new() -> Self {
        Part1Grid {
            inner: vec![false; 101 * 101 * 101],
        }
    }

    fn set(&mut self, x: i32, y: i32, z: i32, value: bool) {
        assert!((-50..=50).contains(&x));
        assert!((-50..=50).contains(&y));
        assert!((-50..=50).contains(&z));

        let x = (x + 50) as usize;
        let y = (y + 50) as usize;
        let z = (z + 50) as usize;

        let index = z * PART1_LEN * PART1_LEN + y * PART1_LEN + x;
        self.inner[index] = value;
    }

    fn count_on(&self) -> usize {
        self.inner.iter().filter(|v| **v).count()
    }
}

fn intersects(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.end() >= b.start() && a.start() <= b.end()
}

#[derive(Debug, Clone)]
struct Part2Cube {
    on: bool,
    xrange: RangeInclusive<i32>,
    yrange: RangeInclusive<i32>,
    zrange: RangeInclusive<i32>,
}

fn overlap(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
    if a.end() < b.start() || b.end() < a.start() {
        None
    } else {
        Some(*a.start().max(b.start())..=*a.end().min(b.end()))
    }
}

impl Part2Cube {
    fn new(step: &InitStep) -> Self {
        Self {
            on: step.on,
            xrange: step.xrange.clone(),
            yrange: step.yrange.clone(),
            zrange: step.zrange.clone(),
        }
    }

    fn carve_out(&self, other: &Self) -> Option<Part2Cube> {
        let xoverlap = overlap(&self.xrange, &other.xrange);
        let yoverlap = overlap(&self.yrange, &other.yrange);
        let zoverlap = overlap(&self.zrange, &other.zrange);

        match (xoverlap, yoverlap, zoverlap) {
            (Some(xoverlap), Some(yoverlap), Some(zoverlap)) => Some(Part2Cube {
                on: !self.on,
                xrange: xoverlap,
                yrange: yoverlap,
                zrange: zoverlap,
            }),
            _ => None,
        }
    }

    fn volume(&self) -> usize {
        let xdiff = (self.xrange.end() - self.xrange.start()) as usize + 1;
        let ydiff = (self.yrange.end() - self.yrange.start()) as usize + 1;
        let zdiff = (self.zrange.end() - self.zrange.start()) as usize + 1;
        xdiff * ydiff * zdiff
    }
}

impl Solution<Day22> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<InitStep>) -> usize {
        let mut grid = Part1Grid::new();
        let grid_range = -50..=50;

        for step in input {
            if intersects(&grid_range, &step.xrange)
                && intersects(&grid_range, &step.yrange)
                && intersects(&grid_range, &step.zrange)
            {
                for x in step.xrange.clone() {
                    for y in step.yrange.clone() {
                        for z in step.zrange.clone() {
                            if grid_range.contains(&x)
                                && grid_range.contains(&y)
                                && grid_range.contains(&z)
                            {
                                grid.set(x, y, z, step.on);
                            }
                        }
                    }
                }
            }
        }
        grid.count_on()
    }

    fn part2(input: &Vec<InitStep>) -> usize {
        let mut cubes: Vec<Part2Cube> = Vec::new();

        for step in input {
            let current = Part2Cube::new(step);

            let mut new_cubes = Vec::new();
            for previous in &cubes {
                new_cubes.extend(previous.carve_out(&current));
            }
            cubes.extend(new_cubes);

            if current.on {
                cubes.push(current);
            }
            // dbg!(&cubes);
        }

        let mut pcounter = 0;
        let mut ncounter = 0;
        for cube in cubes {
            let volume = cube.volume();
            if cube.on {
                pcounter += volume;
            } else {
                ncounter += volume;
            }
        }

        pcounter - ncounter
    }
}
