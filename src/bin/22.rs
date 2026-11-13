use std::collections::BTreeMap;

use advent_of_code_2022::shared::{PartSolution, Parts};
use hashbrown::HashMap;
use hashbrown::hash_map::Entry;

advent_of_code_2022::solution!(67390_usize, 95291_usize);

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    /// .
    Open,
    /// #
    Solid,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Open => write!(f, "."),
            Self::Solid => write!(f, "#"),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Open,
            '#' => Tile::Solid,
            _ => {
                return Err("Invalid tile");
            },
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl std::ops::Add<Turn> for Facing {
    type Output = Facing;

    fn add(self, rhs: Turn) -> Self::Output {
        match self {
            Facing::Right => match rhs {
                Turn::Left => Facing::Up,
                Turn::Right => Facing::Down,
            },
            Facing::Down => match rhs {
                Turn::Left => Facing::Right,
                Turn::Right => Facing::Left,
            },
            Facing::Left => match rhs {
                Turn::Left => Facing::Down,
                Turn::Right => Facing::Up,
            },
            Facing::Up => match rhs {
                Turn::Left => Facing::Left,
                Turn::Right => Facing::Right,
            },
        }
    }
}

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Right,
}

impl std::fmt::Debug for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Left => write!(f, "L"),
            Self::Right => write!(f, "R"),
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Move(usize),
    Turn(Turn),
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Instruction::Move(count) => write!(f, "{}", count),
            Instruction::Turn(direction) => write!(f, "{:?}", direction),
        }
    }
}

struct ParseInputResult {
    tiles: BTreeMap<(usize, usize), Tile>,
    instructions: Vec<Instruction>,
    row_to_column_min_max: BTreeMap<usize, (usize, usize)>,
    column_to_row_min_max: BTreeMap<usize, (usize, usize)>,
}

fn parse_input(input: &&str) -> ParseInputResult {
    let mut tiles = BTreeMap::new();

    let mut lines = input.lines().enumerate();

    let mut row_to_column_min_max = BTreeMap::<usize, (usize, usize)>::new();
    let mut column_to_row_min_max = BTreeMap::<usize, (usize, usize)>::new();

    loop {
        let (row_index, line) = lines.next().expect("Always a next line in the Grid");

        if line.is_empty() {
            break;
        }

        for (column_index, tile) in line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| Tile::try_from(c).ok().map(|t| (i, t)))
        {
            row_to_column_min_max
                .entry(row_index)
                .and_modify(|&mut (ref mut min, ref mut max)| {
                    *min = (*min).min(column_index);
                    *max = (*max).max(column_index);
                })
                .or_insert((column_index, column_index));

            column_to_row_min_max
                .entry(column_index)
                .and_modify(|&mut (ref mut min, ref mut max)| {
                    *min = (*min).min(row_index);
                    *max = (*max).max(row_index);
                })
                .or_insert((row_index, row_index));

            tiles.insert((row_index, column_index), tile);
        }

        // parse tiles
    }

    let (_, instructions_line) = lines.next().expect("Instructions line");

    let mut instructions = Vec::new();

    let mut buffer = String::new();

    for c in instructions_line.chars() {
        if c.is_numeric() {
            buffer.push(c);
        } else {
            // 'L' or 'R'
            if !buffer.is_empty() {
                instructions.push(Instruction::Move(buffer.parse().unwrap()));
                buffer.clear();
            }

            instructions.push(match c {
                'L' => Instruction::Turn(Turn::Left),
                'R' => Instruction::Turn(Turn::Right),
                _ => panic!(),
            });
        }
    }

    if !buffer.is_empty() {
        instructions.push(Instruction::Move(buffer.parse().unwrap()));
    }

    ParseInputResult {
        tiles,
        instructions,
        row_to_column_min_max,
        column_to_row_min_max,
    }
}
use std::ops::RangeInclusive;

fn wrap_add(range: &RangeInclusive<usize>, x: usize, n: usize) -> usize {
    let min = *range.start();
    let len = range.end() - min + 1;
    min + (x - min + n % len) % len
}

