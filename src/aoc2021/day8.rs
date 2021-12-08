use crate::aoc2021::Aoc2021;
use advent_of_code_traits::days::Day8;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use std::collections::HashMap;

#[derive(Debug)]
pub struct LineEntry {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

impl ParseInput<Day8> for Aoc2021 {
    type Parsed = Vec<LineEntry>;

    fn parse_input(input: &str) -> Vec<LineEntry> {
        let mut entries = Vec::new();
        for line in input.lines() {
            let mut parts = line.split('|');
            let pattern_part = parts.next().unwrap();
            let output_part = parts.next().unwrap();

            let patterns = pattern_part
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.trim().to_owned())
                .collect();
            let outputs = output_part
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.trim().to_owned())
                .collect();

            let entry = LineEntry { patterns, outputs };
            assert_eq!(entry.patterns.len(), 10);
            assert_eq!(entry.outputs.len(), 4);
            entries.push(entry);
        }
        entries
    }
}

/*
0 => abcefg, 6
1 => cf, 2
2 => acdeg, 5
3 => acdfg, 5
4 => bdcf, 4
5 => abdfg, 5
6 => abdefg, 6
7 => acf, 3
8 => abcdefg, 7
9 => abcdfg, 6

groups:

     1 | 7 | 4 | 2,3,5 | 0,6,9 | 8
a => 0 | 1 | 0 | 3     | 3     | 1
b => 0 | 0 | 1 | 1     | 3     | 1
c => 1 | 1 | 1 | 2     | 2     | 1
d => 0 | 0 | 1 | 3     | 2     | 1
e => 0 | 0 | 0 | 1     | 2     | 1
f => 1 | 1 | 1 | 2     | 3     | 1
g => 0 | 0 | 0 | 3     | 3     | 1

*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Signature {
    inner: [u8; 8],
}

impl Signature {
    fn from_vec(values: Vec<u8>) -> Self {
        assert_eq!(values.len(), 8);

        let mut inner = [0; 8];
        for (i, val) in values.into_iter().enumerate() {
            inner[i] = val;
        }

        Self { inner }
    }

    fn inc(&mut self, index: usize) {
        self.inner[index] += 1;
    }
}

macro_rules! sig {
    ($($x:expr),+ $(,)?) => {
        Signature::from_vec(vec![0, 0, $($x),+, 1])
    };
}

fn compute_true_signatures() -> HashMap<u8, Signature> {
    let mut res = HashMap::new();
    res.insert(b'a', sig![0, 1, 0, 3, 3]);
    res.insert(b'b', sig![0, 0, 1, 1, 3]);
    res.insert(b'c', sig![1, 1, 1, 2, 2]);
    res.insert(b'd', sig![0, 0, 1, 3, 2]);
    res.insert(b'e', sig![0, 0, 0, 1, 2]);
    res.insert(b'f', sig![1, 1, 1, 2, 3]);
    res.insert(b'g', sig![0, 0, 0, 3, 3]);
    res
}

fn get_mapped(signature: &Signature, true_sigs: &HashMap<u8, Signature>) -> Option<u8> {
    for (target, sig) in true_sigs {
        if signature == sig {
            return Some(*target);
        }
    }
    None
}

fn digit_from_str(s: &[u8]) -> u8 {
    match s {
        b"abcefg" => 0,
        b"cf" => 1,
        b"acdeg" => 2,
        b"acdfg" => 3,
        b"bcdf" => 4,
        b"abdfg" => 5,
        b"abdefg" => 6,
        b"acf" => 7,
        b"abcdefg" => 8,
        b"abcdfg" => 9,
        _ => unreachable!(),
    }
}

impl Solution<Day8> for Aoc2021 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<LineEntry>) -> u32 {
        let mut counter = 0;
        for entry in input {
            for output in &entry.outputs {
                match output.len() {
                    2 | 3 | 4 | 7 => counter += 1,
                    _ => {}
                }
            }
        }
        counter
    }

    fn part2(input: &Vec<LineEntry>) -> u32 {
        let true_signatures = compute_true_signatures();

        let mut sum = 0;
        for entry in input {
            let mut signatures = vec![Signature::default(); 7];

            for pattern in &entry.patterns {
                for c in pattern.bytes() {
                    signatures[(c - b'a') as usize].inc(pattern.len());
                }
            }

            let mut mapping = HashMap::new();

            for (source, sig) in signatures.iter().enumerate() {
                let source = source as u8 + b'a';
                let target = get_mapped(sig, &true_signatures);
                mapping.insert(source, target.unwrap());
            }

            let mut final_output = 0u32;
            for output in &entry.outputs {
                let mut mapped: Vec<u8> = output
                    .bytes()
                    .map(|c| mapping.get(&c).unwrap())
                    .copied()
                    .collect();
                mapped.sort_unstable();
                let digit = digit_from_str(&mapped);
                final_output = final_output * 10 + digit as u32;
            }
            sum += final_output;
        }
        sum
    }
}
