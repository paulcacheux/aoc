use crate::aoc2023::Aoc2023;
use crate::traits::days::Day5;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug)]
struct MappingRange {
    destination: u64,
    source: u64,
    len: u64,
}

#[derive(Debug)]
struct RangeTransformResult {
    pre: Option<(u64, u64)>,
    post: Option<(u64, u64)>,
    transformed: (u64, u64),
}

impl MappingRange {
    fn transform(&self, value: u64) -> Option<u64> {
        if self.source <= value && value < self.source + self.len {
            Some(self.destination + (value - self.source))
        } else {
            None
        }
    }

    fn transform_range(&self, start: u64, len: u64) -> Option<RangeTransformResult> {
        let common_begin = self.source.max(start);
        let common_end: u64 = (self.source + self.len).min(start + len);

        if common_end <= common_begin + 1 {
            return None;
        }

        let common_len = common_end - common_begin;

        let pre = if common_begin > start {
            Some((start, common_begin - start))
        } else {
            None
        };

        let post = if common_end < start + len {
            Some((common_end, start + len - common_end))
        } else {
            None
        };

        let transformed = (self.destination + (common_begin - self.source), common_len);

        Some(RangeTransformResult {
            pre,
            post,
            transformed,
        })
    }
}

#[derive(Debug, Default)]
pub struct Mapping {
    from: String,
    to: String,
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn transform(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if let Some(res) = range.transform(value) {
                return res;
            }
        }
        value
    }

    fn transform_range(&self, start: u64, len: u64) -> Vec<(u64, u64)> {
        let mut transformed = Vec::new();
        let mut open_ranges = vec![(start, len)];

        while let Some((curr_start, curr_len)) = open_ranges.pop() {
            let mut found = false;
            for range in &self.ranges {
                if let Some(res) = range.transform_range(curr_start, curr_len) {
                    open_ranges.extend(res.pre);
                    open_ranges.extend(res.post);
                    transformed.push(res.transformed);
                    found = true;
                    break;
                }
            }
            if !found {
                transformed.push((curr_start, curr_len));
            }
        }
        transformed
    }
}

#[derive(Debug)]
pub struct Input {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

impl ParseInput<Day5> for Aoc2023 {
    type Parsed = Input;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut lines_iter = input.lines();

        let seeds = lines_iter
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_ascii_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();

        lines_iter.next().unwrap(); // skip empty line

        let mut mappings = Vec::new();
        let mut current = Mapping::default();
        for line in lines_iter {
            if let Some(desc) = line.strip_suffix(" map:") {
                let (from, to) = desc.split_once("-to-").unwrap();
                current.from = from.to_owned();
                current.to = to.to_owned();
            } else if line.trim().is_empty() {
                mappings.push(current);
                current = Mapping::default();
            } else {
                let mut values = line.split_ascii_whitespace();
                let destination = values.next().unwrap().parse().unwrap();
                let source = values.next().unwrap().parse().unwrap();
                let len = values.next().unwrap().parse().unwrap();
                current.ranges.push(MappingRange {
                    destination,
                    source,
                    len,
                })
            }
        }

        if !current.from.is_empty() {
            mappings.push(current);
        }

        Input { seeds, mappings }
    }
}

impl Solution<Day5> for Aoc2023 {
    type Part1Output = u64;
    type Part2Output = u64;

    fn part1(input: &Input) -> u64 {
        input
            .seeds
            .iter()
            .map(|&seed| {
                let mut current = seed;
                for mapping in &input.mappings {
                    current = mapping.transform(current);
                }
                current
            })
            .min()
            .unwrap()
    }

    fn part2(input: &Input) -> u64 {
        input
            .seeds
            .chunks_exact(2)
            .map(|seed| (seed[0], seed[1]))
            .flat_map(|(start, len)| {
                let mut current = vec![(start, len)];
                for mapping in &input.mappings {
                    current = current
                        .into_iter()
                        .flat_map(|(s, l)| mapping.transform_range(s, l))
                        .collect();
                }
                current
            })
            .map(|(start, _)| start)
            .min()
            .unwrap()
    }
}
