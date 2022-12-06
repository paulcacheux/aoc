use std::fmt::Display;

pub mod days {
    #![allow(non_upper_case_globals, dead_code)]
    pub const Day1: u32 = 1;
    pub const Day2: u32 = 2;
    pub const Day3: u32 = 3;
    pub const Day4: u32 = 4;
    pub const Day5: u32 = 5;
    pub const Day6: u32 = 6;
    pub const Day7: u32 = 7;
    pub const Day8: u32 = 8;
    pub const Day9: u32 = 9;
    pub const Day10: u32 = 10;
    pub const Day11: u32 = 11;
    pub const Day12: u32 = 12;
    pub const Day13: u32 = 13;
    pub const Day14: u32 = 14;
    pub const Day15: u32 = 15;
    pub const Day16: u32 = 16;
    pub const Day17: u32 = 17;
    pub const Day18: u32 = 18;
    pub const Day19: u32 = 19;
    pub const Day20: u32 = 20;
    pub const Day21: u32 = 21;
    pub const Day22: u32 = 22;
    pub const Day23: u32 = 23;
    pub const Day24: u32 = 24;
    pub const Day25: u32 = 25;
}

#[allow(non_upper_case_globals)]
pub const Part1: u32 = 1;
#[allow(non_upper_case_globals)]
pub const Part2: u32 = 2;

pub trait ParseInput<const D: u32> {
    type Parsed;

    fn parse_input(input: &str) -> Self::Parsed;
}

pub trait Solution<const D: u32>: ParseInput<D> {
    type Part1Output: Display;
    type Part2Output: Display;

    fn part1(input: &<Self as ParseInput<D>>::Parsed) -> Self::Part1Output;
    fn part2(input: &<Self as ParseInput<D>>::Parsed) -> Self::Part2Output;
}
