use clap::Parser;

mod aoc2019;
mod aoc2021;
mod helpers;

use helpers::Results;

#[derive(Parser)]
#[clap(version = "1.0", author = "Paul C. <paulcacheux@gmail.com>")]
struct Options {
    /// Use test input
    #[clap(long)]
    test: bool,
    /// Advent year
    #[clap(long, default_value = "2021")]
    year: u32,
    /// Advent day
    #[clap(long)]
    day: u32,
}

fn run_day(year: u32, day: u32, test: bool) -> Result<Option<()>, Box<dyn std::error::Error>> {
    let (input_path, results) = if test {
        (format!("./inputs/{}/day{}_test.txt", year, day), None)
    } else {
        (
            format!("./inputs/{}/day{}.txt", year, day),
            Some(Results::parse(year)?),
        )
    };
    let input = std::fs::read_to_string(input_path)?;

    let year_runner = match year {
        2019 => aoc2019::run_solution_for_day,
        2021 => aoc2021::run_solution_for_day,
        _ => panic!("undefined year {}", year),
    };

    if year_runner(day, &input, results) {
        Ok(Some(()))
    } else {
        Ok(None)
    }
}

fn main() {
    let opts = Options::parse();

    match run_day(opts.year, opts.day, opts.test) {
        Ok(None) => {
            panic!("no solution available for that day ({})", opts.day)
        }
        Ok(Some(())) => {}
        Err(err) => panic!("Error loading day: {}", err),
    }
}
