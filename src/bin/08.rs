use std::collections::HashMap;

use advent_of_code_2022::shared::{PartSolution, Parts};

advent_of_code_2022::solution!(1688, 410_400);

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct RowColumn {
    row_index: usize,
    col_index: usize,
}

impl RowColumn {
    fn new(row_index: usize, col_index: usize) -> Self {
        Self {
            row_index,
            col_index,
        }
    }
}

struct Field {
    cells: Vec<Vec<usize>>,
}

impl Field {
    fn cell(
        &self,
        RowColumn {
            row_index,
            col_index,
        }: RowColumn,
    ) -> usize {
        self.cells[row_index][col_index]
    }
}

fn parse_lines(input: &str) -> Field {
    let mut cells = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for byte in line.as_bytes() {
            row.push((byte - b'0').into());
        }
        cells.push(row);
    }

    Field { cells }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::Left,
            Direction::Right,
            Direction::Top,
            Direction::Bottom,
        ]
    }
}

fn travel(
    direction: Direction,
    field: &Field,
    mut row_column: RowColumn,
    change: usize,
) -> Option<RowColumn> {
    match direction {
        Direction::Top if change <= row_column.row_index => row_column.row_index -= change,
        Direction::Bottom if row_column.row_index + change < field.cells.len() => {
            row_column.row_index += change;
        },
        Direction::Left if change <= row_column.col_index => row_column.col_index -= change,
        Direction::Right
            if row_column.col_index + change < field.cells[row_column.row_index].len() =>
        {
            row_column.col_index += change;
        },
        Direction::Left | Direction::Right | Direction::Top | Direction::Bottom => return None,
    }

    Some(row_column)
}

fn find_max_r(
    cache: &mut HashMap<(Direction, RowColumn), usize>,
    field: &Field,
    direction: Direction,
    row_column: RowColumn,
) -> usize {
    if let Some(&cached) = cache.get(&(direction, row_column)) {
        return cached;
    }

    let max_in_direction = travel(direction, field, row_column, 1).map_or(0, |new_row_column| {
        find_max_r(cache, field, direction, new_row_column)
    });

    let max = field.cell(row_column).max(max_in_direction);

    cache.insert((direction, row_column), max);

    max
}

fn count_visible_from_any_side(field: &Field) -> usize {
    let mut cache = HashMap::new();
    find_visible(&mut cache, field)
}

fn find_visible(cache: &mut HashMap<(Direction, RowColumn), usize>, field: &Field) -> usize {
    let mut count = 0;

    for row_index in 0..field.cells.len() {
        for col_index in 0..field.cells[row_index].len() {
            let row_column = RowColumn::new(row_index, col_index);
            let min_any_direction = Direction::all()
                .iter()
                .map(|direction| {
                    travel(*direction, field, row_column, 1)
                        .map(|traveled| find_max_r(cache, field, *direction, traveled))
                })
                .min()
                .unwrap();

            match min_any_direction {
                Some(min) if field.cell(row_column) <= min => {
                    // lowest any direction is higher than us, we're invisible :'(
                },
                _ => {
                    // None = edge, so visible
                    // Or all values are less than us, ergo we're visible
                    count += 1;
                },
            }
        }
    }

    count
}

fn max_scenic_score(field: &Field) -> usize {
    let mut cache = HashMap::new();
    find_max_scenic_score(&mut cache, field)
}

fn find_max_scenic_score(
    cache: &mut HashMap<(Direction, RowColumn), usize>,
    field: &Field,
) -> usize {
    let mut max_scenic = 0;

    for row_index in 0..field.cells.len() {
        for col_index in 0..field.cells[row_index].len() {
            let row_column = RowColumn::new(row_index, col_index);

            let scenics = Direction::all()
                .iter()
                .map(|direction| {
                    travel(*direction, field, row_column, 1).map_or(0, |next| {
                        find_scenic_score_r(cache, field, *direction, next, field.cell(row_column))
                    })
                })
                .product();

            max_scenic = max_scenic.max(scenics);
        }
    }

    max_scenic
}

fn find_scenic_score_r(
    cache: &mut HashMap<(Direction, RowColumn), usize>,
    field: &Field,
    direction: Direction,
    row_column: RowColumn,
    max: usize,
) -> usize {
    let value = field.cell(row_column);

    if value >= max {
        // we reached our local max
        return 1;
    }

    if let Some(&cached) = cache.get(&(direction, row_column)) {
        return cached;
    }

    let mut result = 0;

    loop {
        if let Some(change) = travel(direction, field, row_column, result + 1)
            .map(|traveled| find_scenic_score_r(cache, field, direction, traveled, value))
        {
            result += change;

            let new_row_column = travel(direction, field, row_column, result).unwrap();

            if field.cell(new_row_column) < max {
                continue;
            }
        }

        break;
    }

    cache.insert((direction, row_column), result + 1);

    result + 1
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let visible = count_visible_from_any_side(&parsed);

        visible.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let max_scenic_score = max_scenic_score(&parsed);

        max_scenic_score.into()
    }
}

#[cfg(test)]
mod tests {

    mod part_1 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts as _};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(1688),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(21),
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts as _};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(410_400),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(8),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }

        #[test]
        fn example_2() {
            let lines = ["22322", "32223", "22322"];

            // let parsed = parse_lines(&lines);

            // let max_scenic_score = max_scenic_score(&parsed);

            // assert_eq!(1, max_scenic_score);

            assert_eq!(
                PartSolution::USize(1),
                (Solution {}).part_2(&lines.join("\n"))
            );
        }
    }
}
