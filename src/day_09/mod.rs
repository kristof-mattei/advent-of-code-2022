use std::cmp::Ordering;
use std::collections::HashSet;

use crate::shared::{Day, PartSolution};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_lines(lines: &[&str]) -> Vec<(Direction, usize)> {
    let mut directions = vec![];
    for line in lines {
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

    for (direction, times) in movements {
        for _ in 0..*times {
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
    match direction {
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

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let movements = parse_lines(&lines);

        let r = move_rope::<2>(&movements);

        r.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let movements = parse_lines(&lines);

        let r = move_rope::<10>(&movements);

        r.into()
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {
        use super::super::{move_rope, parse_lines, Solution};
        use super::get_example;
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(6_332), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let movements = parse_lines(&lines);

            let r = move_rope::<2>(&movements);

            assert_eq!(13, r);
        }
    }

    mod part_2 {
        use super::super::{move_rope, parse_lines, Solution};
        use super::get_example;
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(2_511), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let movements = parse_lines(&lines);

            let r = move_rope::<10>(&movements);

            assert_eq!(1, r);
        }

        #[test]
        fn example_2() {
            let lines = vec!["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];

            let movements = parse_lines(&lines);

            let r = move_rope::<10>(&movements);

            assert_eq!(36, r);
        }
    }
}
