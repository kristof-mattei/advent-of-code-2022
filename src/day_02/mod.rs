use crate::shared::{Day, PartSolution};

fn parse_lines(lines: &[&str]) -> Vec<(RPS, RPS)> {
    let mut result = Vec::new();
    for line in lines {
        let mut chars = line.chars();

        let first: RPS = chars.next().unwrap().into();
        // space
        let _ = chars.next();
        let second: RPS = chars.next().unwrap().into();

        result.push((first, second));
    }

    result
}

fn calculate_score(rounds: Vec<(RPS, RPS)>) -> u32 {
    rounds.into_iter().map(score).sum()
}

enum RPS {
    Rock,
    Paper,
    Sissors,
}

impl From<RPS> for u32 {
    fn from(rps: RPS) -> Self {
        match rps {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Sissors => 3,
        }
    }
}

fn score((left, right): (RPS, RPS)) -> u32 {
    let score = match left {
        RPS::Rock => match right {
            RPS::Rock => 3,
            RPS::Paper => 6,
            RPS::Sissors => 0,
        },
        RPS::Paper => match right {
            RPS::Rock => 0,
            RPS::Paper => 3,
            RPS::Sissors => 6,
        },
        RPS::Sissors => match right {
            RPS::Rock => 6,
            RPS::Paper => 0,
            RPS::Sissors => 3,
        },
    };

    score + <RPS as Into<u32>>::into(right)
}

impl From<char> for RPS {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => RPS::Rock,
            'B' | 'Y' => RPS::Paper,
            'C' | 'Z' => RPS::Sissors,
            _ => unreachable!(),
        }
    }
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let rounds = parse_lines(&lines);
        let score = calculate_score(rounds);

        score.into()
    }

    fn part_2(&self) -> PartSolution {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {
        use crate::{
            day_02::Solution,
            day_02::{calculate_score, parse_lines, test::get_example},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::U32(67658), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let rounds = parse_lines(&lines);

            let score = calculate_score(rounds);

            assert_eq!(score, 15);
        }
    }

    mod part_2 {}
}
