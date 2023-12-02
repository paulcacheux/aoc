use crate::aoc2023::Aoc2023;
use crate::traits::days::Day2;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Default)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    entries: Vec<RGB>,
}

impl ParseInput<Day2> for Aoc2023 {
    type Parsed = Vec<Game>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.lines().map(|line| {
            let line = line.strip_prefix("Game ").unwrap();

            let double_dot = line.find(": ").unwrap();
            let (before, after) = line.split_at(double_dot);
            let after = &after[2..];

            let id = before.parse().unwrap();

            let entries = after.split("; ").map(|round| {
                let mut rgb = RGB::default();
                for color in round.split(", ") {
                    let (count, color_name) = color.split_at(color.find(' ').unwrap());
                    let count = count.parse().unwrap();
                    match &color_name[1..] {
                        "red" => rgb.red = count,
                        "green" => rgb.green = count,
                        "blue" => rgb.blue = count,
                        _ => unreachable!()
                    }
                }
                rgb
            }).collect();

            Game{
                id,
                entries
            }
        }).collect()
    }
}

impl Solution<Day2> for Aoc2023 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Game>) -> u32 {
        let mut res = 0;
        for game in input {
            let mut possible = true;
            for entry in &game.entries {
                if entry.red > 12 || entry.green > 13 || entry.blue > 14 {
                    possible = false;
                    break;
                }
            }

            if possible {
                res += game.id;
            }
        }
        res
    }

    fn part2(input: &Vec<Game>) -> u32 {
        let mut res = 0;
        for game in input {
            let mut max_rgb = RGB::default();
            for entry in &game.entries {
                if entry.red > max_rgb.red {
                    max_rgb.red = entry.red;
                }
                if entry.green > max_rgb.green {
                    max_rgb.green = entry.green;
                }
                if entry.blue > max_rgb.blue {
                    max_rgb.blue = entry.blue;
                }
            }
            let power = max_rgb.red as u32 * max_rgb.green as u32 * max_rgb.blue as u32;
            res += power;
        }
        res
    }
}
