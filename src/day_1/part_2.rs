use std::collections::HashMap;

use crate::shared::{AoCError, AoCResult};
use crate::utils::read_file;

fn find_sum_of_2_is_2020(fixed: u32, rest: Vec<u32>) -> Option<(u32, u32)> {
    let mut missing_to_value: HashMap<i32, i32> = HashMap::new();

    for n in rest {
        match missing_to_value.get(&(n as i32)) {
            Some(x) => {
                return Some(((*x) as u32, n));
            }
            None => {
                missing_to_value.insert(2020_i32 - fixed as i32 - n as i32, n as i32);
            }
        }
    }

    None
}

fn find_sum_of_3_is_2020(numbers: &[u32]) -> Option<(u32, u32, u32)> {
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
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_1/input.txt".into())?;

    let numbers: Vec<u32> = split
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let (entry1, entry2, entry3) = find_sum_of_3_is_2020(&numbers).ok_or(AoCError {
        message: "Didn't find a sum of x + y + z = 2020".to_string(),
    })?;

    Ok(AoCResult::Ofu32(
        (entry1 * entry2 * entry3).try_into().unwrap(),
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn outcome() {
        assert_eq!(AoCResult::Ofu32(100_655_544), find_solution().unwrap());
    }
}
