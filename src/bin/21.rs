use std::str::FromStr;

use advent_of_code_2022::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2022::solution!(276_156_919_469_632_usize, 3_441_198_826_073_usize);

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
    fn calculate(self, left: usize, right: usize) -> usize {
        match self {
            Operand::Plus => left + right,
            Operand::Minus => left - right,
            Operand::Times => left * right,
            Operand::Divide => left / right,
        }
    }

    /// Returns `left` such that `left <operand> right == result`.
    fn solve_left(self, result: usize, right: usize) -> usize {
        match self {
            Operand::Plus => result - right,
            Operand::Minus => result + right,
            Operand::Times => result / right,
            Operand::Divide => result * right,
        }
    }

    /// Returns `right` such that `left <operand> right == result`.
    fn solve_right(self, result: usize, left: usize) -> usize {
        match self {
            Operand::Plus => result - left,
            Operand::Minus => left - result,
            Operand::Times => result / left,
            Operand::Divide => left / result,
        }
    }
}

const ROOT: &str = "root";
const HUMAN: &str = "humn";

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

impl FromStr for Operand {
    type Err = &'static str;

    fn from_str(operand: &str) -> Result<Self, Self::Err> {
        match operand {
            "+" => Ok(Operand::Plus),
            "-" => Ok(Operand::Minus),
            "*" => Ok(Operand::Times),
            "/" => Ok(Operand::Divide),
            _ => Err("Invalid operand kind"),
        }
    }
}

fn parse_calculate(rest: &str) -> Result<(String, Operand, String), &'static str> {
    let (left, rest) = rest.split_once(' ').ok_or("Operand not found")?;
    let (operand, right) = rest.split_once(' ').ok_or("Right side not found")?;

    Ok((left.to_owned(), operand.parse()?, right.to_owned()))
}

fn parse_input(input: &str) -> Result<HashMap<String, Rest>, &'static str> {
    let monkeys = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

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

/// Returns `None` when the value of `to_resolve` depends on `humn`.
fn resolve_without_human(monkeys: &HashMap<String, Rest>, to_resolve: &str) -> Option<usize> {
    if to_resolve == HUMAN {
        return None;
    }

    match monkeys[to_resolve] {
        Rest::Yell(v) => Some(v),
        Rest::Calculate(ref left, operand, ref right) => {
            let left = resolve_without_human(monkeys, left)?;
            let right = resolve_without_human(monkeys, right)?;

            Some(operand.calculate(left, right))
        },
    }
}

/// Returns the number `humn` must yell for `to_solve` to evaluate to `target`.
fn solve_for_human(monkeys: &HashMap<String, Rest>, to_solve: &str, target: usize) -> usize {
    if to_solve == HUMAN {
        return target;
    }

    let Rest::Calculate(ref left, operand, ref right) = monkeys[to_solve] else {
        panic!("{} yells a number but depends on {}", to_solve, HUMAN);
    };

    match (
        resolve_without_human(monkeys, left),
        resolve_without_human(monkeys, right),
    ) {
        (None, Some(right)) => solve_for_human(monkeys, left, operand.solve_left(target, right)),
        (Some(left), None) => solve_for_human(monkeys, right, operand.solve_right(target, left)),
        _ => panic!("Exactly one side of {} must depend on {}", to_solve, HUMAN),
    }
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let monkeys = parse_input(input).unwrap();

        (resolve(&monkeys, ROOT)).into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let mut monkeys = parse_input(input).unwrap();

        let Some(&mut Rest::Calculate(_, ref mut operand, _)) = monkeys.get_mut(ROOT) else {
            panic!("{} must calculate", ROOT);
        };

        // `left == right` is the same as `left - right == 0`
        *operand = Operand::Minus;

        solve_for_human(&monkeys, ROOT, 0).into()
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
                3_441_198_826_073_usize,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(301, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
