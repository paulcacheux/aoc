use std::fmt;
use std::fs::File;
use std::time::Duration;

use crate::traits::{ParseInput, Part1, Part2, Solution};
use serde::Deserialize;

pub struct Results {
    days: Vec<DayExpectedResult>,
}

impl Results {
    pub fn parse(year: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let f = File::open(format!("./results/{year}.json"))?;
        let days: Vec<DayExpectedResult> = serde_json::from_reader(f)?;

        Ok(Self { days })
    }

    pub fn results_for_day(&self, day: usize) -> Option<&DayExpectedResult> {
        self.days.iter().find(|&r| r.day == day)
    }
}

#[derive(Debug, Deserialize)]
pub struct DayExpectedResult {
    day: usize,
    part1: String,
    part2: String,
}

#[derive(Debug)]
pub struct DayResult {
    day: u32,
    part: u32,
    output: String,
    elapsed: Duration,
}

impl DayResult {
    fn check_expected(&self, expected: Option<&DayExpectedResult>) {
        if let Some(expected) = expected {
            let expected_output = match self.part {
                1 => expected.part1.as_str(),
                2 => expected.part2.as_str(),
                _ => unreachable!(),
            };
            assert_eq!(expected_output, self.output);
        }
    }
}

impl fmt::Display for DayResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Day {}, Part {}, in {:?}\n{}",
            self.day, self.part, self.elapsed, self.output,
        )
    }
}

macro_rules! inner_run {
    ($P:tt, $F:expr, $input:expr) => {{
        let start = std::time::Instant::now();
        let output = $F(&$input);
        let elapsed = start.elapsed();

        DayResult {
            day: D,
            part: $P,
            output: output.to_string(),
            elapsed,
        }
    }};
}

pub struct TimingData {
    pub parsing: Duration,
    pub part1: Duration,
    pub part2: Duration,
}

pub fn run<A: Solution<D>, const D: u32>(
    input: &str,
    expected: Option<&DayExpectedResult>,
) -> TimingData {
    let start = std::time::Instant::now();
    let input = <A as ParseInput<D>>::parse_input(input);
    let parsing_elapsed = start.elapsed();
    println!("Day {D}, parsing in {parsing_elapsed:?}");

    let part1 = inner_run!(Part1, A::part1, input);
    println!("{part1}");
    part1.check_expected(expected);

    let part2 = inner_run!(Part2, A::part2, input);
    println!("{part2}");
    part2.check_expected(expected);

    TimingData {
        parsing: parsing_elapsed,
        part1: part1.elapsed,
        part2: part2.elapsed,
    }
}
