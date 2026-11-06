use std::str::FromStr;

use advent_of_code_2022::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2022::solution!(276_156_919_469_632_usize);

#[derive(Debug)]
struct Monkey {
    name: String,
    rest: Rest,
}

#[derive(Debug)]
enum Rest {
    Yell(usize),
    Calculate(String, Operand, String),
}

#[derive(Copy, Clone, Debug)]
enum Operand {
    Plus,
    Minus,
    Times,
    Divide,
}
impl Operand {
    fn calculate(&self, left: usize, right: usize) -> usize {
        match *self {
            Operand::Plus => left + right,
            Operand::Minus => left - right,
            Operand::Times => left * right,
            Operand::Divide => left / right,
        }
    }
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (name, rest) = line.split_once(": ").ok_or("Missing @ separator")?;

        Ok(if let Ok(yell) = usize::from_str(rest) {
            Monkey {
                name: name.to_owned(),
                rest: Rest::Yell(yell),
            }
        } else {
            let (left, operand, right) = parse_calculate(rest)?;
            Monkey {
                name: name.to_owned(),
                rest: Rest::Calculate(left, operand, right),
            }
        })
    }
}

fn parse_calculate(rest: &str) -> Result<(String, Operand, String), &'static str> {
    let operand_index = rest.trim().find(' ').ok_or("Operand not found")?;

    let operand = match &rest[operand_index + 1..operand_index + 2] {
        "+" => Operand::Plus,
        "-" => Operand::Minus,
        "*" => Operand::Times,
        "/" => Operand::Divide,
        _ => return Err("Invalid operand kind"),
    };

    let left = &rest[0..operand_index];
    let right = &rest[operand_index + 2..];

    let left = left.trim().to_owned();
    let right = right.trim().to_owned();

    Ok((left, operand, right))
}

fn parse_input(input: &str) -> Result<HashMap<String, Rest>, &'static str> {
    let monkeys = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    println!("{:?}", monkeys);

    Ok(monkeys
        .into_iter()
        .map(|Monkey { name, rest }| (name, rest))
        .collect())
}

fn resolve(monkeys: &HashMap<String, Rest>, to_resolve: &str) -> usize {
    let rest = &monkeys[to_resolve];

    match *rest {
        Rest::Yell(ref v) => *v,
        Rest::Calculate(ref left, operand, ref right) => {
            let left = resolve(monkeys, left);
            let right = resolve(monkeys, right);

            operand.calculate(left, right)
        },
    }
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let monkeys = parse_input(input).unwrap();

        (resolve(&monkeys, "root")).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use advent_of_code_2022::shared::Parts as _;
        use advent_of_code_2022::shared::solution::read_file;
        use pretty_assertions::assert_eq;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                276_156_919_469_632_usize,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(152, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::Parts as _;
        use advent_of_code_2022::shared::solution::read_file;
        use pretty_assertions::assert_eq;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                13_084_440_324_666_i64,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                1_623_178_306,
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
