use std::{cell::Cell, collections::HashSet};

use crate::shared::{Day, PartSolution};

fn parse_lines(lines: &[String]) -> Vec<Vec<u32>> {
    let mut field: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        field.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }

    field
}

fn calculate_risk_level(low_points: &[(usize, usize)], heatmap: &[Vec<u32>]) -> u32 {
    let mut low_point_values: Vec<u32> = Vec::new();

    for (x, y) in low_points {
        let low_point_value: u32 = heatmap[*y][*x];

        low_point_values.push(low_point_value);
    }

    low_point_values.iter().map(|x| x + 1).sum()
}

fn value_smaller_than_all_neighbors(
    value: u32,
    heatmap: &[Vec<u32>],
    neighbors: &HashSet<Coordinates>,
) -> bool {
    neighbors.iter().all(|(x, y)| heatmap[*y][*x] > value)
}

fn get_low_points(heatmap: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();

    for (row_index, row) in heatmap.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            let neighbors = get_neighbors(heatmap, row_index, column_index);

            if value_smaller_than_all_neighbors(*value, heatmap, &neighbors) {
                let neighbor_values: Vec<u32> =
                    neighbors.iter().map(|(x, y)| heatmap[*y][*x]).collect();
                println!(
                    "{} is smaller than all values in neighbors ({:?})",
                    *value, neighbor_values
                );
                low_points.push((column_index, row_index));
            }
        }
    }

    low_points
}

type Coordinates = (usize, usize);

fn get_neighbors<T>(
    heatmap: &[Vec<T>],
    row_index: usize,
    column_index: usize,
) -> HashSet<Coordinates> {
    let mut coordinates = HashSet::new();

    if column_index != 0 {
        coordinates.insert((column_index - 1, row_index));
    }

    if column_index + 1 < heatmap[row_index].len() {
        coordinates.insert((column_index + 1, row_index));
    }

    if row_index != 0 {
        coordinates.insert((column_index, row_index - 1));
    }

    if row_index + 1 < heatmap.len() {
        coordinates.insert((column_index, row_index + 1));
    }

    coordinates
}

fn visit_neighbors_that_are_not_nine(
    heatmap: &[Vec<u32>],
    row_index: usize,
    column_index: usize,
    mut visited_neighbors: HashSet<Coordinates>,
) -> HashSet<Coordinates> {
    let neighbors = get_neighbors(heatmap, row_index, column_index);

    for (x, y) in neighbors {
        if heatmap[y][x] != 9 && !visited_neighbors.contains(&(x, y)) {
            // let mut clone = visited_neighbors.clone();
            visited_neighbors.insert((x, y));

            visited_neighbors = visit_neighbors_that_are_not_nine(heatmap, y, x, visited_neighbors);
        }
    }

    visited_neighbors
}

fn get_basins(heatmap: &[Vec<u32>], low_points: &[(usize, usize)]) -> Vec<Vec<u32>> {
    let mut basins: Vec<Vec<u32>> = Vec::new();

    for (column_index, row_index) in low_points {
        let basin_coordinates = visit_neighbors_that_are_not_nine(
            heatmap,
            *row_index,
            *column_index,
            HashSet::<Coordinates>::new(),
        );

        let basin_values = basin_coordinates
            .iter()
            .map(|(x, y)| heatmap[*y][*x])
            .collect();

        println!("We started with low point {} at x: {}, y: {} and got a set of neighbors with values {:?}", heatmap[*row_index][*column_index], column_index, row_index,  basin_values);

        basins.push(basin_values);
    }

    println!("Basins 1: {:?}", basins);

    basins
}

#[derive(Default)]
struct HeatMapCell {
    value: u32,
    visisted: Cell<bool>,
}

