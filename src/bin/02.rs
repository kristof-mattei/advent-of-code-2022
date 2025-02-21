use advent_of_code_2022::shared::{PartSolution, Parts};

advent_of_code_2022::solution!(12855, 13726);

fn parse_lines<T>(input: &str) -> Vec<(Rps, T)>
where
    T: From<char>,
{
    let mut result = Vec::new();
    for line in input.lines().map(str::trim) {
        let mut chars = line.chars();

        let first: Rps = chars.next().unwrap().into();
        // space
        let _ = chars.next();
        let second: T = chars.next().unwrap().into();

        result.push((first, second));
    }

    result
}

fn calculate_score_part_1(rounds: Vec<(Rps, Rps)>) -> u32 {
    rounds.into_iter().map(score).sum()
}

fn calculate_score_part_2(rounds: Vec<(Rps, Expected)>) -> u32 {
    rounds.into_iter().map(answer_and_score).sum()
}

enum Expected {
    Lose,
    Draw,
    Win,
}

impl From<char> for Expected {
    fn from(c: char) -> Self {
        match c {
            'X' => Expected::Lose,
            'Y' => Expected::Draw,
            'Z' => Expected::Win,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Sissors,
}

impl From<Rps> for u32 {
    fn from(rps: Rps) -> Self {
        match rps {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Sissors => 3,
        }
    }
}

fn score((left, right): (Rps, Rps)) -> u32 {
    let score = match left {
        Rps::Rock => match right {
            Rps::Rock => 3,
            Rps::Paper => 6,
            Rps::Sissors => 0,
        },
        Rps::Paper => match right {
            Rps::Rock => 0,
            Rps::Paper => 3,
            Rps::Sissors => 6,
        },
        Rps::Sissors => match right {
            Rps::Rock => 6,
            Rps::Paper => 0,
            Rps::Sissors => 3,
        },
    };

    score + <Rps as Into<u32>>::into(right)
}

fn answer_and_score((left, right): (Rps, Expected)) -> u32 {
    let right_should_play = match right {
        Expected::Lose => match left {
            Rps::Rock => Rps::Sissors,
            Rps::Paper => Rps::Rock,
            Rps::Sissors => Rps::Paper,
        },
        Expected::Draw => left,
        Expected::Win => match left {
            Rps::Rock => Rps::Paper,
            Rps::Paper => Rps::Sissors,
            Rps::Sissors => Rps::Rock,
        },
    };

    score((left, right_should_play))
}

impl From<char> for Rps {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Rps::Rock,
            'B' | 'Y' => Rps::Paper,
            'C' | 'Z' => Rps::Sissors,
            _ => unreachable!(),
        }
    }
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let rounds = parse_lines(input);
        let score = calculate_score_part_1(rounds);

        score.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let rounds = parse_lines(input);
        let score = calculate_score_part_2(rounds);

        score.into()
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
                PartSolution::U32(12855),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::U32(15),
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
                PartSolution::U32(13726),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::U32(12),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
