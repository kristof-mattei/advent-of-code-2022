use std::collections::HashMap;

use crate::errors::AoCError;
use crate::utils::read_file;

fn find_sum_of_2_is_2020(fixed: i32, rest: Vec<i32>) -> Option<(i32, i32)> {
    let mut missing_to_value: HashMap<i32, i32> = HashMap::new();

    for n in rest {
        match missing_to_value.get(&n) {
            Some(x) => {
                return Some((*x, n));
            }
            None => {
                missing_to_value.insert(2020 - fixed - n, n);
            }
        }
    }

    None
}

fn find_sum_of_3_is_2020(numbers: &[i32]) -> Option<(i32, i32, i32)> {
    for n in numbers {
        let mut vec_without_n = numbers.to_owned();
        vec_without_n.retain(|r| *r != *n);

        match find_sum_of_2_is_2020(*n, vec_without_n) {
            None => (),
            Some((part2, part3)) => {
                return Some((*n, part2, part3));
            }
        }
    }

    None
}

// https://adventofcode.com/2020/day/1
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_1/input.txt".into())?;

    let numbers: Vec<i32> = split
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let (entry1, entry2, entry3) = find_sum_of_3_is_2020(&numbers).ok_or(AoCError {
        message: "Didn't find a sum of x + y + z = 2020".to_string(),
    })?;

    Ok((entry1 * entry2 * entry3).try_into().unwrap())
}

#[test]
fn outcome() {
    assert_eq!(100_655_544, find_solution().unwrap());
}
