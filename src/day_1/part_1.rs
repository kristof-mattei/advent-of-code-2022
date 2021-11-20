use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[path = "../error/error.rs"]
mod error;
use self::error::AoCError;

fn read_file() -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open("./src/day_1/input.txt")?; // nasty path resolve
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let split: Vec<String> = contents
        .split_ascii_whitespace()
        .map(|f| f.to_owned())
        .collect();

    Ok(split)
}

fn find_sum_is_2020(numbers: Vec<u32>) -> Option<(u32, u32)> {
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
pub fn day_1_part_1() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file()?;

    let numbers: Vec<u32> = split
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let (entry1, entry2) = find_sum_is_2020(numbers).ok_or(AoCError {
        message: "Didn't find a sum of x + y = 2020".to_string(),
    })?;

    Ok(entry1 * entry2)
}

#[test]
fn outcome() {
    assert_eq!(1019571, day_1_part_1().unwrap());
}
