use std::{collections::VecDeque, sync::LazyLock};

use advent_of_code_2022::shared::{PartSolution, Parts};
use enum_map::{enum_map, Enum, EnumMap};
use regex::Regex;

advent_of_code_2022::solution!(1150, 37367);

#[derive(Debug, Clone, Copy, Enum)]
#[repr(u8)]
enum Mineral {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl TryFrom<&str> for Mineral {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mineral = match value {
            "ore" => Mineral::Ore,
            "clay" => Mineral::Clay,
            "geode" => Mineral::Geode,
            "obsidian" => Mineral::Obsidian,
            _ => {
                return Err("Invalid mineral");
            },
        };

        Ok(mineral)
    }
}

#[derive(Debug)]
struct Blueprint {
    number: usize,
    robots: Vec<Robot>,
}

#[derive(Debug)]
struct Robot {
    builds: Mineral,
    needs: EnumMap<Mineral, usize>,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct State {
    inventory: EnumMap<Mineral, usize>,
    production: EnumMap<Mineral, usize>,
}

impl State {
    fn start() -> Self {
        State {
            inventory: EnumMap::default(),
            production: enum_map! {
                Mineral::Ore => 1,
                Mineral::Clay | Mineral::Geode | Mineral::Obsidian => 0,
            },
        }
    }
}

static ROBOT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:Each (ore|geode|clay|obsidian) robot costs (\d+) (ore)(?: and (\d+) (clay|obsidian))?\. ?)").unwrap()
});

static BLUEPRINT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Blueprint (\d+):").unwrap());

fn parse_blueprint(line: &str) -> Blueprint {
    let blueprint = BLUEPRINT_REGEX.captures(line).unwrap();
    let blueprint_number = blueprint.get(1).unwrap().as_str().parse::<usize>().unwrap();

    let robots = ROBOT_REGEX
        .captures_iter(line)
        .map(|robot| {
            let needs = [2, 4]
                .iter()
                .filter_map(|&i| {
                    robot.get(i).zip(robot.get(i + 1)).map(|(count, element)| {
                        (
                            element.as_str().try_into().unwrap(),
                            count.as_str().parse::<usize>().unwrap(),
                        )
                    })
                })
                .fold(EnumMap::default(), |mut acc, (mineral, amount_needed)| {
                    acc[mineral] = amount_needed;

                    acc
                });

            Robot {
                builds: robot.get(1).unwrap().as_str().try_into().unwrap(),
                needs,
            }
        })
        .collect::<Vec<Robot>>();

    Blueprint {
        number: blueprint_number,
        robots,
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input.trim().lines().map(parse_blueprint).collect()
}

fn blueprint_recursive(blueprint: &Blueprint, minutes: usize, state: State) -> usize {
    let mut max = 0;
    let mut to_do: VecDeque<(usize, State)> =
        [(minutes, state)].into_iter().collect::<VecDeque<_>>();

    // no need to build a robo
    let max_per_robot = blueprint.robots.iter().fold(
        EnumMap::from_array([0, 0, 0, usize::MAX]),
        |mut acc, robot| {
            // don't cap geodes
            for mineral in [Mineral::Ore, Mineral::Clay, Mineral::Obsidian] {
                acc[mineral] = acc[mineral].max(robot.needs[mineral]);
            }

            acc
        },
    );

    while let Some((minutes, state)) = to_do.pop_front() {
        if minutes == 0 {
            println!("HIT");
            max = max.max(state.inventory[Mineral::Geode]);
            continue;
        }

        for robot in &blueprint.robots {
            // if our current state has max_per_robot of what this robot produces, it's a wasted effort
            if state.production[robot.builds] >= max_per_robot[robot.builds] {
                continue;
            }

            // how long do we need to wait to build this robot?
            let wait_time: usize = [Mineral::Ore, Mineral::Clay, Mineral::Obsidian]
                .into_iter()
                .map(|mineral| {
                    let robot_needs = robot.needs[mineral];
                    let we_have = state.inventory[mineral];

                    // this mineral is not an issue, either because the robot doesn't need it, or we have enough
                    if robot_needs <= we_have {
                        0
                    } else if state.production[mineral] == 0 {
                        // we don't have the robot yet
                        usize::MAX - 1
                    } else {
                        (
                            // how many more minutes do we have to wait to fullfill this requirement?
                            robot_needs - we_have
                        )
                            .div_ceil(state.production[mineral])
                    }
                })
                .max()
                .unwrap();

            // one extra to actually build the robot
            let wait_time = wait_time + 1;

            if wait_time >= minutes {
                // can't wait THAT long
                continue;
            }

            let mut new_state = state.clone();

            // progress our wait time
            for (mineral, per_minute) in &state.production {
                new_state.inventory[mineral] += per_minute * wait_time;
            }

            // build robot
            for (mineral, &amount) in &robot.needs {
                new_state.inventory[mineral] -= amount;
            }

            // add built robot to state
            new_state.production[robot.builds] += 1;

            // can we actually build more geodes with the remaining time than the max geodes we have?
            let remaining_time = minutes - wait_time;

            // 1 + 2 + 3 + 4 + ... + n - 1 = ( n * (n - 1) ) / 2
            // that way we can find out the total amount of geodes produced on a certain day,
            // if we build 1 robot per day starting from 0
            let build_now_and_produce = ((remaining_time - 1) * remaining_time) / 2;

            if build_now_and_produce
                + new_state.inventory[Mineral::Geode]
                + (remaining_time * new_state.production[Mineral::Geode])
                <= max
            {
                continue;
            }

            to_do.push_back((remaining_time, new_state));
        }

        max = max
            .max((state.production[Mineral::Geode] * (minutes)) + state.inventory[Mineral::Geode]);
    }

    max
}

fn best_blueprint_24(input: &str) -> PartSolution {
    let blueprints = parse_input(input);

    let state = State::start();

    blueprints
        .into_iter()
        .map(|blueprint| blueprint.number * blueprint_recursive(&blueprint, 24, state.clone()))
        .sum::<usize>()
        .into()
}

fn best_blueprint_32(input: &str) -> PartSolution {
    let mut blueprints = parse_input(input);
    blueprints.truncate(3);

    let state = State::start();

    blueprints
        .into_iter()
        .take(3)
        .map(|blueprint| blueprint_recursive(&blueprint, 32, state.clone()))
        .product::<usize>()
        .into()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        best_blueprint_24(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        best_blueprint_32(input)
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(1150, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(33, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(37367, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(3472, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
