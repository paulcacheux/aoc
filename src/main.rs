#![feature(iter_array_chunks)]
#![feature(get_many_mut)]
#![feature(generators)]
#![feature(iter_from_generator)]

use std::time::Duration;

use clap::Parser;

mod aoc2019;
mod aoc2021;
mod aoc2022;
mod helpers;
mod traits;

use helpers::Results;

#[derive(Parser)]
#[command(version = "1.0", author = "Paul C. <paulcacheux@gmail.com>")]
struct Options {
    /// Use test input
    #[arg(long)]
    test: bool,
    /// Advent year
    #[arg(long, default_value = "2022")]
    year: u32,
    /// Advent day
    #[arg(long)]
    day: Option<u32>,
}

fn run_day(
    year: u32,
    day: u32,
    test: bool,
) -> Result<Option<Duration>, Box<dyn std::error::Error>> {
    let (input_path, results) = if test {
        (format!("./inputs/{year}/day{day}_test.txt"), None)
    } else {
        (
            format!("./inputs/{year}/day{day}.txt"),
            Some(Results::parse(year)?),
        )
    };
    let input = std::fs::read_to_string(input_path)?;

    let year_runner = match year {
        2019 => aoc2019::run_solution_for_day,
        2021 => aoc2021::run_solution_for_day,
        2022 => aoc2022::run_solution_for_day,
        _ => panic!("undefined year {year}"),
    };

    Ok(year_runner(day, &input, results))
}

fn run_all(year: u32, test: bool) {
    let mut total = Duration::ZERO;
    for day in 1..=25 {
        if let Ok(Some(elapsed)) = run_day(year, day, test) {
            total += elapsed;
        }
    }
    println!("Total: {total:?}");
}

fn main() {
    let opts = Options::parse();

    if let Some(day) = opts.day {
        match run_day(opts.year, day, opts.test) {
            Ok(None) => {
                panic!("no solution available for that day ({day})")
            }
            Ok(Some(_)) => {}
            Err(err) => panic!("Error loading day: {err}"),
        }
    } else {
        run_all(opts.year, opts.test)
    }
}
