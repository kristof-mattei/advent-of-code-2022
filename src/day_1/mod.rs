use crate::shared::{Day, PartSolution};

fn parse_lines(lines: &[String]) -> Vec<u32> {
    lines.iter().map(|s| s.parse::<u32>().unwrap()).collect()
}

pub fn count_increments(list: &[u32]) -> u32 {
    let mut count = 0;
    for i in list.windows(2) {
        if i[1] > i[0] {
            count += 1;
        }
    }

    count
}

pub fn count_window_of_3_increments(list: &[u32]) -> u32 {
    let mut count = 0;
    let mut previous_window: u32 = 0;

    for i in list.windows(3) {
        let current_window: u32 = i.iter().sum();

        if current_window > previous_window {
            count += 1;
        }

        previous_window = current_window;
    }

    count - 1
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let numbers = parse_lines(&lines);

        PartSolution::U32(count_increments(&numbers))
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let numbers = parse_lines(&lines);

        PartSolution::U32(count_window_of_3_increments(&numbers))
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
            day_1::{count_increments, parse_lines, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(1722), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let depth_measurements = parse_lines(&lines);

            assert_eq!(count_increments(&depth_measurements), 7);
        }
    }
    mod part_2 {
        use crate::{
            day_1::{count_window_of_3_increments, parse_lines, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(1748), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let depth_measurements = parse_lines(&lines);

            assert_eq!(count_window_of_3_increments(&depth_measurements), 5);
        }
    }
}
