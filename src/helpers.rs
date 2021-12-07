use std::fs::File;

use advent_of_code_traits::{ParseEachInput, Part1, Part2, Solution};
use serde::Deserialize;

pub struct Results {
    days: Vec<DayResult>,
}

impl Results {
    pub fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let f = File::open("./results/2021.json")?;
        let days: Vec<DayResult> = serde_json::from_reader(f)?;

        Ok(Self { days })
    }

    pub fn results_for_day(&self, day: usize) -> Option<&DayResult> {
        for r in &self.days {
            if r.day == day {
                return Some(r);
            }
        }
        None
    }
}

#[derive(Debug, Deserialize)]
pub struct DayResult {
    day: usize,
    part1: String,
    part2: String,
}

macro_rules! inner_run {
    ($P:tt, $F:expr, $input:expr, $expected:expr) => {{
        let parsed_input = <A as ParseEachInput<D, $P>>::parse_input($input);
        let start = std::time::Instant::now();
        let output = $F(&parsed_input);
        let elapsed = start.elapsed();
        println!("Day {}, Part {}: {}, in {:?}", D, $P, output, elapsed,);
        if let Some(expected) = $expected {
            assert_eq!(expected, output.to_string());
        }
    }};
}

pub fn run<A: Solution<D>, const D: u32>(input: &str, result: Option<&DayResult>) {
    inner_run!(Part1, A::part1, input, result.map(|r| r.part1.as_str()));
    inner_run!(Part2, A::part2, input, result.map(|r| r.part2.as_str()));
}
