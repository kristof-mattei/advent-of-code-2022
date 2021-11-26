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
        if index == length as i32 {
            return Ok(accumulator);
        }
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
            Operation::Nop(_) | Operation::Jmp(_) => Some(index),
            _ => None,
        })
        .collect();

    for to_swap_index in to_swap {
        let beginning = build_new_vector(&operations, to_swap_index);

        match execute_until_same_line_reached(&beginning) {
            Ok(acc) => return Ok(AoCResult::Ofi32(acc)),
            _ => continue,
        }
    }

    Err(Box::new(AoCError {
        message: "No non-terminating combination found".to_string(),
    }))
}

fn build_new_vector(operations: &[Operation], to_swap_index: usize) -> Vec<Operation> {
    // take the part up to the index
    let mut copy: Vec<_> = operations
        .iter()
        .take(to_swap_index)
        .map(|o| o.clone())
        .collect();

    let mut rest: Vec<_> = operations
        .iter()
        .skip(to_swap_index + 1)
        .take(operations.len() - to_swap_index - 1)
        .map(|o| o.clone())
        .collect();

    let flipped_operation = match operations[to_swap_index] {
        Operation::Nop(x) => Operation::Jmp(x),
        Operation::Jmp(x) => Operation::Nop(x),
        Operation::Acc(_) => panic!("index shouldn't refer to acc"),
    };

    copy.push(flipped_operation);

    copy.append(&mut rest);

    copy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(AoCResult::Ofi32(920), find_solution().unwrap());
    }

    #[test]
    fn sample_data() {
        let input: Vec<String> = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .iter()
        .map(|s| (*s).to_string())
        .collect();

        let operations = parse_lines(&input);

        let new_vector = build_new_vector(&operations, 7);

        println!("{:#?}", new_vector);

        let acc = execute_until_same_line_reached(&new_vector);

        assert_eq!(8, acc.unwrap());
    }

    #[test]
    fn pieces() {
        let vec1: Vec<i32> = (0..=10).into_iter().collect();

        println!("{:?}", vec1);

        const SPLIT_AT: usize = 5;

        let vec2: Vec<&i32> = vec1.iter().take(SPLIT_AT).collect();

        println!("{:?}", vec2);

        let vec3: Vec<&i32> = vec1
            .iter()
            .skip(SPLIT_AT + 1)
            .take(vec1.len() - SPLIT_AT - 1)
            .collect();

        println!("{:?}", vec3);
    }
}
