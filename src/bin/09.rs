#![expect(clippy::string_slice, reason = "Advent of Code is ASCII only")]

use std::cmp::Ordering;
use std::collections::HashSet;

use advent_of_code_2022::shared::{PartSolution, Parts};

advent_of_code_2022::solution!(6_332, 2_511);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_lines(input: &str) -> Vec<(Direction, usize)> {
    let mut directions = vec![];

    for line in input.lines() {
        let times = line[2..].parse::<usize>().unwrap();

        let direction = match line.as_bytes()[0] {
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => {
                panic!()
            },
        };

        directions.push((direction, times));
    }

    directions
}

fn move_rope<const LEN: usize>(movements: &[(Direction, usize)]) -> usize {
    let mut tail_visits = HashSet::<(isize, isize)>::new();

    let mut rope = vec![(0, 0); LEN];

    for &(ref direction, times) in movements {
        for _ in 0..times {
            move_head(direction, &mut rope[0]);

            for i in 1..LEN {
                let (left, right) = rope.split_at_mut(i);

                follow_tail(&left[i - 1], &mut right[0]);
            }

            tail_visits.insert(rope[LEN - 1]);
        }
    }

    tail_visits.len()
}

fn move_head(direction: &Direction, head: &mut (isize, isize)) {
    match *direction {
        Direction::Up => head.0 -= 1,
        Direction::Down => head.0 += 1,

        Direction::Left => head.1 -= 1,
        Direction::Right => head.1 += 1,
    }
}

fn follow_tail(head: &(isize, isize), tail: &mut (isize, isize)) {
    if head.0.abs_diff(tail.0) >= 2 || head.1.abs_diff(tail.1) >= 2 {
        // we need to take a single action

        match (head.0.cmp(&tail.0), head.1.cmp(&tail.1)) {
            (Ordering::Equal, Ordering::Less) => {
                tail.1 -= 1;
            },
            (Ordering::Equal, Ordering::Greater) => {
                tail.1 += 1;
            },
            (Ordering::Less, Ordering::Equal) => {
                tail.0 -= 1;
            },
            (Ordering::Greater, Ordering::Equal) => {
                tail.0 += 1;
            },
            (Ordering::Less, Ordering::Less) => {
                tail.0 -= 1;
                tail.1 -= 1;
            },
            (Ordering::Less, Ordering::Greater) => {
                tail.0 -= 1;
                tail.1 += 1;
            },
            (Ordering::Greater, Ordering::Less) => {
                tail.0 += 1;
                tail.1 -= 1;
            },
            (Ordering::Greater, Ordering::Greater) => {
                tail.0 += 1;
                tail.1 += 1;
            },
            _ => panic!(),
        }
    }
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let movements = parse_lines(input);

        let r = move_rope::<2>(&movements);

        r.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let movements = parse_lines(input);

        let r = move_rope::<10>(&movements);

        r.into()
    }
}

#[cfg(test)]
mod tests {

    mod part_1 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts as _};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(6_332),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(13),
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts as _};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(2_511),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(1),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }

        #[test]
        fn example_2() {
            let lines = ["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];

            assert_eq!(
                PartSolution::USize(36),
                (Solution {}).part_2(&lines.join("\n"))
            );
        }
    }
}
