use core::panic;

use crate::{shared::AoCResult, utils::read_file};

fn calculate_step_up_differences(input: &[u32]) -> (u32, u32, u32) {
    let mut stepup_1 = 0;
    let mut stepup_2 = 0;
    let mut stepup_3 = 0;

    let mut copy: Vec<u32> = input.iter().copied().collect();

    copy.sort_unstable();

    copy.reverse();

    let mut previous = 0;

    loop {
        match copy.pop() {
            Some(next) => {
                println!(
                    "Previous: {}, next: {}, diff: {}",
                    previous,
                    next,
                    (next - previous)
                );

                match next - previous {
                    1 => stepup_1 += 1,
                    2 => stepup_2 += 1,
                    3 => stepup_3 += 1,
                    _ => {
                        panic!("This really shouldn't happen")
                    }
                }

                previous = next;
            }
            None => return (stepup_1, stepup_2, stepup_3 + 1),
        }
    }
}

// https://adventofcode.com/2020/day/9
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_10/input.txt".into())?;
    let input: Vec<u32> = split.iter().map(|s| s.parse::<u32>().unwrap()).collect();

    let (s1, _, s3) = calculate_step_up_differences(&input);
    Ok(AoCResult::Ofu32(s1 * s3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(AoCResult::Ofu32(1820), find_solution().unwrap());
    }

    #[test]
    fn test_example_1() {
        let input: Vec<u32> = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"]
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let (stepup_1, _stepup_2, stepup_3) = calculate_step_up_differences(&input);

        assert_eq!(7, stepup_1);
        assert_eq!(5, stepup_3);
    }

    #[test]
    fn test_example_2() {
        let input: Vec<u32> = vec![
            "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45",
            "19", "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34",
            "10", "3",
        ]
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

        let (stepup_1, _stepup_2, stepup_3) = calculate_step_up_differences(&input);

        assert_eq!(22, stepup_1);
        assert_eq!(10, stepup_3);
    }
}
