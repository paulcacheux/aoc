use crate::aoc2022::Aoc2022;
use crate::traits::days::Day7;
use crate::traits::ParseInput;
use crate::traits::Solution;
use std::collections::HashMap;
use std::path::PathBuf;

impl ParseInput<Day7> for Aoc2022 {
    type Parsed = Vec<String>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.lines().map(ToOwned::to_owned).collect()
    }
}

const CD_PREFIX: &str = "$ cd ";
const DIR_PREFIX: &str = "dir ";

fn compute_dir_sizes(lines: &[String]) -> HashMap<PathBuf, u64> {
    let mut directory_size: HashMap<PathBuf, u64> = HashMap::new();
    let mut current_stack = vec!["/"];

    for line in lines {
        match line.trim() {
            "$ cd /" => {
                current_stack.clear();
                current_stack.push("/")
            }
            "$ cd .." => {
                current_stack.pop();
            }
            "$ ls" => {}
            cd if line.starts_with(CD_PREFIX) => {
                let part = cd[CD_PREFIX.len()..].trim();
                current_stack.push(part);
            }
            _ if line.starts_with(DIR_PREFIX) => {}
            other => {
                let mut parts = other.split_ascii_whitespace();
                let size: u64 = parts.next().unwrap().parse().unwrap();

                let mut current = PathBuf::new();
                for &part in &current_stack {
                    current.push(part);
                    *directory_size.entry(current.clone()).or_default() += size;
                }
            }
        }
    }
    directory_size
}

impl Solution<Day7> for Aoc2022 {
    type Part1Output = u64;
    type Part2Output = u64;

    fn part1(input: &Vec<String>) -> u64 {
        let directory_size = compute_dir_sizes(input);
        directory_size
            .values()
            .filter(|&&size| size <= 100000)
            .sum()
    }

    fn part2(input: &Vec<String>) -> u64 {
        let directory_size = compute_dir_sizes(input);
        let root_size = directory_size.get(&PathBuf::from("/")).unwrap();
        let search_for = 30000000 - (70000000 - root_size);

        directory_size
            .values()
            .filter(|&&size| size >= search_for)
            .min()
            .copied()
            .unwrap()
    }
}
