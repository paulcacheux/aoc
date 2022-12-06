use crate::aoc2022::Aoc2022;
use crate::traits::days::Day6;
use crate::traits::ParseInput;
use crate::traits::Solution;
use std::num::NonZeroUsize;

impl ParseInput<Day6> for Aoc2022 {
    type Parsed = Vec<u8>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.trim().bytes().collect()
    }
}

fn cindex(c: u8) -> usize {
    (c - b'a') as usize
}

const INDEX_SPACE_SIZE: usize = 26;

fn compute_first_index<const SIZE: usize>(input: &[u8]) -> usize {
    let mut start = 0;
    'main: while start < (input.len() - SIZE) {
        let mut stats = [None::<NonZeroUsize>; INDEX_SPACE_SIZE];
        for (i, &c) in input.iter().enumerate().skip(start).take(SIZE) {
            let ci = cindex(c);
            if let Some(pos) = stats[ci] {
                start = pos.get(); // skip all the repetitive checks
                continue 'main;
            }
            stats[ci] = NonZeroUsize::new(i + 1);
        }
        return start + SIZE;
    }

    unreachable!()
}

impl Solution<Day6> for Aoc2022 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<u8>) -> usize {
        compute_first_index::<4>(input)
    }

    fn part2(input: &Vec<u8>) -> usize {
        compute_first_index::<14>(input)
    }
}
