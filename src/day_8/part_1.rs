use std::collections::HashSet;

use crate::{shared::AoCResult, utils::read_file};

#[derive(PartialEq, Debug)]
enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn map_operation(operation: &str, argument: i32) -> Operation {
    match operation {
        "acc" => Operation::Acc(argument),
        "jmp" => Operation::Jmp(argument),
        "nop" => Operation::Nop(argument),
        _ => panic!("Yea, no"),
    }
}

fn parse_lines(lines: &[String]) -> Vec<Operation> {
    let mut instructions = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();

        let argument = split[1].parse::<i32>().unwrap();
        let operation = map_operation(split[0], argument);

        instructions.push(operation);
    }

    instructions
}

fn execute_until_same_line_reached(operations: &[Operation]) -> i32 {
    let length = operations.len();

    let mut has_visited: HashSet<usize> = HashSet::new();

    let mut index: i32 = 0;

    let mut accumulator = 0;

    loop {
        index = index.wrapping_rem_euclid(length as i32);

        if !has_visited.insert(index as usize) {
            break;
        };

        match operations[index as usize] {
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
        }
    }

    accumulator
}

// https://adventofcode.com/2020/day/8
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_8/input.txt".into())?;

    let operations = parse_lines(&split);

    let accumulator = execute_until_same_line_reached(&operations);

    Ok(AoCResult::Ofi32(accumulator))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(AoCResult::Ofi32(1584), find_solution().unwrap());
    }

    #[test]
    fn test_nop_postive() {
        let input = vec!["nop +1".to_string()];

        let expected = Operation::Nop(1);
        let parsed = parse_lines(&input);

        assert_eq!(parsed[0], expected);
    }

    #[test]
    fn test_nop_negative() {
        let input = vec!["nop -20".to_string()];

        let expected = Operation::Nop(-20);
        let parsed = parse_lines(&input);

        assert_eq!(parsed[0], expected);
    }

    #[test]
    fn test_jmp_positive() {
        let input = vec!["jmp +3".to_string()];

        let expected = Operation::Jmp(3);
        let parsed = parse_lines(&input);

        assert_eq!(parsed[0], expected);
    }

    #[test]
    fn test_jmp_negative() {
        let input = vec!["jmp -4".to_string()];

        let expected = Operation::Jmp(-4);
        let parsed = parse_lines(&input);

        assert_eq!(parsed[0], expected);
    }

    #[test]
    fn test_acc_positive() {
        let input = vec!["acc +5".to_string()];

        let expected = Operation::Acc(5);
        let parsed = parse_lines(&input);

        assert_eq!(parsed[0], expected);
    }

    #[test]
    fn test_acc_negative() {
        let input = vec!["acc -6".to_string()];

        let expected = Operation::Acc(-6);
        let parsed = parse_lines(&input);

        assert_eq!(parsed[0], expected);
    }

    #[test]
    fn modulo_test() {
        let items: Vec<char> = ('a'..='j').into_iter().collect();

        let length = items.len() as i32;

        for i in -10..=length {
            let index = (i as i32).wrapping_rem_euclid(length);

            println!("{} ({}): {}", i, index, items[index as usize]);
        }
    }

    #[test]
    fn test_example() {
        let input: Vec<String> = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .iter()
        .map(|s| (*s).to_string())
        .collect();

        let operations = parse_lines(&input);

        let acc = execute_until_same_line_reached(&operations);

        assert_eq!(5, acc);
    }
}
