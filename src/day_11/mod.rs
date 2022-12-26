use std::collections::VecDeque;

use regex::Regex;

use crate::shared::{Day, PartSolution};

enum Operation {
    Plus(Operand),
    Times(Operand),
}

enum Operand {
    Old,
    Scalar(i64),
}

struct Monkey {
    _id: usize,
    activity: usize,
    items: VecDeque<i64>,
    operation: Operation,
    test: i64,
    target_true: usize,
    target_false: usize,
}

fn parse_lines(lines: &str) -> Vec<Monkey> {
    let regex = Regex::new(
        r"Monkey (?P<monkey>\d):\n  Starting items: (?P<items>[\d ,]+)\n  Operation: new = old (?P<operation>\*|\+) (?P<operand>[\d]+|old)\n  Test: divisible by (?P<divisible>\d+)\n    If true: throw to monkey (?P<target_true>\d)\n    If false: throw to monkey (?P<target_false>\d)\n",
    ).unwrap();

    let mut monkeys = Vec::new();

    for cap in regex.captures_iter(lines) {
        let monkey_id: usize = cap["monkey"].parse().unwrap();

        let items = cap["items"]
            .split(',')
            .map(|item| item.trim().parse().unwrap())
            .collect::<VecDeque<_>>();

        let operand = if &cap["operand"] == "old" {
            Operand::Old
        } else {
            Operand::Scalar(cap["operand"].parse().unwrap())
        };

        let operation = match &cap["operation"] {
            "+" => Operation::Plus(operand),
            "*" => Operation::Times(operand),
            _ => {
                panic!()
            },
        };

        let divisible: i64 = cap["divisible"].parse().unwrap();
        let target_true: usize = cap["target_true"].parse().unwrap();
        let target_false: usize = cap["target_false"].parse().unwrap();

        monkeys.push(Monkey {
            _id: monkey_id,
            activity: 0,
            items,
            operation,
            test: divisible,
            target_true,
            target_false,
        });
    }

    monkeys
}

fn execute_operation(input: i64, operation: &Operation) -> i64 {
    match operation {
        Operation::Plus(Operand::Old) => panic!(),
        Operation::Times(Operand::Old) => input * input,
        Operation::Plus(Operand::Scalar(s)) => input + s,
        Operation::Times(Operand::Scalar(s)) => input * s,
    }
}

enum WorryReducer {
    Divide,
    Mod(i64),
}

fn monkey_business(monkeys: &mut [Monkey], worry_reducer: &WorryReducer) {
    for i in 0..monkeys.len() {
        let mut items = std::mem::take(&mut monkeys[i].items);

        while let Some(item) = items.pop_front() {
            // this monkey inspected something
            (monkeys[i].activity) += 1;

            let result = execute_operation(item, &monkeys[i].operation);

            let result = match worry_reducer {
                WorryReducer::Divide => result / 3,
                WorryReducer::Mod(m) => result % m,
            };

            let target_monkey = if 0 == (result % monkeys[i].test) {
                monkeys[i].target_true
            } else {
                monkeys[i].target_false
            };

            monkeys[target_monkey].items.push_back(result);
        }
    }
}

fn do_monkey_business(
    mut monkeys: Vec<Monkey>,
    times: usize,
    worry_reducer: &WorryReducer,
) -> usize {
    (1..=times).for_each(|_| {
        monkey_business(&mut monkeys, worry_reducer);
    });

    monkeys.sort_unstable_by(|a, b| b.activity.cmp(&a.activity));

    monkeys[0].activity * monkeys[1].activity
}

fn do_monkey_business_part_1(monkeys: Vec<Monkey>) -> usize {
    do_monkey_business(monkeys, 20, &WorryReducer::Divide)
}

fn do_monkey_business_part_2(monkeys: Vec<Monkey>) -> usize {
    let m = monkeys.iter().fold(1, |acc, curr| (acc * curr.test));

    do_monkey_business(monkeys, 10_000, &WorryReducer::Mod(m))
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines = include_str!("input.txt");

        let parsed = parse_lines(lines);

        let result = do_monkey_business_part_1(parsed);

        result.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines = include_str!("input.txt");

        let parsed = parse_lines(lines);

        let result = do_monkey_business_part_2(parsed);

        result.into()
    }
}

#[cfg(test)]
mod test {
    fn get_example<'a>() -> &'a str {
        include_str!("example.txt")
    }

    mod part_1 {
        use super::super::{do_monkey_business_part_1, parse_lines, Solution};
        use super::get_example;
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(118_674), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(lines);

            let result = do_monkey_business_part_1(parsed);

            assert_eq!(result, 10605);
        }
    }

    mod part_2 {
        use super::super::{do_monkey_business_part_2, parse_lines, Solution};
        use super::get_example;
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(32_333_418_600), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(lines);

            let result = do_monkey_business_part_2(parsed);

            assert_eq!(2_713_310_158, result);
        }
    }
}
