#![expect(clippy::cast_possible_truncation)]
#![expect(clippy::cast_possible_wrap)]
use advent_of_code_2022::shared::{PartSolution, Parts};

advent_of_code_2022::solution!(1087i64, 13_084_440_324_666_i64);

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect()
}

fn decode(input: &str, key: i64, times: usize) -> PartSolution {
    let mut numbers: Vec<(usize, i64)> = parse_input(input)
        .into_iter()
        .map(|v| v * key)
        .enumerate()
        .collect();

    for _ in 0..times {
        for original_index in 0..numbers.len() {
            let old_index = numbers.iter().position(|x| x.0 == original_index).unwrap();

            let value = numbers[old_index].1;

            // calculate new position
            let new_index = old_index as isize + value as isize;

            // % doesn't work for negative values.
            // move by length - 1 otherwise we move one too far
            let new_index = new_index.rem_euclid(numbers.len() as isize - 1) as usize;

            // move position
            let tmp = numbers.remove(old_index);
            numbers.insert(new_index, tmp);
        }
    }

    let zero = numbers.iter().position(|&(_, value)| value == 0).unwrap();

    let thousand = numbers[(zero + 1_000) % numbers.len()].1;
    let two_thousand = numbers[(zero + 2_000) % numbers.len()].1;
    let three_thousand = numbers[(zero + 3_000) % numbers.len()].1;

    PartSolution::I64(thousand + two_thousand + three_thousand)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        decode(input, 1, 1)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        decode(input, 811_589_153, 10)
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use advent_of_code_2022::shared::Parts;
        use advent_of_code_2022::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(1087, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example() {
            assert_eq!(3, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::Parts;
        use advent_of_code_2022::shared::solution::read_file;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                13_084_440_324_666_i64,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                1_623_178_306,
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
