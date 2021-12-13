use std::collections::HashMap;

use crate::shared::{Day, PartSolution};

fn find_or_insert(caves: &mut CaveSystem, left_cave: String, right_cave: String) {
    caves
        .entry(left_cave)
        .or_insert_with(Vec::new)
        .push(right_cave);
}

type CaveSystem = HashMap<String, Vec<String>>;

fn build_cave_system(lines: &[String]) -> CaveSystem {
    let mut caves: CaveSystem = CaveSystem::default();

    for line in lines {
        let pieces: Vec<String> = line.split('-').map(Into::into).collect();

        let left = pieces.get(0).unwrap();
        let right = pieces.get(1).unwrap();

        find_or_insert(&mut caves, left.clone(), right.clone());
    }

    caves
}

fn go_forth(
    _cave_system: &HashMap<String, Vec<String>>,
    _current_cave: &str,
    _visited: &mut Vec<String>,
) -> u32 {
    0
}

fn calculate_all_paths(cave_system: &CaveSystem) -> u32 {
    let start = cave_system
        .iter()
        .find(|(name, _)| *name == "start")
        .map(|(name, _)| name)
        .unwrap();

    let mut visited: Vec<String> = Vec::new();

    go_forth(cave_system, &start.clone(), &mut visited)
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let cave_system: CaveSystem = build_cave_system(&lines);

        let _paths: u32 = calculate_all_paths(&cave_system);

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
        #[should_panic]
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
