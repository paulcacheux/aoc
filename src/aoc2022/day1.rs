use crate::aoc2022::Aoc2022;
use crate::traits::days::Day1;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day1> for Aoc2022 {
    type Parsed = Vec<Vec<u32>>;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut res = vec![vec![]];

        for line in input.lines() {
            let line = line.trim();

            if let Ok(value) = line.parse() {
                res.last_mut().unwrap().push(value);
            } else {
                res.push(Vec::new());
            }
        }

        res
    }
}

impl Solution<Day1> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Vec<u32>>) -> u32 {
        input.iter().map(|elf| elf.iter().sum()).max().unwrap()
    }

    fn part2(input: &Vec<Vec<u32>>) -> u32 {
        let sums = input.iter().map(|elf| elf.iter().sum());
        let (mut max1, mut max2, mut max3) = (0, 0, 0);

        for s in sums {
            if s >= max1 {
                max3 = max2;
                max2 = max1;
                max1 = s;
            } else if s >= max2 {
                max3 = max2;
                max2 = s;
            } else if s >= max3 {
                max3 = s;
            }
        }

        max1 + max2 + max3
    }
}
