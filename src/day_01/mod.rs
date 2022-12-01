use crate::shared::{Day, PartSolution};

fn get_max(lines: &[&str]) -> u32 {
    let mut max = 0;

    let mut current = 0;

    for line in lines {
        if line.is_empty() {
            if current > max {
                max = current;
            }

            current = 0;
            continue;
        }

        current += line.parse::<u32>().unwrap();
    }

    max
}

fn get_top_3(lines: &[&str]) -> u32 {
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;

    let mut current = 0;
    for line in lines {
        if line.is_empty() {
            println!("Current: {current}");
            if current > max1 {
                max3 = max2;
                max2 = max1;
                max1 = current;
            } else if current > max2 {
                max3 = max2;
                max2 = current;
            } else if current > max3 {
                max3 = current;
            }

            current = 0;
            continue;
        }

        current += line.parse::<u32>().unwrap();
    }

    max1 + max2 + max3
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let max = get_max(&lines);

        PartSolution::U32(max)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let max = get_top_3(&lines);

        PartSolution::U32(max)
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {
        use crate::{
            day_01::{get_max, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(67658), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let max = get_max(&lines);

            assert_eq!(max, 24000);
        }
    }

    mod part_2 {
        use crate::{
            day_01::{get_top_3, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(1748), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let top3 = get_top_3(&lines);

            assert_eq!(top3, 5);
        }
    }
}
