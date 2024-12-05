use crate::shared::{Day, PartSolution};

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            whut => Err(format!("What do I do with {whut}")),
        }
    }
}

fn parse_lines(lines: &str) -> Vec<Direction> {
    let directions = lines
        .chars()
        .into_iter()
        .map(|c| c.try_into().unwrap())
        .collect::<Vec<_>>();

    directions
}

type Shape = [[char; 4]; 4];

const H_BAR: Shape = [
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
    ['#', '#', '#', '#'],
];

const CROSS: Shape = [
    [' ', '#', ' ', ' '],
    ['#', '#', '#', ' '],
    [' ', '#', ' ', ' '],
    [' ', ' ', ' ', ' '],
];

const REV_L: Shape = [
    [' ', ' ', '#', ' '],
    [' ', ' ', '#', ' '],
    ['#', '#', '#', ' '],
    [' ', ' ', ' ', ' '],
];

const V_BAR: Shape = [
    ['#', ' ', ' ', ' '],
    ['#', ' ', ' ', ' '],
    ['#', ' ', ' ', ' '],
    ['#', ' ', ' ', ' '],
];

const BLOCK: Shape = [
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
    ['#', '#', ' ', ' '],
    ['#', '#', ' ', ' '],
];

const BLOCKS: [Shape; 5] = [H_BAR, CROSS, REV_L, V_BAR, BLOCK];

fn calculate_height(directions: &[Direction]) -> usize {
    let playfield = vec![['-'; 7]; 1];

    0
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: &str = include_str!("input.txt");

        let directions = parse_lines(lines);

        calculate_height(&directions).into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: &str = include_str!("input.txt");

        let directions = parse_lines(lines);

        calculate_height(&directions).into()
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> &'static str {
        include_str!("example.txt")
    }

    mod part_1 {
        use super::super::parse_lines;
        use super::super::Solution;
        use super::get_example;
        use crate::day_17::calculate_height;
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(1_297_683), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(lines);

            let height = calculate_height(&parsed);

            assert_eq!(3068, height);
        }
    }

    mod part_2 {
        use crate::day_17::calculate_height;
        use crate::shared::Day;
        use crate::shared::PartSolution;

        use super::super::parse_lines;
        use super::super::Solution;
        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(5_756_764), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let directions = parse_lines(lines);

            let height = calculate_height(&directions);

            assert_eq!(0, height);
        }
    }
}
