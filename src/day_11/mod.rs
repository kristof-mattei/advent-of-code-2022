use std::{cell::Cell, collections::HashSet};

use crate::shared::{Day, PartSolution};

#[derive(Default)]
struct Octopus {
    energy: u8,
}

fn parse_lines(lines: &[String]) -> Vec<Vec<Octopus>> {
    let mut field = Vec::new();

    for line in lines {
        field.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .map(|x| Octopus { energy: x })
                .collect(),
        );
    }

    field
}

// type Coordinates = (usize, usize);

// fn get_neighbors<T>(
//     heatmap: &[Vec<T>],
//     row_index: usize,
//     column_index: usize,
// ) -> HashSet<Coordinates> {
//     let mut coordinates = HashSet::new();

//     if column_index != 0 {
//         coordinates.insert((column_index - 1, row_index));
//     }

//     if column_index + 1 < heatmap[row_index].len() {
//         coordinates.insert((column_index + 1, row_index));
//     }

//     if row_index != 0 {
//         coordinates.insert((column_index, row_index - 1));
//     }

//     if row_index + 1 < heatmap.len() {
//         coordinates.insert((column_index, row_index + 1));
//     }

//     coordinates
// }

// fn visit_neighbors_that_are_not_nine(
//     heatmap: &[Vec<u32>],
//     row_index: usize,
//     column_index: usize,
//     mut visited_neighbors: HashSet<Coordinates>,
// ) -> HashSet<Coordinates> {
//     let neighbors = get_neighbors(heatmap, row_index, column_index);

//     for (x, y) in neighbors {
//         if heatmap[y][x] != 9 && !visited_neighbors.contains(&(x, y)) {
//             // let mut clone = visited_neighbors.clone();
//             visited_neighbors.insert((x, y));

//             visited_neighbors = visit_neighbors_that_are_not_nine(heatmap, y, x, visited_neighbors);
//         }
//     }

//     visited_neighbors
// }

// fn get_basins(heatmap: &[Vec<u32>], low_points: &[(usize, usize)]) -> Vec<Vec<u32>> {
//     let mut basins: Vec<Vec<u32>> = Vec::new();

//     for (column_index, row_index) in low_points {
//         let basin_coordinates = visit_neighbors_that_are_not_nine(
//             heatmap,
//             *row_index,
//             *column_index,
//             HashSet::<Coordinates>::new(),
//         );

//         let basin_values = basin_coordinates
//             .iter()
//             .map(|(x, y)| heatmap[*y][*x])
//             .collect();

//         println!("We started with low point {} at x: {}, y: {} and got a set of neighbors with values {:?}", heatmap[*row_index][*column_index], column_index, row_index,  basin_values);

//         basins.push(basin_values);
//     }

//     println!("Basins 1: {:?}", basins);

//     basins
// }

// fn visit_neighbors_that_are_not_nine_2(
//     heatmap: &[Vec<Octopus>],
//     row_index: usize,
//     column_index: usize,
// ) {
//     let neighbors = get_neighbors(heatmap, row_index, column_index);

//     for (x, y) in neighbors {
//         if heatmap[y][x].value != 9 && !heatmap[y][x].visisted.get() {
//             // let mut clone = visited_neighbors.clone();
//             heatmap[y][x].visisted.set(true);

//             visit_neighbors_that_are_not_nine_2(heatmap, y, x);
//         }
//     }
// }

// fn heatmap_u32_heatmap_cell(heatmap: &[Vec<u32>]) -> Vec<Vec<HeatMapCell>> {
//     let mut new_heatmap = Vec::new();

//     for line in heatmap {
//         new_heatmap.push(
//             line.iter()
//                 .map(|value| HeatMapCell {
//                     value: *value,
//                     ..HeatMapCell::default()
//                 })
//                 .collect(),
//         );
//     }

//     new_heatmap
// }

// fn get_visisted_values(visisted_heatmap: &[Vec<HeatMapCell>]) -> Vec<u32> {
//     let mut visited_values = Vec::new();

//     for line in visisted_heatmap {
//         for cell in line {
//             if cell.visisted.get() {
//                 visited_values.push(cell.value);
//             }
//         }
//     }

//     visited_values
// }

// fn get_basins_2(heatmap: &[Vec<u32>], low_points: &[(usize, usize)]) -> Vec<Vec<u32>> {
//     let mut basins: Vec<Vec<u32>> = Vec::new();

//     for (column_index, row_index) in low_points {
//         let visitable_heatmap = heatmap_u32_heatmap_cell(heatmap);

//         visit_neighbors_that_are_not_nine_2(&visitable_heatmap, *row_index, *column_index);

//         let basin_values = get_visisted_values(&visitable_heatmap);

//         println!("We started with low point {} at x: {}, y: {} and got a set of neighbors with values {:?}", heatmap[*row_index][*column_index], column_index, row_index,  basin_values);

//         basins.push(basin_values);
//     }

//     println!("Basins 2: {:?}", basins);

//     basins
// }

// fn calculate_basin_scores(basins: &[Vec<u32>]) -> Vec<usize> {
//     let mut basin_values_added: Vec<usize> = basins.iter().map(Vec::len).collect();

//     basin_values_added.sort_by(|a, b| b.cmp(a)); // largest to smallest

//     basin_values_added
// }

fn step(octopus_field: &[Vec<Octopus>]) -> u32 {
    0
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        // let heatmap = parse_lines(&lines);

        // let low_points = get_low_points(&heatmap);

        // let basins = get_basins(&heatmap, &low_points);
        // let basin_scores = calculate_basin_scores(&basins);

        // let basins_2 = get_basins_2(&heatmap, &low_points);
        // let basin_scores_2 = calculate_basin_scores(&basins_2);

        // assert_eq!(basin_scores, basin_scores_2);

        // PartSolution::USize(basin_scores.iter().take(3).product::<usize>())

        PartSolution::None
    }

    fn part_2(&self) -> PartSolution {
        PartSolution::USize(0)
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
            day_11::{parse_lines, step, Octopus, Solution},
            shared::{Day, PartSolution},
        };

        use super::get_example;

        fn to_vec_string(vec_str: &[&str]) -> Vec<String> {
            vec_str.iter().map(|x| (*x).to_owned()).collect()
        }

        fn back_to_vec_string(octopus_field: &[Vec<Octopus>]) -> Vec<String> {
            let mut lines = Vec::new();

            for octopus_line in octopus_field {
                lines.push(
                    octopus_line
                        .iter()
                        .map(|x| x.energy.to_string())
                        .collect::<Vec<_>>()
                        .join(""),
                );
            }

            lines
        }

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::USize(827_904));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let octopus_field = parse_lines(&lines);
        }

        #[test]
        fn example_step_by_step() {
            let lines: Vec<String> =
                to_vec_string(&vec!["11111", "19991", "19191", "19991", "11111"]);

            let octopus_field = parse_lines(&lines);

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
}
