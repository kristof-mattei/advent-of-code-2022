use crate::shared::{Day, PartSolution};

struct CaveSystem {}

fn build_cave_system(lines: &[String]) -> CaveSystem {
    todo!()
}

fn calculate_all_paths(cave_system: &CaveSystem) -> u32 {
    0
}
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
            day_12::{build_cave_system, calculate_all_paths, CaveSystem, Solution},
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

            let cave_system: CaveSystem = build_cave_system(&lines);

            let paths: u32 = calculate_all_paths(&cave_system);

            assert_eq!(paths, 10);
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
