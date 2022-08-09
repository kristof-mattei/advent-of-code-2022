use std::{cell::Cell, collections::HashSet};

use crate::shared::{Day, PartSolution};

type Octopus = Cell<u8>;
type Coordinates = (usize, usize);

fn parse_lines(lines: &[&str]) -> Vec<Vec<Octopus>> {
    let mut field = Vec::new();

    for line in lines {
        field.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .map(Cell::new)
                .collect(),
        );
    }

    field
}

fn get_neighbors<T>(
    octopus_field: &[Vec<T>],
    row_index: usize,
    column_index: usize,
) -> HashSet<Coordinates> {
    let mut neighbors = HashSet::new();

    let rows = octopus_field.len();
    let columns = octopus_field.get(0).map(Vec::len).unwrap_or_default();

    // clockwise:
    // left up
    // up
    // right up
    // right
    // right down
    // bottom
    // left down
    // left

    let can_go_left = column_index > 0;
    let can_go_up = row_index > 0;

    let can_go_right = column_index + 1 < columns;
    let can_go_down = row_index + 1 < rows;

    // left up
    if can_go_left && can_go_up {
        neighbors.insert((row_index - 1, column_index - 1));
    }

    // up
    if can_go_up {
        neighbors.insert((row_index - 1, column_index));
    }

    // right up
    if can_go_up && can_go_right {
        neighbors.insert((row_index - 1, column_index + 1));
    }

    // right
    if can_go_right {
        neighbors.insert((row_index, column_index + 1));
    }

    // right down
    if can_go_down && can_go_right {
        neighbors.insert((row_index + 1, column_index + 1));
    }

    // down
    if can_go_down {
        neighbors.insert((row_index + 1, column_index));
    }

    // left down
    if can_go_down && can_go_left {
        neighbors.insert((row_index + 1, column_index - 1));
    }

    // left
    if can_go_left {
        neighbors.insert((row_index, column_index - 1));
    }

    neighbors
}

fn process_flash(octopus_field: &[Vec<Octopus>], row_index: usize, column_index: usize) -> u32 {
    let mut flashed = 0;
    let octopus = &octopus_field[row_index][column_index];

    if octopus.get() > 9 {
        octopus.set(0);

        let neighbors = get_neighbors(octopus_field, row_index, column_index);

        for (neightbor_row_index, neighbor_column_index) in neighbors {
            let neighbor_octopus = &octopus_field[neightbor_row_index][neighbor_column_index];

            let new_neighbor_energy = match neighbor_octopus.get() {
                0 => 0, // once flashed, we don't flash until next round
                x => x + 1,
            };

            neighbor_octopus.set(new_neighbor_energy);

            flashed += process_flash(octopus_field, neightbor_row_index, neighbor_column_index);
        }

        flashed += 1;
    }

    flashed
}

fn step(octopus_field: &[Vec<Octopus>]) -> u32 {
    for row in octopus_field {
        for octopus in row {
            let val = octopus.get();

            octopus.set(val + 1);
        }
    }

    let mut flashed: u32 = 0;

    for row_index in 0..octopus_field.len() {
        for column_index in 0..octopus_field[row_index].len() {
            flashed += process_flash(octopus_field, row_index, column_index);
        }
    }

    flashed
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let octopus_field = parse_lines(&lines);

        let mut flashes = 0;

        for _ in 0..100 {
            flashes += step(&octopus_field);
        }

        PartSolution::U32(flashes)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let octopus_field = parse_lines(&lines);
        let field_size =
            octopus_field.len() * octopus_field.get(0).map(Vec::len).unwrap_or_default();

        let mut steps: u32 = 0;
        loop {
            let flashes = step(&octopus_field);

            steps += 1;
            if flashes == field_size as u32 {
                return PartSolution::U32(steps);
            }
        }
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
            day_11::{parse_lines, step, Octopus, Solution},
            shared::{Day, PartSolution},
        };

        use super::get_example;

        fn back_to_vec_string(octopus_field: &[Vec<Octopus>]) -> Vec<String> {
            let mut lines = Vec::new();

            for octopus_line in octopus_field {
                lines.push(octopus_line.iter().map(|x| x.get().to_string()).collect());
            }

            lines
        }

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(1_755));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let octopus_field = parse_lines(&lines);

            let mut flashes = 0;

            for _ in 0..100 {
                flashes += step(&octopus_field);
            }

            assert_eq!(1656, flashes);
        }

        #[test]
        fn example_step_by_step() {
            let lines = &["11111", "19991", "19191", "19991", "11111"];

            let octopus_field = parse_lines(lines);

            let _ = step(&octopus_field);

            assert_eq!(
                vec!["34543", "40004", "50005", "40004", "34543"],
                back_to_vec_string(&octopus_field)
            );

            let _ = step(&octopus_field);

            assert_eq!(
                vec!["45654", "51115", "61116", "51115", "45654",],
                back_to_vec_string(&octopus_field)
            );
        }
    }

    mod part_2 {

        use crate::{
            day_11::{parse_lines, step, Solution},
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U32(212));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let octopus_field = parse_lines(&lines);

            let field_size =
                octopus_field.len() * octopus_field.get(0).map(Vec::len).unwrap_or_default();

            let mut steps: u32 = 0;
            loop {
                let flashes = step(&octopus_field);

                steps += 1;
                if flashes == field_size as u32 {
                    assert_eq!(steps, 195);
                    break;
                }
            }
        }
    }
}