fn visit_neighbors_that_are_not_nine_2(
    heatmap: &[Vec<HeatMapCell>],
    row_index: usize,
    column_index: usize,
) {
    let neighbors = get_neighbors(heatmap, row_index, column_index);

    for (x, y) in neighbors {
        if heatmap[y][x].value != 9 && !heatmap[y][x].visisted.get() {
            // let mut clone = visited_neighbors.clone();
            heatmap[y][x].visisted.set(true);

            visit_neighbors_that_are_not_nine_2(heatmap, y, x);
        }
    }
}

fn heatmap_u32_heatmap_cell(heatmap: &[Vec<u32>]) -> Vec<Vec<HeatMapCell>> {
    let mut new_heatmap = Vec::new();

    for line in heatmap {
        new_heatmap.push(
            line.iter()
                .map(|value| HeatMapCell {
                    value: *value,
                    ..HeatMapCell::default()
                })
                .collect(),
        );
    }

    new_heatmap
}

fn get_visisted_values(visisted_heatmap: &[Vec<HeatMapCell>]) -> Vec<u32> {
    let mut visited_values = Vec::new();

    for line in visisted_heatmap {
        for cell in line {
            if cell.visisted.get() {
                visited_values.push(cell.value);
            }
        }
    }

    visited_values
}

fn get_basins_2(heatmap: &[Vec<u32>], low_points: &[(usize, usize)]) -> Vec<Vec<u32>> {
    let mut basins: Vec<Vec<u32>> = Vec::new();

    for (column_index, row_index) in low_points {
        let visitable_heatmap = heatmap_u32_heatmap_cell(heatmap);

        visit_neighbors_that_are_not_nine_2(&visitable_heatmap, *row_index, *column_index);

        let basin_values = get_visisted_values(&visitable_heatmap);

        println!("We started with low point {} at x: {}, y: {} and got a set of neighbors with values {:?}", heatmap[*row_index][*column_index], column_index, row_index,  basin_values);

        basins.push(basin_values);
    }

    println!("Basins 2: {:?}", basins);

    basins
}

fn calculate_basin_scores(basins: &[Vec<u32>]) -> Vec<usize> {
    let mut basin_values_added: Vec<usize> = basins.iter().map(Vec::len).collect();

    basin_values_added.sort_by(|a, b| b.cmp(a)); // largest to smallest

    basin_values_added
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let heatmap = parse_lines(&lines);

        let low_points = get_low_points(&heatmap);

        PartSolution::U32(calculate_risk_level(&low_points, &heatmap))
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let heatmap = parse_lines(&lines);

        let low_points = get_low_points(&heatmap);

        let basins = get_basins(&heatmap, &low_points);
        let basin_scores = calculate_basin_scores(&basins);

        let basins_2 = get_basins_2(&heatmap, &low_points);
        let basin_scores_2 = calculate_basin_scores(&basins_2);

        assert_eq!(basin_scores, basin_scores_2);

        PartSolution::USize(basin_scores.iter().take(3).product::<usize>())
    }
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
            day_09::{calculate_risk_level, get_low_points, parse_lines, Solution},
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(585));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let heatmap = parse_lines(&lines);

            let low_points = get_low_points(&heatmap);

            assert_eq!(15, calculate_risk_level(&low_points, &heatmap));
        }
    }

    mod part_2 {

        use crate::{
            day_09::{
                calculate_basin_scores, get_basins, get_basins_2, get_low_points, parse_lines,
                Solution,
            },
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::USize(827_904));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let heatmap = parse_lines(&lines);

            let low_points = get_low_points(&heatmap);

            let basins = get_basins(&heatmap, &low_points);
            let basin_scores = calculate_basin_scores(&basins);

            let basins_2 = get_basins_2(&heatmap, &low_points);
            let basin_scores_2 = calculate_basin_scores(&basins_2);

            assert_eq!(basin_scores, basin_scores_2);

            assert_eq!(1134, basin_scores.iter().take(3).product::<usize>());
            assert_eq!(1134, basin_scores_2.iter().take(3).product::<usize>());
        }
    }
}
