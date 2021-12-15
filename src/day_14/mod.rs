use std::{collections::HashMap, hash::Hash};

use crate::shared::{Day, PartSolution};

fn parse_lines(lines: &[String]) -> (Vec<char>, HashMap<String, char>) {
    let mut dictionary = HashMap::new();

    for line in lines.iter().skip(2) {
        let split = line.split(" -> ").collect::<Vec<_>>();

        let from = split.get(0).unwrap().to_string();
        let to = split.get(1).unwrap().parse::<char>().unwrap();

        dictionary.insert(from, to);
    }

    let template = (&lines[0]).chars().collect::<Vec<_>>();

    (template, dictionary)
}

fn parse_lines_part_2(input: &Vec<char>) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for cc in input.windows(2) {
        let lookup: String = cc.iter().collect();

        map.entry(lookup).and_modify(|c| *c += 1).or_insert(1);
    }

    map
}

fn parse_polymer(input: &Vec<char>, pair_insertion_rules: &HashMap<String, char>) -> Vec<char> {
    let mut new_string: Vec<char> = Vec::new();

    // let chars: Vec<char> = input.chars().collect();
    new_string.push(input[0]);

    for cc in input.windows(2) {
        let lookup: String = cc.iter().collect();

        let translated = pair_insertion_rules.get(&lookup).unwrap();

        new_string.push(*translated);
        new_string.push(cc[1]);
    }

    new_string // .iter().collect()
}

fn parse_polymer_part_2(
    input: &HashMap<String, u32>,
    pair_insertion_rules: &HashMap<String, char>,
) -> HashMap<String, u32> {
    let mut part_2 = HashMap::new();

    for (key, value) in input.iter().filter(|(_, v)| **v > 0) {
        let c_new = pair_insertion_rules.get(key).unwrap();

        // *value -= 1;

        for _ in 0..*value {
            let chars_vec = key.chars().collect::<Vec<_>>();

            let c1 = chars_vec.get(0).unwrap();
            let c2 = chars_vec.get(1).unwrap();

            part_2
                .entry(vec![*c1, *c_new].iter().collect::<String>())
                .and_modify(|c| *c += 1)
                .or_insert(1);

            part_2
                .entry(vec![*c_new, *c2].iter().collect::<String>())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    part_2
}

fn count_min_and_max<T>(input: &[T]) -> (u64, u64)
where
    T: Eq + Hash + Copy,
{
    let mut counts: HashMap<T, u64> = HashMap::new();
    for i in input {
        let c = counts.entry(*i).or_insert(0);

        *c += 1;
    }

    (
        counts.iter().map(|(_, v)| *v).min().unwrap(),
        counts.iter().map(|(_, v)| *v).max().unwrap(),
    )
}

fn dump_string(polymer: &Vec<char>, polymer_groups_set: &HashMap<String, u32>) -> Vec<char> {
    let first_char = polymer[0];
    let last_char = polymer[polymer.len() - 1];

    // // polymer_groups_set
    // //     .entry((vec![first_char, last_char]).iter().collect::<String>())
    // //     .and_modify(|c| *c += 1)
    // //     .or_insert(1);

    let mut min_max_string = Vec::new();

    for (key, value) in polymer_groups_set {
        let chars = key.chars().collect::<Vec<_>>();
        for _ in 0..*value {
            min_max_string.push(*chars.get(0).unwrap());
            min_max_string.push(*chars.get(1).unwrap());
        }
    }

    min_max_string.push(first_char);
    min_max_string.push(last_char);
    min_max_string.sort();

    min_max_string
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let (mut polymer, pair_insertion_rules) = parse_lines(&lines);

        for i in 1..=10 {
            polymer = parse_polymer(&polymer, &pair_insertion_rules);

            println!("After step {}: {}", i, polymer.iter().collect::<String>());
        }

        let (min, max) = count_min_and_max(&polymer);

        PartSolution::U64(max - min)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let (polymer, pair_insertion_rules) = parse_lines(&lines);

        let mut polymer_groups_set = parse_lines_part_2(&polymer);
        let _ = dump_string(&polymer, &polymer_groups_set);

        for i in 1..=40 {
            println!("Step {}", i);
            polymer_groups_set = parse_polymer_part_2(&polymer_groups_set, &pair_insertion_rules);
        }

        let min_max_string = dump_string(&polymer, &polymer_groups_set);

        let (min, max) = count_min_and_max(&min_max_string);
        PartSolution::U64(max / 2 - min / 2)
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
            day_14::{count_min_and_max, parse_lines, parse_polymer, Solution},
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U64(2851));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let (mut new_string, pair_insertion_rules) = parse_lines(&lines);

            for i in 1..=10 {
                new_string = parse_polymer(&new_string, &pair_insertion_rules);

                println!(
                    "After step {}: {}",
                    i,
                    new_string.iter().collect::<String>()
                );
            }

            let (min, max) = count_min_and_max(&new_string);

            assert_eq!(min, 161);
            assert_eq!(max, 1749);
        }
    }

    mod part_2 {

        use crate::{
            day_14::{
                count_min_and_max, dump_string, parse_lines, parse_lines_part_2,
                parse_polymer_part_2, test::get_example, Solution,
            },
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U64(2851));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let (polymer, pair_insertion_rules) = parse_lines(&lines);

            let mut polymer_groups_set = parse_lines_part_2(&polymer);
            let _ = dump_string(&polymer, &polymer_groups_set);

            for i in 1..=10 {
                polymer_groups_set =
                    parse_polymer_part_2(&polymer_groups_set, &pair_insertion_rules);

                let min_max_string = dump_string(&polymer, &polymer_groups_set);

                let (min, max) = count_min_and_max(&min_max_string);

                println!(
                    "After {}: {}, min: {}, max: {}",
                    i,
                    min_max_string.iter().collect::<String>(),
                    min,
                    max
                );
            }

            let min_max_string = dump_string(&polymer, &polymer_groups_set);

            let (min, max) = count_min_and_max(&min_max_string);

            assert_eq!(min / 2, 161);
            assert_eq!(max / 2, 1749);
        }
    }
}
