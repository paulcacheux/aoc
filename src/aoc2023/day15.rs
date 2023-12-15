use crate::aoc2023::Aoc2023;
use crate::traits::days::Day15;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day15> for Aoc2023 {
    type Parsed = Vec<String>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.split(',').map(|part| part.to_owned()).collect()
    }
}

impl Solution<Day15> for Aoc2023 {
    type Part1Output = u64;
    type Part2Output = u64;

    fn part1(input: &Vec<String>) -> u64 {
        let mut sum = 0;
        for part in input {
            let part = part.trim();
            let mut hasher = Hasher::default();
            for b in part.bytes() {
                hasher.write(b);
            }
            sum += hasher.state;
        }
        sum
    }

    fn part2(_input: &Vec<String>) -> u64 {
        todo!()
    }
}

#[derive(Debug, Default)]
struct Hasher {
    state: u64,
}

impl Hasher { 
    fn write(&mut self, c: u8) {
        self.state += c as u64;
        self.state *= 17;
        self.state %= 256;
    }
}