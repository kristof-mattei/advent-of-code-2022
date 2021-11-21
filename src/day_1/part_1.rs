use std::collections::HashMap;

use crate::errors::AoCError;
use crate::utils::read_file;

fn find_sum_of_2_is_2020(numbers: Vec<u32>) -> Option<(u32, u32)> {
    let mut missing_to_value: HashMap<u32, u32> = HashMap::new();

    for n in numbers {
        match missing_to_value.get(&n) {
            Some(x) => {
                return Some((*x, n));
            }
            None => {
                missing_to_value.insert(2020 - n, n);
            }
        }
    }

    None
}

// https://adventofcode.com/2020/day/1
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_1/input.txt".into())?;

    let numbers: Vec<u32> = split
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let (entry1, entry2) = find_sum_of_2_is_2020(numbers).ok_or(AoCError {
        message: "Didn't find a sum of x + y = 2020".to_string(),
    })?;

    Ok(entry1 * entry2)
}

#[test]
fn outcome() {
    assert_eq!(1_019_571, find_solution().unwrap());
}
