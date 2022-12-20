use ahash::HashSet;
use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;
use regex::Regex;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day19;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Default, Clone, Copy)]
struct Cost {
    ore: u16,
    clay: u16,
    obsidian: u16,
}

#[derive(Debug)]
pub struct Blueprint {
    id: u16,
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
    max_use: Cost,
}

impl Blueprint {
    fn new(id: u16, ore: Cost, clay: Cost, obsidian: Cost, geode: Cost) -> Self {
        let arr = [ore, clay, obsidian, geode];
        let max_use = Cost {
            ore: arr.iter().map(|c| c.ore).max().unwrap(),
            clay: arr.iter().map(|c| c.clay).max().unwrap(),
            obsidian: arr.iter().map(|c| c.obsidian).max().unwrap(),
        };

        Self {
            id,
            ore_robot: ore,
            clay_robot: clay,
            obsidian_robot: obsidian,
            geode_robot: geode,
            max_use,
        }
    }
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

                Blueprint::new(id, ore_robot, clay_robot, obsidian_robot, geode_robot)
            })
            .collect()
    }
}

impl Solution<Day19> for Aoc2022 {
    type Part1Output = u16;
    type Part2Output = u16;

    fn part1(input: &Vec<Blueprint>) -> u16 {
        input.par_iter().map(|bp| solve::<24>(bp) * bp.id).sum()
    }

    fn part2(input: &Vec<Blueprint>) -> u16 {
        let end_index = std::cmp::min(3, input.len());
        input[..end_index].par_iter().map(solve::<32>).product()
    }
}

fn solve<const STEPS: u16>(bp: &Blueprint) -> u16 {
    let init_state = State {
        step: 0,
        bot: RobotState {
            ore_robot: 1,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut queue = Vec::with_capacity(64);
    queue.push(init_state);
    let mut visited = HashSet::default();

    let mut max = 0;
    while let Some(current) = queue.pop() {
        if current.step == STEPS {
            if current.count.geode_count > max {
                max = current.count.geode_count;
            }
            continue;
        }

        if current.best_possible::<STEPS>() <= max {
            continue;
        }

        visited.insert(current);

        for next in current.next_states::<STEPS>(bp) {
            if !visited.contains(&next) {
                queue.push(next);
            }
        }
    }
    max
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    step: u16,

    bot: RobotState,
    count: CountState,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RobotState {
    ore_robot: u16,
    clay_robot: u16,
    obsidian_robot: u16,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CountState {
    ore_count: u16,
    clay_count: u16,
    obsidian_count: u16,
    geode_count: u16,
}

impl CountState {
    fn can_buy(mut self, cost: Cost) -> Option<Self> {
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
    fn best_possible<const STEPS: u16>(&self) -> u16 {
        // compute the best possible geode count if we create a robot
        // each step
        let remaining_steps = STEPS - self.step;
        if remaining_steps == 0 {
            return self.count.geode_count;
        }

        self.count.geode_count + remaining_steps * (remaining_steps - 1) / 2
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
    }

    fn next_states<const STEPS: u16>(mut self, bp: &Blueprint) -> impl Iterator<Item = Self> {
        self.step += 1;

        let ore_bot = bp.ore_robot;
        let clay_bot = bp.clay_robot;
        let obs_bot = bp.obsidian_robot;
        let geode_bot = bp.geode_robot;
        let max_use = bp.max_use;

        std::iter::from_generator(move || {
            // not buying
            let mut ns = self;
            ns.collect();
            yield ns;

            // buying
            if let Some(next) = self.count.can_buy(geode_bot) {
                let mut ns = self.prepare(next);
                // directly add all geodes instead of creating a robot
                ns.count.geode_count += STEPS - ns.step;
                yield ns;
            }
            if self.bot.obsidian_robot < max_use.obsidian {
                if let Some(next) = self.count.can_buy(obs_bot) {
                    let mut ns = self.prepare(next);
                    ns.bot.obsidian_robot += 1;
                    yield ns;
                }
            }
            if self.bot.clay_robot < max_use.clay {
                if let Some(next) = self.count.can_buy(clay_bot) {
                    let mut ns = self.prepare(next);
                    ns.bot.clay_robot += 1;
                    yield ns;
                }
            }
            if self.bot.ore_robot < max_use.ore {
                if let Some(next) = self.count.can_buy(ore_bot) {
                    let mut ns = self.prepare(next);
                    ns.bot.ore_robot += 1;
                    yield ns;
                }
            }
        })
    }
}
