use crate::aoc2021::Aoc2021;
use crate::traits::days::Day5;
use crate::traits::ParseInput;
use crate::traits::Solution;
use ahash::AHashMap;
use regex::Regex;

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

fn common(input: &[(Point, Point)], with_diags: bool) -> usize {
    let mut points: AHashMap<Point, usize> = AHashMap::new();

    for pair in input {
        let (Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) = *pair;

        if x1 == x2 {
            for y in InclusiveRangeDir::new(y1, y2) {
                *points.entry(Point { x: x1, y }).or_default() += 1;
            }
        } else if y1 == y2 {
            for x in InclusiveRangeDir::new(x1, x2) {
                *points.entry(Point { x, y: y1 }).or_default() += 1;
            }
        } else if with_diags {
            let xvalues = InclusiveRangeDir::new(x1, x2);
            let yvalues = InclusiveRangeDir::new(y1, y2);

            for (x, y) in xvalues.zip(yvalues) {
                *points.entry(Point { x, y }).or_default() += 1;
            }
        }
    }

    points.values().filter(|&&v| v > 1).count()
}

impl Solution<Day5> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<(Point, Point)>) -> usize {
        common(input, false)
    }

    fn part2(input: &Vec<(Point, Point)>) -> usize {
        common(input, true)
    }
}

struct InclusiveRangeDir {
    start: u32,
    end: u32,
    rev: bool,
    exhausted: bool,
}

impl InclusiveRangeDir {
    fn new(start: u32, end: u32) -> Self {
        InclusiveRangeDir {
            start,
            end,
            rev: start > end,
            exhausted: false,
        }
    }
}

impl Iterator for InclusiveRangeDir {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        let res = Some(self.start);
        if self.start == self.end {
            self.exhausted = true;
        } else if self.rev {
            self.start -= 1;
        } else {
            self.start += 1;
        }
        res
    }
}
