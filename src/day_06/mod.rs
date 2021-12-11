use std::collections::HashMap;

use crate::shared::{Day, PartSolution};

fn parse_lines(lines: &[String]) -> Vec<u8> {
    let fishes_nearby = lines[0]
        .split(',')
        .map(|f| f.parse::<u8>().unwrap())
        .collect();

    fishes_nearby
}

fn age_fishes(fishes: &mut Vec<u8>) {
    let mut new_fishes_to_append: usize = 0;

    for fish in fishes.iter_mut() {
        if *fish == 0 {
            *fish = 6;
            new_fishes_to_append += 1;
        } else {
            *fish -= 1;
        }
    }

    fishes.resize(fishes.len() + new_fishes_to_append, 8);
}

fn speed_up_fishes(fishes_nearby: &[u8]) -> HashMap<u8, u64> {
    let mut fish_counts: HashMap<u8, u64> = HashMap::new();

    for fish in fishes_nearby {
        let fish_count = *fish_counts.get(fish).unwrap_or(&0);

        fish_counts.insert(*fish, fish_count + 1);
    }

    fish_counts
}

fn age_fishes_fast(fishes: &mut HashMap<u8, u64>) {
    // fishes age 0 will be reset to age 6, and will spawn new fishes aged 8
    let fishes_which_will_spawn_new_fishes = *fishes.get(&0).unwrap_or(&0);

    for i in 1..=8 {
        let current = *fishes.get(&i).unwrap_or(&0);

        fishes.insert(i - 1, current);
    }

    // all fishes reset create 1 offspring
    fishes.insert(8, fishes_which_will_spawn_new_fishes);

    // and add the ones at age 0 back to the pool, but they start back at 6
    let entry_6 = fishes.entry(6).or_default();
    *entry_6 += fishes_which_will_spawn_new_fishes;
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let mut fishes = parse_lines(&lines);
        for _ in 0..80 {
            age_fishes(&mut fishes);
        }

        PartSolution::USize(fishes.len())
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let fishes = parse_lines(&lines);

        let mut fast_fishes = speed_up_fishes(&fishes);
        for _ in 1..=256 {
            age_fishes_fast(&mut fast_fishes);
        }

        PartSolution::U64(fast_fishes.iter().map(|(_, v)| v).sum::<u64>())
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
            day_06::{age_fishes, parse_lines, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::USize(395_627));
        }

        #[test]
        fn example() {
            let lines: Vec<String> = get_example();

            let mut fishes: Vec<u8> = parse_lines(&lines);

            for _ in 0..18 {
                age_fishes(&mut fishes);
                println!("{:?}", fishes);
            }

            assert_eq!(26, fishes.len());
        }

        #[test]
        fn example_2() {
            let lines: Vec<String> = get_example();

            let mut fishes: Vec<u8> = parse_lines(&lines);

            for _ in 0..80 {
                age_fishes(&mut fishes);
            }

            assert_eq!(5934, fishes.len());
        }
    }

    mod part_2 {
        use crate::{
            day_06::{age_fishes_fast, parse_lines, speed_up_fishes, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U64(1_767_323_539_209));
        }

        #[test]
        fn example() {
            let lines: Vec<String> = get_example();

            let fishes = parse_lines(&lines);

            let mut fast_fishes = speed_up_fishes(&fishes);

            for _ in 1..=18 {
                age_fishes_fast(&mut fast_fishes);
            }

            assert_eq!(26, fast_fishes.iter().map(|(_, v)| v).sum::<u64>());
        }

        #[test]
        fn example_2() {
            let lines: Vec<String> = get_example();

            let fishes = parse_lines(&lines);

            let mut fast_fishes = speed_up_fishes(&fishes);

            for _ in 1..=80 {
                age_fishes_fast(&mut fast_fishes);
            }

            assert_eq!(5934, fast_fishes.iter().map(|(_, v)| v).sum::<u64>());
        }

        #[test]
        fn example_3() {
            let lines: Vec<String> = get_example();

            let fishes = parse_lines(&lines);

            let mut fast_fishes = speed_up_fishes(&fishes);

            for _ in 1..=256 {
                age_fishes_fast(&mut fast_fishes);
            }

            assert_eq!(
                26_984_457_539,
                fast_fishes.iter().map(|(_, v)| v).sum::<u64>()
            );
        }
    }
}
