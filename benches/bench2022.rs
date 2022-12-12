use aoc::{days::Day12, ParseInput, Solution};
use criterion::{criterion_group, criterion_main, Criterion};

fn run_day12() {
    let input = std::fs::read_to_string("./inputs/2022/day12.txt").unwrap();
    let input = <aoc::aoc2022::Aoc2022 as ParseInput<Day12>>::parse_input(input);
    let part1res = <aoc::aoc2022::Aoc2022 as Solution<Day12>>::part1(input);
    let part2res = <aoc::aoc2022::Aoc2022 as Solution<Day12>>::part2(input);
}

fn bench_day12(c: &mut Criterion) {
    c.bench_function("day 12", |b| b.iter(|| run_day12()));
}

criterion_group!(benches, bench_day12);
criterion_main!(benches);
