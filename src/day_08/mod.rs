use std::collections::HashMap;

use crate::shared::{Day, PartSolution};

fn parse_lines(lines: &[&str]) -> Vec<Vec<usize>> {
    let mut field = vec![];
    for line in lines {
        let mut row = vec![];
        for byte in line.as_bytes() {
            row.push((byte - b'0').into());
        }
        field.push(row);
    }

    field
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

fn find_max(
    cache: &mut HashMap<(Direction, usize, usize), usize>,
    lines: &[Vec<usize>],
    direction: Direction,
    row: usize,
    column: usize,
) -> usize {
    if let Some(&cached) = cache.get(&(direction, row, column)) {
        return cached;
    }

    let max_in_direction = match direction {
        Direction::Top if row > 0 => find_max(cache, lines, Direction::Top, row - 1, column),
        Direction::Bottom if row + 1 < lines.len() => {
            find_max(cache, lines, Direction::Bottom, row + 1, column)
        },
        Direction::Left if column > 0 => find_max(cache, lines, Direction::Left, row, column - 1),
        Direction::Right if column + 1 < lines[row].len() => {
            find_max(cache, lines, Direction::Right, row, column + 1)
        },
        _ => 0,
    };

    let max = lines[row][column].max(max_in_direction);

    cache.insert((direction, row, column), max);

    max
}

fn count_visible_from_any_side(lines: &[Vec<usize>]) -> usize {
    let mut cache = HashMap::new();
    find_visible(&mut cache, lines)
}

fn find_visible(
    cache: &mut HashMap<(Direction, usize, usize), usize>,
    lines: &[Vec<usize>],
) -> usize {
    let mut count = 0;

    for row_index in 0..lines.len() {
        for col_index in 0..lines[row_index].len() {
            if row_index == 0
                || col_index == 0
                || row_index + 1 == lines.len()
                || col_index + 1 == lines[row_index].len()
            {
                count += 1;
            } else {
                let max_top = find_max(cache, lines, Direction::Top, row_index - 1, col_index);
                let max_bottom =
                    find_max(cache, lines, Direction::Bottom, row_index + 1, col_index);

                let max_left = find_max(cache, lines, Direction::Left, row_index, col_index - 1);
                let max_right = find_max(cache, lines, Direction::Right, row_index, col_index + 1);

                let min_any_size = max_top.min(max_bottom).min(max_left).min(max_right);

                if lines[row_index][col_index] > min_any_size {
                    count += 1;
                }
            }
        }
    }

    count
}

fn find_max_local_scenic_score(
    cache: &mut HashMap<(Direction, usize, usize), usize>,
    lines: &[Vec<usize>],
    direction: Direction,
    row: usize,
    column: usize,
    max: usize,
) -> usize {
    if lines[row][column] >= max {
        return 1;
    }

    // let to_jump = cache.get(&(direction, row, column)).copied().unwrap_or(0) + 1;
    let to_jump = cache.get(&(direction, row, column)).copied().unwrap_or(0) + 1;

    let max_in_direction = match direction {
        Direction::Top if row >= to_jump => {
            find_max_local_scenic_score(cache, lines, Direction::Top, row - to_jump, column, max)
        },
        Direction::Bottom if row + to_jump < lines.len() => {
            find_max_local_scenic_score(cache, lines, Direction::Bottom, row + to_jump, column, max)
        },
        Direction::Left if column >= to_jump => {
            find_max_local_scenic_score(cache, lines, Direction::Left, row, column - to_jump, max)
        },
        Direction::Right if column + to_jump < lines[row].len() => {
            find_max_local_scenic_score(cache, lines, Direction::Right, row, column + to_jump, max)
        },
        _ => 0,
    };

    // TODO cache?

    1 + max_in_direction
}

fn find_max_scenic_score(
    cache: &mut HashMap<(Direction, usize, usize), usize>,
    lines: &[Vec<usize>],
) -> usize {
    let mut max_scenic = 0;

    for row_index in 0..lines.len() {
        let mut dbg_output_1 = vec![];
        let mut dbg_output_2 = vec![];
        let mut dbg_output_3 = vec![];

        for col_index in 0..lines[row_index].len() {
            let scenic_top = (row_index > 0)
                .then(|| {
                    find_max_local_scenic_score(
                        cache,
                        lines,
                        Direction::Top,
                        row_index - 1,
                        col_index,
                        lines[row_index][col_index],
                    )
                })
                .unwrap_or(0);

            let scenic_bottom = (row_index + 1 < lines.len())
                .then(|| {
                    find_max_local_scenic_score(
                        cache,
                        lines,
                        Direction::Bottom,
                        row_index + 1,
                        col_index,
                        lines[row_index][col_index],
                    )
                })
                .unwrap_or(0);

            let scenic_left = (col_index > 0)
                .then(|| {
                    find_max_local_scenic_score(
                        cache,
                        lines,
                        Direction::Left,
                        row_index,
                        col_index - 1,
                        lines[row_index][col_index],
                    )
                })
                .unwrap_or(0);

            let scenic_right = (col_index + 1 < lines[row_index].len())
                .then(|| {
                    find_max_local_scenic_score(
                        cache,
                        lines,
                        Direction::Right,
                        row_index,
                        col_index + 1,
                        lines[row_index][col_index],
                    )
                })
                .unwrap_or(0);

            dbg_output_1.push(format!(
                " {}  ",
                nu_ansi_term::Color::Red.paint(scenic_top.to_string())
            ));
            dbg_output_2.push(format!(
                "{}{}{} ",
                nu_ansi_term::Color::Red.paint(scenic_left.to_string()),
                (lines[row_index][col_index]),
                nu_ansi_term::Color::Red.paint(scenic_right.to_string()),
            ));
            dbg_output_3.push(format!(
                " {}  ",
                nu_ansi_term::Color::Red.paint(scenic_bottom.to_string())
            ));

            // println!("row: {row_index}, column: {col_index}, val: {}, top scenic: {scenic_top}, bottom scenic: {scenic_bottom}, left scenic: {scenic_left}, right scenic: {scenic_right}", lines[row_index][col_index]);

            max_scenic = max_scenic.max(scenic_top * scenic_bottom * scenic_left * scenic_right);
        }

        println!("{}", dbg_output_1.into_iter().collect::<String>());
        println!("{}", dbg_output_2.into_iter().collect::<String>());
        println!("{}", dbg_output_3.into_iter().collect::<String>());
        println!();
    }

    max_scenic
}

fn max_scenic_score(lines: &[Vec<usize>]) -> usize {
    let mut cache = HashMap::new();
    find_max_scenic_score(&mut cache, lines)
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let parsed = parse_lines(&lines);

        let visible = count_visible_from_any_side(&parsed);

        visible.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let parsed = parse_lines(&lines);

        let max_scenic_score = max_scenic_score(&parsed);

        max_scenic_score.into()
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
        use super::{
            super::{count_visible_from_any_side, parse_lines, Solution},
            get_example,
        };
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(1688), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            let visible = count_visible_from_any_side(&parsed);

            assert_eq!(21, visible);
        }
    }

    mod part_2 {
        use super::{
            super::{parse_lines, Solution},
            get_example,
        };
        use crate::{
            day_08::max_scenic_score,
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(410_400), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            let max_scenic_score = max_scenic_score(&parsed);

            assert_eq!(8, max_scenic_score);
        }

        #[test]
        fn example_2() {
            let lines = ["22322", "32223", "22322"];

            let parsed = parse_lines(&lines);

            let max_scenic_score = max_scenic_score(&parsed);

            assert_eq!(1, max_scenic_score);
        }
    }
}
