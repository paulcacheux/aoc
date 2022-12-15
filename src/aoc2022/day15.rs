use std::ops::RangeInclusive;

use regex::Regex;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day15;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Default)]
pub struct Sensor {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

impl Sensor {
    fn distance_to_beacon(&self) -> u32 {
        let (bx, by) = self.beacon;
        self.distance_to(bx, by)
    }

    fn distance_to(&self, x: i32, y: i32) -> u32 {
        let (sx, sy) = self.sensor;
        sx.abs_diff(x) + sy.abs_diff(y)
    }
}

impl ParseInput<Day15> for Aoc2022 {
    type Parsed = Vec<Sensor>;

    fn parse_input(input: &str) -> Self::Parsed {
        let line_re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();

        input
            .lines()
            .map(str::trim)
            .map(|line| {
                let captures = line_re.captures(line).unwrap();
                let sx = captures.get(1).unwrap().as_str().parse().unwrap();
                let sy = captures.get(2).unwrap().as_str().parse().unwrap();
                let bx = captures.get(3).unwrap().as_str().parse().unwrap();
                let by = captures.get(4).unwrap().as_str().parse().unwrap();
                Sensor {
                    sensor: (sx, sy),
                    beacon: (bx, by),
                }
            })
            .collect()
    }
}

impl Solution<Day15> for Aoc2022 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<Sensor>) -> usize {
        let y = 2000000i32; // 10 for test input
        let mut ranges = Vec::with_capacity(input.len());
        let mut sensor_and_beacons = Vec::with_capacity(input.len() * 2);

        // collect ranges, and points to substract
        for sensor in input {
            let sdtb = sensor.distance_to_beacon();
            let dy = y.abs_diff(sensor.sensor.1);
            if dy > sdtb {
                continue;
            }

            let minx = sensor.sensor.0 - sdtb as i32 + dy as i32;
            let maxx = sensor.sensor.0 + sdtb as i32 - dy as i32;
            ranges.push(minx..=maxx);

            sensor_and_beacons.push(sensor.sensor);
            sensor_and_beacons.push(sensor.beacon);
        }
        sensor_and_beacons.sort();
        sensor_and_beacons.dedup();

        // coallesce ranges with union
        ranges.sort_by_key(|r| *r.start());
        let mut new_ranges: Vec<RangeInclusive<i32>> = Vec::new();
        for r in ranges {
            if let Some(last) = new_ranges.last_mut() {
                if *last.end() >= *r.start() {
                    *last = *last.start()..=std::cmp::max(*last.end(), *r.end());
                    continue;
                }
            }
            new_ranges.push(r);
        }

        // count
        let mut count = 0;
        for r in new_ranges {
            count += (r.end() - r.start() + 1) as usize;

            for point in &sensor_and_beacons {
                if point.1 == y && r.contains(&point.0) {
                    count -= 1;
                }
            }
        }
        count
    }

    fn part2(input: &Vec<Sensor>) -> usize {
        let meta_range = 0..=4000000;
        // let meta_range = 0..=20;

        for sensor in input {
            // walk around the frontier, and check if in any other sensor radius
            let sdtb = sensor.distance_to_beacon() as i32;
            let mut miny = sensor.sensor.1 - sdtb - 1;
            let mut maxy = sensor.sensor.1 + sdtb + 1;

            if miny < *meta_range.start() {
                miny = *meta_range.start();
            }
            if maxy > *meta_range.end() {
                maxy = *meta_range.end();
            }

            for y in miny..=maxy {
                let dy = y.abs_diff(sensor.sensor.1);
                let minx = sensor.sensor.0 - sdtb + dy as i32 - 1;
                let maxx = sensor.sensor.0 + sdtb - dy as i32 + 1;

                if meta_range.contains(&minx) {
                    if let Some(res) = part2_is_res(input, minx, y) {
                        // stop as soon as one if found
                        return res;
                    }
                }

                if meta_range.contains(&maxx) {
                    if let Some(res) = part2_is_res(input, maxx, y) {
                        return res;
                    }
                }
            }
        }
        unreachable!()
    }
}

#[inline]
fn part2_is_res(sensors: &[Sensor], x: i32, y: i32) -> Option<usize> {
    if sensors
        .iter()
        .all(|sensor| sensor.distance_to(x, y) > sensor.distance_to_beacon())
    {
        Some(x as usize * 4000000 + y as usize)
    } else {
        None
    }
}
