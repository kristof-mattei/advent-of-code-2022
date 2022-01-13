use std::cell::Cell;

use crate::shared::{Day, PartSolution};

enum Instruction {
    X(usize),
    Y(usize),
}

fn parse_lines(lines: &[&str]) -> (Vec<Vec<Cell<bool>>>, Vec<Instruction>) {
    let mut field = Vec::new();

    let mut coordinates = Vec::new();

    for line in lines
        .iter()
        .filter(|l| !l.starts_with("fold") && !l.is_empty())
    {
        let split: Vec<_> = line.split(',').collect();

        let x = split.get(0).unwrap().parse::<usize>().unwrap();
        let y = split.get(1).unwrap().parse::<usize>().unwrap();

        coordinates.push((x, y));
    }

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines.iter().filter(|l| l.starts_with("fold")) {
        let line = line.replace("fold along ", "");
        let split: Vec<_> = line.split('=').collect();

        let plane = split.get(0).unwrap().parse::<char>().unwrap();
        let x_y = split.get(1).unwrap().parse::<usize>().unwrap();

        let instruction = match plane {
            'x' => Instruction::X(x_y),
            'y' => Instruction::Y(x_y),
            _ => panic!("Unknown fold instruction"),
        };

        instructions.push(instruction);
    }

    // do not build the field boundaries with the coordinates, this fails
    // if there's nothing on the last row / column.
    // use the max of the fold line * 2 + 1

    let columns = instructions
        .iter()
        .filter_map(|i| {
            if let Instruction::X(c) = i {
                Some(c)
            } else {
                None
            }
        })
        .max()
        .map(|i| i * 2)
        .map(|i| i + 1)
        .unwrap();

    let rows = instructions
        .iter()
        .filter_map(|i| {
            if let Instruction::Y(r) = i {
                Some(r)
            } else {
                None
            }
        })
        .max()
        .map(|i| i * 2)
        .map(|i| i + 1)
        .unwrap();

    for _ in 0..rows {
        let mut v = Vec::new();
        v.resize(columns, Cell::new(false));
        field.push(v);
    }

    for (x, y) in coordinates {
        field[y][x].set(true);
    }

    (field, instructions)
}

fn pretty_print(field: &[Vec<Cell<bool>>]) -> Vec<String> {
    field
        .iter()
        .map(|f| {
            f.iter()
                .map(|f| if f.get() { "#" } else { "." })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect()
}

fn fold(field: &mut Vec<Vec<Cell<bool>>>, instruction: &Instruction) {
    match instruction {
        Instruction::X(fold_on_column) => {
            for row in field {
                for c in 0..*fold_on_column {
                    let mirrored_column = (fold_on_column * 2) - c;

                    let cell_value = row[c].get();

                    let mirrored_value = row[mirrored_column].get();

                    row[c].set(cell_value | mirrored_value);
                }

                row.truncate(*fold_on_column);
            }
        },
        Instruction::Y(fold_on_row) => {
            let columns = field.get(0).map_or(0, Vec::len);

            for r in 0..*fold_on_row {
                let mirrored_row = (fold_on_row * 2) - r;

                for c in 0..columns {
                    let cell_value = field[r][c].get();

                    let mirrored_value = field[mirrored_row][c].get();

                    field[r][c].set(cell_value | mirrored_value);
                }
            }

            field.truncate(*fold_on_row);
        },
    }
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let (mut field, fold_instructions) = parse_lines(&lines);

        fold(&mut field, fold_instructions.get(0).unwrap());

        PartSolution::USize(
            field
                .iter()
                .map(|r| r.iter().filter(|x| x.get()).count())
                .sum::<usize>(),
        )
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let (mut field, fold_instructions) = parse_lines(&lines);

        for fold_instruction in fold_instructions {
            fold(&mut field, &fold_instruction);
        }

        PartSolution::Vec(pretty_print(&field))
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt").lines().collect()
    }

    mod part_1 {

        use crate::{
            day_13::{fold, parse_lines, pretty_print, Solution},
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::USize(638));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let (mut field, fold_instructions) = parse_lines(&lines);

            println!("Field:");
            for field_line in pretty_print(&field) {
                println!("{}", field_line);
            }

            println!("End field");

            fold(&mut field, fold_instructions.get(0).unwrap());

            #[rustfmt::skip]
            let lines_after_fold_1: Vec<String> = vec![
                "#.##..#..#.",
                "#...#......",
                "......#...#",
                "#...#......",
                ".#.#..#.###",
                "...........",
                "...........",
            ]
            .iter()
            .map(|l| (*l).to_owned())
            .collect();

            assert_eq!(lines_after_fold_1, pretty_print(&field));

            println!("Field:");
            for field_line in pretty_print(&field) {
                println!("{}", field_line);
            }

            assert_eq!(
                17,
                field
                    .iter()
                    .map(|r| r.iter().filter(|x| x.get()).count())
                    .sum::<usize>()
            );

            fold(&mut field, fold_instructions.get(1).unwrap());

            #[rustfmt::skip]
            let lines_after_fold_2: Vec<String> = vec![
                "#####",
                "#...#",
                "#...#",
                "#...#",
                "#####",
                ".....",
                ".....",
            ]
            .iter()
            .map(|l| (*l).to_owned())
            .collect();

            assert_eq!(lines_after_fold_2, pretty_print(&field));

            println!("Field:");
            for field_line in pretty_print(&field) {
                println!("{}", field_line);
            }
        }
    }

    mod part_2 {
        use crate::{
            day_13::Solution,
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            let solution: Vec<String> = vec![
                ".##....##..##..#..#.###...##..###..###..",
                "#..#....#.#..#.#.#..#..#.#..#.#..#.#..#.",
                "#.......#.#....##...###..#..#.#..#.###..",
                "#.......#.#....#.#..#..#.####.###..#..#.",
                "#..#.#..#.#..#.#.#..#..#.#..#.#....#..#.",
                ".##...##...##..#..#.###..#..#.#....###..",
            ]
            .iter()
            .map(|l| (*l).to_owned())
            .collect();

            assert_eq!((Solution {}).part_2(), PartSolution::Vec(solution));
        }
    }
}
