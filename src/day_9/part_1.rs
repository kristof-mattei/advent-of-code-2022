use std::{collections::HashMap, ops::Deref};

use crate::{
    shared::{AoCError, AoCResult},
    utils::read_file,
};

// taken from day_1 and modified
fn find_sum_of_2_is_x(numbers: &[u64], target: u64) -> Result<(), AoCError> {
    let mut missing_to_value: HashMap<i64, u64> = HashMap::new();

    for n in numbers {
        match missing_to_value.get(&((*n) as i64)) {
            Some(_) => {
                return Ok(());
            }
            None => {
                missing_to_value.insert(target as i64 - *n as i64, *n);
            }
        }
    }

    Err(AoCError {
        message: format!(
            "No combination found in the last {} that equals {}",
            numbers.iter().count(),
            target
        ),
    })
}

fn slide_until_sum_of_any_2_in_last_x_is_not_current_value(input: &[u64], last_x: usize) -> u64 {
    let mut offset = 0;

    loop {
        let to_test: Vec<u64> = input.iter().skip(offset).take(last_x).map(|x| *x).collect();

        let target_sum_iter = input
            .iter()
            .skip(offset + last_x + 1)
            .collect::<Vec<&u64>>();

        let target_sum = target_sum_iter.get(0).unwrap().deref();

        match find_sum_of_2_is_x(&to_test, *target_sum) {
            Ok(_) => {
                println!(
                    "Couln't find matching sum of {} in {:?}",
                    target_sum, to_test
                );

                offset += 1
            }
            Err(_) => return *target_sum,
        }
    }
}
// https://adventofcode.com/2020/day/9
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_9/input.txt".into())?;
    let input: Vec<u64> = split.iter().map(|s| s.parse::<u64>().unwrap()).collect();

    Ok(AoCResult::Ofu64(
        slide_until_sum_of_any_2_in_last_x_is_not_current_value(&input, 25),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(AoCResult::Ofu64(7), find_solution().unwrap());
    }

    #[test]
    fn test_example() {
        let input: Vec<u64> = vec![
            "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
            "127", "219", "299", "277", "309", "576",
        ]
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

        let value = slide_until_sum_of_any_2_in_last_x_is_not_current_value(&input, 5);

        assert_eq!(127, value);
    }
}
