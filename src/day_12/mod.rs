use crate::shared::{Day, PartSolution};

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let _lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        PartSolution::None
    }

    fn part_2(&self) -> PartSolution {
        let _lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        PartSolution::None
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<String> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {

        use crate::{
            day_12::Solution,
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::None);
        }

        #[test]
        fn example() {
            let lines = get_example();

            assert!(!lines.is_empty());
        }
    }

    mod part_2 {

        use crate::{
            day_12::Solution,
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::None);
        }
        #[test]
        fn example() {
            let lines = get_example();

            assert!(!lines.is_empty());
        }
    }
}
