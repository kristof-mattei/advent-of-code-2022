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

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

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

        PartSolution::U64(max - min)
    }

    fn part_2(&self) -> PartSolution {
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

    mod part_2 {}
}
