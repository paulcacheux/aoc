use crate::aoc::Aoc2021;
use advent_of_code_traits::days::Day5;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl ParseInput<Day5> for Aoc2021 {
    type Parsed = Vec<(Point, Point)>;

    fn parse_input(input: &str) -> Vec<(Point, Point)> {
        let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

        let mut pairs = Vec::new();
        for line in input.lines() {
            let c = re.captures(line).expect("non matching line");
            let x1: u32 = c.get(1).unwrap().as_str().parse().unwrap();
            let y1: u32 = c.get(2).unwrap().as_str().parse().unwrap();
            let p1 = Point { x: x1, y: y1 };
            let x2: u32 = c.get(3).unwrap().as_str().parse().unwrap();
            let y2: u32 = c.get(4).unwrap().as_str().parse().unwrap();
            let p2 = Point { x: x2, y: y2 };

            pairs.push((p1, p2));
        }

        pairs
    }
}

impl Solution<Day5> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<(Point, Point)>) -> usize {
        let mut points: HashMap<Point, usize> = HashMap::new();

        for pair in input {
            let (Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) = *pair;
            if x1 != x2 && y1 != y2 {
                continue;
            }

            let (x1, x2) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            let (y1, y2) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

            for x in x1..=x2 {
                for y in y1..=y2 {
                    *points.entry(Point { x, y }).or_default() += 1;
                }
            }
        }

        points.values().filter(|&&v| v > 1).count()
    }

    fn part2(input: &Vec<(Point, Point)>) -> usize {
        let mut points: HashMap<Point, usize> = HashMap::new();

        for pair in input {
            let (Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) = *pair;

            if x1 == x2 {
                let (y1, y2) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
                for y in y1..=y2 {
                    *points.entry(Point { x: x1, y }).or_default() += 1;
                }
            } else if y1 == y2 {
                let (x1, x2) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
                for x in x1..=x2 {
                    *points.entry(Point { x, y: y1 }).or_default() += 1;
                }
            } else {
                let xvalues = inclusive_range(x1, x2);
                let yvalues = inclusive_range(y1, y2);

                for (x, y) in xvalues.into_iter().zip(yvalues) {
                    *points.entry(Point { x, y }).or_default() += 1;
                }
            }
        }

        points.values().filter(|&&v| v > 1).count()
    }
}

fn inclusive_range(a: u32, b: u32) -> Vec<u32> {
    let range = if a <= b { a..=b } else { b..=a };
    if a <= b {
        range.into_iter().collect()
    } else {
        range.into_iter().rev().collect()
    }
}
