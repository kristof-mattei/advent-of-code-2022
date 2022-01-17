use regex::Regex;

use crate::shared::{Day, PartSolution};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    on: bool,
    start: Point,
    end: Point,
}

fn parse_lines(lines: &[&str]) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let regex =
        Regex::new(r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
            .unwrap();

    for line in lines {
        let captures = regex.captures(line.trim()).unwrap();

        instructions.push(Instruction {
            on: &captures[1] == "on",
            start: Point {
                x: captures[2].parse().unwrap(),
                y: captures[4].parse().unwrap(),
                z: captures[6].parse().unwrap(),
            },
            end: Point {
                x: captures[3].parse().unwrap(),
                y: captures[5].parse().unwrap(),
                z: captures[7].parse().unwrap(),
            },
        });
    }

    instructions
}

fn get_on_cuboids(instructions: &[Instruction]) -> Vec<Cuboid> {
    let mut on_cuboids: Vec<Cuboid> = Vec::new();

    for instruction in instructions.iter() {
        let cuboid = Cuboid::new(
            instruction.start.x,
            instruction.end.x,
            instruction.start.y,
            instruction.end.y,
            instruction.start.z,
            instruction.end.z,
        );

        let mut touched_one = true;

        while touched_one {
            let overlapped = on_cuboids.iter().rev().find(|c| c.overlaps(&cuboid));

            let position = match overlapped {
                Some(o) => on_cuboids.iter().rev().position(|oc| oc == o),
                None => None,
            };

            match position {
                Some(p) => {
                    // find the index of the overlapped one
                    let removed = on_cuboids.remove(on_cuboids.len() - 1 - p);
                    let new_pieces = removed.subtract(&cuboid);

                    for np in new_pieces {
                        on_cuboids.insert(p, np);
                    }
                },
                None => touched_one = false,
            }
        }

        if instruction.on {
            on_cuboids.push(cuboid);
        }
    }

    on_cuboids
}

fn calculate_on_points_naive(instructions: &[Instruction], min: i32, max: i32) -> u64 {
    get_on_cuboids(instructions)
        .into_iter()
        .map(|cuboid| {
            Cuboid::new(
                cuboid.x_min.max(min),
                cuboid.x_max.min(max),
                cuboid.y_min.max(min),
                cuboid.y_max.min(max),
                cuboid.z_min.max(min),
                cuboid.z_max.min(max),
            )
        })
        .map(|cuboid| cuboid.volume())
        .sum()
}

fn calculate_on_points(instructions: &[Instruction]) -> u64 {
    get_on_cuboids(instructions)
        .iter()
        .map(Cuboid::volume)
        .sum()
}

#[derive(PartialEq, Eq, Debug)]
struct Cuboid {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl Cuboid {
    fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }

    fn subtract(self, other: &Cuboid) -> Vec<Cuboid> {
        if self.overlaps(other) {
            [
                Cuboid::new(
                    self.x_min,
                    other.x_min - 1,
                    self.y_min,
                    self.y_max,
                    self.z_min,
                    self.z_max,
                ),
                Cuboid::new(
                    other.x_max + 1,
                    self.x_max,
                    self.y_min,
                    self.y_max,
                    self.z_min,
                    self.z_max,
                ),
                Cuboid::new(
                    self.x_min.max(other.x_min),
                    self.x_max.min(other.x_max),
                    self.y_min,
                    other.y_min - 1,
                    self.z_min,
                    self.z_max,
                ),
                Cuboid::new(
                    self.x_min.max(other.x_min),
                    self.x_max.min(other.x_max),
                    other.y_max + 1,
                    self.y_max,
                    self.z_min,
                    self.z_max,
                ),
                Cuboid::new(
                    self.x_min.max(other.x_min),
                    self.x_max.min(other.x_max),
                    self.y_min.max(other.y_min),
                    self.y_max.min(other.y_max),
                    self.z_min,
                    other.z_min - 1,
                ),
                Cuboid::new(
                    self.x_min.max(other.x_min),
                    self.x_max.min(other.x_max),
                    self.y_min.max(other.y_min),
                    self.y_max.min(other.y_max),
                    other.z_max + 1,
                    self.z_max,
                ),
            ]
            .into_iter()
            .filter(|c| !c.is_empty())
            .collect()
        } else {
            vec![self]
        }
    }

    fn is_empty(&self) -> bool {
        self.x_max < self.x_min || self.y_max < self.y_min || self.z_max < self.z_min
    }

