use std::{collections::HashMap, hash::Hash, ops::AddAssign};

use crate::shared::{Day, PartSolution};

fn parse_lines(lines: &[&str]) -> (Vec<char>, HashMap<Key, char>) {
    let mut dictionary = HashMap::new();

    for line in lines.iter().skip(2) {
        let split = line.split(" -> ").collect::<Vec<_>>();

        let from = (*split.first().unwrap()).to_string();

        let cc = from.chars().collect::<Vec<char>>();

        let c0 = cc[0];
        let c1 = cc[1];

        let to = split.get(1).unwrap().parse::<char>().unwrap();

        dictionary.insert(Key { c0, c1 }, to);
    }

    let template = (lines[0]).chars().collect::<Vec<_>>();

    (template, dictionary)
}

#[derive(PartialEq, Eq, Hash)]
struct Key {
    c0: char,
    c1: char,
}

fn parse_lines_part_2(input: &[char]) -> HashMap<Key, u64> {
    let mut map = HashMap::new();

    for cc in input.windows(2) {
        let key = Key {
            c0: cc[0],
            c1: cc[1],
        };

        map.entry(key).and_modify(|c| *c += 1).or_insert(1);
    }

    map
}

fn parse_polymer(input: &[char], pair_insertion_rules: &HashMap<Key, char>) -> Vec<char> {
    let mut new_string: Vec<char> = vec![input[0]];

    for cc in input.windows(2) {
        let lookup = Key {
            c0: cc[0],
            c1: cc[1],
        };

        let translated = pair_insertion_rules.get(&lookup).unwrap();

        new_string.push(*translated);
        new_string.push(cc[1]);
    }

    new_string
}

fn parse_polymer_part_2(
    input: &HashMap<Key, u64>,
    pair_insertion_rules: &HashMap<Key, char>,
) -> HashMap<Key, u64> {
    let mut new_counts = HashMap::new();

    for (key, value) in input.iter().filter(|(_, v)| **v > 0) {
        let c_new = pair_insertion_rules.get(key).unwrap();

        let chars_vec = key;

        let c0 = chars_vec.c0;
        let c1 = chars_vec.c1;

        insert_or_add(&mut new_counts, Key { c0, c1: *c_new }, *value);
        insert_or_add(&mut new_counts, Key { c0: *c_new, c1 }, *value);
    }

    new_counts
}

fn polymer_to_hashmap(input: &[char]) -> HashMap<char, u64> {
    let mut counts: HashMap<char, u64> = HashMap::new();
    for i in input {
        insert_or_add(&mut counts, *i, 1);
    }

    counts
}

fn get_min_and_max_hashmap(hashmap: &HashMap<char, u64>) -> (u64, u64) {
    (
        hashmap.iter().map(|(_, v)| *v).min().unwrap(),
        hashmap.iter().map(|(_, v)| *v).max().unwrap(),
    )
}

fn insert_or_add<T: Eq + Hash, U: AddAssign + Default>(
    hashmap: &mut HashMap<T, U>,
    key: T,
    value: U,
) {
    let entry = hashmap.entry(key).or_default();

    *entry += value;
}

fn polymer_to_hashmap_part2(
    polymer: &[char],
    polymer_groups_set: &HashMap<Key, u64>,
) -> HashMap<char, u64> {
    let first_char = polymer[0];
    let last_char = polymer[polymer.len() - 1];

    let mut counts = HashMap::new();

    for (key, value) in polymer_groups_set {
        insert_or_add(&mut counts, key.c0, *value);
        insert_or_add(&mut counts, key.c1, *value);
    }

    insert_or_add(&mut counts, first_char, 1);
    insert_or_add(&mut counts, last_char, 1);

    counts.iter_mut().for_each(|(_, amount)| *amount /= 2);

    counts
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let (mut polymer, pair_insertion_rules) = parse_lines(&lines);

        for i in 1..=10 {
            polymer = parse_polymer(&polymer, &pair_insertion_rules);

            println!("After step {}: {}", i, polymer.iter().collect::<String>());
        }

        let chars_with_count = polymer_to_hashmap(&polymer);

        let (min, max) = get_min_and_max_hashmap(&chars_with_count);

        PartSolution::U64(max - min)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let (polymer, pair_insertion_rules) = parse_lines(&lines);

        let mut polymer_groups_set = parse_lines_part_2(&polymer);

        for i in 1..=40 {
            println!("Step {}", i);
            polymer_groups_set = parse_polymer_part_2(&polymer_groups_set, &pair_insertion_rules);
        }

        let chars_with_count = polymer_to_hashmap_part2(&polymer, &polymer_groups_set);

        let (min, max) = get_min_and_max_hashmap(&chars_with_count);

        PartSolution::U64(max - min)
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt").lines().collect()
    }

    mod part_1 {

        use crate::{
            day_14::{
                get_min_and_max_hashmap, parse_lines, parse_polymer, polymer_to_hashmap, Solution,
            },
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

            let (mut polymer, pair_insertion_rules) = parse_lines(&lines);

            for i in 1..=10 {
                polymer = parse_polymer(&polymer, &pair_insertion_rules);

                println!("After step {}: {}", i, polymer.iter().collect::<String>());
            }

            let chars_with_count = polymer_to_hashmap(&polymer);

            let (min, max) = get_min_and_max_hashmap(&chars_with_count);

            assert_eq!(min, 161);
            assert_eq!(max, 1749);
        }
    }

    mod part_2 {

        use crate::{
            day_14::{
                get_min_and_max_hashmap, parse_lines, parse_lines_part_2, parse_polymer_part_2,
                polymer_to_hashmap_part2, test::get_example, Solution,
            },
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(
                (Solution {}).part_2(),
                PartSolution::U64(10_002_813_279_337)
            );
        }

        #[test]
        fn example() {
            let lines = get_example();

            let (polymer, pair_insertion_rules) = parse_lines(&lines);

            let mut polymer_groups_set = parse_lines_part_2(&polymer);

            for _ in 1..=10 {
                polymer_groups_set =
                    parse_polymer_part_2(&polymer_groups_set, &pair_insertion_rules);
            }

            let chars_with_count = polymer_to_hashmap_part2(&polymer, &polymer_groups_set);

            let (min, max) = get_min_and_max_hashmap(&chars_with_count);

            assert_eq!(min, 161);
            assert_eq!(max, 1749);
        }
    }
}
