use crate::aoc2022::Aoc2022;
use crate::traits::days::Day10;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

const ADDX_PREFIX: &str = "addx ";

impl ParseInput<Day10> for Aoc2022 {
    type Parsed = Vec<Instruction>;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let inst = match line.trim() {
                "noop" => Instruction::Noop,
                addx if addx.starts_with(ADDX_PREFIX) => {
                    let offset = addx[ADDX_PREFIX.len()..].parse().unwrap();
                    Instruction::AddX(offset)
                }
                _ => unreachable!(),
            };

            instructions.push(inst);
        }
        instructions
    }
}

fn eval<F>(instructions: &[Instruction], mut cb: F)
where
    F: FnMut(usize, i32),
{
    let mut x = 1;

    let mut current_cycle = 1;
    for instruction in instructions {
        let prex = x;
        let cycles = match instruction {
            Instruction::Noop => 1,
            Instruction::AddX(offset) => {
                x += offset;
                2
            }
        };
        for _ in 0..cycles {
            cb(current_cycle, prex);
            current_cycle += 1;
        }
    }
}

const PART1_POINTS: [usize; 6] = [20, 60, 100, 140, 180, 220];

#[derive(Debug)]
pub struct Crt {
    pixels: [bool; 40 * 6],
}

impl Crt {
    fn new() -> Self {
        Crt {
            pixels: [false; 40 * 6],
        }
    }

    fn set_cycle(&mut self, cycle: usize, value: bool) {
        self.pixels[cycle - 1] = value;
    }
}

impl std::fmt::Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..6 {
            for x in 0..40 {
                write!(f, "{}", if self.pixels[y * 40 + x] { "#" } else { "." })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Solution<Day10> for Aoc2022 {
    type Part1Output = i32;
    type Part2Output = Crt;

    fn part1(input: &Vec<Instruction>) -> i32 {
        let mut res = 0;
        eval(input, |cycle, x| {
            if PART1_POINTS.contains(&cycle) {
                res += cycle as i32 * x;
            }
        });
        res
    }

    fn part2(input: &Vec<Instruction>) -> Crt {
        let mut crt = Crt::new();
        eval(input, |cycle, x| {
            let cycle0 = cycle - 1;
            let cx = (cycle0 % 40) as i32;
            let pixel = (cx - x).abs() <= 1;
            crt.set_cycle(cycle, pixel);
        });
        crt
    }
}