    fn overlaps(&self, other: &Cuboid) -> bool {
        !(other.x_min > self.x_max
            || other.x_max < self.x_min
            || other.y_min > self.y_max
            || other.y_max < self.y_min
            || other.z_min > self.z_max
            || other.z_max < self.z_min)
    }

    fn volume(&self) -> u64 {
        // max plus one as our ranges are inclusive
        let x_dim = u64::try_from(self.x_max + 1 - self.x_min).unwrap_or_default();
        let y_dim = u64::try_from(self.y_max + 1 - self.y_min).unwrap_or_default();
        let z_dim = u64::try_from(self.z_max + 1 - self.z_min).unwrap_or_default();
        x_dim * y_dim * z_dim
    }
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let instructions = parse_lines(&lines);

        let on_points = calculate_on_points_naive(&instructions, -50, 50);

        PartSolution::U64(on_points)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let instructions = parse_lines(&lines);

        let on_points = calculate_on_points(&instructions);

        PartSolution::U64(on_points)
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt").lines().collect()
    }

    fn get_larger_example() -> Vec<&'static str> {
        include_str!("example_larger.txt").lines().collect()
    }

    fn get_example_part_2() -> Vec<&'static str> {
        include_str!("example_part_2.txt").lines().collect()
    }

    mod part_1 {
        use crate::{
            day_22::{
                calculate_on_points_naive, parse_lines, test::get_larger_example, Instruction,
                Point, Solution,
            },
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U64(580_012));
        }

        #[test]
        fn example_parse_input() {
            let example_lines = get_example();

            let reboot_steps = parse_lines(&example_lines);

            assert_eq!(
                vec![
                    Instruction {
                        on: true,
                        start: Point {
                            x: 10,
                            y: 10,
                            z: 10,
                        },
                        end: Point {
                            x: 12,
                            y: 12,
                            z: 12
                        }
                    },
                    Instruction {
                        on: true,
                        start: Point {
                            x: 11,
                            y: 11,
                            z: 11,
                        },
                        end: Point {
                            x: 13,
                            y: 13,
                            z: 13
                        }
                    },
                    Instruction {
                        on: false,
                        start: Point { x: 9, y: 9, z: 9 },
                        end: Point {
                            x: 11,
                            y: 11,
                            z: 11
                        }
                    },
                    Instruction {
                        on: true,
                        start: Point {
                            x: 10,
                            y: 10,
                            z: 10,
                        },
                        end: Point {
                            x: 10,
                            y: 10,
                            z: 10
                        }
                    }
                ],
                reboot_steps
            );
        }

        #[test]
        fn example() {
            let example_lines = get_example();

            let instructions = parse_lines(&example_lines);

            let on_points = calculate_on_points_naive(&instructions, -50, 50);

            assert_eq!(39, on_points);
        }

        #[test]
        fn calculate_points() {
            let instructions = vec![Instruction {
                on: true,
                start: Point {
                    x: 967,
                    y: 45373,
                    z: 27513,
                },
                end: Point {
                    x: 23432,
                    y: 81175,
                    z: 53682,
                },
            }];
            let on_points = calculate_on_points_naive(&instructions, -50, 50);

            assert_eq!(0, on_points);
        }

        #[test]
        fn calculate_points_2() {
            let instructions = vec![Instruction {
                on: true,
                start: Point {
                    x: -55,
                    y: 1,
                    z: 10,
                },
                end: Point {
                    x: -45,
                    y: 10,
                    z: 10,
                },
            }];

            let on_points = calculate_on_points_naive(&instructions, -50, 50);

            assert_eq!(60, on_points);
        }

        #[test]
        fn larger_example() {
            let example_lines = get_larger_example();

            let instructions = parse_lines(&example_lines);

            let on_points = calculate_on_points_naive(&instructions, -50, 50);

            assert_eq!(590_784, on_points);
        }
    }

    mod part_2 {

        use crate::{
            day_22::{
                calculate_on_points, parse_lines, test::get_example_part_2, Cuboid, Solution,
            },
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(
                (Solution {}).part_2(),
                PartSolution::U64(1_334_238_660_555_542)
            );
        }

        #[test]
        fn cuboid_volume() {
            let cuboid = Cuboid::new(0, 5, 0, 5, 0, 5);

            assert_eq!(216, cuboid.volume());
        }

        #[test]
        fn example() {
            let example_lines = get_example_part_2();

            let instructions = parse_lines(&example_lines);

            let on_points = calculate_on_points(&instructions);

            assert_eq!(2_758_514_936_282_235, on_points);
        }
    }
}
