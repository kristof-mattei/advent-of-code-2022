use advent_of_code_2022::shared::{PartSolution, Parts};
use hashbrown::HashSet;

advent_of_code_2022::solution!(8252, 2828);

fn parse_lines(input: &str) -> Vec<(HashSet<char>, HashSet<char>)> {
    let mut result = Vec::new();

    for line in input.lines().map(str::trim) {
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

fn calculate_chunk_overlap(input: &str) -> u32 {
    let mut score = 0;

    for chunk in input
        .lines()
        .map(str::trim)
        .collect::<Vec<&str>>()
        .chunks(3)
    {
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

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let split_lines = parse_lines(input);

        let score = calculate_overlap(split_lines);

        score.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let score = calculate_chunk_overlap(input);

        score.into()
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts};

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::U32(8252),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::U32(157),
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts};

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::U32(2828),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::U32(70),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
