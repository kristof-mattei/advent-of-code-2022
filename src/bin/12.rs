use std::collections::{BinaryHeap, HashMap};

use advent_of_code_2022::shared::{PartSolution, Parts};

advent_of_code_2022::solution!(517, 512);

type Coordinates = (usize, usize);

#[derive(Clone)]
enum Direction {
    Ascending,
    Descending,
}

#[derive(PartialEq)]
enum Cell {
    Start,
    End,
    Value(u8),
}

impl Cell {
    fn elevation(&self) -> u8 {
        match *self {
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Value(v) => v,
        }
    }
}

fn parse_lines(input: &str) -> Vec<Vec<Cell>> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut parsed_line = Vec::<_>::new();

        for byte in line.bytes() {
            let v = match byte {
                b'S' => Cell::Start,
                b'E' => Cell::End,
                b @ (b'a'..=b'z') => {
                    // this way 'a' becomes '0'
                    Cell::Value(b - b'a')
                },
                _ => {
                    panic!("WTF?")
                },
            };

            parsed_line.push(v);
        }

        result.push(parsed_line);
    }

    result
}

fn find_start(field: &[Vec<Cell>]) -> (usize, usize) {
    for (row_index, row) in field.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            if Cell::Start == *value {
                return (row_index, column_index);
            }
        }
    }

    panic!("Start not found");
}

fn find_goal(field: &[Vec<Cell>]) -> (usize, usize) {
    for (row_index, row) in field.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            if Cell::End == *value {
                return (row_index, column_index);
            }
        }
    }

    panic!("Start not found");
}

fn find_shortest_distance(field: &[Vec<Cell>]) -> usize {
    let start = find_start(field);

    let r = a_star(field, start, &Direction::Ascending);

    // don't add the start position
    r.len() - 1
}

fn get_neighbors<'f>(
    field: &'f [Vec<Cell>],
    coordinates: &'f Coordinates,
    direction: &'f Direction,
) -> impl Iterator<Item = Coordinates> + 'f {
    let &(ref row_index, ref column_index) = coordinates;

    let transformations = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    transformations
        .into_iter()
        .filter_map(|(r, c)| {
            Some((
                row_index.checked_add_signed(r)?,
                column_index.checked_add_signed(c)?,
            ))
        })
        .filter(|&(r, c)| field.get(r).and_then(|row| row.get(c)).is_some())
        .filter(|&(r, c)| match *direction {
            Direction::Ascending => {
                field[r][c].elevation() <= field[*row_index][*column_index].elevation() + 1
            },
            Direction::Descending => {
                field[r][c].elevation() + 1 >= field[*row_index][*column_index].elevation()
            },
        })
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

fn distance(
    field: &[Vec<Cell>],
    current: Coordinates,
    neighbor: Coordinates,
    direction: &Direction,
) -> u32 {
    match *direction {
        Direction::Ascending => {
            match (field[current.0][current.1].elevation())
                .cmp(&(field[neighbor.0][neighbor.1].elevation()))
            {
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Equal => 10,
                std::cmp::Ordering::Greater => 100,
            }
        },
        Direction::Descending => distance(field, neighbor, current, &Direction::Ascending),
    }
}

fn heuristic(field: &[Vec<Cell>], current: Coordinates) -> u32 {
    u32::from(field[current.0][current.1].elevation())
}

#[derive(PartialEq, Eq)]
struct Node(Coordinates, u32);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(field: &[Vec<Cell>], start: Coordinates, direction: &Direction) -> Vec<Coordinates> {
    let mut open_set = BinaryHeap::from([Node(start, heuristic(field, start))]);

    let mut came_from = HashMap::<Coordinates, Coordinates>::new();

    let mut g_score = HashMap::from([(start, 0)]);

    while let Some(current) = open_set.pop() {
        let current = current.0;

        match *direction {
            Direction::Ascending => {
                if field[current.0][current.1] == Cell::End {
                    return reconstruct_path(&came_from, current);
                }
            },
            Direction::Descending => {
                if field[current.0][current.1] == Cell::Start
                    || field[current.0][current.1] == Cell::Value(0)
                {
                    return reconstruct_path(&came_from, current);
                }
            },
        }

        let neighbors = get_neighbors(field, &current, direction);

        for neighbor in neighbors {
            let tentative_g_score =
                g_score.get(&current).unwrap() + distance(field, current, neighbor, direction);

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, current);

                g_score.insert(neighbor, tentative_g_score);

                let g_score_with_heurisitc = tentative_g_score + heuristic(field, neighbor);

                open_set.push(Node(neighbor, g_score_with_heurisitc));
            }
        }
    }

    panic!("No solution found")
}

fn find_first_a_from_end(field: &[Vec<Cell>]) -> usize {
    let start = find_goal(field);

    let r = a_star(field, start, &Direction::Descending);

    // don't add the start position
    r.len() - 1
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let field = parse_lines(input);

        let result = find_shortest_distance(&field);

        result.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let field = parse_lines(input);

        let result = find_first_a_from_end(&field);

        result.into()
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
                PartSolution::USize(517),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(31),
                (Solution {}).part_1(&read_file("examples", &DAY))
            );

            // v..v<<<<
            // >v.vv<<^
            // .>vv>E^^
            // ..v>>>^^
            // ..>>>>>^

            // 31 steps
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts as _};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(512),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(29),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
