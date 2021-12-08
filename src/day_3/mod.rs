use crate::shared::{Day, PartSolution};
fn parse_diagnostic_report(diagnostic_report_lines: &[u32], bits_to_consider: usize) -> (u32, u32) {
    let mut set_count_by_column: Vec<u32> = vec![0; bits_to_consider];

    for line in diagnostic_report_lines {
        for i in 0..bits_to_consider {
            if (line >> i & 0b0001) == 1 {
                set_count_by_column[i as usize] += 1;
            }
        }
    }

    let half_amount_of_lines = diagnostic_report_lines.len() as u32 / 2;

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..bits_to_consider {
        if set_count_by_column[i as usize] > half_amount_of_lines {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    (gamma, epsilon)
}

enum Common {
    Least,
    Most,
}

fn find_common(lines: &[u32], bit_index: usize, common: &Common) -> usize {
    let mut bits = Vec::new();

    for line in lines {
        bits.push((line >> bit_index) & 0b01);
    }

    let count_of_1 = bits.iter().filter(|x| **x == 1).count();
    let count_of_0 = bits.iter().filter(|x| **x == 0).count();

    match common {
        Common::Least => {
            if count_of_1 >= count_of_0 {
                0
            } else {
                1
            }
        }
        Common::Most => {
            if count_of_1 >= count_of_0 {
                1
            } else {
                0
            }
        }
    }
}

fn move_and_reduce(
    filtered_lines: &[u32],
    bits_to_consider: usize,
    bit_index: usize,
    common: &Common,
) -> Vec<u32> {
    if filtered_lines.len() == 1 {
        return filtered_lines.to_vec();
    }

    let mut new_set: Vec<u32> = Vec::new();

    let most_or_least_common =
        find_common(filtered_lines, bits_to_consider - bit_index - 1, common);

    for line in filtered_lines {
        if (line >> (bits_to_consider - bit_index - 1) & 0b01) == most_or_least_common as u32 {
            new_set.push(*line);
        }
    }

    move_and_reduce(&new_set, bits_to_consider, bit_index + 1, common)
}

fn parse_diagnostic_report_2(
    diagnostic_report_lines: &[u32],
    bits_to_consider: usize,
) -> (u32, u32) {
    let oxygen = move_and_reduce(diagnostic_report_lines, bits_to_consider, 0, &Common::Most);
    let co2 = move_and_reduce(diagnostic_report_lines, bits_to_consider, 0, &Common::Least);

    (oxygen[0], co2[0])
}

fn parse_lines(lines: &[String]) -> (Vec<u32>, usize) {
    let mut parsed = Vec::new();

    for line in lines {
        parsed.push(u32::from_str_radix(line, 2).unwrap());
    }

    let bits_to_consider = match lines.get(0) {
        Some(l) => l.len(),
        None => 0,
    };

    (parsed, bits_to_consider)
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let (diagnostic_report_lines, bits_to_consider) = parse_lines(&lines);

        let (gamma, epsilon) = parse_diagnostic_report(&diagnostic_report_lines, bits_to_consider);

        PartSolution::U32(gamma * epsilon)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let (diagnostic_report_lines, bits_to_consider) = parse_lines(&lines);

        let (oxygen, co2) = parse_diagnostic_report_2(&diagnostic_report_lines, bits_to_consider);

        PartSolution::U32(oxygen * co2)
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
            day_3::{parse_diagnostic_report, parse_lines, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(4_160_394));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let (instructions, bits_to_consider) = parse_lines(&lines);

            let (gamma, epsilon) = parse_diagnostic_report(&instructions, bits_to_consider);

            assert_eq!(gamma, 22);
            assert_eq!(epsilon, 9);
        }
    }
    mod part_2 {
        use crate::{
            day_3::{parse_diagnostic_report_2, parse_lines, test::get_example, Solution},
            shared::{Day, PartSolution},
        };
        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U32(4_125_600));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let (instructions, bits_to_consider) = parse_lines(&lines);

            let (oxygen, co2) = parse_diagnostic_report_2(&instructions, bits_to_consider);

            assert_eq!(oxygen, 23);
            assert_eq!(co2, 10);
        }
    }
}
