use crate::aoc2022::Aoc2022;
use crate::traits::days::Day25;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day25> for Aoc2022 {
    type Parsed = Vec<String>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.lines().map(str::trim).map(str::to_owned).collect()
    }
}

impl Solution<Day25> for Aoc2022 {
    type Part1Output = String;
    type Part2Output = u32;

    fn part1(input: &Vec<String>) -> String {
        let sum: i64 = input.iter().map(|s| decode_snafu(s)).sum();
        encode_snafu(sum)
    }

    fn part2(_input: &Vec<String>) -> u32 {
        // no part 2 here, it's day 25 after all
        0
    }
}

fn decode_snafu(value: &str) -> i64 {
    let mut res = 0;
    for digit in value.chars() {
        let coeff = match digit {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        };

        res = res * 5 + coeff;
    }
    res
}

fn encode_snafu(mut value: i64) -> String {
    let mut digits = Vec::new();
    let mut ret = false;
    while value != 0 {
        let mut rem = value.rem_euclid(5);
        value = value.div_euclid(5);

        if ret {
            rem += 1;
        }
        ret = false;

        match rem {
            0 | 1 | 2 => digits.push(std::char::from_digit(rem as _, 5).unwrap()),
            3 => {
                digits.push('=');
                ret = true;
            }
            4 => {
                digits.push('-');
                ret = true;
            }
            5 => {
                digits.push('0');
                ret = true;
            }
            _ => unreachable!(),
        }
    }

    digits.into_iter().rev().collect()
}
