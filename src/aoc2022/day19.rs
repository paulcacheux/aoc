use ahash::HashSet;
use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;
use regex::Regex;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day19;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

    min_use: Cost,
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

        let min_use = Cost {
            ore: arr.iter().map(|c| c.ore).min().unwrap(),
            clay: arr.iter().map(|c| c.clay).min().unwrap(),
            obsidian: arr.iter().map(|c| c.obsidian).min().unwrap(),
        };

        Self {
            id,
            ore_robot: ore,
            clay_robot: clay,
            obsidian_robot: obsidian,
            geode_robot: geode,

            min_use,
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
        bot: Cost {
            ore: 1,
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
            if current.geode > max {
                max = current.geode;
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

    bot: Cost,
    count: Cost,
    geode: u16,
}

impl State {
    fn can_buy(mut self, cost: Cost) -> Option<State> {
        if cost.ore > self.count.ore
            || cost.clay > self.count.clay
            || cost.obsidian > self.count.obsidian
        {
            return None;
        }

        self.count.ore -= cost.ore;
        self.count.clay -= cost.clay;
        self.count.obsidian -= cost.obsidian;

        self.collect();
        Some(self)
    }

    fn move_ahead<const STEPS: u16>(&mut self, min_use: Cost) {
        while self.count.ore < min_use.ore
            && self.count.clay < min_use.clay
            && self.count.obsidian < min_use.obsidian
            && self.step <= STEPS
        {
            self.step += 1;
            self.collect();
        }
    }

    fn best_possible<const STEPS: u16>(&self) -> u16 {
        // compute the best possible geode count if we create a robot
        // each step
        let remaining_steps = STEPS - self.step;
        if remaining_steps == 0 {
            return self.geode;
        }

        self.geode + remaining_steps * (remaining_steps - 1) / 2
    }

    fn collect(&mut self) {
        self.count.ore += self.bot.ore;
        self.count.clay += self.bot.clay;
        self.count.obsidian += self.bot.obsidian;
    }

    fn next_states<const STEPS: u16>(mut self, bp: &Blueprint) -> impl Iterator<Item = Self> {
        self.step += 1;

        let ore_bot = bp.ore_robot;
        let clay_bot = bp.clay_robot;
        let obs_bot = bp.obsidian_robot;
        let geode_bot = bp.geode_robot;
        let min_use = bp.min_use;
        let max_use = bp.max_use;

        std::iter::from_generator(move || {
            // not buying
            let mut ns = self;
            ns.collect();
            ns.move_ahead::<STEPS>(min_use);
            yield ns;

            // buying
            if let Some(mut next) = self.can_buy(geode_bot) {
                // directly add all geodes instead of creating a robot
                next.geode += STEPS - ns.step;
                yield next;
            }
            if self.bot.obsidian < max_use.obsidian {
                if let Some(mut next) = self.can_buy(obs_bot) {
                    next.bot.obsidian += 1;
                    yield next;
                }
            }
            if self.bot.clay < max_use.clay {
                if let Some(mut next) = self.can_buy(clay_bot) {
                    next.bot.clay += 1;
                    yield next;
                }
            }
            if self.bot.ore < max_use.ore {
                if let Some(mut next) = self.can_buy(ore_bot) {
                    next.bot.ore += 1;
                    yield next;
                }
            }
        })
    }
}
