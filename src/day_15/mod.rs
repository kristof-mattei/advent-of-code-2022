use std::{cell::Cell, collections::HashMap};

use crate::shared::{Day, PartSolution};

type Chiton = (u32, Cell<bool>);
type Coordinates = (usize, usize);

fn parse_lines(lines: &[&str]) -> Vec<Vec<Chiton>> {
    let mut field = Vec::new();

    for line in lines {
        field.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u32)
                .map(|x| (x, Cell::new(false)))
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

fn distance(field: &[Vec<Chiton>], current: Coordinates, neighbor: Coordinates) -> u32 {
    // // intially I only had the neighbor's value here, but adding the current value increases
    // // variability and speeds up the algorithm
    field[current.0][current.1].0 + field[neighbor.0][neighbor.1].0
}

fn heuristic(field: &[Vec<Chiton>], current: Coordinates) -> u32 {
    // // intially I only had the neighbor's value here, but adding the current value increases
    // // variability and speeds up the algorithm
    // field[current.0][current.1] + field[neighbor.0][neighbor.1]

    ((field.len() - current.0) + (field[0].len() - current.1)) as u32
}

fn a_star(field: &mut [Vec<Chiton>], start: Coordinates, goal: Coordinates) -> Vec<Coordinates> {
    let mut open_set = Vec::from([(start, heuristic(field, start))]);

    let mut came_from = HashMap::<Coordinates, Coordinates>::new();

    let mut g_score = HashMap::from([(start, 0)]);

    let mut f_score = HashMap::from([(start, heuristic(field, start))]);

    while !open_set.is_empty() {
        open_set.sort_unstable_by(|(_, v), (_, v2)| v2.cmp(v));
        let current = open_set.pop().unwrap().0;
        // old way of getting current:

        // let current = *open_set
        //     .iter()
        //     .map(|os| (os, f_score.get(os)))
        //     .min_by(|(_, value1), (_, value2)| value1.cmp(value2))
        //     .unwrap()
        //     .0;

        // open_set.remove(&current);

        field[current.0][current.1].1.set(true);

        if current == goal {
            return reconstruct_path(&came_from, current);
        }

        let neighbors = get_neighbors(field, current);

        for neighbor in neighbors {
            let tentative_g_score =
                g_score.get(&current).unwrap() + distance(field, current, neighbor);

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, current);

                g_score.insert(neighbor, tentative_g_score);

                let g_score_with_heurisitc = tentative_g_score + heuristic(field, neighbor);
                f_score.insert(neighbor, g_score_with_heurisitc);

                // here we deviate from A*
                // we actually keep the latest the last f_score with it
                // but since we cannot use a HashSet (doesn't retain order)
                // or BinaryHeap (no random access)
                // we do some manual work
                // always remove the item if it exists
                // and push it again
                // we re-sort the set once we need to re-evaluate
                // and this is even better than say a binary heap, as that one sorts on every insert
                if let Some(position) = open_set.iter().position(|(c, _)| *c == neighbor) {
                    open_set.remove(position);
                }

                open_set.push((neighbor, g_score_with_heurisitc));

                // before the code looked like
                // open_set was a HashSet<_>
                // if !open_set.contains(&neighbor) {
                //     open_set.insert(neighbor);
                // }
                // and at the beginning of the loop we would find and sort them against f_score

                // the current version is much faster, as we
                // * retain the sorted elements
                // * defer resorting only when needed
            }
        }
    }

    panic!("No solution found")
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let mut parsed = parse_lines(&lines);

        let max_row = parsed.len() - 1;
        let max_col = parsed[0].len() - 1;

        let cheapest = a_star(&mut parsed, (0, 0), (max_row, max_col));

        dump_field(&parsed);

        PartSolution::U32(
            cheapest
                .iter()
                .skip(1)
                .map(|(r, c)| (parsed[*r][*c]).0)
                .sum::<u32>(),
        )
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let mut parsed = parse_lines(&lines);

        duplicate_x_times(&mut parsed, 4);

        let max_row = parsed.len() - 1;
        let max_col = parsed[0].len() - 1;

        let cheapest = a_star(&mut parsed, (0, 0), (max_row, max_col));

        PartSolution::U32(
            cheapest
                .iter()
                .skip(1)
                .map(|(r, c)| (parsed[*r][*c]).0)
                .sum::<u32>(),
        )
    }
}

fn roll_over_after_9(val: &mut u32) {
    *val += 1;

    if *val > 9 {
        *val = 1;
    }
}

fn dump_field(field: &[Vec<Chiton>]) {
    for r in field {
        for c in r {
            let color: u32 = if c.1.get() { 31 } else { 0 };
            print!("\x1b[{}m{}\x1b[0m", color, c.0);
        }

        println!();
    }
}

fn duplicate_x_times(original: &mut Vec<Vec<Chiton>>, times: u32) {
    for r in original.iter_mut() {
        let mut to_roll_over_and_re_insert = r.clone();

        for _ in 0..times {
            for f in &mut to_roll_over_and_re_insert {
                roll_over_after_9(&mut f.0);
            }

            let mut clone = to_roll_over_and_re_insert.clone();

            r.append(&mut clone);
        }
    }

    let mut to_roll_over_and_re_insert = original
        .iter()
        .map(|v| Vec::from_iter(v.clone()))
        .collect::<Vec<_>>();

    for _ in 0..times {
        // bump all numbers
        for inner in &mut to_roll_over_and_re_insert {
            for f in inner.iter_mut() {
                roll_over_after_9(&mut f.0);
            }
        }

        let mut clone = to_roll_over_and_re_insert.clone();

        original.append(&mut clone);
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt").lines().collect()
    }

    fn get_example_5x() -> Vec<&'static str> {
        include_str!("example 5x.txt").lines().collect()
    }

    mod part_1 {
        use crate::{
            day_15::{a_star, dump_field, parse_lines, Solution},
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

            let mut parsed = parse_lines(&lines);

            let max_row = parsed.len() - 1;
            let max_col = parsed[0].len() - 1;

            let cheapest = a_star(&mut parsed, (0, 0), (max_row, max_col));

            dump_field(&parsed);

            assert_eq!(
                40,
                cheapest
                    .iter()
                    .skip(1)
                    .map(|(r, c)| (parsed[*r][*c]).0)
                    .sum::<u32>()
            );
        }
    }

    mod part_2 {
        use super::{get_example, get_example_5x};
        use crate::{
            day_15::{a_star, duplicate_x_times, parse_lines, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U32(2907));
        }

        #[test]
        fn example() {
            let lines = get_example_5x();

            let mut parsed = parse_lines(&lines);

            let max_row = parsed.len() - 1;
            let max_col = parsed[0].len() - 1;

            let cheapest = a_star(&mut parsed, (0, 0), (max_row, max_col));

            assert_eq!(
                315,
                cheapest
                    .iter()
                    .skip(1)
                    .map(|(r, c)| (parsed[*r][*c]).0)
                    .sum::<u32>()
            );
        }
        #[test]
        fn test_duplication() {
            let lines = get_example();
            let lines_5x = get_example_5x();

            let mut parsed = parse_lines(&lines);

            let parsed_5x = parse_lines(&lines_5x);

            duplicate_x_times(&mut parsed, 4);

            assert_eq!(parsed, parsed_5x);
        }
    }
}
