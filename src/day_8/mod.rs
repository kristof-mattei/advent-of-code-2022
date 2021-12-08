use crate::shared::{Day, PartSolution};

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        PartSolution::U32(0)
    }

    fn part_2(&self) -> PartSolution {
        PartSolution::U32(0)
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
            day_8::{test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(0));
        }

        #[test]
        fn playground() {
            let _lines = get_example();

            // let lines: _ = parse_lines(&lines);

            // assert_eq!(0, solve(&lines));
        }
    }

    mod part_2 {
        use crate::{
            day_8::Solution,
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U32(0));
        }

        #[test]
        fn example() {
            let _lines = get_example();

            // let lines: _ = parse_lines(&lines);

            // assert_eq!(0, solve(&lines));
        }
    }
}
