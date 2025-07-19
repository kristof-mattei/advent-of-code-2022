use advent_of_code_2022::shared::{PartSolution, Parts};

advent_of_code_2022::solution!(542, 900);

fn parse_lines(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    let mut result = Vec::new();

    for line in input.lines().map(str::trim) {
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

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let score = count_fully_contained(parsed);

        score.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let score = count_any_overlap(parsed);

        score.into()
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
                PartSolution::U32(542),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::U32(2),
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
                PartSolution::U32(900),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::U32(4),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
