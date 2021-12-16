use std::collections::{HashMap, HashSet};

use crate::shared::{Day, PartSolution};

type Chiton = u32;
type Coordinates = (usize, usize);

fn parse_lines(lines: &[String]) -> Vec<Vec<Chiton>> {
    let mut field = Vec::new();

    for line in lines {
        field.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u32)
                // .map(Cell::new)
                .collect(),
        );
    }

    field
}

fn get_neighbors<T>(chiton_field: &[Vec<T>], coordinates: Coordinates) -> Vec<Coordinates> {
    let mut neighbors = Vec::new();

    let (row_index, column_index) = coordinates;

    let rows = chiton_field.len();
    let columns = chiton_field.get(0).map(Vec::len).unwrap_or_default();

    let can_go_left = column_index > 0;
    let can_go_up = row_index > 0;

    let can_go_right = column_index + 1 < columns;
    let can_go_down = row_index + 1 < rows;

    // up
    if can_go_up {
        neighbors.push((row_index - 1, column_index));
    }

    // right
    if can_go_right {
        neighbors.push((row_index, column_index + 1));
    }

    // down
    if can_go_down {
        neighbors.push((row_index + 1, column_index));
    }

    // left
    if can_go_left {
        neighbors.push((row_index, column_index - 1));
    }

    neighbors
}

fn reconstruct_path(
    came_from: &HashMap<Coordinates, Coordinates>,
    mut current: Coordinates,
) -> Vec<Coordinates> {
    let mut total_path = vec![current];

    while let Some(c) = came_from.get(&current) {
        total_path.push(*c);

        current = *c;
    }

    total_path.reverse();
    total_path
}

fn d(field: &[Vec<u32>], current: Coordinates, neighbor: Coordinates) -> u32 {
    let (r, c) = current;

    field[r][c]
}

fn h(field: &[Vec<u32>], current: Coordinates) -> u32 {
    field[current.0][current.1]
}

fn a_star(field: &[Vec<u32>], start: Coordinates, goal: Coordinates) -> Vec<Coordinates> {
    let mut open_set = HashSet::<Coordinates>::from_iter(vec![start]);

    let mut came_from = HashMap::<Coordinates, Coordinates>::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, h(field, start));

    while !open_set.is_empty() {
        let current = *open_set
            .iter()
            .map(|os| (os, f_score.get(os)))
            .min_by(|(_, value1), (_, value2)| value1.cmp(value2))
            .unwrap()
            .0;

        println!("{:?}", current);

        if current == goal {
            return reconstruct_path(&came_from, current);
        }

        open_set.remove(&current);

        let neighbors = get_neighbors(field, current);

        for neighbor in neighbors {
            let tentative_g_score = g_score.get(&current).unwrap() + d(field, current, neighbor);

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, current);

                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(field, neighbor));

                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                }
            }
        }
    }

    panic!("No solution found")
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let parsed = parse_lines(&lines);
        let max_row = parsed.len() - 1;
        let max_col = parsed[0].len() - 1;

        let cheapest = a_star(&parsed, (0, 0), (max_row, max_col));

        PartSolution::U32(
            cheapest
                .iter()
                .skip(1)
                .map(|(r, c)| (parsed[*r][*c]))
                .sum::<u32>(),
        )
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let mut parsed = parse_lines(&lines);

        duplicate_x_times(&mut parsed, 5);

        let max_row = parsed.len() - 1;
        let max_col = parsed[0].len() - 1;

        let cheapest = a_star(&parsed, (0, 0), (max_row, max_col));

        PartSolution::U32(
            cheapest
                .iter()
                .skip(1)
                .map(|(r, c)| (parsed[*r][*c]))
                .sum::<u32>(),
        )
    }
}

fn duplicate_x_times(original: &mut Vec<Vec<u32>>, times: u32) {
    for r in original {
        let mut to_roll_over_and_re_insert = r.iter().copied().collect::<Vec<_>>();

        for _ in 0..times {
            to_roll_over_and_re_insert.iter_mut().for_each(|f| {
                *f += 1;
                if *f > 9 {
                    *f = 0
                }
            });

            let mut clone = to_roll_over_and_re_insert.clone();

            r.append(&mut clone);
        }
    }

    // let mut to_roll_over_and_re_insert = original
    //     .iter()
    //     .map(|v| Vec::from_iter(v.clone()))
    //     .collect::<Vec<_>>();

    // for _ in 0..times {
    //     for _ in 0..times {
    //         to_roll_over_and_re_insert.iter_mut().for_each(|inner| {
    //             inner.iter_mut().for_each(|f| {
    //                 *f += 1;
    //                 if *f > 9 {
    //                     *f = 0
    //                 }
    //             })
    //         })
    //     }
    // }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<String> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    fn get_example_5x() -> Vec<String> {
        include_str!("example 5x.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {
        use crate::{
            day_15::{a_star, parse_lines, Solution},
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(604));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            let max_row = parsed.len() - 1;
            let max_col = parsed[0].len() - 1;

            let cheapest = a_star(&parsed, (0, 0), (max_row, max_col));

            assert_eq!(
                40,
                cheapest
                    .iter()
                    .skip(1)
                    .map(|(r, c)| (parsed[*r][*c]))
                    .sum::<u32>()
            );
        }
    }

    mod part_2 {
        use crate::{
            day_15::{a_star, parse_lines, test::get_example_5x, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U32(604));
        }

        #[test]
        fn example() {
            let lines = get_example_5x();

            let parsed = parse_lines(&lines);

            let max_row = parsed.len() - 1;
            let max_col = parsed[0].len() - 1;

            let cheapest = a_star(&parsed, (0, 0), (max_row, max_col));

            assert_eq!(
                315,
                cheapest
                    .iter()
                    .skip(1)
                    .map(|(r, c)| (parsed[*r][*c]))
                    .sum::<u32>()
            );
        }
    }
}
