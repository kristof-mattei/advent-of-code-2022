use crate::shared::{Day, PartSolution};

#[derive(PartialEq, Copy, Clone, Debug)]
enum Symbol {
    OpenParentese,
    CloseParentese,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    OpenChevron,
    CloseChevron,
}

impl Symbol {
    fn is_open(self) -> bool {
        match self {
            Symbol::OpenParentese
            | Symbol::OpenBracket
            | Symbol::OpenBrace
            | Symbol::OpenChevron => true,
            Symbol::CloseParentese
            | Symbol::CloseBracket
            | Symbol::CloseBrace
            | Symbol::CloseChevron => false,
        }
    }

    fn get_opposite(self) -> Symbol {
        match self {
            Symbol::OpenParentese => Symbol::CloseParentese,
            Symbol::CloseParentese => Symbol::OpenParentese,
            Symbol::OpenBracket => Symbol::CloseBracket,
            Symbol::CloseBracket => Symbol::OpenBracket,
            Symbol::OpenBrace => Symbol::CloseBrace,
            Symbol::CloseBrace => Symbol::OpenBrace,
            Symbol::OpenChevron => Symbol::CloseChevron,
            Symbol::CloseChevron => Symbol::OpenChevron,
        }
    }
}

fn calculate_winnings(first_illegal_characters: &[Symbol]) -> u32 {
    first_illegal_characters
        .iter()
        .map(|x| match x {
            Symbol::CloseParentese => 3,
            Symbol::CloseBracket => 57,
            Symbol::CloseBrace => 1197,
            Symbol::CloseChevron => 25137,
            _ => panic!("An open character can never be illegal"),
        })
        .sum()
}

fn find_first_illegal_character(symbols: &[Symbol]) -> Option<Symbol> {
    let mut opens: Vec<&Symbol> = Vec::new();

    for symbol in symbols {
        if symbol.is_open() {
            opens.push(symbol);
        } else {
            // it's a close

            match opens.pop() {
                None => return Some(*symbol),
                Some(s) => {
                    if s.get_opposite() != *symbol {
                        return Some(*symbol);
                    }
                }
            }
        }
    }

    None
}

fn calculate_completion(symbols: &[Symbol]) -> Vec<Symbol> {
    let mut opens: Vec<Symbol> = Vec::new();

    for symbol in symbols {
        if symbol.is_open() {
            opens.push(*symbol);
        } else {
            // it's a close

            opens.pop();
        }
    }

    opens.reverse();

    opens.iter().map(|x| x.get_opposite()).collect()
}

fn get_closing_score(symbol: Symbol) -> u64 {
    match symbol {
        Symbol::CloseParentese => 1,
        Symbol::CloseBracket => 2,
        Symbol::CloseBrace => 3,
        Symbol::CloseChevron => 4,
        _ => panic!("OMG WTF WHY DID YOU GIVE ME OPENING STUFF?"),
    }
}

fn calculate_score(completion: &[Symbol]) -> u64 {
    let mut score: u64 = 0;

    for s in completion {
        score *= 5;
        score += get_closing_score(*s);
    }

    score
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let into_symbols: Vec<Vec<Symbol>> = parse_lines(&lines);

        let mut first_illegal_characters: Vec<Symbol> = Vec::new();

        for line_into_symbols in into_symbols {
            match find_first_illegal_character(&line_into_symbols) {
                Some(symbol) => first_illegal_characters.push(symbol),
                None => println!("All good"),
            };
        }

        PartSolution::U32(calculate_winnings(&first_illegal_characters))
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let into_symbols: Vec<Vec<Symbol>> = parse_lines(&lines);

        let mut scores: Vec<u64> = Vec::new();

        for valid_line in into_symbols
            .iter()
            .filter(|line_into_symbols| find_first_illegal_character(line_into_symbols).is_none())
        {
            let completion: Vec<Symbol> = calculate_completion(valid_line);

            scores.push(calculate_score(&completion));
        }

        scores.sort_unstable();

        PartSolution::U64(scores[scores.len() / 2])
    }
}

fn parse_lines(lines: &[String]) -> Vec<Vec<Symbol>> {
    let mut parsed = Vec::new();
    for line in lines {
        parsed.push(
            line.chars()
                .map(|x| match x {
                    '(' => Symbol::OpenParentese,
                    ')' => Symbol::CloseParentese,
                    '[' => Symbol::OpenBracket,
                    ']' => Symbol::CloseBracket,
                    '{' => Symbol::OpenBrace,
                    '}' => Symbol::CloseBrace,
                    '<' => Symbol::OpenChevron,
                    '>' => Symbol::CloseChevron,
                    _ => panic!("OMG WTF BBQ"),
                })
                .collect(),
        );
    }
    parsed
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<String> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {
        use crate::{
            day_10::{
                calculate_winnings, find_first_illegal_character, parse_lines, Solution, Symbol,
            },
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(392_139));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let into_symbols: Vec<Vec<Symbol>> = parse_lines(&lines);

            let mut first_illegal_characters: Vec<Symbol> = Vec::new();

            for line_into_symbols in into_symbols {
                match find_first_illegal_character(&line_into_symbols) {
                    None => println!("All good"),
                    Some(symbol) => first_illegal_characters.push(symbol),
                };
            }

            assert_eq!(26397, calculate_winnings(&first_illegal_characters));
        }
    }

    mod part_2 {
        use crate::{
            day_10::{
                calculate_completion, calculate_score, find_first_illegal_character, parse_lines,
                Solution, Symbol,
            },
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U64(4_001_832_844));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let into_symbols: Vec<Vec<Symbol>> = parse_lines(&lines);

            let mut scores: Vec<u64> = Vec::new();

            for valid_line in into_symbols.iter().filter(|line_into_symbols| {
                find_first_illegal_character(line_into_symbols).is_none()
            }) {
                let completion: Vec<Symbol> = calculate_completion(valid_line);

                scores.push(calculate_score(&completion));
            }

            scores.sort_unstable();

            assert_eq!(288_957, scores[scores.len() / 2]);
        }
    }
}
