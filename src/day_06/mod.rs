use crate::shared::{Day, PartSolution};

fn find_start_of<const C: usize>(line: &str) -> usize {
    let mut start = 0;

    let bytes = line.as_bytes();

    for (i, byte) in bytes.iter().enumerate() {
        let p = &line
            .as_bytes()
            .iter()
            .skip(start)
            .take(i - start)
            .rposition(|v| v == byte);

        match p {
            Some(index) => {
                start = start + index + 1;
            },
            None if i - start + 1 == C => {
                return i + 1;
            },
            _ => {},
        }
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
        use crate::day_06::{find_start_of, Solution};
        use crate::shared::{Day, PartSolution};

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
        use crate::day_06::{find_start_of, Solution};
        use crate::shared::{Day, PartSolution};

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
