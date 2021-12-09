use crate::aoc2019::intcode::IntCodeVM;
use crate::aoc2019::Aoc2019;
use advent_of_code_traits::days::Day2;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

impl ParseInput<Day2> for Aoc2019 {
    type Parsed = Vec<i32>;

    fn parse_input(input: &str) -> Vec<i32> {
        input
            .trim()
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
}

impl Solution<Day2> for Aoc2019 {
    type Part1Output = i32;
    type Part2Output = i32;

    fn part1(input: &Vec<i32>) -> i32 {
        let mut memory = input.clone();
        memory[1] = 12;
        memory[2] = 2;

        let mut vm = IntCodeVM::new(memory);
        while vm.running {
            vm.step();
        }
        vm.memory[0]
    }

    fn part2(input: &Vec<i32>) -> i32 {
        let target = 19690720;
        for noun in 0..input.len() {
            for verb in 0..input.len() {
                let noun = noun as i32;
                let verb = verb as i32;

                let mut memory = input.clone();
                memory[1] = noun;
                memory[2] = verb;

                let mut vm = IntCodeVM::new(memory);
                while vm.running {
                    vm.step();
                }
                if vm.memory[0] == target {
                    return noun * 100 + verb;
                }
            }
        }
        panic!("not found")
    }
}
