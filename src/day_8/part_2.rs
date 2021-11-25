use std::collections::HashSet;

use crate::{
    shared::{AoCError, AoCResult},
    utils::read_file,
};

use super::part_1::{parse_lines, Operation};

fn execute_until_same_line_reached(operations: &[Operation]) -> Result<i32, AoCError> {
    let length = operations.len();

    let mut has_visited: HashSet<usize> = HashSet::new();

    let mut index: i32 = 0;

    let mut accumulator = 0;

    loop {
        index = index.wrapping_rem_euclid(length as i32);

        if !has_visited.insert(index as usize) {
            return Err(AoCError {
                message: "Program did not terminate".to_string(),
            });
        };

        match operations.get(index as usize) {
            Some(operation) => match operation {
                Operation::Acc(acc) => {
                    accumulator += acc;
                    index += 1;
                }
                Operation::Jmp(jmp) => {
                    index += jmp;
                }
                Operation::Nop(_) => {
                    index += 1;
                }
            },
            None => return Ok(accumulator),
        }
    }
}

// https://adventofcode.com/2020/day/8
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_8/input.txt".into())?;

    let operations = parse_lines(&split);

    let to_swap: Vec<usize> = operations
        .iter()
        .enumerate()
        .filter_map(|(index, f)| match f {
            Operation::Acc(_) | Operation::Jmp(_) => Some(index),
            _ => None,
        })
        .collect();

    for to_swap_index in to_swap {
        // take the part up to the index
        let part1: Vec<&Operation> = (&operations).iter().take(to_swap_index).collect();

        let part2: Vec<&Operation> = operations
            .iter()
            .skip(to_swap_index)
            .take(operations.len() - to_swap_index - 1)
            .collect();
    }

    let accumulator = execute_until_same_line_reached(&operations);

    Ok(AoCResult::Ofi32(accumulator.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(AoCResult::Ofi32(1584), find_solution().unwrap());
    }
}
