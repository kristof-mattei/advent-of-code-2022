use core::fmt;
use std::collections::{HashMap, HashSet};

use crate::shared::{Day, PartSolution};

enum Register {
    W,
    X,
    Y,
    Z,
}

enum RegisterOrValue {
    Register,
    Value(i32),
}
enum Instruction {
    Input(Register),
    Add(Register, RegisterOrValue),
    Mul(Register, RegisterOrValue),
    Div(Register, RegisterOrValue),
    Mod(Register, RegisterOrValue),
    Eql(Register, RegisterOrValue),
}

struct Alu {
    registers: HashMap<Register, i32>,
    input: Vec<i32>,
}

impl Alu {
    fn new(input: Vec<i32>) -> Self {
        Self {
            registers: HashMap::new(),
            input: input.iter().copied().rev().collect(),
        }
    }
}

fn parse_lines(input: &[&str]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input {
        let split = line.split(' ').collect::<Vec<_>>();

        let part2 = split.get(1).map(|p2| p2.chars().next().unwrap());

        let param1 = match part2 {
            'w' => Register::W,
            'x' => Register::X,
            'y' => Register::Y,
            'z' => Register::Z,
            _ => unreachable!(),
        };

        let part1 = split.get(0).unwrap();

        match *part1 {
            "foo" => {},
            _ => unreachable!(),
        }
    }

    instructions
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let instructions = parse_lines(&lines);

        PartSolution::None
    }

    fn part_2(&self) -> PartSolution {
        let _lines: Vec<&str> = include_str!("input.txt").lines().collect();

        PartSolution::None
    }
}

#[cfg(test)]
mod test {

    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt").lines().collect()
    }

    mod part_1 {
        use crate::{
            day_25::{parse_lines, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(329));
        }

        #[test]
        fn example() {
            // let example_lines = get_example();

            // let mut board = parse_lines(&example_lines);

            // move_cucumbers(&mut board);

            // let expected = [
            //     "..>>v>vv..",
            //     "..v.>>vv..",
            //     "..>>v>>vv.",
            //     "..>>>>>vv.",
            //     "v......>vv",
            //     "v>v....>>v",
            //     "vvv.....>>",
            //     ">vv......>",
            //     ".>v.vv.v..",
            // ];

            // assert_eq!(parse_lines(&expected), board);
        }
    }
}