fn wrap_sub(range: &RangeInclusive<usize>, x: usize, n: usize) -> usize {
    let min = *range.start();
    let len = range.end() - min + 1;
    min + (x - min + len - n % len) % len
}

fn follow_instructions(
    ParseInputResult {
        tiles,
        instructions,
        row_to_column_min_max,
        column_to_row_min_max,
    }: ParseInputResult,
) -> usize {
    let mut facing = Facing::Right;
    let mut row_index = 0_usize;
    let mut column_index = row_to_column_min_max[&0].0;

    for instruction in instructions {
        match instruction {
            Instruction::Move(count) => {
                for _ in 0..count {
                    let wrap = match facing {
                        Facing::Right | Facing::Down => wrap_add,
                        Facing::Left | Facing::Up => wrap_sub,
                    };

                    let next_position = match facing {
                        Facing::Right | Facing::Left => {
                            let (min, max) = row_to_column_min_max[&row_index];

                            (row_index, wrap(&(min..=max), column_index, 1))
                        },
                        Facing::Down | Facing::Up => {
                            let (min, max) = column_to_row_min_max[&column_index];

                            (wrap(&(min..=max), row_index, 1), column_index)
                        },
                    };

                    match tiles[&next_position] {
                        Tile::Solid => break,
                        Tile::Open => (row_index, column_index) = next_position,
                    }
                }
            },
            Instruction::Turn(turn) => {
                facing = facing + turn;
            },
        }
    }

    to_password(row_index, column_index, facing)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector3 {
    const X: Vector3 = Vector3 { x: 1, y: 0, z: 0 };
    const Y: Vector3 = Vector3 { x: 0, y: 1, z: 0 };
    const Z: Vector3 = Vector3 { x: 0, y: 0, z: 1 };

    fn dot(self, rhs: Vector3) -> i32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Mul<i32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Orientation of one face of the folded cube.
#[derive(Clone, Copy)]
struct Face {
    normal: Vector3,
    /// Direction of increasing column.
    right: Vector3,
    /// Direction of increasing row.
    down: Vector3,
}

impl Face {
    fn direction(self, facing: Facing) -> Vector3 {
        match facing {
            Facing::Right => self.right,
            Facing::Down => self.down,
            Facing::Left => -self.right,
            Facing::Up => -self.down,
        }
    }

    fn facing(self, direction: Vector3) -> Facing {
        [Facing::Right, Facing::Down, Facing::Left, Facing::Up]
            .into_iter()
            .find(|&facing| self.direction(facing) == direction)
            .expect("Direction lies in the plane of the face")
    }

    /// The adjacent face in `facing`, folded away from the viewer.
    fn fold(self, facing: Facing) -> Face {
        match facing {
            Facing::Right => Face {
                normal: self.right,
                right: -self.normal,
                down: self.down,
            },
            Facing::Down => Face {
                normal: self.down,
                right: self.right,
                down: -self.normal,
            },
            Facing::Left => Face {
                normal: -self.right,
                right: self.normal,
                down: self.down,
            },
            Facing::Up => Face {
                normal: -self.down,
                right: self.right,
                down: self.normal,
            },
        }
    }

    /// Coordinates are doubled, so the cube spans `-size..=size` and every tile center is an integer point.
    fn tile_center(self, size: usize, row_offset: usize, column_offset: usize) -> Vector3 {
        let size = i32::try_from(size).expect("Size fits in i32");

        let centered =
            |offset: usize| i32::try_from(2 * offset + 1).expect("Offset fits in i32") - size;

        self.normal * size + self.right * centered(column_offset) + self.down * centered(row_offset)
    }
}

fn neighbor_cell((row, column): (usize, usize), facing: Facing) -> Option<(usize, usize)> {
    match facing {
        Facing::Right => Some((row, column + 1)),
        Facing::Down => Some((row + 1, column)),
        Facing::Left => Some((row, column.checked_sub(1)?)),
        Facing::Up => Some((row.checked_sub(1)?, column)),
    }
}

/// Maps each face cell `(row / size, column / size)` of the net to its orientation on the cube.
fn fold_faces(
    tiles: &BTreeMap<(usize, usize), Tile>,
    size: usize,
) -> HashMap<(usize, usize), Face> {
    let &(first_row, first_column) = tiles.keys().next().expect("Map has tiles");
    let first_cell = (first_row / size, first_column / size);

    let mut faces = HashMap::from([(
        first_cell,
        Face {
            normal: Vector3::Z,
            right: Vector3::X,
            down: Vector3::Y,
        },
    )]);

    let mut pending = vec![first_cell];

    while let Some(cell) = pending.pop() {
        let face = faces[&cell];

        for facing in [Facing::Right, Facing::Down, Facing::Left, Facing::Up] {
            let Some(neighbor) = neighbor_cell(cell, facing) else {
                continue;
            };

            if !tiles.contains_key(&(neighbor.0 * size, neighbor.1 * size)) {
                continue;
            }

            if let Entry::Vacant(vacant) = faces.entry(neighbor) {
                vacant.insert(face.fold(facing));
                pending.push(neighbor);
            }
        }
    }

    faces
}

fn follow_instructions_on_cube(
    ParseInputResult {
        tiles,
        instructions,
        row_to_column_min_max,
        ..
    }: ParseInputResult,
) -> usize {
    let size = (tiles.len() / 6).isqrt();
    let edge = i32::try_from(size).expect("Size fits in i32");
    let faces = fold_faces(&tiles, size);

    let face_at = |row: usize, column: usize| faces[&(row / size, column / size)];

    let tiles_by_point = tiles
        .iter()
        .map(|(&(row, column), &tile)| {
            let point = face_at(row, column).tile_center(size, row % size, column % size);

            (point, (tile, (row, column)))
        })
        .collect::<HashMap<_, _>>();

    let start_column = row_to_column_min_max[&0].0;
    let start_face = face_at(0, start_column);

    let mut position = start_face.tile_center(size, 0, start_column % size);
    let mut normal = start_face.normal;
    let mut direction = start_face.right;

    for instruction in instructions {
        match instruction {
            Instruction::Move(count) => {
                for _ in 0..count {
                    let ahead = position + direction * 2;

                    let (next_position, next_direction, next_normal) =
                        if ahead.dot(direction) < edge {
                            (ahead, direction, normal)
                        } else {
                            // over the edge, onto the face whose normal is `direction`
                            (position + direction - normal, -normal, direction)
                        };

                    match tiles_by_point[&next_position].0 {
                        Tile::Solid => break,
                        Tile::Open => {
                            position = next_position;
                            direction = next_direction;
                            normal = next_normal;
                        },
                    }
                }
            },
            Instruction::Turn(Turn::Left) => {
                direction = direction.cross(normal);
            },
            Instruction::Turn(Turn::Right) => {
                direction = normal.cross(direction);
            },
        }
    }

    let (_, (row, column)) = tiles_by_point[&position];

    to_password(row, column, face_at(row, column).facing(direction))
}

fn to_password(row_index: usize, column_index: usize, facing: Facing) -> usize {
    ((row_index + 1) * 1000)
        + ((column_index + 1) * 4)
        + (match facing {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        })
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parse_input_result = parse_input(&input);

        let password = follow_instructions(parse_input_result);

        password.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parse_input_result = parse_input(&input);

        let password = follow_instructions_on_cube(parse_input_result);

        password.into()
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use advent_of_code_2022::shared::Parts as _;
        use advent_of_code_2022::shared::solution::read_file;
        use pretty_assertions::assert_eq;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                67390_usize,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(6032, (Solution {}).part_1(&read_file("examples", &DAY)));
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::Parts as _;
        use advent_of_code_2022::shared::solution::read_file;
        use pretty_assertions::assert_eq;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                95291_usize,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(5031, (Solution {}).part_2(&read_file("examples", &DAY)));
        }
    }
}
