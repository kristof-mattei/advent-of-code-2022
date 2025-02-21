use std::cmp::Ordering;
use std::fmt::Write;

use advent_of_code_2022::shared::{PartSolution, Parts};

advent_of_code_2022::solution!(5185, 23_751);

#[derive(Clone, PartialEq, Eq)]
enum Pair {
    Vec(Vec<Pair>),
    Value(u32),
}

impl std::fmt::Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pair::Vec(v) => {
                f.write_str("[")?;

                let mut s = String::new();

                for (i, p) in v.iter().enumerate() {
                    write!(s, "{:?}", p)?;

                    if i + 1 != v.len() {
                        s.push(',');
                    }
                }

                f.write_str(s.as_str())?;

                f.write_str("]")?;
            },
            Pair::Value(v) => {
                f.write_str(&v.to_string())?;
            },
        }

        Ok(())
    }
}

fn find_right_orders(pairs: Vec<(Vec<Pair>, Vec<Pair>)>) -> usize {
    let mut sum = 0;
    for (index, (pair_1, pair_2)) in pairs.into_iter().enumerate() {
        if Ordering::Less == determine_largest(&pair_1, &pair_2) {
            sum += index + 1;
        }
    }

    sum
}

fn parse_line(chars: &[char]) -> (Pair, usize) {
    let mut buffer = String::new();

    let mut v = Vec::new();

    let mut current = 0;

    while current < chars.len() {
        match chars[current] {
            '[' => {
                let (pair, skipped) = parse_line(&chars[current + 1..]);

                v.push(pair);

                current += skipped + 1;
            },
            ']' => {
                if !buffer.is_empty() {
                    let value = Pair::Value(str::parse(&buffer).unwrap());

                    buffer.clear();

                    v.push(value);
                }

                return (Pair::Vec(v), current + 1);
            },
            ',' => {
                if !buffer.is_empty() {
                    let value = Pair::Value(str::parse(&buffer).unwrap());

                    buffer.clear();

                    v.push(value);
                }

                current += 1;
            },
            d @ '0'..='9' => {
                buffer.push(d);

                current += 1;
            },
            _ => {
                panic!()
            },
        }
    }

    (Pair::Vec(v), current + 1)
}

fn as_vec(pair: Pair) -> Vec<Pair> {
    if let Pair::Vec(v) = pair { v } else { panic!() }
}

fn parse_lines(input: &str) -> Vec<Vec<Pair>> {
    let mut result = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let (pair, _) = parse_line(&(line.chars().collect::<Vec<char>>()));

        result.push(as_vec(pair));
    }

    result
}

fn parse_lines_part_1(input: &str) -> Vec<(Vec<Pair>, Vec<Pair>)> {
    let mut parsed = parse_lines(input);

    let mut result = Vec::new();

    while !parsed.is_empty() {
        let p1 = parsed.remove(0);
        let p2 = parsed.remove(0);

        result.push((p1, p2));
    }

    result
}

fn determine_largest(left: &[Pair], right: &[Pair]) -> Ordering {
    match (left, right) {
        ([Pair::Value(v1)], [Pair::Value(v2)]) => v1.cmp(v2),
        (l, [r @ Pair::Value(_)]) => determine_largest(l, &[Pair::Vec(vec![r.clone()])]),
        ([l @ Pair::Value(_)], r) => determine_largest(&[Pair::Vec(vec![l.clone()])], r),
        ([Pair::Vec(l)], [Pair::Vec(r)]) => determine_largest(l, r),
        ([], [_, ..]) => std::cmp::Ordering::Less,
        ([_, ..], []) => std::cmp::Ordering::Greater,
        ([l, l_rest @ ..], [r, r_rest @ ..]) => {
            match determine_largest(&[l.clone()], &[r.clone()]) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => determine_largest(l_rest, r_rest),
            }
        },
        (&[], &[]) => Ordering::Equal,
    }
}

fn sort_and_find_position(mut list: Vec<Vec<Pair>>) -> usize {
    let p_2 = vec![Pair::Vec(vec![Pair::Vec(vec![Pair::Value(2)])])];
    let p_6 = vec![Pair::Vec(vec![Pair::Vec(vec![Pair::Value(6)])])];

    list.push(p_2.clone());
    list.push(p_6.clone());

    list.sort_by(|x, y| determine_largest(x, y));

    let position_1 = list.iter().position(|x| x == &p_2);
    let position_2 = list.iter().position(|x| x == &p_6);

    (position_1.unwrap() + 1) * (position_2.unwrap() + 1)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_lines_part_1(input);

        let sum = find_right_orders(parsed);

        sum.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let sum = sort_and_find_position(parsed);

        sum.into()
    }
}

#[cfg(test)]
mod tests {

    mod part_1 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(5185),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(13),
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(23_751),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(140),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
