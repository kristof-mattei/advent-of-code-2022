use std::collections::HashSet;

use crate::shared::{Day, PartSolution};

fn parse_lines(lines: &[&str]) -> Vec<(HashSet<char>, HashSet<char>)> {
    let mut result = Vec::new();

    for line in lines {
        let mut chars = line.chars().collect::<Vec<char>>();

        let second = chars.split_off(chars.len() / 2);

        result.push((
            chars.into_iter().collect::<HashSet<_>>(),
            second.into_iter().collect::<HashSet<_>>(),
        ));
    }

    result
}

fn char_to_value(c: char) -> u32 {
    match c {
        lower @ 'a'..='z' => u32::from(lower) - u32::from('a') + 1,
        upper @ 'A'..='Z' => u32::from(upper) - u32::from('A') + 27,
        _ => {
            panic!("really?")
        },
    }
}

fn calculate_overlap(split_lines: Vec<(HashSet<char>, HashSet<char>)>) -> u32 {
    let mut sum = 0;
    for (left, right) in split_lines {
        let c = *left.intersection(&right).next().unwrap();

        let value = char_to_value(c);

        sum += value;
    }
    sum
}

fn calculate_chunk_overlap(lines: &[&str]) -> u32 {
    let mut score = 0;

    for chunk in lines.chunks(3) {
        let reduced = chunk
            .iter()
            .map(|line| line.chars().collect::<HashSet<char>>())
            .reduce(|acc, cur| acc.intersection(&cur).copied().collect::<_>());

        let common_char = reduced
            .and_then(|r| r.into_iter().next())
            .unwrap_or_default();

        score += char_to_value(common_char);
    }

    score
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let split_lines = parse_lines(&lines);

        let score = calculate_overlap(split_lines);

        score.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let score = calculate_chunk_overlap(&lines);

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
        use super::super::test::get_example;
        use super::super::{calculate_overlap, parse_lines, Solution};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(8252), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines: Vec<&str> = get_example();

            let split_lines = parse_lines(&lines);

            let score = calculate_overlap(split_lines);

            assert_eq!(score, 157);
        }
    }

    mod part_2 {
        use super::super::test::get_example;
        use super::super::{calculate_chunk_overlap, Solution};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(2828), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines: Vec<&str> = get_example();

            let score = calculate_chunk_overlap(&lines);

            assert_eq!(score, 70);
        }
    }
}
