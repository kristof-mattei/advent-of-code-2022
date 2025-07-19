use advent_of_code_2022::shared::{PartSolution, Parts};

advent_of_code_2022::solution!(67658, 200_158);

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

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let lines: Vec<&str> = input.lines().collect();

        let max = get_max(&lines);

        max.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let lines: Vec<&str> = input.lines().collect();

        let max = get_top_3(&lines);

        max.into()
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts as _};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::U32(67658),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::U32(24000),
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts as _};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::U32(200_158),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(45000, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
