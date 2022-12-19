use ahash::HashSet;
use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;
use regex::Regex;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day19;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Default)]
struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[derive(Debug)]
pub struct Blueprint {
    id: u32,
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
}

impl ParseInput<Day19> for Aoc2022 {
    type Parsed = Vec<Blueprint>;

    fn parse_input(input: &str) -> Self::Parsed {
        let line_re = Regex::new(
            r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.",
        ).unwrap();

        input
            .lines()
            .map(str::trim)
            .map(|line| {
                let captures = line_re.captures(line).unwrap();
                let id = captures.get(1).unwrap().as_str().parse().unwrap();

                let ore_cost = captures.get(2).unwrap().as_str().parse().unwrap();
                let ore_robot = Cost {
                    ore: ore_cost,
                    ..Default::default()
                };

                let clay_cost = captures.get(3).unwrap().as_str().parse().unwrap();
                let clay_robot = Cost {
                    ore: clay_cost,
                    ..Default::default()
                };

                let obsidian_ore_cost = captures.get(4).unwrap().as_str().parse().unwrap();
                let obsidian_clay_cost = captures.get(5).unwrap().as_str().parse().unwrap();
                let obsidian_robot = Cost {
                    ore: obsidian_ore_cost,
                    clay: obsidian_clay_cost,
                    ..Default::default()
                };

                let geode_ore_cost = captures.get(6).unwrap().as_str().parse().unwrap();
                let geode_obsidian_cost = captures.get(7).unwrap().as_str().parse().unwrap();
                let geode_robot = Cost {
                    ore: geode_ore_cost,
                    obsidian: geode_obsidian_cost,
                    ..Default::default()
                };

                Blueprint {
                    id,
                    ore_robot,
                    clay_robot,
                    obsidian_robot,
                    geode_robot,
                }
            })
            .collect()
    }
}

impl Solution<Day19> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Blueprint>) -> u32 {
        input
            .par_iter()
            .map(|bp| {
                let max = solve::<24>(bp);
                println!("{} => {max}", bp.id);
                max * bp.id
            })
            .sum()
    }

    fn part2(input: &Vec<Blueprint>) -> u32 {
        let end_index = std::cmp::min(3, input.len());

        input[..end_index]
            .par_iter()
            .map(|bp| {
                let max = solve::<32>(bp);
                println!("{} => {max}", bp.id);
                max
            })
            .product()
    }
}

fn solve<const STEPS: u32>(bp: &Blueprint) -> u32 {
    let init_state = State {
        step: 0,
        bot: RobotState {
            ore_robot: 1,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut queue = vec![init_state];
    let mut visited = HashSet::default();

    let mut max = 0;
    while let Some(current) = queue.pop() {
        visited.insert(current);

        if current.step == STEPS {
            if current.count.geode_count > max {
                max = current.count.geode_count;
            }
            continue;
        }

        if current.best_possible::<STEPS>() <= max {
            continue;
        }

        for next in current.next_states(bp).into_iter() {
            if !visited.contains(&next) {
                queue.push(next);
            }
        }
    }
    max
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    step: u32,

    bot: RobotState,
    count: CountState,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RobotState {
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: u32,
    geode_robot: u32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CountState {
    ore_count: u32,
    clay_count: u32,
    obsidian_count: u32,
    geode_count: u32,
}

impl CountState {
    fn can_buy(mut self, cost: &Cost) -> Option<Self> {
        if cost.ore > self.ore_count
            || cost.clay > self.clay_count
            || cost.obsidian > self.obsidian_count
        {
            return None;
        }

        self.ore_count -= cost.ore;
        self.clay_count -= cost.clay;
        self.obsidian_count -= cost.obsidian;
        Some(self)
    }
}

impl State {
    fn best_possible<const STEPS: u32>(&self) -> u32 {
        let remaining_steps = STEPS - self.step;
        if remaining_steps == 0 {
            return 0;
        }

        let mut count = self.count.geode_count;
        let mut geode_bot = self.bot.geode_robot;
        for _ in 0..remaining_steps {
            count += geode_bot;
            geode_bot += 1;
        }
        count
    }

    fn prepare(mut self, count: CountState) -> Self {
        self.count = count;
        self.collect();
        self
    }

    fn collect(&mut self) {
        self.count.ore_count += self.bot.ore_robot;
        self.count.clay_count += self.bot.clay_robot;
        self.count.obsidian_count += self.bot.obsidian_robot;
        self.count.geode_count += self.bot.geode_robot;
    }

    fn next_states(mut self, bp: &Blueprint) -> Vec<Self> {
        self.step += 1;

        let mut nexts = Vec::with_capacity(5);

        // not buying
        let mut ns = self;
        ns.collect();
        nexts.push(ns);
        // buying
        if let Some(next) = self.count.can_buy(&bp.ore_robot) {
            let mut ns = self.prepare(next);
            ns.bot.ore_robot += 1;
            nexts.push(ns);
        }
        if let Some(next) = self.count.can_buy(&bp.clay_robot) {
            let mut ns = self.prepare(next);
            ns.bot.clay_robot += 1;
            nexts.push(ns);
        }
        if let Some(next) = self.count.can_buy(&bp.obsidian_robot) {
            let mut ns = self.prepare(next);
            ns.bot.obsidian_robot += 1;
            nexts.push(ns);
        }
        if let Some(next) = self.count.can_buy(&bp.geode_robot) {
            let mut ns = self.prepare(next);
            ns.bot.geode_robot += 1;
            nexts.push(ns);
        }
        nexts
    }
}
