use std::collections::HashMap;

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

fn generate_points(instruction: &Instruction, min: i32, max: i32) -> Vec<Point> {
    let mut points = Vec::new();

    if !(instruction.start.x > max
        || instruction.end.x < min
        || instruction.start.y > max
        || instruction.end.y < min
        || instruction.start.z > max
        || instruction.end.z < min)
    {
        let x1 = instruction.start.x.max(min);
        let x2 = instruction.end.x.min(max);

        let y1 = instruction.start.y.max(min);
        let y2 = instruction.end.y.min(max);

        let z1 = instruction.start.z.max(min);
        let z2 = instruction.end.z.min(max);

        for x in x1.min(x2)..=x1.max(x2) {
            for y in y1.min(y2)..=y1.max(y2) {
                for z in z1.min(z2)..=z1.max(z2) {
                    points.push(Point { x, y, z });
                }
            }
        }
    }

    points
}
fn calculate_on_points(instructions: &[Instruction]) -> usize {
    let mut on_points = HashMap::<Point, bool>::new();

    for (i, instruction) in instructions.iter().enumerate() {
        println!("Processing instruction {}/{}", i + 1, instructions.len());

        println!(
            "Generating points for instruction {}/{}",
            i + 1,
            instructions.len()
        );
        let points = generate_points(instruction, i32::MIN, i32::MAX);
        println!(
            "Generated points for instruction {}/{}",
            i + 1,
            instructions.len()
        );

        if instruction.on {
            for p in points {
                on_points.insert(p, true);
            }
        } else {
            for p in points {
                on_points.entry(p).and_modify(|on| {
                    println!("Turned off an existing point");
                    *on = false;
                });
            }
        }
    }

    on_points.iter().filter(|(_, on)| **on).count()
}

fn calculate_on_points_naive(instructions: &[Instruction], min: i32, max: i32) -> usize {
    let mut on_points = HashMap::<Point, bool>::new();

    for (i, instruction) in instructions.iter().enumerate() {
        println!("Processing instruction {}/{}", i + 1, instructions.len());

        println!(
            "Generating points for instruction {}/{}",
            i + 1,
            instructions.len()
        );
        let points = generate_points(instruction, min, max);
        println!(
            "Generated points for instruction {}/{}",
            i + 1,
            instructions.len()
        );

        if instruction.on {
            for p in points {
                on_points.insert(p, true);
            }
        } else {
            for p in points {
                on_points.entry(p).and_modify(|on| {
                    println!("Turned off an existing point");
                    *on = false;
                });
            }
        }
    }

    // let ct = on_points
    //     .iter()
    //     .filter(|p| {
    //         p.x >= min && p.x <= max && p.y >= min && p.y <= max && p.z >= min && p.z <= max
    //     })
    //     .count();

    // ct
    on_points.iter().filter(|(_, on)| **on).count()
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let instructions = parse_lines(&lines);

        let on_points = calculate_on_points_naive(&instructions, -50, 50);

        PartSolution::USize(on_points)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let instructions = parse_lines(&lines);

        let _on_points = calculate_on_points_naive(&instructions, i32::MIN, i32::MAX);

        PartSolution::None
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
                calculate_on_points_naive, generate_points, parse_lines, test::get_larger_example,
                Instruction, Point, Solution,
            },
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::USize(580_012));
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
        fn example_first_step() {
            let example_lines = get_example();

            let reboot_steps = parse_lines(&example_lines);

            let unrolled_points = vec![
                Point {
                    x: 10,
                    y: 10,
                    z: 10,
                },
                Point {
                    x: 10,
                    y: 10,
                    z: 11,
                },
                Point {
                    x: 10,
                    y: 10,
                    z: 12,
                },
                Point {
                    x: 10,
                    y: 11,
                    z: 10,
                },
                Point {
                    x: 10,
                    y: 11,
                    z: 11,
                },
                Point {
                    x: 10,
                    y: 11,
                    z: 12,
                },
                Point {
                    x: 10,
                    y: 12,
                    z: 10,
                },
                Point {
                    x: 10,
                    y: 12,
                    z: 11,
                },
                Point {
                    x: 10,
                    y: 12,
                    z: 12,
                },
                Point {
                    x: 11,
                    y: 10,
                    z: 10,
                },
                Point {
                    x: 11,
                    y: 10,
                    z: 11,
                },
                Point {
                    x: 11,
                    y: 10,
                    z: 12,
                },
                Point {
                    x: 11,
                    y: 11,
                    z: 10,
                },
                Point {
                    x: 11,
                    y: 11,
                    z: 11,
                },
                Point {
                    x: 11,
                    y: 11,
                    z: 12,
                },
                Point {
                    x: 11,
                    y: 12,
                    z: 10,
                },
                Point {
                    x: 11,
                    y: 12,
                    z: 11,
                },
                Point {
                    x: 11,
                    y: 12,
                    z: 12,
                },
                Point {
                    x: 12,
                    y: 10,
                    z: 10,
                },
                Point {
                    x: 12,
                    y: 10,
                    z: 11,
                },
                Point {
                    x: 12,
                    y: 10,
                    z: 12,
                },
                Point {
                    x: 12,
                    y: 11,
                    z: 10,
                },
                Point {
                    x: 12,
                    y: 11,
                    z: 11,
                },
                Point {
                    x: 12,
                    y: 11,
                    z: 12,
                },
                Point {
                    x: 12,
                    y: 12,
                    z: 10,
                },
                Point {
                    x: 12,
                    y: 12,
                    z: 11,
                },
                Point {
                    x: 12,
                    y: 12,
                    z: 12,
                },
            ];

            let result = generate_points(&reboot_steps[0], i32::MIN, i32::MAX);

            assert_eq!(unrolled_points, result);
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

        use crate::day_22::{calculate_on_points, parse_lines, test::get_example_part_2};

        #[test]
        fn outcome() {
            // assert_eq!( (Solution {}).part_2(), PartSolution::U64(306_719_685_234_774));
        }

        #[test]
        fn example() {
            let example_lines = get_example_part_2();

            let instructions = parse_lines(&example_lines);

            // let on_points = calculate_on_points(&instructions, i32::MIN, i32::MAX);

            // assert_eq!(2_758_514_936_282_235, on_points);
        }
    }
}
