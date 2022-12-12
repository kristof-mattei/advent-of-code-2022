use crate::shared::{Day, PartSolution};

fn get_max(lines: &[&str]) -> u32 {
    let mut max = 0;

    let mut current = 0;

    // chain because the .lines() skips the final empty line
    for line in lines.iter().chain(&[""]) {
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

fn shift(current: u32, max: &mut [u32; 3]) {
    let shift_after = max.iter().position(|v| v < &current);

    let Some(index) = shift_after else { return };

    let mut c_i = max.len() - 1;

    while c_i > (index) {
        max.swap(c_i, c_i - 1);

        c_i -= 1;
    }

    max[index] = current;
}

fn get_top_3(lines: &[&str]) -> u32 {
    let mut max = [0; 3];

    let mut current = 0;

    // chain because the .lines() skips the final empty line
    for line in lines.iter().chain(&[""]) {
        if line.is_empty() {
            shift(current, &mut max);

            current = 0;
            continue;
        }

        current += line.parse::<u32>().unwrap();
    }

    max.iter().sum()
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let max = get_max(&lines);

        max.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let max = get_top_3(&lines);

        max.into()
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
        use crate::day_01::test::get_example;
        use crate::day_01::{get_max, Solution};
        use crate::shared::{Day, PartSolution};

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
        use crate::day_01::test::get_example;
        use crate::day_01::{get_top_3, Solution};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(200_158), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let top3 = get_top_3(&lines);

            assert_eq!(top3, 45000);
        }
    }
}
