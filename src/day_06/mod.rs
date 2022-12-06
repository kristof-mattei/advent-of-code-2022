use std::collections::VecDeque;

use crate::shared::{Day, PartSolution};

fn find_start_of<const C: usize>(line: &str) -> usize {
    let mut v = VecDeque::with_capacity(C);

    for (start, byte) in line.chars().into_iter().enumerate() {
        let p = v.iter().rposition(|v| *v == byte);

        match p {
            Some(index) => {
                // we found the match, pop from left up until the match is gone
                for _ in 0..=index {
                    v.pop_front();
                }
            },
            None if v.len() == C - 1 => {
                // do we have 3 items and is the incoming not present? Winner!
                // minus one because the system expects 1-based
                return start + 1;
            },
            _ => {
                // No match but less than 3? Nothing to do yet
            },
        }

        v.push_back(byte);
    }

    unreachable!()
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let start_of_transmission = find_start_of::<4>(lines[0]);

        start_of_transmission.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let start_of_message = find_start_of::<14>(lines[0]);

        start_of_message.into()
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use crate::{
            day_06::find_start_of,
            day_06::Solution,
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(1804), (Solution {}).part_1());
        }

        #[test]
        fn example_1() {
            let start = find_start_of::<4>("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
            assert_eq!(start, 7);
        }

        #[test]
        fn example_2() {
            let start = find_start_of::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz");
            assert_eq!(start, 5);
        }

        #[test]
        fn example_3() {
            let start = find_start_of::<4>("nppdvjthqldpwncqszvftbrmjlhg");
            assert_eq!(start, 6);
        }

        #[test]
        fn example_4() {
            let start = find_start_of::<4>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
            assert_eq!(start, 10);
        }

        #[test]
        fn example_5() {
            let start = find_start_of::<4>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
            assert_eq!(start, 11);
        }
    }

    mod part_2 {
        use crate::{
            day_06::{find_start_of, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(2508), (Solution {}).part_2());
        }

        #[test]
        fn example_1() {
            let start = find_start_of::<14>("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
            assert_eq!(start, 19);
        }

        #[test]
        fn example_2() {
            let start = find_start_of::<14>("bvwbjplbgvbhsrlpgdmjqwftvncz");
            assert_eq!(start, 23);
        }

        #[test]
        fn example_3() {
            let start = find_start_of::<14>("nppdvjthqldpwncqszvftbrmjlhg");
            assert_eq!(start, 23);
        }

        #[test]
        fn example_4() {
            let start = find_start_of::<14>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
            assert_eq!(start, 29);
        }

        #[test]
        fn example_5() {
            let start = find_start_of::<14>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
            assert_eq!(start, 26);
        }
    }
}
