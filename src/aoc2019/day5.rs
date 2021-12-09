use crate::aoc2019::intcode::IntCodeVM;
use crate::aoc2019::Aoc2019;
use advent_of_code_traits::days::Day5;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

impl ParseInput<Day5> for Aoc2019 {
    type Parsed = Vec<i32>;

    fn parse_input(input: &str) -> Vec<i32> {
        input
            .trim()
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl Solution<Day5> for Aoc2019 {
    type Part1Output = i32;
    type Part2Output = i32;

    fn part1(input: &Vec<i32>) -> i32 {
        let mut vm = IntCodeVM::with_input(input.clone(), vec![1]);
        while vm.running {
            vm.step();
        }
        vm.output.pop().unwrap()
    }

    fn part2(input: &Vec<i32>) -> i32 {
        let mut vm = IntCodeVM::with_input(input.clone(), vec![5]);
        while vm.running {
            vm.step();
        }
        vm.output.pop().unwrap()
    }
}
