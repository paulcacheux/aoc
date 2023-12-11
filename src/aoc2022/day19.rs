use std::collections::HashSet;

use regex::Regex;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day19;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Cost {
    ore: u8,
    clay: u8,
    obsidian: u8,
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
        input.iter().map(|bp| solve::<24>(bp) * bp.id).sum()
    }

    fn part2(input: &Vec<Blueprint>) -> u16 {
        let end_index = std::cmp::min(3, input.len());
        input[..end_index].iter().map(solve::<32>).product()
    }
}

fn solve<const STEPS: u8>(bp: &Blueprint) -> u16 {
    let init_state = State {
        step: 0,
        key: Key {
            bot: Cost {
                ore: 1,
                ..Default::default()
            },
            ..Default::default()
        },
    };

    let mut queue = Vec::with_capacity(64);
    queue.push(init_state);
    let mut visited: HashSet<u64> = HashSet::default();

    let mut max = 0;
    while let Some(current) = queue.pop() {
        if current.step == STEPS {
            if current.key.geode > max {
                max = current.key.geode;
            }
            continue;
        }

        if current.best_possible::<STEPS>() <= max {
            continue;
        }

        visited.insert(current.key_u64());
        queue.extend(
            current
                .next_states::<STEPS>(bp)
                .filter(|next| !visited.contains(&next.key_u64())),
        );
    }
    max
}

#[derive(Default, Debug, Clone, Copy)]
struct State {
    step: u8,
    key: Key,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Key {
    bot: Cost,
    count: Cost,
    geode: u16,
}

impl State {
    #[inline]
    fn key_u64(&self) -> u64 {
        // what an horrible hack, but it seems the hasher if faster on u64...
        assert_eq!(std::mem::size_of::<Key>(), std::mem::size_of::<u64>());
        unsafe { std::mem::transmute(self.key) }
    }

    #[inline]
    fn can_buy(&self, cost: Cost) -> Option<State> {
        if cost.ore > self.key.count.ore
            || cost.clay > self.key.count.clay
            || cost.obsidian > self.key.count.obsidian
        {
            return None;
        }

        let mut next = *self;
        next.key.count.ore -= cost.ore;
        next.key.count.clay -= cost.clay;
        next.key.count.obsidian -= cost.obsidian;
        next.collect();

        Some(next)
    }

    fn move_ahead<const STEPS: u8>(&mut self, min_use: Cost) {
        while self.key.count.ore < min_use.ore
            && self.key.count.clay < min_use.clay
            && self.key.count.obsidian < min_use.obsidian
            && self.step <= STEPS
        {
            self.step += 1;
            self.collect();
        }
    }

    fn best_possible<const STEPS: u8>(&self) -> u16 {
        // compute the best possible geode count if we create a robot
        // each step
        let remaining_steps = (STEPS - self.step) as u16;
        if remaining_steps == 0 {
            return self.key.geode;
        }

        self.key.geode + remaining_steps * (remaining_steps - 1) / 2
    }

    #[inline]
    fn collect(&mut self) {
        self.key.count.ore += self.key.bot.ore;
        self.key.count.clay += self.key.bot.clay;
        self.key.count.obsidian += self.key.bot.obsidian;
    }

    fn next_states<const STEPS: u8>(mut self, bp: &Blueprint) -> impl Iterator<Item = Self> {
        self.step += 1;

        let ore_bot = bp.ore_robot;
        let clay_bot = bp.clay_robot;
        let obs_bot = bp.obsidian_robot;
        let geode_bot = bp.geode_robot;
        let min_use = bp.min_use;
        let max_use = bp.max_use;

        std::iter::from_coroutine(move || {
            // not buying
            let mut ns = self;
            ns.collect();
            ns.move_ahead::<STEPS>(min_use);
            yield ns;

            // buying
            if let Some(mut next) = self.can_buy(geode_bot) {
                // directly add all geodes instead of creating a robot
                next.key.geode += (STEPS - ns.step) as u16;
                yield next;
            }
            if self.key.bot.obsidian < max_use.obsidian {
                if let Some(mut next) = self.can_buy(obs_bot) {
                    next.key.bot.obsidian += 1;
                    yield next;
                }
            }
            if self.key.bot.clay < max_use.clay {
                if let Some(mut next) = self.can_buy(clay_bot) {
                    next.key.bot.clay += 1;
                    yield next;
                }
            }
            if self.key.bot.ore < max_use.ore {
                if let Some(mut next) = self.can_buy(ore_bot) {
                    next.key.bot.ore += 1;
                    yield next;
                }
            }
        })
    }
}
