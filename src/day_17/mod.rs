
use crate::shared::{Day, PartSolution};

fn parse_lines(_lines: &[&str]) -> Vec<u32> {
    todo!()
}
pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let _parsed = parse_lines(&lines);

        todo!()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let _parsed = parse_lines(&lines);

        todo!()
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
        use super::super::parse_lines;
        use super::super::Solution;
        use super::get_example;
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(1_297_683), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            assert_eq!(Vec::<u32>::new(), parsed);
        }
    }

    mod part_2 {
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

            let parsed = parse_lines(&lines);

            assert_eq!(Vec::<u32>::new(), parsed);
        }
    }
}
