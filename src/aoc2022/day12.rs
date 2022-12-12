use crate::aoc2022::grid::Grid;
use crate::aoc2022::Aoc2022;
use crate::traits::days::Day12;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day12> for Aoc2022 {
    type Parsed = Grid<u8>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input)
    }
}

impl Solution<Day12> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Grid<u8>) -> u32 {
        todo!()
    }

    fn part2(input: &Grid<u8>) -> u32 {
        todo!()
    }
}
