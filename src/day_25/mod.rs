use core::fmt;
use std::collections::HashSet;

use crate::shared::{Day, PartSolution};

#[derive(PartialEq, Eq, Debug)]
enum Cucumber {
    East,
    South,
}

impl fmt::Display for Cucumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cucumber::East => write!(f, ">"),
            Cucumber::South => write!(f, "v"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Board {
    cucumbers: Vec<Vec<Option<Cucumber>>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cucumbers {
            for maybe_cucumber in row {
                match maybe_cucumber {
                    Some(c) => write!(f, "{}", c),
                    None => write!(f, "."),
                }?;
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    fn x_dim(&self) -> usize {
        self.cucumbers.len()
    }

    fn y_dim(&self) -> usize {
        self.cucumbers.get(0).map(Vec::len).unwrap_or_default()
    }

    fn try_move(
        &mut self,
        x: usize,
        y: usize,
        invalid_targets: &HashSet<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        let x_dim = self.x_dim();
        let y_dim = self.y_dim();

        let cucumber = self.cucumbers.get(x).and_then(|r| match r.get(y) {
            Some(c) => c.as_ref(),
            None => None,
        });

        match cucumber {
            Some(Cucumber::East) => {
                let new_y = (y + 1) % y_dim;

                if self.cucumbers[x][new_y].is_none() && !invalid_targets.contains(&(x, new_y)) {
                    self.cucumbers[x][y] = None;
                    self.cucumbers[x][new_y] = Some(Cucumber::East);

                    Some((x, new_y))
                } else {
                    None
                }
            },
            Some(Cucumber::South) => {
                let new_x = (x + 1) % x_dim;

                if self.cucumbers[new_x][y].is_none() && !invalid_targets.contains(&(new_x, y)) {
                    self.cucumbers[x][y] = None;
                    self.cucumbers[new_x][y] = Some(Cucumber::South);

                    Some((new_x, y))
                } else {
                    None
                }
            },
            None => None,
        }
    }
}

fn parse_lines(input: &[&str]) -> Board {
    let mut cucumbers = Vec::new();

    for line in input {
        let mut board_line = Vec::new();

        for c in line.chars() {
            board_line.push(match c {
                'v' => Some(Cucumber::South),
                '>' => Some(Cucumber::East),
                '.' => None,
                _ => unreachable!(),
            });
        }

        cucumbers.push(board_line);
    }

    Board { cucumbers }
}

fn move_cucumbers_in_direction(board: &mut Board, direction: Cucumber) -> bool {
    let mut moved = HashSet::new();
    let mut invalid = HashSet::new();

    let expected = Some(direction);

    for x in 0..board.x_dim() {
        for y in 0..board.y_dim() {
            if !moved.contains(&(x, y)) && board.cucumbers[x][y] == expected {
                if let Some(r) = board.try_move(x, y, &invalid) {
                    invalid.insert((x, y));
                    moved.insert(r);
                }
            }
        }
    }

    !moved.is_empty()
}

fn move_cucumbers(board: &mut Board) -> u32 {
    println!("Initial state: \n{}", board);
    let mut step = 0;
    loop {
        step += 1;
        let moved_east = move_cucumbers_in_direction(board, Cucumber::East);
        let moved_south = move_cucumbers_in_direction(board, Cucumber::South);

        println!("After step {}:\n{}", step, board);
        if !moved_east && !moved_south {
            break;
        }
    }

    step
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let mut board = parse_lines(&lines);

        let steps_taken = move_cucumbers(&mut board);

        PartSolution::U32(steps_taken)
    }

    fn part_2(&self) -> PartSolution {
        let _lines: Vec<&str> = include_str!("input.txt").lines().collect();

        PartSolution::None
    }
}

#[cfg(test)]
mod test {
    fn get_smaller_example() -> Vec<&'static str> {
        include_str!("example_smaller.txt").lines().collect()
    }

    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt").lines().collect()
    }

    mod part_1 {
        use crate::{
            day_25::{
                move_cucumbers, move_cucumbers_in_direction, parse_lines,
                test::get_smaller_example, Board, Cucumber, Solution,
            },
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(329));
        }
        #[test]
        fn parse_test() {
            let input = "...>>>>>...";

            let parsed = parse_lines(&[input]);

            let expected = Board {
                cucumbers: vec![vec![
                    None,
                    None,
                    None,
                    Some(Cucumber::East),
                    Some(Cucumber::East),
                    Some(Cucumber::East),
                    Some(Cucumber::East),
                    Some(Cucumber::East),
                    None,
                    None,
                    None,
                ]],
            };

            assert_eq!(expected, parsed);
        }

        #[test]
        fn smaller_example() {
            let example_lines = get_smaller_example();

            let mut board = parse_lines(&example_lines);

            move_cucumbers_in_direction(&mut board, Cucumber::East);
            move_cucumbers_in_direction(&mut board, Cucumber::South);

            move_cucumbers_in_direction(&mut board, Cucumber::East);
            move_cucumbers_in_direction(&mut board, Cucumber::South);

            move_cucumbers_in_direction(&mut board, Cucumber::East);
            move_cucumbers_in_direction(&mut board, Cucumber::South);

            move_cucumbers_in_direction(&mut board, Cucumber::East);
            move_cucumbers_in_direction(&mut board, Cucumber::South);

            let expected = [
                ">......", "..v....", "..>.v..", ".>.v...", "...>...", ".......", "v......",
            ];

            let parsed_expected = parse_lines(&expected);
            println!("{}", parsed_expected);
            println!("{}", board);
            assert_eq!(parse_lines(&expected), board);
        }

        #[test]
        fn example() {
            let example_lines = get_example();

            let mut board = parse_lines(&example_lines);

            move_cucumbers(&mut board);

            let expected = [
                "..>>v>vv..",
                "..v.>>vv..",
                "..>>v>>vv.",
                "..>>>>>vv.",
                "v......>vv",
                "v>v....>>v",
                "vvv.....>>",
                ">vv......>",
                ".>v.vv.v..",
            ];

            assert_eq!(parse_lines(&expected), board);
        }

        #[test]
        fn example_after_steps() {
            let example_lines = get_example();

            let mut board = parse_lines(&example_lines);

            let steps_taken = move_cucumbers(&mut board);

            assert_eq!(58, steps_taken);
        }
    }
}
