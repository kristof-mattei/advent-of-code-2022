use std::collections::HashMap;

use crate::shared::{Day, PartSolution};
use std::cmp;

fn parse_lines(lines: &[String]) -> Vec<u32> {
    lines[0]
        .split(',')
        .map(|f| f.parse::<u32>().unwrap())
        .collect()
}

fn find_minimum(crabs: &[u32]) -> u32 {
    let minimum_depth = *crabs.iter().min().unwrap();
    let max_depth = *crabs.iter().max().unwrap();

    let mut min = u32::MAX;

    for depth in minimum_depth..=max_depth {
        let mut fuel_needed_for_depth: u32 = 0;

        for crab in crabs {
            let fuel_needed = cmp::max(*crab, depth) - cmp::min(*crab, depth);

            fuel_needed_for_depth += fuel_needed;

            // no need to continue if we are already over the last minimum
            if fuel_needed_for_depth > min {
                break;
            }
        }

        if fuel_needed_for_depth < min {
            min = fuel_needed_for_depth;
        }
    }

    min
}

fn calculate_fuel_needed_2(distance: u32, cache: &mut HashMap<u32, u32>) -> u32 {
    *cache.entry(distance).or_insert_with(|| {
        let mut fuel_needed: u32 = 0;
        for i in 1..=distance {
            fuel_needed += i;
        }
        fuel_needed
    })
}

fn find_minimum_2(crabs: &[u32]) -> u32 {
    let minimum_depth = *crabs.iter().min().unwrap();
    let max_depth = *crabs.iter().max().unwrap();

    let mut min = u32::MAX;

    let mut fuel_needed_cache = HashMap::<u32, u32>::new();

    for depth in minimum_depth..=max_depth {
        let mut fuel_needed_for_depth: u32 = 0;

        for crab in crabs {
            let crab_distance_from_depth = cmp::max(*crab, depth) - cmp::min(*crab, depth);

            fuel_needed_for_depth +=
                calculate_fuel_needed_2(crab_distance_from_depth, &mut fuel_needed_cache);

            // no need to continue if we are already over the last minimum
            if fuel_needed_for_depth > min {
                break;
            }
        }

        if fuel_needed_for_depth < min {
            min = fuel_needed_for_depth;
        }
    }

    min
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let crabs = parse_lines(&lines);

        PartSolution::U32(find_minimum(&crabs))
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let crabs = parse_lines(&lines);

        PartSolution::U32(find_minimum_2(&crabs))
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
            day_07::{find_minimum, parse_lines, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(339_321));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let crabs: Vec<u32> = parse_lines(&lines);

            assert_eq!(37, find_minimum(&crabs));
        }
    }

    mod part_2 {
        use std::collections::HashMap;

        use crate::{
            day_07::{
                calculate_fuel_needed_2, find_minimum_2, parse_lines, test::get_example, Solution,
            },
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U32(95_476_244));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let crabs: Vec<u32> = parse_lines(&lines);

            assert_eq!(168, find_minimum_2(&crabs));
        }

        #[test]
        fn fuel_needed() {
            assert_eq!(1, calculate_fuel_needed_2(1, &mut HashMap::new()));
            assert_eq!(3, calculate_fuel_needed_2(2, &mut HashMap::new()));
            assert_eq!(15, calculate_fuel_needed_2(5, &mut HashMap::new()));
        }
    }
}
