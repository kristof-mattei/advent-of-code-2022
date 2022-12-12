use crate::shared::{Day, PartSolution};

fn parse_lines(lines: &[&str]) -> Vec<((u32, u32), (u32, u32))> {
    let mut result = Vec::new();

    for line in lines {
        let mut numbers = line
            .split(&[',', '-'][..])
            .map(|s| s.parse::<u32>().unwrap());

        let first = numbers.next().unwrap();
        let second = numbers.next().unwrap();
        let third = numbers.next().unwrap();
        let fourth = numbers.next().unwrap();

        result.push(((first, second), (third, fourth)));
    }

    result
}

fn one_fully_contains_other(left: (u32, u32), right: (u32, u32)) -> bool {
    left.0 <= right.0 && left.1 >= right.1
}

fn one_overlaps_other(left: (u32, u32), right: (u32, u32)) -> bool {
    let range = left.0..=left.1;

    range.contains(&right.0) || range.contains(&right.1)
}

fn count_fully_contained(lines: Vec<((u32, u32), (u32, u32))>) -> u32 {
    let mut count = 0;

    for (left, right) in lines {
        if one_fully_contains_other(left, right) || one_fully_contains_other(right, left) {
            count += 1;
        }
    }

    count
}

fn count_any_overlap(lines: Vec<((u32, u32), (u32, u32))>) -> u32 {
    let mut count = 0;

    for (left, right) in lines {
        if one_overlaps_other(left, right) || one_overlaps_other(right, left) {
            count += 1;
        }
    }

    count
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let parsed = parse_lines(&lines);

        let score = count_fully_contained(parsed);

        score.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let parsed = parse_lines(&lines);

        let score = count_any_overlap(parsed);

        score.into()
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
        use crate::day_04::test::get_example;
        use crate::day_04::{count_fully_contained, parse_lines, Solution};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(542), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines: Vec<&str> = get_example();

            let split_lines = parse_lines(&lines);

            let score = count_fully_contained(split_lines);

            assert_eq!(score, 2);
        }
    }

    mod part_2 {
        use crate::day_04::test::get_example;
        use crate::day_04::{count_any_overlap, parse_lines, Solution};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(900), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines: Vec<&str> = get_example();

            let split_lines = parse_lines(&lines);

            let score = count_any_overlap(split_lines);

            assert_eq!(score, 4);
        }
    }
}
