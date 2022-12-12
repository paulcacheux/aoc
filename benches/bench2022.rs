#![feature(concat_idents)]

use std::hint::black_box;

use aoc::traits::{days::*, ParseInput, Solution};
use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! gen_bench {
    ($benchday:ident, $dayconst:ident) => {
        fn $benchday(c: &mut Criterion) {
            fn runner(input: &str) {
                let input = <aoc::aoc2022::Aoc2022 as ParseInput<$dayconst>>::parse_input(&input);
                black_box(<aoc::aoc2022::Aoc2022 as Solution<$dayconst>>::part1(
                    &input,
                ));
                black_box(<aoc::aoc2022::Aoc2022 as Solution<$dayconst>>::part2(
                    &input,
                ));
            }

            let input = std::fs::read_to_string(format!(
                "./inputs/2022/{}.txt",
                stringify!($dayconst).to_lowercase()
            ))
            .unwrap();
            c.bench_function(stringify!($dayconst), |b| b.iter(|| runner(&input)));
        }
    };
}

macro_rules! gen_bench_group {
    ($(($benchday:ident, $dayconst:ident)),+) => {
        $(gen_bench!($benchday, $dayconst);)+

        criterion_group!(benches, $($benchday,)+);
    };
}

gen_bench_group!(
    (bench_day1, Day1),
    (bench_day2, Day2),
    (bench_day3, Day3),
    (bench_day4, Day4),
    (bench_day5, Day5),
    (bench_day6, Day6),
    (bench_day7, Day7),
    (bench_day8, Day8),
    (bench_day9, Day9),
    (bench_day10, Day10),
    (bench_day11, Day11),
    (bench_day12, Day12)
);

criterion_main!(benches);
